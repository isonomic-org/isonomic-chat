// Copyright 2026 Anud Inc.
// SPDX-License-Identifier: GPL-3.0-or-later

//! An engine error part way through a reply: a failure the engine gives up
//! on keeps what was written and offers Try again; one it tries again
//! starts the answer over without doubling it.

extern crate ui as gpui;

use conversation_host::{ConversationHost, ScriptedHost, ScriptedReply};
use conversation_model::{InputState, TurnState};
use conversation_ui::{ConversationCopy, ConversationView, StatusButton};
use std::rc::Rc;
use ui::test::TestWindowExt as _;
use ui::{
    AppContext as _, Bounds, Entity, TestAppContext, WindowBounds, WindowHandle, WindowOptions,
    base::Root, px, size,
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
    Harness { window, view, host }
}

impl Harness {
    fn send(&self, cx: &mut TestAppContext, text: &str) {
        let text = text.to_owned();
        cx.update(|cx| self.view.update(cx, |view, cx| view.send(text, cx)));
        cx.run_until_parked();
    }

    fn pump(&self, cx: &mut TestAppContext, words: usize) {
        self.host.pump(words);
        cx.run_until_parked();
    }

    fn last(&self, cx: &mut TestAppContext) -> (String, TurnState) {
        cx.update(|cx| {
            let view = self.view.read(cx);
            let turn = view.thread().turns().last().cloned().expect("a turn");
            (turn.reply(), turn.state)
        })
    }

    fn shows(&self, cx: &mut TestAppContext, which: StatusButton) -> bool {
        cx.run_until_parked();
        cx.update_window(self.window.into(), |_, window, cx| {
            window.render_frame(cx);
            window.render_frame(cx);
            let id = self
                .view
                .read(cx)
                .status_button(which)
                .unwrap_or_else(|| ui::ElementId::from("not-shown"));
            window.try_find(id).is_some()
        })
        .expect("the window is open")
    }

    fn click(&self, cx: &mut TestAppContext, which: StatusButton) {
        cx.update_window(self.window.into(), |_, window, cx| {
            window.render_frame(cx);
            window.render_frame(cx);
            let id = self.view.read(cx).status_button(which).expect("shown");
            window.click(id, cx);
        })
        .expect("the window is open");
        cx.run_until_parked();
    }
}

#[ui::test]
fn a_failure_part_way_keeps_the_words_and_offers_try_again(cx: &mut TestAppContext) {
    let h = open(cx);
    h.host.script(
        ScriptedReply::text("One two three four five six.")
            .fail_after(3, "The model stopped responding."),
    );
    h.send(cx, "Count");
    h.pump(cx, 3);
    assert_eq!(h.last(cx).0, "One two three ");
    h.pump(cx, 1);

    let (reply, state) = h.last(cx);
    assert_eq!(
        state,
        TurnState::Failed("The model stopped responding.".into())
    );
    assert_eq!(reply.trim_end(), "One two three", "what was written stays");
    assert_eq!(
        cx.update(|cx| h.view.read(cx).input_state()),
        InputState::Ready
    );
    assert!(!h.host.is_busy());
    assert!(h.shows(cx, StatusButton::TryAgain));

    // Try again writes a whole answer.
    h.host
        .script(ScriptedReply::text("One two three four five six."));
    h.click(cx, StatusButton::TryAgain);
    h.host.finish();
    cx.run_until_parked();
    assert_eq!(
        h.last(cx),
        ("One two three four five six.".into(), TurnState::Done)
    );
}

#[ui::test]
fn a_passing_error_starts_the_answer_over_without_doubling_it(cx: &mut TestAppContext) {
    let h = open(cx);
    h.host.script(
        ScriptedReply::text("One two three four five six.").retry_after(2, "The model is busy."),
    );
    h.send(cx, "Count");
    h.pump(cx, 2);
    assert_eq!(h.last(cx).0, "One two ");
    h.pump(cx, 1);
    assert_eq!(
        h.last(cx).0,
        "",
        "the first try's words go when it starts over"
    );
    assert_eq!(
        cx.update(|cx| h.view.read(cx).input_state()),
        InputState::Busy
    );

    h.host.finish();
    cx.run_until_parked();
    assert_eq!(
        h.last(cx),
        ("One two three four five six.".into(), TurnState::Done)
    );
    assert!(!h.shows(cx, StatusButton::TryAgain));
}
