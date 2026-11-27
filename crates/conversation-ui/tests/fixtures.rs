// Copyright 2026 Anud Inc.
// SPDX-License-Identifier: GPL-3.0-or-later

//! The conversation view against every recorded fixture, opened whole and
//! replayed word by word: both end on the same thread, every tool call shows
//! its row, and the view never panics or leaves a turn open.

extern crate ui as gpui;

use conversation_host::{ConversationHost, FixtureHost};
use conversation_model::schema::SessionId;
use conversation_model::{Thread, TurnState};
use conversation_ui::{ConversationCopy, ConversationView, tool_row_id};
use harness_testkit::fixtures;
use std::rc::Rc;
use ui::test::TestWindowExt as _;
use ui::{
    AppContext as _, Bounds, Entity, TestAppContext, WindowBounds, WindowHandle, WindowOptions,
    base::Root, px, size,
};

fn open(
    cx: &mut TestAppContext,
    host: &FixtureHost,
    session: &SessionId,
) -> (WindowHandle<Root>, Entity<ConversationView>) {
    open_sized(cx, host, session, 800.)
}

fn open_sized(
    cx: &mut TestAppContext,
    host: &FixtureHost,
    session: &SessionId,
    height: f32,
) -> (WindowHandle<Root>, Entity<ConversationView>) {
    cx.update(ui::init);
    let shared: Rc<dyn ConversationHost> = Rc::new(host.clone());
    let session = session.clone();
    let (window, view) = cx.update(|cx| {
        let (window, view) = ui::open_window(
            WindowOptions {
                window_bounds: Some(WindowBounds::Windowed(Bounds {
                    origin: Default::default(),
                    size: size(px(900.), px(height)),
                })),
                ..Default::default()
            },
            cx,
            move |window, cx| {
                cx.new(|cx| {
                    ConversationView::open(shared, session, ConversationCopy::default(), window, cx)
                })
            },
        )
        .expect("open the test window");
        (window.downcast::<Root>().expect("a root view"), view)
    });
    cx.run_until_parked();
    (window, view)
}

fn frame(cx: &mut TestAppContext, window: WindowHandle<Root>) {
    cx.update_window(window.into(), |_, window, cx| window.render_frame(cx))
        .expect("the window is open");
}

fn shows(cx: &mut TestAppContext, window: WindowHandle<Root>, id: ui::ElementId) -> bool {
    cx.run_until_parked();
    cx.update_window(window.into(), |_, window, cx| {
        window.render_frame(cx);
        window.render_frame(cx);
        window.try_find(id).is_some()
    })
    .expect("the window is open")
}

/// The parts of a turn the view must agree on.
fn summary(thread: &Thread) -> Vec<(String, String, TurnState, usize)> {
    thread
        .turns()
        .iter()
        .map(|t| (t.prompt.clone(), t.reply(), t.state.clone(), t.tools.len()))
        .collect()
}

#[ui::test]
fn every_fixture_opens_whole(cx: &mut TestAppContext) {
    for fixture in fixtures::all() {
        let host = FixtureHost::new(&fixture.events);
        for session in host.sessions() {
            let (window, view) = open(cx, &host, &session);
            frame(cx, window);
            let shown = cx.update(|cx| summary(&view.read(cx).thread()));
            assert!(!shown.is_empty(), "{}: nothing shown", fixture.name);
            for turn in cx.update(|cx| view.read(cx).thread().turns().to_vec()) {
                assert!(!turn.state.is_open(), "{}: an open turn", fixture.name);
            }
        }
    }
}

#[ui::test]
fn every_fixture_replays_to_the_same_thread(cx: &mut TestAppContext) {
    for fixture in fixtures::all() {
        // The 10,000-event thread replays in the gallery; here it would
        // take minutes in a debug build.
        if fixture.events.len() > 2_000 {
            continue;
        }
        let step = 1;
        let whole = FixtureHost::new(&fixture.events);
        let host = FixtureHost::new(&fixture.events);
        host.replay();
        for session in host.sessions() {
            let (_, opened) = open(cx, &whole, &session);
            let expected = cx.update(|cx| summary(&opened.read(cx).thread()));
            let (window, view) = open(cx, &host, &session);
            while host.advance(step) {
                cx.run_until_parked();
                frame(cx, window);
            }
            cx.run_until_parked();
            frame(cx, window);
            let shown = cx.update(|cx| summary(&view.read(cx).thread()));
            assert_eq!(shown, expected, "{}", fixture.name);
        }
    }
}

#[ui::test]
fn every_tool_call_shows_its_row(cx: &mut TestAppContext) {
    for fixture in [fixtures::tool_heavy(), fixtures::approval()] {
        let host = FixtureHost::new(&fixture.events);
        for session in host.sessions() {
            let (window, view) = open_sized(cx, &host, &session, 4000.);
            let calls: Vec<_> = cx.update(|cx| {
                let thread = view.read(cx).thread();
                let last = thread.turns().last().cloned();
                last.map(|t| t.tools.iter().map(|u| u.call.clone()).collect())
                    .unwrap_or_default()
            });
            for call in calls {
                assert!(
                    shows(
                        cx,
                        window,
                        ui::ElementId::NamedChild(tool_row_id(&call).into(), "head".into())
                    ),
                    "{}: no row for {}",
                    fixture.name,
                    call.as_str()
                );
            }
        }
    }
}

#[ui::test]
fn a_recorded_conversation_refuses_writes(cx: &mut TestAppContext) {
    let fixture = fixtures::chat_basic();
    let host = FixtureHost::new(&fixture.events);
    let session = host.sessions()[0].clone();
    let (_, view) = open(cx, &host, &session);
    let logged = host.events(&session).len();
    let before = cx.update(|cx| summary(&view.read(cx).thread()));
    cx.update(|cx| view.update(cx, |view, cx| view.send("Another question".into(), cx)));
    cx.run_until_parked();
    let after = cx.update(|cx| summary(&view.read(cx).thread()));
    // The log is untouched, and the earlier turns are as they were.
    assert_eq!(host.events(&session).len(), logged);
    assert_eq!(after[..before.len()], before[..]);
    assert!(
        after
            .iter()
            .skip(before.len())
            .all(|t| t.2 != TurnState::Done)
    );
}
