//! Headless tests of Chat's shell and home: routing, sending from home,
//! rename, delete and the keyboard toggles.

extern crate ui as gpui;

use chat::{ChatShell, Route};
use ui::test::TestWindowExt as _;
use ui::{
    AppContext as _, Bounds, Entity, Focusable as _, SharedString, TestAppContext, Window,
    WindowBounds, WindowHandle, WindowOptions, base::Root, px, size,
};

/// The platform's primary modifier, as Chat binds it.
#[cfg(target_os = "macos")]
const MOD: &str = "cmd";
#[cfg(not(target_os = "macos"))]
const MOD: &str = "ctrl";

fn open(cx: &mut TestAppContext) -> (WindowHandle<Root>, Entity<ChatShell>) {
    cx.update(|cx| {
        ui::init(cx);
        chat::init(cx);
    });
    cx.update(|cx| {
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
                let shell = cx.new(|cx| ChatShell::new(window, cx));
                let focus = shell.read(cx).focus_handle(cx);
                window.focus(&focus, cx);
                shell
            },
        )
        .expect("open the test window");
        (window.downcast::<Root>().expect("a root view"), shell)
    })
}

fn with_window<R>(
    cx: &mut TestAppContext,
    window: WindowHandle<Root>,
    f: impl FnOnce(&mut Window, &mut ui::App) -> R,
) -> R {
    cx.update_window(window.into(), |_, window, cx| {
        window.render_frame(cx);
        f(window, cx)
    })
    .expect("the window is open")
}

#[ui::test]
fn home_is_the_first_route_and_send_waits_for_text(cx: &mut TestAppContext) {
    let (window, shell) = open(cx);
    with_window(cx, window, |window, cx| {
        assert_eq!(shell.read(cx).route(), &Route::Home);
        assert!(window.try_find("greeting").is_some());
        // Send has nothing to send: clicking it changes nothing.
        window.click("composer-send", cx);
        assert_eq!(shell.read(cx).route(), &Route::Home);
    });
}

#[ui::test]
fn sending_from_home_starts_a_titled_chat_below_the_pinned_ones(cx: &mut TestAppContext) {
    let (window, shell) = open(cx);
    let before = cx.update(|cx| shell.read(cx).sessions().len());
    with_window(cx, window, |window, cx| {
        shell.update(cx, |shell, cx| {
            shell.set_draft("What is a hash function?\nMore detail here.", window, cx)
        });
        window.render_frame(cx);
        window.press("enter", cx);
    });
    cx.run_until_parked();
    cx.update(|cx| {
        let shell = shell.read(cx);
        assert_eq!(shell.sessions().len(), before + 1);
        let new = &shell.sessions()[1];
        assert_eq!(new.title.as_ref(), "What is a hash function?");
        assert!(!new.pinned);
        assert_eq!(shell.route(), &Route::Session(new.id.clone()));
        assert!(shell.draft(cx).is_empty(), "sending clears the composer");
    });
}

#[ui::test]
fn new_chat_from_the_keyboard_goes_home(cx: &mut TestAppContext) {
    let (window, shell) = open(cx);
    with_window(cx, window, |window, cx| {
        shell.update(cx, |shell, cx| shell.open_session("s2".into(), window, cx));
        window.render_frame(cx);
        assert_eq!(
            shell.read(cx).route(),
            &Route::Session(SharedString::from("s2"))
        );
        assert!(window.try_find("chat-title").is_some());
        let focus = shell.read(cx).focus_handle(cx);
        window.focus(&focus, cx);
        window.press(&format!("{MOD}-shift-o"), cx);
        assert_eq!(shell.read(cx).route(), &Route::Home);
    });
}

#[ui::test]
fn keyboard_toggles_hide_and_show_both_panels(cx: &mut TestAppContext) {
    let (window, shell) = open(cx);
    with_window(cx, window, |window, cx| {
        assert!(window.try_find("new-chat").is_some());
        assert!(window.try_find("context_sidebar").is_some());

        window.press(&format!("{MOD}-\\"), cx);
        window.render_frame(cx);
        assert!(!shell.read(cx).sidebar_open());
        assert!(window.try_find("new-chat").is_none());
        assert!(window.try_find("header-sidebar-toggle").is_some());

        window.press(&format!("{MOD}-alt-\\"), cx);
        window.render_frame(cx);
        assert!(!shell.read(cx).context_panel_open());
        assert!(window.try_find("context_sidebar").is_none());

        window.press(&format!("{MOD}-\\"), cx);
        window.press(&format!("{MOD}-alt-\\"), cx);
        window.render_frame(cx);
        assert!(shell.read(cx).sidebar_open());
        assert!(shell.read(cx).context_panel_open());
    });
}

#[ui::test]
fn rename_keeps_a_new_title_and_ignores_an_empty_one(cx: &mut TestAppContext) {
    let (window, shell) = open(cx);
    let title = |cx: &mut TestAppContext| {
        cx.update(|cx| {
            let shell = shell.read(cx);
            shell
                .sessions()
                .iter()
                .find(|s| s.id.as_ref() == "s3")
                .map(|s| s.title.to_string())
        })
    };

    with_window(cx, window, |window, cx| {
        shell.update(cx, |shell, cx| {
            shell.open_session("s3".into(), window, cx);
            shell.begin_rename(window, cx);
        });
        window.render_frame(cx);
        assert!(shell.read(cx).is_renaming());
        window.press(&format!("{MOD}-a"), cx);
        window.input("Loops, made faster", cx);
        window.press("enter", cx);
    });
    cx.run_until_parked();
    assert_eq!(title(cx).as_deref(), Some("Loops, made faster"));
    cx.update(|cx| assert!(!shell.read(cx).is_renaming()));

    with_window(cx, window, |window, cx| {
        shell.update(cx, |shell, cx| shell.begin_rename(window, cx));
        window.render_frame(cx);
        window.press(&format!("{MOD}-a"), cx);
        window.press("backspace", cx);
        window.press("enter", cx);
    });
    cx.run_until_parked();
    assert_eq!(title(cx).as_deref(), Some("Loops, made faster"));
}

#[ui::test]
fn delete_drops_the_chat_and_goes_home(cx: &mut TestAppContext) {
    let (window, shell) = open(cx);
    with_window(cx, window, |window, cx| {
        shell.update(cx, |shell, cx| {
            shell.open_session("s4".into(), window, cx);
            shell.delete_current(window, cx);
        });
        let shell = shell.read(cx);
        assert_eq!(shell.route(), &Route::Home);
        assert!(shell.sessions().iter().all(|s| s.id.as_ref() != "s4"));
    });
}
