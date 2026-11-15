//! Chat in a standalone window, for development. Static data until the kit
//! and the SDK adapter are wired.

use isonomic_chat::ChatShell;
use ui::assets::AllAssets;
use ui::{AppContext as _, Bounds, Focusable as _, WindowBounds, WindowOptions, px, size};

fn main() {
    ui::application().with_assets(AllAssets).run(|cx| {
        ui::init(cx);
        isonomic_chat::init(cx);
        cx.activate(true);
        let bounds = Bounds::centered(None, size(px(1440.), px(900.)), cx);
        ui::open_window(
            WindowOptions {
                window_bounds: Some(WindowBounds::Windowed(bounds)),
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
        .expect("failed to open the Chat window");
    });
}
