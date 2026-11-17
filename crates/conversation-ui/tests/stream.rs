// Copyright 2026 Anud Inc.
// SPDX-License-Identifier: GPL-3.0-or-later

//! A reply streamed one word per frame, as a real engine streams tokens,
//! stays drawn whole once a later turn starts.

extern crate ui as gpui;

use conversation_host::{ConversationHost, ScriptedHost, ScriptedReply};
use conversation_model::TurnState;
use conversation_ui::{ConversationCopy, ConversationView};
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
                    size: size(px(900.), px(420.)),
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
    fn frame<R>(
        &self,
        cx: &mut TestAppContext,
        f: impl FnOnce(&mut Window, &mut ui::App) -> R,
    ) -> R {
        cx.run_until_parked();
        cx.update_window(self.window.into(), |_, window, cx| {
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

    /// Streams the reply one word per drawn frame until it ends.
    fn stream_word_by_word(&self, cx: &mut TestAppContext) {
        while self.host.pump(1) {
            self.frame(cx, |_, _| ());
        }
        self.frame(cx, |_, _| ());
    }

    /// The text drawn inside reply `index`, joined.
    fn drawn_reply(&self, cx: &mut TestAppContext, index: u64) -> String {
        let id = ui::ElementId::NamedInteger("reply".into(), index);
        self.frame(cx, |window, _| {
            ui::base::test_support::texts(window)
                .into_iter()
                .filter(|text| text.path().contains(&id))
                .map(|text| text.text().to_owned())
                .collect::<Vec<_>>()
                .join(" ")
        })
    }

    fn reply(&self, cx: &mut TestAppContext, index: usize) -> (String, TurnState) {
        cx.update(|cx| {
            let thread = self.view.read(cx).thread();
            let turn = &thread.turns()[index];
            (turn.reply(), turn.state.clone())
        })
    }
}

const ITINERARY: &str = "Here is a one-day itinerary for Lisbon:\n\n\
    1. **Morning:** coffee and a pastry in Belem, then the monastery.\n\
    2. **Afternoon:** the tram up to Alfama and the castle walls.\n\
    3. **Evening:** dinner in Bairro Alto and the view from a miradouro.";

#[ui::test]
fn an_earlier_reply_stays_whole_when_a_later_turn_starts(cx: &mut TestAppContext) {
    let h = open(cx);
    h.host.script(ScriptedReply::text(ITINERARY));
    h.host.script(ScriptedReply::text("** Morning in Belem."));
    h.send(cx, "Plan one day in Lisbon");
    h.stream_word_by_word(cx);
    assert_eq!(h.reply(cx, 0), (ITINERARY.into(), TurnState::Done));
    let whole = h.drawn_reply(cx, 0);
    assert!(whole.contains("miradouro"), "drawn in full: {whole:?}");

    h.send(cx, "Plan one day in Lisbon");
    h.host.pump(1);
    h.frame(cx, |_, _| ());
    h.frame(cx, |window, cx| window.click("stop", cx));
    cx.run_until_parked();
    assert_eq!(h.reply(cx, 1).1, TurnState::Stopped);

    let after = h.drawn_reply(cx, 0);
    assert!(
        after.contains("miradouro") && after.contains("Alfama"),
        "turn 1 is still drawn whole after turn 2 starts: {after:?}"
    );
    // Stopped at its first piece, an opening `**`: no raw markup is drawn.
    assert_eq!(h.reply(cx, 1).0, "** ");
    let partial = h.drawn_reply(cx, 1);
    assert!(!partial.contains('*'), "no raw markup: {partial:?}");
}

impl Harness {
    /// Draws a frame without letting pending work finish first.
    fn raw_frame(&self, cx: &mut TestAppContext) {
        cx.update_window(self.window.into(), |_, window, cx| window.render_frame(cx))
            .expect("the window is open");
    }
}

/// As a fast engine streams: frames are drawn while the text view is still
/// parsing earlier pieces in the background.
#[ui::test]
fn frames_drawn_mid_parse_still_end_with_the_whole_reply(cx: &mut TestAppContext) {
    let h = open(cx);
    h.host.script(ScriptedReply::text(ITINERARY));
    h.host.script(ScriptedReply::text("**Morning** in Belem."));
    h.send(cx, "Plan one day in Lisbon");
    let mut step = 0usize;
    while h.host.pump(1) {
        step += 1;
        for _ in 0..(step % 4) + 1 {
            cx.executor().tick();
        }
        h.raw_frame(cx);
    }
    for _ in 0..3 {
        cx.executor().tick();
        h.raw_frame(cx);
    }
    let whole = h.drawn_reply(cx, 0);
    assert!(whole.contains("miradouro"), "drawn in full: {whole:?}");

    h.send(cx, "Plan one day in Lisbon");
    h.host.pump(1);
    cx.executor().tick();
    h.raw_frame(cx);
    h.frame(cx, |window, cx| window.click("stop", cx));
    cx.run_until_parked();
    let after = h.drawn_reply(cx, 0);
    assert!(
        after.contains("miradouro") && after.contains("Alfama"),
        "turn 1 is still drawn whole after turn 2 starts: {after:?}"
    );
}
