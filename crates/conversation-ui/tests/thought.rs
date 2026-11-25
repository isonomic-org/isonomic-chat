// Copyright 2026 Anud Inc.
// SPDX-License-Identifier: GPL-3.0-or-later

//! The reasoning row once a reply is written: it says how long the model
//! thought and opens to the reasoning. A reply without reasoning has none.

extern crate ui as gpui;

use conversation_host::{ConversationHost, ScriptedHost, ScriptedReply};
use conversation_ui::{ConversationCopy, ConversationView};
use std::rc::Rc;
use ui::test::TestWindowExt as _;
use ui::{
    AppContext as _, Bounds, ElementId, Entity, TestAppContext, WindowBounds, WindowHandle,
    WindowOptions, base::Root, px, size,
};

fn open(cx: &mut TestAppContext) -> (WindowHandle<Root>, Entity<ConversationView>, ScriptedHost) {
    cx.update(ui::init);
    let host = ScriptedHost::new();
    let (window, view) = cx.update(|cx| {
        let shared: Rc<dyn ConversationHost> = Rc::new(host.clone());
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
                cx.new(|cx| {
                    ConversationView::new(shared, "test", ConversationCopy::default(), window, cx)
                })
            },
        )
        .expect("open the test window");
        (window.downcast::<Root>().expect("a root view"), view)
    });
    (window, view, host)
}

fn row_shown(cx: &mut TestAppContext, window: WindowHandle<Root>) -> bool {
    cx.run_until_parked();
    cx.update_window(window.into(), |_, window, cx| {
        window.render_frame(cx);
        window.render_frame(cx);
        // The row is a disclosure button; its head is the part a person
        // presses.
        let row = ElementId::NamedInteger("thought-row".into(), 0);
        window
            .try_find(ElementId::NamedChild(row.into(), "head".into()))
            .is_some()
    })
    .expect("the window is open")
}

#[ui::test]
fn a_reply_with_reasoning_says_how_long_it_thought(cx: &mut TestAppContext) {
    let (window, view, host) = open(cx);
    host.script(ScriptedReply::text("Four.").with_reasoning("Two and two make four."));
    cx.update(|cx| view.update(cx, |view, cx| view.send("Two and two?".into(), cx)));
    cx.run_until_parked();
    host.finish();
    cx.run_until_parked();

    let turn = cx.update(|cx| {
        view.read(cx)
            .thread()
            .turns()
            .last()
            .cloned()
            .expect("a turn")
    });
    assert_eq!(turn.thinking(), "Two and two make four.");
    assert!(
        turn.thought_ms.is_some_and(|ms| ms > 0),
        "{:?}",
        turn.thought_ms
    );
    assert!(row_shown(cx, window));
}

#[ui::test]
fn a_reply_without_reasoning_has_no_row(cx: &mut TestAppContext) {
    let (window, view, host) = open(cx);
    host.script(ScriptedReply::text("Four."));
    cx.update(|cx| view.update(cx, |view, cx| view.send("Two and two?".into(), cx)));
    cx.run_until_parked();
    host.finish();
    cx.run_until_parked();

    let turn = cx.update(|cx| {
        view.read(cx)
            .thread()
            .turns()
            .last()
            .cloned()
            .expect("a turn")
    });
    assert_eq!(turn.reply(), "Four.");
    assert_eq!(turn.thought_ms, None);
    assert!(!row_shown(cx, window));
}

#[test]
fn the_time_reads_in_whole_seconds_then_minutes() {
    let copy = ConversationCopy::default();
    assert_eq!(copy.thought_for(None), "Thought");
    assert_eq!(copy.thought_for(Some(0)), "Thought for 1 second");
    assert_eq!(copy.thought_for(Some(1_000)), "Thought for 1 second");
    assert_eq!(copy.thought_for(Some(1_200)), "Thought for 2 seconds");
    assert_eq!(copy.thought_for(Some(59_000)), "Thought for 59 seconds");
    assert_eq!(copy.thought_for(Some(60_000)), "Thought for 1 minute");
    assert_eq!(copy.thought_for(Some(150_000)), "Thought for 3 minutes");
}
