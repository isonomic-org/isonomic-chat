//! The thread model against the harness's recorded fixtures.

use conversation_model::schema::{DeltaKind, Event};
use conversation_model::schema::{LiveDelta, SessionEvent, SessionId, TurnId};
use conversation_model::{InputState, Thread, TurnState, Update};
use harness_testkit::fixtures;

/// The events of one session in a fixture.
fn session(events: &[SessionEvent], id: &str) -> Vec<SessionEvent> {
    events
        .iter()
        .filter(|event| event.session.as_str() == id)
        .cloned()
        .collect()
}

#[test]
fn every_fixture_folds_and_ends_closed_or_ready() {
    for fixture in fixtures::all() {
        let mut sessions: Vec<SessionId> = Vec::new();
        for event in &fixture.events {
            if !sessions.contains(&event.session) {
                sessions.push(event.session.clone());
            }
        }
        for id in sessions {
            let events = session(&fixture.events, id.as_str());
            let thread = Thread::from_events(&events);
            assert_eq!(
                thread.last_seq(),
                events.last().unwrap().seq,
                "{}",
                fixture.name
            );
            for turn in thread.turns() {
                assert!(
                    !turn.state.is_open(),
                    "{}: open turn {:?}",
                    fixture.name,
                    turn.ids
                );
            }
            assert_ne!(thread.input_state(), InputState::Busy, "{}", fixture.name);
        }
    }
}

#[test]
fn chat_basic_reads_as_prompts_and_replies() {
    let fixture = fixtures::chat_basic();
    let events = session(&fixture.events, "ses_0001");
    let thread = Thread::from_events(&events);
    let turns = thread.turns();

    assert!(thread.title().is_some());
    assert_eq!(
        turns[0].prompt,
        "What are three good names for a neighborhood book club?"
    );
    assert!(turns[0].reply().contains("The Porch Readers"));
    // The title step runs after the reply; its text is never in the turn.
    assert!(
        turns[0].reply().ends_with("in a particular tone?"),
        "{:?}",
        turns[0].reply()
    );
    assert!(!turns[0].thinking().is_empty());
    assert_eq!(turns[0].state, TurnState::Done);

    // Stop then continue: one exchange, the reply carried on in place.
    let stopped = turns
        .iter()
        .find(|turn| turn.ids.len() == 2)
        .expect("the continued turn spans two host turns");
    assert!(
        stopped
            .reply()
            .contains("Tonight we'll get to know each other")
    );
    assert_eq!(stopped.state, TurnState::Done);

    let refused = turns
        .iter()
        .find(|turn| matches!(turn.state, TurnState::Refused(_)))
        .expect("a refusal");
    let TurnState::Refused(message) = &refused.state else {
        unreachable!()
    };
    assert!(message.starts_with("I can't help"));
}

#[test]
fn a_stopped_reply_can_be_continued_until_the_continue_arrives() {
    let fixture = fixtures::chat_basic();
    let events = session(&fixture.events, "ses_0001");
    let cut = events
        .iter()
        .position(|e| matches!(&e.event, Event::TurnStart(s) if s.cause == conversation_model::schema::Cause::Continue))
        .unwrap();
    let thread = Thread::from_events(&events[..cut]);
    assert_eq!(thread.input_state(), InputState::Continuable);
    assert_eq!(thread.turns().last().unwrap().state, TurnState::Stopped);
}

#[test]
fn deltas_stream_until_the_durable_reply_replaces_them() {
    let fixture = fixtures::chat_basic();
    let events = session(&fixture.events, "ses_0001");
    let reply_at = events
        .iter()
        .position(|e| matches!(e.event, Event::Assistant(_)))
        .unwrap();
    let mut thread = Thread::from_events(&events[..reply_at]);
    assert_eq!(thread.input_state(), InputState::Busy);

    let reply = &events[reply_at];
    let delta = |text: &str| LiveDelta {
        session: reply.session.clone(),
        turn: reply.turn.clone().unwrap(),
        step: reply.step.clone().unwrap(),
        after_seq: reply.seq - 1,
        block: 1,
        kind: DeltaKind::Text,
        text: text.into(),
    };
    thread.apply(&Update::Delta(delta("Here ")));
    thread.apply(&Update::Delta(delta("are")));
    assert_eq!(thread.turns()[0].reply(), "Here are");

    thread.apply(&Update::from(reply.clone()));
    assert!(thread.turns()[0].reply().starts_with("Here are three:"));
    // A late delta for a committed step is dropped.
    thread.apply(&Update::Delta(delta(" late")));
    assert!(!thread.turns()[0].reply().contains("late"));
}

#[test]
fn replayed_events_are_ignored() {
    let fixture = fixtures::chat_basic();
    let events = session(&fixture.events, "ses_0001");
    let once = Thread::from_events(&events);
    let mut twice = once.clone();
    for event in &events {
        twice.apply_event(event);
    }
    assert_eq!(once, twice);
}

