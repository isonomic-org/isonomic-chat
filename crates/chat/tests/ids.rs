//! The ids the window's scripted QA finds Chat's controls by.

extern crate ui as gpui;

use conversation_host::ScriptedHost;
use conversation_model::{Availability, ModelCatalog, ModelOption};
use isonomic_chat::{ChatShell, ConversationHost};
use std::rc::Rc;
use ui::extra::model_picker;
use ui::test::TestWindowExt as _;
use ui::{
    AppContext as _, Bounds, Entity, TestAppContext, WindowBounds, WindowHandle, WindowOptions,
    base::Root, px, size,
};

fn open(cx: &mut TestAppContext) -> (WindowHandle<Root>, Entity<ChatShell>) {
    open_on(cx, None)
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
            |window, cx| cx.new(|cx| ChatShell::new(backend, window, cx)),
        )
        .expect("open the test window");
        (window.downcast::<Root>().expect("a root view"), shell)
    })
}

#[ui::test]
fn each_chat_row_has_its_menu_trigger_by_session_id(cx: &mut TestAppContext) {
    let (window, shell) = open(cx);
    let ids: Vec<String> = cx.update(|cx| {
        shell
            .read(cx)
            .sessions()
            .iter()
            .map(|s| s.id.to_string())
            .collect()
    });
    cx.update_window(window.into(), |_, window, cx| {
        window.render_frame(cx);
        for id in &ids {
            let trigger = format!("chat-row-menu-{id}");
            let found = window
                .try_find(ui::SharedString::from(trigger.clone()))
                .is_some();
            assert!(found, "{trigger}");
        }
    })
    .expect("the window is open");
}

fn found(window: &mut ui::Window, id: impl Into<ui::ElementId>) -> bool {
    window.try_find(id).is_some()
}

#[ui::test]
fn rows_menu_items_and_search_rows_have_their_ids(cx: &mut TestAppContext) {
    let (window, shell) = open(cx);
    let ids: Vec<String> = cx.update(|cx| {
        shell
            .read(cx)
            .sessions()
            .iter()
            .map(|s| s.id.to_string())
            .collect()
    });
    let first = ids[0].clone();
    cx.update_window(window.into(), |_, window, cx| {
        window.render_frame(cx);
        for id in &ids {
            assert!(
                found(window, ui::SharedString::from(format!("chat-row-{id}"))),
                "row {id}"
            );
        }
        // The row menu's items, once it is open.
        window.click(ui::SharedString::from(format!("chat-row-menu-{first}")), cx);
    })
    .expect("the window is open");
    cx.run_until_parked();
    cx.update_window(window.into(), |_, window, cx| {
        window.render_frame(cx);
        for item in ["open", "pin", "rename", "delete"] {
            let id = format!("chat-row-menu-{first}-{item}");
            assert!(found(window, ui::SharedString::from(id.clone())), "{id}");
        }
    })
    .expect("the window is open");

    // Search: each chat's row, and its last-active time.
    cx.update_window(window.into(), |_, window, cx| {
        shell.update(cx, |shell, cx| shell.open_search(window, cx));
    })
    .expect("the window is open");
    cx.run_until_parked();
    cx.update_window(window.into(), |_, window, cx| {
        window.render_frame(cx);
        for id in &ids {
            let row = format!("command-{id}");
            assert!(found(window, ui::SharedString::from(row.clone())), "{row}");
            let meta = format!("{row}-meta");
            assert!(
                found(window, ui::SharedString::from(meta.clone())),
                "{meta}"
            );
        }
    })
    .expect("the window is open");
}

#[ui::test]
fn the_model_pickers_list_and_rows_have_their_ids(cx: &mut TestAppContext) {
    let host = ScriptedHost::new();
    let option = |id: &str, name: &str| ModelOption {
        id: id.into(),
        name: name.into(),
        remote: false,
        availability: Availability::Installed,
    };
    host.set_models(ModelCatalog::new(
        vec![option("small", "Small"), option("medium", "Medium")],
        Some("medium".into()),
    ));
    let host: Rc<dyn ConversationHost> = Rc::new(host);
    let backend = isonomic_chat::Backend {
        host,
        store: Rc::new(isonomic_chat::store::MemoryStore::default()),
        install_model: None,
    };
    let (window, _shell) = open_on(cx, Some(backend));
    cx.run_until_parked();
    cx.update_window(window.into(), |_, window, cx| {
        window.render_frame(cx);
        window.click((ui::ElementId::from("model-picker"), "trigger"), cx);
    })
    .expect("the window is open");
    cx.run_until_parked();
    cx.update_window(window.into(), |_, window, cx| {
        window.render_frame(cx);
        assert!(found(window, model_picker::list_id("model-picker")));
        let chosen = window
            .try_find(model_picker::row_id("model-picker", "medium"))
            .expect("the medium row");
        assert_eq!(chosen.selected(), Some(true));
        let other = window
            .try_find(model_picker::row_id("model-picker", "small"))
            .expect("the small row");
        assert_eq!(other.selected(), Some(false));
    })
    .expect("the window is open");
}
