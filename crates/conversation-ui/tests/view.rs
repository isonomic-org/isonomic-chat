//! Headless tests of the conversation view against the scripted host: send,
//! stream, stop, continue, refusal and a refused send.

extern crate ui as gpui;

use conversation_host::{ConversationHost, ScriptedHost, ScriptedReply};
use conversation_model::{InputState, TurnState};
use conversation_ui::{ConversationCopy, ConversationEvent, ConversationView};
use std::cell::RefCell;
use std::rc::Rc;
use ui::test::TestWindowExt as _;
use ui::{
    AppContext as _, Bounds, Entity, TestAppContext, Window, WindowBounds, WindowHandle,
    WindowOptions, base::Root, px, size,
};

struct Harness {
    window: WindowHandle<Root>,
    view: Entity<ConversationView>,
    host: ScriptedHost,
    events: Rc<RefCell<Vec<ConversationEvent>>>,
}

fn open(cx: &mut TestAppContext) -> Harness {
    open_with_models(cx, conversation_model::ModelCatalog::default())
}

fn open_with_models(cx: &mut TestAppContext, models: conversation_model::ModelCatalog) -> Harness {
    cx.update(ui::init);
    let host = ScriptedHost::new();
    host.set_models(models);
    let events = Rc::new(RefCell::new(Vec::new()));
    let (window, view) = cx.update(|cx| {
        let shared: Rc<dyn ConversationHost> = Rc::new(host.clone());
        let log = events.clone();
        let (window, view) = ui::open_window(
            WindowOptions {
                window_bounds: Some(WindowBounds::Windowed(Bounds {
                    origin: Default::default(),
                    size: size(px(900.), px(800.)),
                })),
                ..Default::default()
            },
            cx,
            move |window, cx| {
                let view = cx.new(|cx| {
                    ConversationView::new(shared, "test", ConversationCopy::default(), window, cx)
                });
                cx.subscribe(&view, move |_, event: &ConversationEvent, _| {
                    log.borrow_mut().push(event.clone())
                })
                .detach();
                view
            },
        )
        .expect("open the test window");
        (window.downcast::<Root>().expect("a root view"), view)
    });
    Harness {
        window,
        view,
        host,
        events,
    }
}

impl Harness {
    fn frame<R>(
        &self,
        cx: &mut TestAppContext,
        f: impl FnOnce(&mut Window, &mut ui::App) -> R,
    ) -> R {
        cx.run_until_parked();
        cx.update_window(self.window.into(), |_, window, cx| {
            // The list measures new rows on one frame and paints them on the next.
            window.render_frame(cx);
            window.render_frame(cx);
            f(window, cx)
        })
        .expect("the window is open")
    }

    fn send(&self, cx: &mut TestAppContext, text: &str) {
        let text = text.to_owned();
        cx.update(|cx| self.view.update(cx, |view, cx| view.send(text, cx)));
        cx.run_until_parked();
    }

    fn pump(&self, cx: &mut TestAppContext, words: usize) {
        self.host.pump(words);
        cx.run_until_parked();
    }

    fn state(&self, cx: &mut TestAppContext) -> InputState {
        cx.update(|cx| self.view.read(cx).input_state())
    }

    fn last(&self, cx: &mut TestAppContext) -> (String, TurnState) {
        cx.update(|cx| {
            let view = self.view.read(cx);
            let thread = view.thread();
            let turn = thread.turns().last().expect("a turn");
            (turn.reply(), turn.state.clone())
        })
    }
}

#[ui::test]
fn a_reply_streams_in_and_the_composer_comes_back(cx: &mut TestAppContext) {
    let h = open(cx);
    h.host
        .script(ScriptedReply::text("Three good names follow.").with_title("Book club names"));
    h.send(cx, "Name my book club");

    assert_eq!(h.state(cx), InputState::Busy);
    assert!(matches!(
        h.events.borrow().first(),
        Some(ConversationEvent::SessionStarted(_))
    ));

    h.pump(cx, 2);
    assert_eq!(h.last(cx), ("Three good ".into(), TurnState::Running));
    h.frame(cx, |window, _| assert!(window.try_find("stop").is_some()));

    h.host.finish();
    cx.run_until_parked();
    assert_eq!(
        h.last(cx),
        ("Three good names follow.".into(), TurnState::Done)
    );
    assert_eq!(h.state(cx), InputState::Ready);
    assert!(
        h.events
            .borrow()
            .contains(&ConversationEvent::TitleChanged("Book club names".into()))
    );
    h.frame(cx, |window, _| assert!(window.try_find("send").is_some()));
}