#[test]
fn an_optimistic_send_is_adopted_by_the_host_turn() {
    let fixture = fixtures::chat_basic();
    let events = session(&fixture.events, "ses_0001");
    let first_turn = events
        .iter()
        .position(|e| matches!(e.event, Event::TurnStart(_)))
        .unwrap();
    let mut thread = Thread::from_events(&events[..first_turn]);
    thread.begin_send("What are three good names?");
    assert_eq!(thread.turns()[0].state, TurnState::Sending);
    assert_eq!(thread.input_state(), InputState::Busy);

    thread.apply_event(&events[first_turn]);
    thread.apply_event(&events[first_turn + 1]);
    assert_eq!(thread.turns().len(), 1);
    assert_eq!(thread.turns()[0].state, TurnState::Running);
    assert_eq!(thread.turns()[0].ids, vec![TurnId::new("trn_0001")],);
}

#[test]
fn a_refused_send_shows_why() {
    let mut thread = Thread::new();
    thread.begin_send("Hello");
    thread.fail_send("A reply is still being written.");
    assert_eq!(
        thread.turns()[0].state,
        TurnState::Rejected("A reply is still being written.".into())
    );
    assert_eq!(thread.input_state(), InputState::Ready);
}

#[test]
fn tool_calls_and_unknown_events_are_kept_generically() {
    let fixture = fixtures::tool_heavy();
    let thread = Thread::from_events(&fixture.events);
    let tools: usize = thread.turns().iter().map(|turn| turn.tools.len()).sum();
    assert!(tools >= 3);
    assert!(
        thread
            .turns()
            .iter()
            .flat_map(|turn| &turn.tools)
            .all(|tool| tool.outcome.is_some())
    );
}

#[test]
fn a_transient_error_shows_as_retrying_until_the_reply_arrives() {
    let fixture = fixtures::tool_heavy();
    let retry_at = fixture
        .events
        .iter()
        .position(|e| matches!(&e.event, Event::Attempt(a) if a.will_retry))
        .expect("the fixture retries a transient error");
    let mut thread = Thread::from_events(&fixture.events[..=retry_at]);
    let turn = thread.turns().last().unwrap();
    assert!(turn.retrying.is_some());
    assert!(turn.state.is_open());
    for event in &fixture.events[retry_at + 1..] {
        thread.apply_event(event);
    }
    assert!(thread.turns().iter().all(|t| t.retrying.is_none()));
}

#[test]
fn only_a_failed_or_rejected_last_turn_can_be_tried_again() {
    let mut thread = Thread::new();
    thread.begin_send("Hello");
    assert_eq!(thread.take_retry(), None, "a send in flight is not retried");
    thread.fail_send("Not connected to the engine.");
    assert_eq!(thread.take_retry().as_deref(), Some("Hello"));
    assert!(
        thread.turns().is_empty(),
        "the rejected local send is dropped"
    );

    let fixture = fixtures::chat_basic();
    let events = session(&fixture.events, "ses_0001");
    let mut done = Thread::from_events(&events);
    assert_eq!(
        done.take_retry(),
        None,
        "a refusal or a done turn is not retried"
    );
}

#[test]
fn awaiting_client_follows_the_session_status() {
    let fixture = fixtures::client_gone();
    let gone_at = fixture
        .events
        .iter()
        .position(|e| matches!(&e.event, Event::SessionStatus(s) if s.status == conversation_model::schema::Status::AwaitingClient))
        .expect("the fixture loses its client");
    assert!(Thread::from_events(&fixture.events[..=gone_at]).awaiting_client());
    assert!(!Thread::from_events(&fixture.events).awaiting_client());
}

#[test]
fn the_compaction_in_autonomous_run_is_a_divider_not_a_turn() {
    let fixture = fixtures::autonomous_run();
    let at = fixture
        .events
        .iter()
        .position(|e| matches!(&e.event, Event::Compacted(c) if c.summary.is_some()))
        .expect("the fixture summarizes day 1");
    let session = fixture.events[at].session.clone();
    let events = session_of(&fixture.events, &session);
    let before = Thread::from_events(events.iter().take_while(|e| e.seq < fixture.events[at].seq));
    let after = Thread::from_events(
        events
            .iter()
            .take_while(|e| e.seq <= fixture.events[at].seq),
    );
    assert_eq!(before.turns().len(), after.turns().len());
    let divided =
        after.turns().iter().any(|t| t.divider_after.is_some()) || after.divider().is_some();
    assert!(divided);
}

fn session_of(events: &[SessionEvent], id: &SessionId) -> Vec<SessionEvent> {
    events
        .iter()
        .filter(|e| &e.session == id)
        .cloned()
        .collect()
}
