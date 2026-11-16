//! Reloading the chat list: an answer asked for before a delete never brings
//! the chat back, the shell says when a reload has finished, and a write the
//! host or the store refuses leaves the list as they hold it.

extern crate ui as gpui;

use conversation_host::{HostError, NewSession, ScriptedHost};
use isonomic_chat::store::MemoryStore;
use isonomic_chat::{ChatShell, ConversationHost};
use std::rc::Rc;
use ui::test::TestWindowExt as _;

/// The platform's primary modifier, as Chat binds it.
#[cfg(target_os = "macos")]
const MOD: &str = "cmd";
#[cfg(not(target_os = "macos"))]
const MOD: &str = "ctrl";
use ui::{
    AppContext as _, Bounds, Entity, SharedString, TestAppContext, WindowBounds, WindowHandle,
    WindowOptions, base::Root, px, size,
};

fn open(cx: &mut TestAppContext, host: ScriptedHost) -> (WindowHandle<Root>, Entity<ChatShell>) {
    open_with(cx, host, Rc::new(MemoryStore::default()))
}

fn open_with(
    cx: &mut TestAppContext,
    host: ScriptedHost,
    store: Rc<MemoryStore>,
) -> (WindowHandle<Root>, Entity<ChatShell>) {
    cx.update(|cx| {
        ui::init(cx);
        isonomic_chat::init(cx);
    });
    let backend = isonomic_chat::Backend {
        host: Rc::new(host) as Rc<dyn ConversationHost>,
        store,
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

fn title(shell: &Entity<ChatShell>, cx: &mut TestAppContext, id: &SharedString) -> Option<String> {
    shell.read_with(cx, |shell, _| {
        shell
            .sessions()
            .iter()
            .find(|s| &s.id == id)
            .map(|s| s.title.to_string())
    })
}

fn refusal() -> HostError {
    HostError::new("unavailable", "The engine could not save that.")
}

#[ui::test]
fn a_delete_the_host_refuses_brings_the_chat_back(cx: &mut TestAppContext) {
    let host = ScriptedHost::new();
    let lisbon = chat(&host, "Lisbon");
    let (window, shell) = open(cx, host.clone());
    cx.run_until_parked();

    host.refuse_next_write(refusal());
    cx.update_window(window.into(), |_, window, cx| {
        shell.update(cx, |shell, cx| shell.delete_session(&lisbon, window, cx));
    })
    .unwrap();
    cx.run_until_parked();
    let error = shell.read_with(cx, |shell, _| shell.save_error().cloned());
    assert_eq!(error.as_deref(), Some("The engine could not save that."));
    assert!(listed(&shell, cx).contains(&lisbon), "the chat is back");

    // A later delete that goes through stays done, and clears the error.
    cx.update_window(window.into(), |_, window, cx| {
        shell.update(cx, |shell, cx| shell.delete_session(&lisbon, window, cx));
    })
    .unwrap();
    cx.run_until_parked();
    assert!(!listed(&shell, cx).contains(&lisbon));
    assert!(shell.read_with(cx, |shell, _| shell.save_error().is_none()));
}

#[ui::test]
fn a_rename_the_host_refuses_shows_the_hosts_title_again(cx: &mut TestAppContext) {
    let host = ScriptedHost::new();
    let lisbon = chat(&host, "Lisbon");
    let (window, shell) = open(cx, host.clone());
    cx.run_until_parked();

    host.refuse_next_write(refusal());
    cx.update_window(window.into(), |_, window, cx| {
        shell.update(cx, |shell, cx| {
            shell.open_session(lisbon.clone(), window, cx);
            shell.begin_rename(window, cx);
        });
        window.render_frame(cx);
        window.press(&format!("{MOD}-a"), cx);
        window.input("Porto instead", cx);
        window.press("enter", cx);
    })
    .unwrap();
    cx.run_until_parked();
    assert_eq!(title(&shell, cx, &lisbon).as_deref(), Some("Lisbon"));
    assert!(shell.read_with(cx, |shell, _| shell.save_error().is_some()));
}

#[ui::test]
fn a_pin_the_store_refuses_is_undone(cx: &mut TestAppContext) {
    let host = ScriptedHost::new();
    let lisbon = chat(&host, "Lisbon");
    let store = Rc::new(MemoryStore::default());
    let (_, shell) = open_with(cx, host, store.clone());
    cx.run_until_parked();

    store.refuse_writes("The store is read-only.");
    shell.update(cx, |shell, cx| shell.set_pinned(&lisbon, true, cx));
    cx.run_until_parked();
    let (pinned, error) = shell.read_with(cx, |shell, _| {
        let pinned = shell.sessions().iter().any(|s| s.id == lisbon && s.pinned);
        (pinned, shell.save_error().cloned())
    });
    assert!(
        !pinned,
        "the pin did not land, so the chat is not shown pinned"
    );
    assert_eq!(error.as_deref(), Some("The store is read-only."));

    store.accept_writes();
    shell.update(cx, |shell, cx| shell.set_pinned(&lisbon, true, cx));
    cx.run_until_parked();
    let pinned = shell.read_with(cx, |shell, _| {
        shell.sessions().iter().any(|s| s.id == lisbon && s.pinned)
    });
    assert!(pinned);
}

#[ui::test]
fn a_list_that_failed_loads_on_try_again(cx: &mut TestAppContext) {
    let host = ScriptedHost::new();
    let lisbon = chat(&host, "Lisbon");
    host.refuse_list(HostError::new(
        "disconnected",
        "Not connected to the engine.",
    ));
    let (_, shell) = open(cx, host.clone());
    cx.run_until_parked();
    let (loaded, error) = shell.read_with(cx, |shell, _| {
        (shell.is_loaded(), shell.load_error().cloned())
    });
    assert!(loaded, "a failed load has finished");
    assert_eq!(error.as_deref(), Some("Not connected to the engine."));
    assert!(listed(&shell, cx).is_empty());

    shell.update(cx, |shell, cx| shell.reload(cx));
    cx.run_until_parked();
    assert!(shell.read_with(cx, |shell, _| shell.load_error().is_none()));
    assert_eq!(listed(&shell, cx), vec![lisbon]);
}
