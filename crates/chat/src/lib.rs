//! Isonomic Chat: the assistant you talk to.
//!
//! Chat is thin. It owns its screens, navigation, home, settings and preset;
//! the conversation kit (`crates/conversation-*`) owns what a conversation
//! means and how it binds to Isonomic UI. Chat never matches on session
//! events, renders a tool call or decides an approval.
//!
//! A chat sent from home runs on the [`ConversationHost`] Chat is given,
//! through the kit's `ConversationView`. The example chats in the sidebar
//! come from [`demo`]; the session list, home composer and context rows are
//! placeholders until the kit's bindings exist (see `docs/name-map.md`).

extern crate ui as gpui;

mod composer;
pub mod copy;
pub mod demo;
mod home;
pub mod icons;
mod metrics;
mod shell;

pub use conversation_host::ConversationHost;
pub use shell::{ChatShell, Route};

use ui::{App, KeyBinding};

ui::actions!(
    chat,
    [
        /// Start a new chat: go home and focus the composer.
        NewChat,
        /// Show or hide the sidebar.
        ToggleSidebar,
        /// Show or hide the context panel.
        ToggleContextPanel
    ]
);

/// The preset every new chat runs.
pub const PRESET: &str = "chat";

/// The key context of Chat's root view.
pub(crate) const KEY_CONTEXT: &str = "ChatShell";

/// Keystrokes for new chat, toggle sidebar and toggle context panel.
#[cfg(target_os = "macos")]
const KEYS: [&str; 3] = ["cmd-shift-o", "cmd-\\", "cmd-alt-\\"];
#[cfg(not(target_os = "macos"))]
const KEYS: [&str; 3] = ["ctrl-shift-o", "ctrl-\\", "ctrl-alt-\\"];

/// Registers Chat's key bindings. Call once at startup, after `ui::init`.
pub fn init(cx: &mut App) {
    let [new_chat, sidebar, context] = KEYS;
    cx.bind_keys([
        KeyBinding::new(new_chat, NewChat, Some(KEY_CONTEXT)),
        KeyBinding::new(sidebar, ToggleSidebar, Some(KEY_CONTEXT)),
        KeyBinding::new(context, ToggleContextPanel, Some(KEY_CONTEXT)),
    ]);
}
