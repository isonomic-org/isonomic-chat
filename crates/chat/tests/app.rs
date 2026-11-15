//! Chat as the window mounts it, against the SDK's headless host: the
//! manifest, the chat list from the host, pins in Chat's schema, and a real
//! turn sent from home.

extern crate ui as gpui;

use conversation_host::{ConversationHost, NewSession};
use conversation_model::TurnState;
use conversation_sdk::SdkHost;
use isonomic_app::App as _;
use isonomic_app::testing::{TestHost, wait_until};
use isonomic_chat::store::{SCHEMA, sql};
use isonomic_chat::{APP_ID, Chat, ChatShell, Route};
use isonomic_proto::SqlValue;
use isonomic_proto::sql::{Column, Rows};
use ui::test::TestWindowExt as _;
use ui::{AppContext as _, Entity, TestAppContext};

fn mount(
    cx: &mut TestAppContext,
    host: &TestHost,
) -> (isonomic_app::testing::OpenApp, Entity<ChatShell>) {
    // The window initialises Isonomic UI before it mounts any app.
    cx.update(ui::init);
    let open = host.open::<Chat>(cx);
    let chat = open.root::<Chat>(cx);
    let shell = chat.read_with(cx, |chat, _| chat.shell().clone());
    wait_until(cx, &shell, |shell| shell.is_loaded());
    (open, shell)
}

fn mine(host: &TestHost) -> Vec<String> {
    host.engine()
        .calls()
        .into_iter()
        .filter(|c| c.app.as_deref() == Some(APP_ID))
        .map(|c| c.method)
        .collect()
}

#[test]
fn the_manifest_asks_for_agents_and_its_own_schema() {
    let manifest = Chat::manifest();
    assert_eq!(manifest.id, "com.isonomic.chat");
    assert_eq!(manifest.id, APP_ID);
    assert_eq!(manifest.name, "Chat");
    let wanted = isonomic_app::Capabilities {
        agents: manifest.capabilities.agents.clone(),
        sql: Some(isonomic_app::SqlAccess::default()),
        ..Default::default()
    };
    assert!(manifest.capabilities.agents.is_some());
    assert_eq!(manifest.capabilities, wanted);
    assert_eq!(manifest.schemas.len(), 1);
    assert_eq!(manifest.schemas[0].name, SCHEMA);
}

#[ui::test]
fn the_window_mounts_chat_on_home_with_the_hosts_chats(cx: &mut TestAppContext) {
    let host = TestHost::new();
    // A chat from an earlier visit, and a pin for it in Chat's schema.
    let (app, _) = host.context::<Chat>();
    let earlier = host
        .runtime()
        .block_on(SdkHost::new(app.client().agents()).create(NewSession {
            preset: "chat".into(),
            title: Some("Trip to Lisbon".into()),
        }))
        .expect("create");
    let pins = Rows {
        columns: vec![Column {
            name: "session".into(),
            kind: "text".into(),
        }],
        rows: vec![vec![SqlValue::Text(earlier.0.clone())]],
    };
    host.engine().sql().respond(SCHEMA, sql::LOAD, pins);

    let (_open, shell) = mount(cx, &host);
    shell.read_with(cx, |shell, _| {
        assert_eq!(shell.route(), &Route::Home);
        let sessions = shell.sessions();
        assert_eq!(sessions.len(), 1, "the host's chats, not the examples");
        assert_eq!(sessions[0].id.as_ref(), earlier.0);
        assert_eq!(sessions[0].title.as_ref(), "Trip to Lisbon");
        assert!(sessions[0].pinned);
        assert!(shell.load_error().is_none());
    });
    let calls = mine(&host);
    assert!(calls.contains(&"sessions.list".to_owned()), "{calls:?}");
    let statements: Vec<String> = host
        .engine()
        .sql()
        .statements()
        .into_iter()
        .filter(|s| s.app.as_deref() == Some(APP_ID))
        .map(|s| s.sql)
        .collect();
    assert_eq!(statements, [sql::CREATE, sql::LOAD]);
}

#[ui::test]
fn pinning_writes_to_chats_schema(cx: &mut TestAppContext) {
    let host = TestHost::new();
    let (app, _) = host.context::<Chat>();
    let session = host
        .runtime()
        .block_on(SdkHost::new(app.client().agents()).create(NewSession {
            preset: "chat".into(),
            title: Some("Notes".into()),
        }))
        .expect("create");
    let (_open, shell) = mount(cx, &host);
    let id = ui::SharedString::from(session.0.clone());
    shell.update(cx, |shell, cx| shell.set_pinned(&id, true, cx));
    assert!(shell.read_with(cx, |shell, _| shell.is_saving()));
    wait_until(cx, &shell, |shell| !shell.is_saving());
    shell.read_with(cx, |shell, _| {
        assert!(shell.sessions()[0].pinned);
        assert!(shell.save_error().is_none());
    });
    let pinned = host
        .engine()
        .sql()
        .statements()
        .into_iter()
        .find(|s| s.sql == sql::PIN)
        .expect("a pin statement");
    assert_eq!(pinned.schema, SCHEMA);
    assert_eq!(pinned.params, [SqlValue::Text(session.0)]);
}

#[ui::test]
fn a_chat_sent_from_home_runs_a_real_turn_and_takes_the_hosts_id(cx: &mut TestAppContext) {
    let host = TestHost::new();
    host.engine().agents().reply("Hello! How can I help?");
    let (open, shell) = mount(cx, &host);
    cx.update_window(open.window.into(), |_, window, cx| {
        shell.update(cx, |shell, cx| shell.set_draft("Hello", window, cx));
        window.render_frame(cx);
        window.press("enter", cx);
    })
    .unwrap();
    let thread = shell.read_with(cx, |shell, _| {
        let id = shell.sessions()[0].id.clone();
        shell.thread(&id).cloned().expect("a conversation")
    });
    wait_until(cx, &thread, |view| {
        view.thread()
            .turns()
            .last()
            .is_some_and(|turn| turn.state == TurnState::Done)
    });
    cx.run_until_parked();
    let calls = mine(&host);
    assert!(calls.contains(&"sessions.create".to_owned()), "{calls:?}");
    assert!(calls.contains(&"sessions.send".to_owned()), "{calls:?}");
    let (prompt, reply) = thread.read_with(cx, |view, _| {
        let thread = view.thread();
        (thread.turns()[0].prompt.clone(), thread.turns()[0].reply())
    });
    assert_eq!(prompt, "Hello");
    assert_eq!(reply, "Hello! How can I help?");
    // The sidebar and the route now name the host's session.
    let session = thread
        .read_with(cx, |view, _| view.session())
        .expect("a session");
    shell.read_with(cx, |shell, _| {
        assert_eq!(shell.sessions()[0].id.as_ref(), session.0);
        assert_eq!(shell.route(), &Route::Session(session.0.clone().into()));
        assert!(shell.thread(&session.0.clone().into()).is_some());
    });
}

#[ui::test]
fn the_window_routes_chat_links_to_chat(cx: &mut TestAppContext) {
    let host = TestHost::new();
    let (open, _) = mount(cx, &host);
    let chat = open.root::<Chat>(cx);
    let tab = open.tab(cx);
    assert!(tab.update(cx, |tab, cx| tab.app().open_link("isonomic://chat/s1", cx)));
    assert!(!tab.update(cx, |tab, cx| tab.app().open_link("isonomic://other/x", cx)));
    chat.read_with(cx, |chat, _| {
        assert_eq!(chat.opened(), ["isonomic://chat/s1"])
    });
}
