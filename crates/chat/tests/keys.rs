//! Chat's shortcuts with focus where a person's is: in the composer.

extern crate ui as gpui;

use isonomic_chat::{ChatShell, Route};
use ui::test::TestWindowExt as _;
use ui::{
    AppContext as _, Bounds, Entity, SharedString, TestAppContext, WindowBounds, WindowHandle,
    WindowOptions, base::Root, px, size,
};

#[cfg(target_os = "macos")]
const MOD: &str = "cmd";
#[cfg(not(target_os = "macos"))]
const MOD: &str = "ctrl";

/// The shell on its example chats, with focus in home's composer.
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

#[ui::test]
fn alt_up_and_down_step_through_the_chats_from_the_composer(cx: &mut TestAppContext) {
    let (window, shell) = open(cx);
    let order: Vec<SharedString> = cx.update(|cx| {
        let sessions = shell.read(cx).sessions();
        let pinned = sessions.iter().filter(|s| s.pinned);
        let rest = sessions.iter().filter(|s| !s.pinned);
        pinned.chain(rest).map(|s| s.id.clone()).collect()
    });
    let route = |cx: &mut TestAppContext| cx.update(|cx| shell.read(cx).route().clone());

    press(cx, window, "alt-down");
    assert_eq!(route(cx), Route::Session(order[0].clone()));
    // Opening a chat puts focus in its composer; the keys still step.
    press(cx, window, "alt-down");
    assert_eq!(route(cx), Route::Session(order[1].clone()));
    press(cx, window, "alt-up");
    assert_eq!(route(cx), Route::Session(order[0].clone()));
}

#[ui::test]
fn both_sidebar_keys_toggle_it_from_the_composer(cx: &mut TestAppContext) {
    let (window, shell) = open(cx);
    let open = |cx: &mut TestAppContext| cx.update(|cx| shell.read(cx).sidebar_open());
    assert!(open(cx));
    press(cx, window, &format!("{MOD}-."));
    assert!(!open(cx));
    press(cx, window, &format!("{MOD}-b"));
    assert!(open(cx));
}