#[ui::test]
fn stop_keeps_the_partial_reply_and_continue_finishes_it(cx: &mut TestAppContext) {
    let h = open(cx);
    h.host.script(ScriptedReply::text(
        "Welcome, everyone. Tonight we pick our first book.",
    ));
    h.send(cx, "Write a welcome");
    h.pump(cx, 2);

    h.frame(cx, |window, cx| window.click("stop", cx));
    cx.run_until_parked();
    assert_eq!(
        h.last(cx),
        ("Welcome, everyone. ".into(), TurnState::Stopped)
    );
    assert_eq!(h.state(cx), InputState::Continuable);

    h.frame(cx, |window, cx| window.click("conversation-continue", cx));
    cx.run_until_parked();
    assert_eq!(h.state(cx), InputState::Busy);

    h.host.finish();
    cx.run_until_parked();
    let (reply, state) = h.last(cx);
    assert_eq!(reply, "Welcome, everyone. Tonight we pick our first book.");
    assert_eq!(state, TurnState::Done);
    let turns = cx.update(|cx| h.view.read(cx).thread().turns().len());
    assert_eq!(turns, 1, "the continued reply stays in its turn");
}

#[ui::test]
fn reasoning_shows_before_the_answer(cx: &mut TestAppContext) {
    let h = open(cx);
    h.host
        .script(ScriptedReply::text("Yes.").with_reasoning("Check the date first."));
    h.send(cx, "Is it Sunday?");
    h.pump(cx, 2);
    let (reply, _) = h.last(cx);
    assert!(reply.is_empty());
    let thinking = cx.update(|cx| h.view.read(cx).thread().turns()[0].thinking());
    assert_eq!(thinking, "Check the ");
    h.frame(cx, |window, _| {
        assert!(window.try_find("stop").is_some());
        assert!(window.try_find("conversation-continue").is_none());
    });
    h.host.finish();
    cx.run_until_parked();
    assert_eq!(h.last(cx), ("Yes.".into(), TurnState::Done));
}

#[ui::test]
fn a_refusal_ends_the_turn_with_its_message(cx: &mut TestAppContext) {
    let h = open(cx);
    h.host
        .script(ScriptedReply::refusal("I can't help with that."));
    h.send(cx, "Something not allowed");
    h.host.finish();
    cx.run_until_parked();
    assert_eq!(
        h.last(cx).1,
        TurnState::Refused("I can't help with that.".into())
    );
    assert_eq!(h.state(cx), InputState::Ready);
}

#[ui::test]
fn a_send_the_host_refuses_shows_why(cx: &mut TestAppContext) {
    let h = open(cx);
    // Nothing scripted: the host refuses the send.
    h.send(cx, "Hello");
    assert_eq!(
        h.last(cx).1,
        TurnState::Rejected("Nothing is scripted to reply.".into())
    );
    assert_eq!(h.state(cx), InputState::Ready);
}

#[ui::test]
fn a_second_prompt_follows_the_first(cx: &mut TestAppContext) {
    let h = open(cx);
    h.host.script(ScriptedReply::text("One."));
    h.host.script(ScriptedReply::text("Two."));
    h.send(cx, "First");
    h.host.finish();
    cx.run_until_parked();
    h.send(cx, "Second");
    h.host.finish();
    cx.run_until_parked();
    let replies: Vec<String> = cx.update(|cx| {
        h.view
            .read(cx)
            .thread()
            .turns()
            .iter()
            .map(|turn| turn.reply())
            .collect()
    });
    assert_eq!(replies, vec!["One.".to_owned(), "Two.".to_owned()]);
    assert_eq!(h.host.sessions().len(), 1);
}

fn catalog() -> conversation_model::ModelCatalog {
    use conversation_model::{Availability, ModelCatalog, ModelOption};
    let option = |id: &str, availability| ModelOption {
        id: id.into(),
        name: id.into(),
        remote: false,
        availability,
    };
    ModelCatalog::new(
        vec![
            option("small", Availability::Installed),
            option("large", Availability::NotInstalled),
        ],
        Some("large".into()),
    )
}

