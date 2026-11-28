// Copyright 2026 Anud Inc.
// SPDX-License-Identifier: GPL-3.0-or-later

//! Chat as an Isonomic app: the manifest the window shows and grants, and
//! the root view it mounts.

use conversation_host::HostError;
use conversation_sdk::{SdkHost, UserActions};
use futures::FutureExt as _;
use isonomic_app::{
    AppContext, Capabilities, Manifest, ModelAccess, Schema, SearchHit, SqlAccess, UserActionError,
};
use isonomic_proto::manifest::AgentsAccess;
use std::rc::Rc;
use ui::{AnyView, App, AppContext as _, Context, Entity, Global, IntoElement, Render, Window};

use crate::ChatShell;

/// Chat's app id. Settings and data are keyed by it.
pub const APP_ID: &str = "com.isonomic.chat";

/// Chat's icon on the window's rail: a name in Isonomic UI's icon catalog,
/// which the window loads as `icons/<name>.svg`.
pub const APP_ICON: &str = "chat";

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
            icon: APP_ICON.into(),
            // Chat's own sessions, presets and artifacts, and its own schema
            // for what only Chat stores (pins). Nothing else.
            capabilities: Capabilities {
                agents: Some(AgentsAccess {}),
                // Any local model the person has; remote ones stay off.
                models: Some(ModelAccess {
                    any: true,
                    ..Default::default()
                }),
                sql: Some(SqlAccess::default()),
                ..Default::default()
            },
            schemas: vec![Schema {
                name: crate::store::SCHEMA.into(),
            }],
        }
    }

    fn hosts_shell_header() -> bool {
        true
    }

    fn build(cx: AppContext, window: &mut Window, app: &mut App) -> AnyView {
        if !app.has_global::<KeysBound>() {
            crate::init(app);
            app.set_global(KeysBound);
        }
        let backend = crate::Backend {
            host: Rc::new(
                SdkHost::new(cx.client().agents())
                    .with_models(cx.client().models())
                    .with_user_actions(user_actions(cx.clone())),
            ),
            store: Rc::new(crate::store::SqlStore::new(cx.client().sql())),
            install_model: Some({
                let app = cx.clone();
                Rc::new(move |model: &str| {
                    app.request(isonomic_app::WindowRequest::InstallModel {
                        model: model.to_owned(),
                    })
                })
            }),
        };
        let shell = app.new(|shell| ChatShell::new(Some(backend), window, shell));
        // The window's header sits in Chat's sidebar; its rail goes.
        shell.update(app, |shell, shell_cx| {
            shell.host_shell_header(cx.clone(), shell_cx)
        });
        // Open on the composer, so the first key typed is the prompt.
        shell.update(app, |shell, cx| shell.new_chat(window, cx));
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

/// The window's user-action tokens, for the person's answers to approvals.
/// The window gives one only while the person is clicking or pressing a
/// key in Chat; otherwise the approval stays pending and they are asked to
/// answer again.
fn user_actions(cx: AppContext) -> UserActions {
    Rc::new(move |session, approval| {
        let token = cx.user_action(&session, &approval);
        async move {
            token.await.map_err(|error| {
                let message = match &error {
                    UserActionError::NoUserInput => "Allow again from this window.".to_owned(),
                    UserActionError::Engine { message, .. } => message.clone(),
                    other => other.to_string(),
                };
                HostError::new(error.code(), message)
            })
        }
        .boxed_local()
    })
}
