// Copyright 2026 Anud Inc.
// SPDX-License-Identifier: GPL-3.0-or-later

//! Delete asks first: the chat goes only when the person confirms.

extern crate ui as gpui;

use isonomic_chat::ChatShell;
use ui::test::TestWindowExt as _;
use ui::{
    AppContext as _, Bounds, Entity, SharedString, TestAppContext, WindowBounds, WindowHandle,
    WindowOptions, base::Root, px, size,
};

/// The shell on its example chats.
fn open(cx: &mut TestAppContext) -> (WindowHandle<Root>, Entity<ChatShell>) {
    cx.update(|cx| {
        ui::init(cx);
        isonomic_chat::init(cx);
    });
    let opened = cx.update(|cx| {
        let (window, shell) = ui::open_window(
            WindowOptions {
                window_bounds: Some(WindowBounds::Windowed(Bounds {
                    origin: Default::default(),
                    size: size(px(1400.), px(900.)),
                })),
                ..Default::default()
            },
            cx,
            |window, cx| {
                let shell = cx.new(|cx| ChatShell::new(None, window, cx));
                shell.update(cx, |shell, cx| shell.new_chat(window, cx));
                shell
            },
        )
        .expect("open the test window");
        (window.downcast::<Root>().expect("a root view"), shell)
    });
    cx.run_until_parked();
    opened
}

fn press(cx: &mut TestAppContext, window: WindowHandle<Root>, key: &str) {
    cx.update_window(window.into(), |_, window, cx| {
        window.render_frame(cx);
        window.press(key, cx);
    })
    .expect("the window is open");
    cx.run_until_parked();
}

/// Ask about the first chat and draw the dialog; returns its id.
fn ask(
    cx: &mut TestAppContext,
    window: WindowHandle<Root>,
    shell: &Entity<ChatShell>,
) -> SharedString {
    let id = shell.read_with(cx, |shell, _| shell.sessions()[0].id.clone());
    let ask = id.clone();
    cx.update_window(window.into(), |_, window, cx| {
        shell.update(cx, |shell, cx| shell.confirm_delete(&ask, window, cx));
        window.render_frame(cx);
    })
    .expect("the window is open");
    cx.run_until_parked();
    id
}

fn has(cx: &mut TestAppContext, shell: &Entity<ChatShell>, id: &SharedString) -> bool {
    shell.read_with(cx, |shell, _| shell.sessions().iter().any(|s| &s.id == id))
}

#[ui::test]
fn delete_waits_for_the_person(cx: &mut TestAppContext) {
    let (window, shell) = open(cx);
    let id = ask(cx, window, &shell);
    assert!(has(cx, &shell, &id), "asking deletes nothing");
    assert_eq!(
        shell.read_with(cx, |s, _| s.confirming_delete().cloned()),
        Some(id.clone())
    );
    press(cx, window, "enter");
    assert!(!has(cx, &shell, &id), "Delete removes the chat");
    assert_eq!(
        shell.read_with(cx, |s, _| s.confirming_delete().cloned()),
        None
    );
}

#[ui::test]
fn escape_keeps_the_chat(cx: &mut TestAppContext) {
    let (window, shell) = open(cx);
    let id = ask(cx, window, &shell);
    press(cx, window, "escape");
    assert!(has(cx, &shell, &id), "Escape keeps the chat");
    assert_eq!(
        shell.read_with(cx, |s, _| s.confirming_delete().cloned()),
        None
    );
}
