//! The ids the window's scripted QA finds Chat's controls by.

extern crate ui as gpui;

use isonomic_chat::ChatShell;
use ui::test::TestWindowExt as _;
use ui::{
    AppContext as _, Bounds, Entity, TestAppContext, WindowBounds, WindowHandle, WindowOptions,
    base::Root, px, size,
};

fn open(cx: &mut TestAppContext) -> (WindowHandle<Root>, Entity<ChatShell>) {
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
            |window, cx| cx.new(|cx| ChatShell::new(None, window, cx)),
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
