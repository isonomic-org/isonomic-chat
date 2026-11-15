//! Chat as the window mounts it, against the SDK's headless host.

extern crate ui as gpui;

use conversation_model::TurnState;
use isonomic_app::App as _;
use isonomic_app::testing::{TestHost, wait_until};
use isonomic_chat::{APP_ID, Chat, Route};
use ui::test::TestWindowExt as _;
use ui::{AppContext as _, TestAppContext};

#[test]
fn the_manifest_names_chat_and_asks_only_for_agents() {
    let manifest = Chat::manifest();
    assert_eq!(manifest.id, APP_ID);
    assert_eq!(manifest.id, "com.isonomic.chat");
    assert_eq!(manifest.name, "Chat");
    assert!(manifest.capabilities.agents.is_some());
    let only_agents = isonomic_app::Capabilities {
        agents: manifest.capabilities.agents.clone(),
        ..Default::default()
    };
    assert_eq!(manifest.capabilities, only_agents);
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
    // Opening Chat calls nothing: sessions start when the person sends.
    let calls = host.engine().calls();
    assert!(calls.iter().all(|c| c.app.as_deref() != Some(APP_ID)));
}

#[ui::test]
fn a_chat_sent_from_home_runs_a_real_turn_under_chats_token(cx: &mut TestAppContext) {
    cx.update(ui::init);
    let host = TestHost::new();
    host.engine().agents().reply("Hello! How can I help?");
    let open = host.open::<Chat>(cx);
    let chat = open.root::<Chat>(cx);
    let shell = chat.read_with(cx, |chat, _| chat.shell().clone());
    cx.update_window(open.window.into(), |_, window, cx| {
        shell.update(cx, |shell, cx| shell.set_draft("Hello", window, cx));
        window.render_frame(cx);
        window.press("enter", cx);
    })
    .unwrap();
    let thread = shell.read_with(cx, |shell, _| {
        let id = shell.sessions()[1].id.clone();
        shell.thread(&id).cloned().expect("a conversation")
    });
    wait_until(cx, &thread, |view| {
        view.thread()
            .turns()
            .last()
            .is_some_and(|turn| turn.state == TurnState::Done)
    });
    let calls = host.engine().calls();
    let mine: Vec<&str> = calls
        .iter()
        .filter(|c| c.app.as_deref() == Some(APP_ID))
        .map(|c| c.method.as_str())
        .collect();
    assert!(mine.starts_with(&["sessions.create"]), "{mine:?}");
    assert!(mine.contains(&"sessions.send"), "{mine:?}");
    let (prompt, reply) = thread.read_with(cx, |view, _| {
        let thread = view.thread();
        let turn = &thread.turns()[0];
        (turn.prompt.clone(), turn.reply())
    });
    assert_eq!(prompt, "Hello");
    assert_eq!(reply, "Hello! How can I help?");
}
