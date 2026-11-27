// Copyright 2026 Anud Inc.
// SPDX-License-Identifier: GPL-3.0-or-later

//! Tool calls in a reply against the scripted host: a row per call, the
//! approval card a call that acts waits on, Allow, Deny, and Stop while it
//! waits. Generic over the tool's name.

extern crate ui as gpui;

use conversation_host::{ConversationHost, ScriptedHost, ScriptedReply, ScriptedTool};
use conversation_model::schema::{Basis, Decision, ToolOutcome, Value};
use conversation_model::{InputState, Turn, TurnState};
use conversation_ui::{
    ConversationCopy, ConversationView, StatusButton, approval_card_id, tool_row_id,
};
use std::rc::Rc;
use ui::test::TestWindowExt as _;
use ui::{
    AppContext as _, Bounds, ElementId, Entity, TestAppContext, WindowBounds, WindowHandle,
    WindowOptions, base::Root, px, size,
};

/// A call's row, by the head a person presses to open it.
fn row(call: &conversation_model::schema::CallId) -> ElementId {
    ElementId::NamedChild(tool_row_id(call).into(), "head".into())
}

/// One of an approval card's buttons: `"approve"` or `"deny"`.
fn button(approval: &conversation_model::schema::ApprovalId, which: &str) -> ElementId {
    ElementId::NamedChild(approval_card_id(approval).into(), which.into())
}

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

fn json(text: &str) -> Value {
    serde_json::from_str(text).expect("json")
}

