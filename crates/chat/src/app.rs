//! Chat as an Isonomic app: the manifest the window shows and grants, and
//! the root view it mounts.

use conversation_sdk::SdkHost;
use isonomic_app::{AppContext, Capabilities, Manifest, SearchHit};
use isonomic_proto::manifest::AgentsAccess;
use std::rc::Rc;
use ui::{
    AnyView, App, AppContext as _, Context, Entity, Focusable, Global, IntoElement, Render, Window,
};

use crate::ChatShell;

/// Chat's app id. Settings and data are keyed by it.
pub const APP_ID: &str = "com.isonomic.chat";

/// The prefix of Chat's deep links.
pub const LINK_PREFIX: &str = "isonomic://chat/";

/// Isonomic Chat, as the window mounts it: `isonomic_app::register::<Chat>()`.
pub struct Chat {
    app: AppContext,
    shell: Entity<ChatShell>,
    opened: Vec<String>,
}

/// Marks Chat's key bindings as registered, so a second build skips them.
struct KeysBound;

impl Global for KeysBound {}

impl Chat {
    /// What the window gave Chat.
    pub fn context(&self) -> &AppContext {
        &self.app
    }

    /// The shell: sidebar, header, panel and context panel.
    pub fn shell(&self) -> &Entity<ChatShell> {
        &self.shell
    }

    /// Deep links handed to Chat so far.
    pub fn opened(&self) -> &[String] {
        &self.opened
    }
}

impl isonomic_app::App for Chat {
    fn manifest() -> Manifest {
        Manifest {
            id: APP_ID.into(),
            name: "Chat".into(),
            version: env!("CARGO_PKG_VERSION").into(),
            icon: "message-circle".into(),
            // Chat's own sessions, presets and artifacts, and nothing else.
            capabilities: Capabilities {
                agents: Some(AgentsAccess {}),
                ..Default::default()
            },
            schemas: vec![],
        }
    }

    fn build(cx: AppContext, window: &mut Window, app: &mut App) -> AnyView {
        if !app.has_global::<KeysBound>() {
            crate::init(app);
            app.set_global(KeysBound);
        }
        let host: Rc<dyn crate::ConversationHost> = Rc::new(SdkHost::new(cx.client().agents()));
        let shell = app.new(|shell| ChatShell::new(Some(host), window, shell));
        let focus = shell.read(app).focus_handle(app);
        window.focus(&focus, app);
        app.new(|_| Chat {
            app: cx,
            shell,
            opened: Vec::new(),
        })
        .into()
    }

    fn open_link(&mut self, url: &str) -> bool {
        if !url.starts_with(LINK_PREFIX) {
            return false;
        }
        self.opened.push(url.to_owned());
        true
    }

    fn search(&mut self, _q: &str) -> Vec<SearchHit> {
        Vec::new()
    }
}

impl Render for Chat {
    fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
        self.shell.clone()
    }
}
