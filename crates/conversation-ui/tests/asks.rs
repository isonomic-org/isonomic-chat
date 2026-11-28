// Copyright 2026 Anud Inc.
// SPDX-License-Identifier: GPL-3.0-or-later

//! A question the model asks the person (`ask_user`): its card above the
//! composer, the reply through the host, a new message while it waits, and
//! the asked call's row once answered.

extern crate ui as gpui;

use conversation_host::{ConversationHost, FixtureHost, ScriptedHost, ScriptedReply, ScriptedTool};
use conversation_model::schema::QuestionId;
use conversation_model::{AskState, Reply, Turn, TurnState};
use conversation_ui::{ConversationCopy, ConversationView, StatusButton, tool_row_id};
use harness_testkit::fixtures;
use std::rc::Rc;
use ui::test::TestWindowExt as _;
use ui::{
    AppContext as _, Bounds, ElementId, Entity, TestAppContext, WindowBounds, WindowHandle,
    WindowOptions, base::Root, px, size,
};

struct Harness {
    window: WindowHandle<Root>,
    view: Entity<ConversationView>,
    host: ScriptedHost,
}

fn open(cx: &mut TestAppContext) -> Harness {
    cx.update(ui::init);
    let host = ScriptedHost::new();
    let (window, view) = cx.update(|cx| {
        let shared: Rc<dyn ConversationHost> = Rc::new(host.clone());
        let (window, view) = ui::open_window(
            WindowOptions {
                window_bounds: Some(WindowBounds::Windowed(Bounds {
                    origin: Default::default(),
                    size: size(px(900.), px(900.)),
                })),
                ..Default::default()
            },
            cx,
            move |window, cx| {
                cx.new(|cx| {
                    ConversationView::new(shared, "test", ConversationCopy::default(), window, cx)
                })
            },
        )
        .expect("open the test window");
        (window.downcast::<Root>().expect("a root view"), view)
    });
    Harness { window, view, host }
}

fn room() -> ScriptedTool {
    ScriptedTool::question(
        "Which room should I book?",
        &[("small", "The small room"), ("big", "The big room")],
    )
}

impl Harness {
    fn settle(&self, cx: &mut TestAppContext) {
        cx.run_until_parked();
        self.host.finish();
        cx.run_until_parked();
        cx.update_window(self.window.into(), |_, window, cx| {
            window.render_frame(cx);
            window.render_frame(cx);
        })
        .expect("the window is open");
        cx.run_until_parked();
    }

    fn send(&self, cx: &mut TestAppContext, prompt: &str) {
        cx.update(|cx| {
            self.view
                .update(cx, |view, cx| view.send(prompt.into(), cx))
        });
        self.settle(cx);
    }

    fn answer(&self, cx: &mut TestAppContext, reply: Reply) {
        cx.update(|cx| self.view.update(cx, |view, cx| view.answer(reply, cx)));
        self.settle(cx);
    }

    fn turns(&self, cx: &mut TestAppContext) -> Vec<Turn> {
        cx.update(|cx| self.view.read(cx).thread().turns().to_vec())
    }

    fn shows(&self, cx: &mut TestAppContext, id: ElementId) -> bool {
        self.settle(cx);
        cx.update_window(self.window.into(), |_, window, _| {
            window.try_find(id).is_some()
        })
        .expect("the window is open")
    }

    fn status(&self, cx: &mut TestAppContext) -> Option<String> {
        cx.update(|cx| {
            self.view
                .read(cx)
                .status_message()
                .map(|message| message.to_string())
        })
    }
}

