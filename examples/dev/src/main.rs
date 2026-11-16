//! Chat in a standalone window, for development. Chats sent from home run on
//! the kit's scripted host, which streams a few canned replies word by word
//! until the SDK adapter lands.

use std::rc::Rc;
use std::time::Duration;

use conversation_host::{ScriptedHost, ScriptedReply};
use isonomic_chat::{ChatShell, ConversationHost};
use ui::assets::AllAssets;
use ui::{AppContext as _, Bounds, Focusable as _, WindowBounds, WindowOptions, px, size};

/// How often the scripted host writes the next word.
const WORD_EVERY: Duration = Duration::from_millis(60);

/// Replies the dev window plays, in order.
fn script(host: &ScriptedHost) {
    let replies = [
        ScriptedReply::text(
            "Here are three ideas:\n\n1. **Start small.** Pick one thing to do today.\n2. **Write it down.** A short list beats a long plan.\n3. **Review on Friday.** Keep what worked.",
        )
        .with_reasoning("A short, practical list fits best.")
        .with_title("Getting started"),
        ScriptedReply::text(
            "Sure. A longer answer streams in word by word, so you can stop it part way and continue it from where it stopped. Try the stop button now, then continue.",
        ),
        ScriptedReply::refusal("I can't help with that, but I'm happy to help with something else."),
    ];
    for _ in 0..20 {
        for reply in &replies {
            host.script(reply.clone());
        }
    }
}

fn main() {
    ui::application().with_assets(AllAssets).run(|cx| {
        ui::init(cx);
        isonomic_chat::init(cx);
        cx.activate(true);
        let host = ScriptedHost::new();
        script(&host);
        let pump = host.clone();
        cx.spawn(async move |cx| {
            loop {
                cx.background_executor().timer(WORD_EVERY).await;
                pump.pump(1);
            }
        })
        .detach();
        let bounds = Bounds::centered(None, size(px(1440.), px(900.)), cx);
        ui::open_window(
            WindowOptions {
                window_bounds: Some(WindowBounds::Windowed(bounds)),
                ..Default::default()
            },
            cx,
            move |window, cx| {
                let host: Rc<dyn ConversationHost> = Rc::new(host);
                let shell = cx.new(|cx| {
                    ChatShell::new(
                        Some(isonomic_chat::Backend {
                            host,
                            store: Rc::new(isonomic_chat::store::MemoryStore::default()),
                            install_model: None,
                        }),
                        window,
                        cx,
                    )
                });
                let focus = shell.read(cx).focus_handle(cx);
                window.focus(&focus, cx);
                shell
            },
        )
        .expect("failed to open the Chat window");
    });
}
