//! Reloading the chat list: an answer asked for before a delete never brings
//! the chat back, and the shell says when a reload has finished.

extern crate ui as gpui;

use conversation_host::{NewSession, ScriptedHost};
use isonomic_chat::{ChatShell, ConversationHost};
use std::rc::Rc;
use ui::{
    AppContext as _, Bounds, Entity, SharedString, TestAppContext, WindowBounds, WindowHandle,
    WindowOptions, base::Root, px, size,
};

fn open(cx: &mut TestAppContext, host: ScriptedHost) -> (WindowHandle<Root>, Entity<ChatShell>) {
    cx.update(|cx| {
        ui::init(cx);
        isonomic_chat::init(cx);
    });
    let backend = isonomic_chat::Backend {
        host: Rc::new(host) as Rc<dyn ConversationHost>,
        store: Rc::new(isonomic_chat::store::MemoryStore::default()),
        install_model: None,
    };
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
            |window, cx| cx.new(|cx| ChatShell::new(Some(backend), window, cx)),
        )
        .expect("open the test window");
        (window.downcast::<Root>().expect("a root view"), shell)
    })
}

fn chat(host: &ScriptedHost, title: &str) -> SharedString {
    let new = NewSession {
        preset: "chat".into(),
        title: Some(title.into()),
        model: None,
    };
    let id = futures::executor::block_on(host.create(new)).unwrap();
    id.0.into()
}

fn listed(shell: &Entity<ChatShell>, cx: &mut TestAppContext) -> Vec<SharedString> {
    shell.read_with(cx, |shell, _| {
        shell.sessions().iter().map(|s| s.id.clone()).collect()
    })
}

#[ui::test]
fn a_reload_answered_after_a_delete_does_not_bring_the_chat_back(cx: &mut TestAppContext) {
    let host = ScriptedHost::new();
    let lisbon = chat(&host, "Lisbon");
    let porto = chat(&host, "Porto");
    let (window, shell) = open(cx, host.clone());
    cx.run_until_parked();
    assert!(listed(&shell, cx).contains(&lisbon));

    // A reload is asked for while the host is slow; then Lisbon is deleted.
    host.hold_lists();
    let before = shell.read_with(cx, |shell, _| shell.loads_finished());
    shell.update(cx, |shell, cx| shell.reload(cx));
    cx.run_until_parked();
    assert!(shell.read_with(cx, |shell, _| shell.is_reloading()));
    cx.update_window(window.into(), |_, window, cx| {
        shell.update(cx, |shell, cx| shell.delete_session(&lisbon, window, cx));
    })
    .unwrap();
    cx.run_until_parked();
    assert!(!listed(&shell, cx).contains(&lisbon));

    // The answer, from before the delete, still lists Lisbon.
    host.release_lists();
    cx.run_until_parked();
    let (reloading, after) = shell.read_with(cx, |shell, _| {
        (shell.is_reloading(), shell.loads_finished())
    });
    assert!(!reloading, "the reload has finished");
    assert_eq!(after, before + 1);
    let chats = listed(&shell, cx);
    assert!(!chats.contains(&lisbon), "deleted stays deleted: {chats:?}");
    assert!(chats.contains(&porto));
}
