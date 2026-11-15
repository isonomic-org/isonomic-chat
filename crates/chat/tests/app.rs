//! Chat as the window mounts it, against the SDK's headless host.

extern crate ui as gpui;

use isonomic_app::App as _;
use isonomic_app::testing::TestHost;
use isonomic_chat::{APP_ID, Chat, Route};
use ui::TestAppContext;

#[test]
fn the_manifest_names_chat_and_asks_for_nothing_yet() {
    let manifest = Chat::manifest();
    assert_eq!(manifest.id, APP_ID);
    assert_eq!(manifest.id, "com.isonomic.chat");
    assert_eq!(manifest.name, "Chat");
    assert!(manifest.capabilities.is_empty());
}

#[ui::test]
fn the_window_mounts_chat_on_home(cx: &mut TestAppContext) {
    // The window initialises Isonomic UI before it mounts any app.
    cx.update(ui::init);
    let host = TestHost::new();
    let open = host.open::<Chat>(cx);
    let chat = open.root::<Chat>(cx);
    open.window
        .update(cx, |_, window, _| window.refresh())
        .unwrap();
    cx.run_until_parked();
    chat.read_with(cx, |chat, cx| {
        assert_eq!(chat.context().id(), APP_ID);
        assert_eq!(chat.shell().read(cx).route(), &Route::Home);
    });
    // Static data: Chat makes no calls of its own yet.
    let calls = host.engine().calls();
    assert!(calls.iter().all(|c| c.app.as_deref() != Some(APP_ID)));
}

#[ui::test]
fn the_window_routes_chat_links_to_chat(cx: &mut TestAppContext) {
    // The window initialises Isonomic UI before it mounts any app.
    cx.update(ui::init);
    let host = TestHost::new();
    let open = host.open::<Chat>(cx);
    let chat = open.root::<Chat>(cx);
    let tab = open.tab(cx);
    assert!(tab.update(cx, |tab, cx| tab.app().open_link("isonomic://chat/s1", cx)));
    assert!(!tab.update(cx, |tab, cx| tab.app().open_link("isonomic://other/x", cx)));
    assert!(
        tab.update(cx, |tab, cx| tab.app().search("hash", cx))
            .is_empty()
    );
    chat.read_with(cx, |chat, _| {
        assert_eq!(chat.opened(), ["isonomic://chat/s1"])
    });
}

#[ui::test]
fn opening_chat_twice_binds_its_keys_once(cx: &mut TestAppContext) {
    // The window initialises Isonomic UI before it mounts any app.
    cx.update(ui::init);
    let host = TestHost::new();
    let _first = host.open::<Chat>(cx);
    let _second = host.open::<Chat>(cx);
    cx.run_until_parked();
}