#[ui::test]
fn a_question_waits_for_the_person_and_their_pick_goes_back(cx: &mut TestAppContext) {
    let h = open(cx);
    h.host
        .script(ScriptedReply::text("Booked the small room.").with_tool(room()));
    h.send(cx, "Book a room for Friday");

    // The turn waits on the question: its card shows above the composer,
    // the status says so, Stop still shows, and nothing has been sent.
    let ask = cx
        .update(|cx| h.view.read(cx).open_ask())
        .expect("a question waits");
    assert_eq!(ask.prompt, "Which room should I book?");
    assert!(h.shows(cx, ElementId::Name("ask-card".into())));
    assert_eq!(h.status(cx).as_deref(), Some("Waiting for your answer"));
    assert!(
        cx.update(|cx| h.view.read(cx).status_button(StatusButton::Stop))
            .is_some()
    );
    assert!(h.host.answers().is_empty(), "nothing sent for the person");

    h.answer(
        cx,
        Reply::Answered {
            selected: vec!["small".into()],
            text: None,
        },
    );
    let session = h.host.sessions()[0].clone();
    assert_eq!(
        h.host.answers(),
        [(
            session,
            ask.id.clone(),
            Reply::Answered {
                selected: vec!["small".into()],
                text: None
            }
        )]
    );
    let turn = h.turns(cx).pop().expect("a turn");
    assert_eq!(turn.state, TurnState::Done);
    assert_eq!(turn.reply(), "Booked the small room.");
    assert_eq!(
        turn.asks[0].answer_text().as_deref(),
        Some("The small room")
    );
    // The card has gone; the asked call's row stays in the thread.
    assert!(!h.shows(cx, ElementId::Name("ask-card".into())));
    let call = turn.tools[0].call.clone();
    assert!(h.shows(
        cx,
        ElementId::NamedChild(tool_row_id(&call).into(), "head".into())
    ));
}

#[ui::test]
fn a_new_message_moves_on_then_is_sent(cx: &mut TestAppContext) {
    let h = open(cx);
    h.host
        .script(ScriptedReply::text("I'll leave the room for now.").with_tool(room()))
        .script(ScriptedReply::text("Lunch is at one."));
    h.send(cx, "Book a room for Friday");
    let ask = cx
        .update(|cx| h.view.read(cx).open_ask())
        .expect("a question");

    h.send(cx, "Actually, when is lunch?");
    // The held message goes once the asking turn has ended.
    h.settle(cx);
    let session = h.host.sessions()[0].clone();
    assert_eq!(h.host.answers(), [(session, ask.id, Reply::MovedOn)]);
    let turns = h.turns(cx);
    assert_eq!(turns.len(), 2, "{turns:?}");
    assert_eq!(turns[0].asks[0].state, AskState::Unanswered);
    assert_eq!(turns[1].prompt, "Actually, when is lunch?");
    assert_eq!(turns[1].reply(), "Lunch is at one.");
}

#[ui::test]
fn stop_while_it_waits_sends_no_answer(cx: &mut TestAppContext) {
    let h = open(cx);
    h.host
        .script(ScriptedReply::text("Booked.").with_tool(room()));
    h.send(cx, "Book a room");
    cx.update(|cx| h.view.update(cx, |view, cx| view.stop(cx)));
    h.settle(cx);
    assert!(h.host.answers().is_empty());
    assert!(cx.update(|cx| h.view.read(cx).open_ask()).is_none());
}

#[ui::test]
fn a_recorded_answer_shows_on_its_row_and_cannot_be_answered_again(cx: &mut TestAppContext) {
    let fixture = fixtures::approval();
    let host = FixtureHost::new(&fixture.events);
    let session = host.sessions()[0].clone();
    let shared: Rc<dyn ConversationHost> = Rc::new(host.clone());
    cx.update(ui::init);
    let view = cx.update(|cx| {
        let (_, view) = ui::open_window(WindowOptions::default(), cx, move |window, cx| {
            cx.new(|cx| {
                ConversationView::open(shared, session, ConversationCopy::default(), window, cx)
            })
        })
        .expect("open the test window");
        view
    });
    cx.run_until_parked();
    let asks: Vec<_> = cx.update(|cx| {
        view.read(cx)
            .thread()
            .turns()
            .iter()
            .flat_map(|turn| turn.asks.clone())
            .collect()
    });
    assert!(!asks.is_empty(), "the fixture asks a question");
    assert!(asks.iter().all(|ask| !ask.is_open()));
    let refused = futures::executor::block_on(host.answer(
        &host.sessions()[0],
        &QuestionId::new("anything"),
        Reply::Skipped,
    ));
    assert!(refused.is_err(), "a recording is read only");
}