#[ui::test]
fn the_picked_model_goes_to_the_new_session(cx: &mut TestAppContext) {
    let h = open_with_models(cx, catalog());
    cx.run_until_parked();
    // Until a pick, the catalog's choice: the recommended model is not
    // installed, so the installed one.
    assert_eq!(
        cx.update(|cx| h.view.read(cx).model().map(str::to_owned)),
        Some("small".into())
    );
    cx.update(|cx| {
        h.view
            .update(cx, |view, cx| view.select_model("large".into(), cx))
    });
    assert!(
        h.events
            .borrow()
            .contains(&ConversationEvent::ModelSelected("large".into()))
    );
    h.host.script(ScriptedReply::text("Hi."));
    h.send(cx, "Hello");
    assert_eq!(h.host.created()[0].model.as_deref(), Some("large"));
}

#[ui::test]
fn model_errors_read_in_plain_words(cx: &mut TestAppContext) {
    let copy = ConversationCopy::default();
    for (code, words) in [
        ("model_unavailable", copy.model_unavailable.clone()),
        ("unknown_model", copy.model_unknown.clone()),
        ("capability_denied", copy.model_denied.clone()),
    ] {
        let h = open_with_models(cx, catalog());
        h.host
            .refuse_create(conversation_host::HostError::new(code, "wire words"));
        h.send(cx, "Hello");
        assert_eq!(
            h.last(cx).1,
            TurnState::Rejected(words.to_string()),
            "{code}"
        );
        if code == "model_unavailable" {
            assert!(
                h.events
                    .borrow()
                    .contains(&ConversationEvent::ModelUnavailable("small".into()))
            );
        }
    }
}

#[ui::test]
fn a_refused_send_can_be_tried_again(cx: &mut TestAppContext) {
    let h = open(cx);
    // Nothing scripted: the host refuses, and try again appears.
    h.send(cx, "Hello");
    assert!(matches!(h.last(cx).1, TurnState::Rejected(_)));
    h.frame(cx, |window, _| {
        assert!(window.try_find("conversation-retry").is_some())
    });

    h.host.script(ScriptedReply::text("Hi there."));
    h.frame(cx, |window, cx| window.click("conversation-retry", cx));
    cx.run_until_parked();
    h.host.finish();
    cx.run_until_parked();
    let turns = cx.update(|cx| h.view.read(cx).thread().turns().len());
    assert_eq!(turns, 1, "the rejected send is replaced, not repeated");
    assert_eq!(h.last(cx), ("Hi there.".into(), TurnState::Done));
    h.frame(cx, |window, _| {
        assert!(window.try_find("conversation-retry").is_none())
    });
}

#[ui::test]
fn a_dropped_stream_reconnects_and_misses_nothing(cx: &mut TestAppContext) {
    let h = open(cx);
    h.host
        .script(ScriptedReply::text("One two three four five six."));
    h.send(cx, "Count");
    h.pump(cx, 2);
    h.host.disconnect();
    cx.run_until_parked();
    assert!(cx.update(|cx| h.view.read(cx).is_reconnecting()));

    // The reply finishes while the view is away.
    h.host.finish();
    cx.executor()
        .advance_clock(std::time::Duration::from_secs(1));
    cx.run_until_parked();
    assert!(!cx.update(|cx| h.view.read(cx).is_reconnecting()));
    assert_eq!(
        h.last(cx),
        ("One two three four five six.".into(), TurnState::Done)
    );
    assert_eq!(h.state(cx), InputState::Ready);
}

#[ui::test]
fn stop_at_the_first_delta_then_continue_finishes_in_the_same_turn(cx: &mut TestAppContext) {
    let h = open(cx);
    h.host.script(ScriptedReply::text("First."));
    h.host.script(ScriptedReply::text(
        "Welcome everyone, tonight we pick a book.",
    ));
    h.send(cx, "Hi");
    h.host.finish();
    cx.run_until_parked();

    h.send(cx, "Write a welcome");
    h.pump(cx, 1);
    assert_eq!(h.last(cx), ("Welcome ".into(), TurnState::Running));
    h.frame(cx, |window, cx| window.click("stop", cx));
    cx.run_until_parked();
    assert_eq!(h.last(cx), ("Welcome ".into(), TurnState::Stopped));

    h.frame(cx, |window, cx| window.click("conversation-continue", cx));
    cx.run_until_parked();
    assert_eq!(
        h.state(cx),
        InputState::Busy,
        "continue opens the turn again"
    );
    h.host.finish();
    cx.run_until_parked();
    assert_eq!(
        h.last(cx),
        (
            "Welcome everyone, tonight we pick a book.".into(),
            TurnState::Done
        )
    );
    let turns = cx.update(|cx| h.view.read(cx).thread().turns().len());
    assert_eq!(turns, 2);
    // Exactly one cancel and one continue went out for that turn.
    let log = h.host.events(&h.host.sessions()[0]);
    let ends: Vec<_> = log
        .iter()
        .filter_map(|e| match &e.event {
            conversation_model::schema::Event::TurnEnd(end) => Some(end.stop.clone()),
            _ => None,
        })
        .collect();
    use conversation_model::schema::TurnStop;
    assert_eq!(ends, [TurnStop::Done, TurnStop::Cancelled, TurnStop::Done]);
}

