//! Headless tests of Chat's shell and home: routing, sending from home,
//! rename, delete and the keyboard toggles.

extern crate ui as gpui;

use conversation_host::{NewSession, ScriptedHost, ScriptedReply, SessionQuery};
use conversation_model::{InputState, TurnState};
use isonomic_chat::{ChatShell, ConversationHost, Route};
use std::rc::Rc;
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

/// The shell on the static example chats, with no backend.
fn open(cx: &mut TestAppContext) -> (WindowHandle<Root>, Entity<ChatShell>) {
    open_on(cx, None)
}

/// The shell on a scripted host and an in-memory store.
fn open_with(
    cx: &mut TestAppContext,
    host: ScriptedHost,
) -> (WindowHandle<Root>, Entity<ChatShell>) {
    let host: Rc<dyn ConversationHost> = Rc::new(host);
    let backend = isonomic_chat::Backend {
        host,
        store: Rc::new(isonomic_chat::store::MemoryStore::default()),
        install_model: None,
    };
    open_on(cx, Some(backend))
}

fn open_on(
    cx: &mut TestAppContext,
    backend: Option<isonomic_chat::Backend>,
) -> (WindowHandle<Root>, Entity<ChatShell>) {
    cx.update(|cx| {
        ui::init(cx);
        isonomic_chat::init(cx);
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
                let shell = cx.new(|cx| ChatShell::new(backend, window, cx));
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
        window.click("send", cx);
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
fn the_sidebar_collapses_to_its_rail_and_back_from_the_keyboard(cx: &mut TestAppContext) {
    let (window, shell) = open(cx);
    with_window(cx, window, |window, cx| {
        window.render_frame(cx);
        assert!(shell.read(cx).sidebar_open());
        window.press(&format!("{MOD}-\\"), cx);
        window.render_frame(cx);
        assert!(!shell.read(cx).sidebar_open());
        // The rail keeps the toggle.
        assert!(window.try_find("sidebar-toggle").is_some());
        window.press(&format!("{MOD}-\\"), cx);
        window.render_frame(cx);
        assert!(shell.read(cx).sidebar_open());
    });
}

#[ui::test]
fn search_opens_from_the_keyboard_and_enter_opens_the_match(cx: &mut TestAppContext) {
    let (window, shell) = open(cx);
    with_window(cx, window, |window, cx| {
        window.render_frame(cx);
        window.press(&format!("{MOD}-k"), cx);
    });
    cx.run_until_parked();
    assert!(cx.update(|cx| shell.read(cx).is_searching()));
    with_window(cx, window, |window, cx| {
        window.render_frame(cx);
        window.input("quantum", cx);
        window.render_frame(cx);
        window.press("enter", cx);
    });
    cx.run_until_parked();
    cx.update(|cx| {
        let shell = shell.read(cx);
        assert!(!shell.is_searching());
        assert_eq!(shell.route(), &Route::Session(SharedString::from("s4")));
    });
}

#[ui::test]
fn pinning_moves_a_chat_to_pinned_and_delete_drops_any_chat(cx: &mut TestAppContext) {
    let (window, shell) = open(cx);
    with_window(cx, window, |window, cx| {
        shell.update(cx, |shell, cx| {
            shell.set_pinned(&"s5".into(), true, cx);
            shell.delete_session(&"s3".into(), window, cx);
        });
        let shell = shell.read(cx);
        let pinned: Vec<&str> = shell
            .sessions()
            .iter()
            .filter(|s| s.pinned)
            .map(|s| s.id.as_ref())
            .collect();
        assert_eq!(pinned, ["s1", "s5"]);
        assert!(shell.sessions().iter().all(|s| s.id.as_ref() != "s3"));
        assert_eq!(shell.route(), &Route::Home);
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

#[ui::test]
fn a_chat_sent_from_home_runs_a_turn_with_stop_and_continue(cx: &mut TestAppContext) {
    let host = ScriptedHost::new();
    host.script(
        ScriptedReply::text("A hash function maps data of any size to a fixed-size value.")
            .with_title("Hash functions"),
    );
    let (window, shell) = open_with(cx, host.clone());
    with_window(cx, window, |window, cx| {
        shell.update(cx, |shell, cx| {
            shell.set_draft("What is a hash function?", window, cx)
        });
        window.render_frame(cx);
        window.press("enter", cx);
    });
    cx.run_until_parked();
    let id = cx.update(|cx| shell.read(cx).sessions()[0].id.clone());
    let thread = cx.update(|cx| shell.read(cx).thread(&id).cloned().expect("a conversation"));
    let state = |cx: &mut TestAppContext| {
        cx.update(|cx| {
            let view = thread.read(cx);
            let turns = view.thread();
            let turn = turns.turns().last().expect("a turn");
            (view.input_state(), turn.reply(), turn.state.clone())
        })
    };

    host.pump(3);
    cx.run_until_parked();
    assert_eq!(state(cx).0, InputState::Busy);
    assert_eq!(state(cx).1, "A hash function ");

    // Stop from the composer: the partial reply stays and continue appears.
    with_window(cx, window, |window, cx| {
        window.render_frame(cx);
        window.click("stop", cx);
    });
    cx.run_until_parked();
    assert_eq!(state(cx).2, TurnState::Stopped);
    with_window(cx, window, |window, cx| {
        window.render_frame(cx);
        window.click(
            conversation_ui::action_button(conversation_ui::CONTINUE_ID),
            cx,
        );
    });
    cx.run_until_parked();
    host.finish();
    cx.run_until_parked();
    let (input, reply, turn) = state(cx);
    assert_eq!(input, InputState::Ready);
    assert_eq!(
        reply,
        "A hash function maps data of any size to a fixed-size value."
    );
    assert_eq!(turn, TurnState::Done);
    // The model's title replaces the one taken from the prompt.
    let title = cx.update(|cx| shell.read(cx).sessions()[0].title.clone());
    assert_eq!(title.as_ref(), "Hash functions");
}

#[ui::test]
fn a_stored_chat_reopens_with_its_history_and_rename_and_delete_reach_the_host(
    cx: &mut TestAppContext,
) {
    // An earlier visit left one chat with one exchange in the host.
    let host = ScriptedHost::new();
    host.script(ScriptedReply::text("Lisbon in three days.").with_title("Lisbon"));
    let new = NewSession {
        preset: "chat".into(),
        title: None,
        model: None,
    };
    let session = futures::executor::block_on(host.create(new)).unwrap();
    let ask = vec![conversation_model::schema::Block::text("Plan a trip")];
    futures::executor::block_on(host.send(&session, ask)).unwrap();
    host.finish();

    let (window, shell) = open_with(cx, host.clone());
    cx.run_until_parked();
    let id: SharedString = session.0.clone().into();
    cx.update(|cx| {
        let sessions = shell.read(cx).sessions();
        assert_eq!(sessions.len(), 1);
        assert_eq!(sessions[0].title.as_ref(), "Lisbon");
    });

    // Opening it replays the stored exchange.
    with_window(cx, window, |window, cx| {
        shell.update(cx, |shell, cx| shell.open_session(id.clone(), window, cx));
    });
    cx.run_until_parked();
    let thread = cx.update(|cx| shell.read(cx).thread(&id).cloned().expect("a conversation"));
    cx.update(|cx| {
        let view = thread.read(cx);
        let turns = view.thread();
        assert_eq!(turns.turns().len(), 1);
        assert_eq!(turns.turns()[0].prompt, "Plan a trip");
        assert_eq!(turns.turns()[0].reply(), "Lisbon in three days.");
    });

    // Rename reaches the host.
    with_window(cx, window, |window, cx| {
        shell.update(cx, |shell, cx| shell.begin_rename(window, cx));
        window.render_frame(cx);
        window.press(&format!("{MOD}-a"), cx);
        window.input("Lisbon, three days", cx);
        window.press("enter", cx);
    });
    cx.run_until_parked();
    let listed = futures::executor::block_on(host.list(SessionQuery::default())).unwrap();
    assert_eq!(listed[0].title.as_deref(), Some("Lisbon, three days"));

    // Delete archives it in the host and goes home.
    with_window(cx, window, |window, cx| {
        shell.update(cx, |shell, cx| shell.delete_current(window, cx));
    });
    cx.run_until_parked();
    assert!(
        futures::executor::block_on(host.list(SessionQuery::default()))
            .unwrap()
            .is_empty()
    );
    cx.update(|cx| {
        assert_eq!(shell.read(cx).route(), &Route::Home);
        assert!(shell.read(cx).sessions().is_empty());
    });
}

#[ui::test]
fn with_no_model_installed_home_asks_the_window_to_install_the_recommended_one(
    cx: &mut TestAppContext,
) {
    use conversation_model::{Availability, ModelCatalog, ModelOption};
    let host = ScriptedHost::new();
    let option = |id: &str, availability| ModelOption {
        id: id.into(),
        name: id.into(),
        remote: false,
        availability,
    };
    host.set_models(ModelCatalog::new(
        vec![
            option("small", Availability::NotInstalled),
            option("large", Availability::NotInstalled),
        ],
        Some("large".into()),
    ));
    let asked = Rc::new(std::cell::RefCell::new(Vec::<String>::new()));
    let log = asked.clone();
    let backend = isonomic_chat::Backend {
        host: Rc::new(host),
        store: Rc::new(isonomic_chat::store::MemoryStore::default()),
        install_model: Some(Rc::new(move |model: &str| {
            log.borrow_mut().push(model.to_owned());
            true
        })),
    };
    let (window, shell) = open_on(cx, Some(backend));
    cx.run_until_parked();
    with_window(cx, window, |window, cx| {
        assert_eq!(
            shell
                .read(cx)
                .missing_model()
                .map(|m| m.id.clone())
                .as_deref(),
            Some("large")
        );
        window.render_frame(cx);
        window.click("install-model", cx);
    });
    assert_eq!(*asked.borrow(), ["large"]);
    cx.update(|cx| {
        assert_eq!(
            shell
                .read(cx)
                .install_asked()
                .map(|m| m.to_string())
                .as_deref(),
            Some("large")
        )
    });
}

#[ui::test]
fn a_narrow_window_collapses_the_sidebar_to_its_rail(cx: &mut TestAppContext) {
    let (window, shell) = open(cx);
    with_window(cx, window, |window, cx| {
        window.render_frame(cx);
        assert!(shell.read(cx).sidebar_open());
    });
    cx.simulate_window_resize(window.into(), size(px(700.), px(900.)));
    with_window(cx, window, |window, cx| {
        window.render_frame(cx);
        assert!(!shell.read(cx).sidebar_open());
        assert!(shell.read(cx).is_narrow());
        // The rail's toggle still expands the sidebar while narrow.
        window.render_frame(cx);
        window.click("sidebar-toggle", cx);
    });
    cx.run_until_parked();
    with_window(cx, window, |window, cx| {
        window.render_frame(cx);
        assert!(shell.read(cx).sidebar_open());
    });
    cx.simulate_window_resize(window.into(), size(px(1400.), px(900.)));
    with_window(cx, window, |window, cx| {
        // Wide again: the sidebar stays expanded.
        window.render_frame(cx);
        assert!(shell.read(cx).sidebar_open());
    });
}

#[ui::test]
fn a_model_picked_on_home_runs_the_next_chat(cx: &mut TestAppContext) {
    use conversation_model::{Availability, ModelCatalog, ModelOption};
    let host = ScriptedHost::new();
    let option = |id: &str| ModelOption {
        id: id.into(),
        name: id.into(),
        remote: false,
        availability: Availability::Installed,
    };
    host.set_models(ModelCatalog::new(
        vec![option("small"), option("medium")],
        Some("medium".into()),
    ));
    host.script(ScriptedReply::text("Hi."));
    let (window, shell) = open_with(cx, host.clone());
    cx.run_until_parked();
    // Choose through the shell, as the footer's model picker does.
    cx.update(|cx| shell.update(cx, |shell, cx| shell.choose_model("small".into(), cx)));
    with_window(cx, window, |window, cx| {
        shell.update(cx, |shell, cx| shell.set_draft("Hello", window, cx));
        window.render_frame(cx);
        window.press("enter", cx);
    });
    cx.run_until_parked();
    assert_eq!(host.created()[0].model.as_deref(), Some("small"));
}

#[ui::test]
fn choosing_a_model_that_is_not_installed_asks_the_window(cx: &mut TestAppContext) {
    use conversation_model::{Availability, ModelCatalog, ModelOption};
    let host = ScriptedHost::new();
    let option = |id: &str, availability| ModelOption {
        id: id.into(),
        name: id.into(),
        remote: false,
        availability,
    };
    host.set_models(ModelCatalog::new(
        vec![
            option("small", Availability::Installed),
            option("large", Availability::NotInstalled),
        ],
        None,
    ));
    let asked = Rc::new(std::cell::RefCell::new(Vec::<String>::new()));
    let log = asked.clone();
    let backend = isonomic_chat::Backend {
        host: Rc::new(host),
        store: Rc::new(isonomic_chat::store::MemoryStore::default()),
        install_model: Some(Rc::new(move |model: &str| {
            log.borrow_mut().push(model.to_owned());
            true
        })),
    };
    let (_window, shell) = open_on(cx, Some(backend));
    cx.run_until_parked();
    cx.update(|cx| shell.update(cx, |shell, cx| shell.choose_model("large".into(), cx)));
    assert_eq!(*asked.borrow(), ["large"]);
    // The pick stays on the model that can answer.
    assert_eq!(
        cx.update(|cx| shell.read(cx).model().map(str::to_owned)),
        None
    );
}