/// A call through a connection that only reads.
fn read_call() -> ScriptedTool {
    ScriptedTool::new(
        "connection_call",
        json(r#"{"connection":"Design files","operation":"list_files","input":{"folder":"Q4"}}"#),
        "Three files: plan, budget, notes.",
    )
}

/// A call through a connection that changes something.
fn act_call() -> ScriptedTool {
    ScriptedTool::new(
        "connection_call",
        json(
            r#"{"connection":"Tasks","operation":"create_task","input":{"title":"Book the room"}}"#,
        ),
        "Created.",
    )
    .asking("connection_call create_task")
}

impl Harness {
    fn send(&self, cx: &mut TestAppContext, prompt: &str) {
        cx.update(|cx| {
            self.view
                .update(cx, |view, cx| view.send(prompt.into(), cx))
        });
        cx.run_until_parked();
        self.host.finish();
        cx.run_until_parked();
    }

    fn last(&self, cx: &mut TestAppContext) -> Turn {
        cx.update(|cx| self.view.read(cx).thread().turns().last().cloned())
            .expect("a turn")
    }

    fn shows(&self, cx: &mut TestAppContext, id: ElementId) -> bool {
        cx.run_until_parked();
        cx.update_window(self.window.into(), |_, window, cx| {
            window.render_frame(cx);
            window.render_frame(cx);
            window.try_find(id).is_some()
        })
        .expect("the window is open")
    }

    fn click(&self, cx: &mut TestAppContext, id: ElementId) {
        cx.update_window(self.window.into(), |_, window, cx| {
            window.render_frame(cx);
            window.render_frame(cx);
            window.click(id, cx);
        })
        .expect("the window is open");
        cx.run_until_parked();
        self.host.finish();
        cx.run_until_parked();
    }

    fn stop_shown(&self, cx: &mut TestAppContext) -> bool {
        cx.update(|cx| self.view.read(cx).status_button(StatusButton::Stop))
            .is_some()
    }
}

#[ui::test]
fn a_read_shows_a_row_with_its_result_and_the_answer(cx: &mut TestAppContext) {
    let h = open(cx);
    h.host
        .script(ScriptedReply::text("You have three files.").with_tool(read_call()));
    h.send(cx, "What is in Q4?");

    let turn = h.last(cx);
    assert_eq!(turn.state, TurnState::Done);
    assert_eq!(turn.reply(), "You have three files.");
    let [tool] = turn.tools.as_slice() else {
        panic!("one call: {:?}", turn.tools);
    };
    assert_eq!(tool.outcome, Some(ToolOutcome::Ok));
    assert_eq!(tool.output, "Three files: plan, budget, notes.");
    assert_eq!(tool.target().connection.as_deref(), Some("Design files"));
    assert!(turn.approvals.is_empty(), "a read asks nobody");
    assert!(h.shows(cx, row(&tool.call)));
}

#[ui::test]
fn any_tool_name_gets_a_row(cx: &mut TestAppContext) {
    let h = open(cx);
    let other = ScriptedTool::new("lookup", json(r#"{"query":"tides"}"#), "High at six.");
    h.host
        .script(ScriptedReply::text("High tide is at six.").with_tool(other));
    h.send(cx, "When is high tide?");
    let turn = h.last(cx);
    assert_eq!(turn.tools[0].tool, "lookup");
    assert!(h.shows(cx, row(&turn.tools[0].call)));
}

#[ui::test]
fn an_action_waits_for_allow_then_runs(cx: &mut TestAppContext) {
    let h = open(cx);
    h.host
        .script(ScriptedReply::text("Booked it.").with_tool(act_call()));
    h.send(cx, "Add a task to book the room");

    // The turn waits: the card shows, Stop still does, nothing more is
    // written.
    let turn = h.last(cx);
    assert!(turn.state.is_open());
    assert_eq!(turn.reply(), "");
    let approval = turn.pending_approval().cloned().expect("an approval waits");
    assert_eq!(
        cx.update(|cx| h.view.read(cx).input_state()),
        InputState::Busy
    );
    assert!(h.stop_shown(cx));
    assert!(h.shows(cx, button(&approval.id, "approve")));
    assert!(h.shows(cx, button(&approval.id, "deny")));
    assert!(h.shows(cx, row(&turn.tools[0].call)));

    h.click(cx, button(&approval.id, "approve"));

    let turn = h.last(cx);
    assert_eq!(turn.state, TurnState::Done);
    assert_eq!(turn.reply(), "Booked it.");
    assert_eq!(turn.tools[0].outcome, Some(ToolOutcome::Ok));
    assert_eq!(
        turn.approvals[0].decision,
        Some((Decision::Allow, Basis::User))
    );
    assert!(
        !h.shows(cx, button(&approval.id, "approve")),
        "the card goes once it is answered"
    );
}

#[ui::test]
fn an_action_denied_is_not_run(cx: &mut TestAppContext) {
    let h = open(cx);
    h.host
        .script(ScriptedReply::text("I didn't add it.").with_tool(act_call()));
    h.send(cx, "Add a task to book the room");
    let approval = h.last(cx).pending_approval().cloned().expect("waits");

    h.click(cx, button(&approval.id, "deny"));

    let turn = h.last(cx);
    assert_eq!(turn.state, TurnState::Done);
    assert_eq!(turn.tools[0].outcome, Some(ToolOutcome::Denied));
    assert!(!turn.approvals[0].allowed());
    assert!(h.shows(cx, row(&turn.tools[0].call)));
}

#[ui::test]
fn stop_while_waiting_denies_and_stops(cx: &mut TestAppContext) {
    let h = open(cx);
    h.host
        .script(ScriptedReply::text("Never written.").with_tool(act_call()));
    h.send(cx, "Add a task to book the room");
    assert!(h.last(cx).pending_approval().is_some());

    cx.update(|cx| h.view.update(cx, |view, cx| view.stop(cx)));
    cx.run_until_parked();

    let turn = h.last(cx);
    assert_eq!(turn.state, TurnState::Stopped);
    assert_eq!(turn.tools[0].outcome, Some(ToolOutcome::Cancelled));
    assert_eq!(
        turn.approvals[0].decision,
        Some((Decision::Deny, Basis::Cancelled))
    );
    assert!(turn.pending_approval().is_none());
    assert!(!h.shows(cx, button(&turn.approvals[0].id, "deny")));
}