#[ui::test]
fn a_refused_continue_shows_why_on_the_turn(cx: &mut TestAppContext) {
    let h = open(cx);
    h.host
        .script(ScriptedReply::text("Welcome everyone, tonight."));
    h.send(cx, "Write a welcome");
    h.pump(cx, 1);
    h.frame(cx, |window, cx| window.click("stop", cx));
    cx.run_until_parked();
    h.host.refuse_resume(conversation_host::HostError::new(
        "continue_refused",
        "The reply is complete.",
    ));
    h.frame(cx, |window, cx| window.click("conversation-continue", cx));
    cx.run_until_parked();
    let refusal = cx.update(|cx| h.view.read(cx).thread().turns()[0].refusal.clone());
    assert_eq!(refusal.as_deref(), Some("The reply is complete."));
    assert!(h.events.borrow().contains(&ConversationEvent::HostError(
        "The reply is complete.".into()
    )));
    // Continuing again clears it once the host opens the turn.
    h.frame(cx, |window, cx| window.click("conversation-continue", cx));
    cx.run_until_parked();
    h.host.finish();
    cx.run_until_parked();
    let turn = cx.update(|cx| h.view.read(cx).thread().turns()[0].clone());
    assert!(turn.refusal.is_none());
    assert_eq!(turn.reply(), "Welcome everyone, tonight.");
}

#[ui::test]
fn a_reply_cut_at_the_limit_offers_continue(cx: &mut TestAppContext) {
    let h = open(cx);
    h.host
        .script(ScriptedReply::text("One two three four five.").cut_after(2));
    h.send(cx, "Count");
    h.host.finish();
    cx.run_until_parked();
    assert_eq!(h.last(cx), ("One two ".into(), TurnState::Truncated));
    assert_eq!(h.state(cx), InputState::Continuable);
    h.frame(cx, |window, cx| window.click("conversation-continue", cx));
    cx.run_until_parked();
    h.host.finish();
    cx.run_until_parked();
    assert_eq!(
        h.last(cx),
        ("One two three four five.".into(), TurnState::Done)
    );
}

#[ui::test]
fn a_compaction_is_a_quiet_divider_not_a_message(cx: &mut TestAppContext) {
    let h = open(cx);
    h.host.script(ScriptedReply::text("Done."));
    h.send(cx, "Hello");
    h.host.finish();
    cx.run_until_parked();
    let session = h.host.sessions()[0].clone();
    h.host
        .compact(&session, "Earlier messages are not shown to the model.");
    cx.run_until_parked();
    cx.update(|cx| {
        let view = h.view.read(cx);
        let thread = view.thread();
        assert_eq!(thread.turns().len(), 1, "no new message");
        assert_eq!(
            thread.turns()[0].divider_after.as_deref(),
            Some("Earlier messages are not shown to the model.")
        );
        assert_eq!(thread.turns()[0].reply(), "Done.");
    });
}

#[ui::test]
fn a_conversation_too_large_shows_the_reason_and_keeps_the_composer(cx: &mut TestAppContext) {
    let h = open(cx);
    let reason = "This conversation is too long for the model. Start a new chat to keep going.";
    h.host.script(ScriptedReply::too_large(reason));
    h.send(cx, "One more question");
    assert_eq!(h.last(cx).1, TurnState::Failed(reason.into()));
    assert_eq!(h.state(cx), InputState::Ready, "the composer stays usable");
    h.frame(cx, |window, _| {
        assert!(window.try_find("send").is_some());
        assert!(
            window.try_find("conversation-retry").is_none(),
            "the same prompt would not fit either"
        );
    });
    let retry = cx.update(|cx| {
        h.view
            .update(cx, |view, _| view.thread().turns()[0].error_code.clone())
    });
    assert_eq!(retry.as_deref(), Some(conversation_model::TOO_LARGE));
}
