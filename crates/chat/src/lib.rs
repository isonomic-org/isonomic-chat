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

mod app;
mod clock;
mod composer;
pub mod copy;
pub mod demo;
mod home;
pub mod icons;
mod metrics;
mod shell;
pub mod store;

pub use app::{APP_ICON, APP_ID, Chat, LINK_PREFIX};
pub use conversation_host::ConversationHost;

use std::rc::Rc;

/// What Chat runs on: the host for sessions, and the store for Chat's own
/// state.
#[derive(Clone)]
pub struct Backend {
    /// Sessions, titles and history.
    pub host: Rc<dyn ConversationHost>,
    /// Pins.
    pub store: Rc<dyn store::Store>,
    /// Ask the window to install a model; `true` if it took the request.
    pub install_model: Option<InstallModel>,
}

/// The window's model install step, as Chat asks for it.
pub type InstallModel = Rc<dyn Fn(&str) -> bool>;
pub use shell::{ChatShell, Route};

use ui::{App, KeyBinding};

ui::actions!(
    chat,
    [
        /// Start a new chat: go home and focus the composer.
        NewChat,
        /// Expand the sidebar or collapse it to its rail.
        ToggleSidebar,
        /// Open the search palette.
        SearchChats,
        /// Open the chat above the open one in the sidebar.
        PreviousChat,
        /// Open the chat below the open one in the sidebar.
        NextChat
    ]
);

/// The preset every new chat runs.
pub const PRESET: &str = "chat";

/// The key context of Chat's root view.
pub(crate) const KEY_CONTEXT: &str = "ChatShell";

/// Keystrokes for new chat, toggle sidebar, search, the chat above and below
/// (design 0.2 keyboard shortcuts), and a second toggle for the sidebar.
#[cfg(target_os = "macos")]
const KEYS: [&str; 6] = [
    "cmd-shift-o",
    "cmd-.",
    "cmd-k",
    "alt-up",
    "alt-down",
    "cmd-b",
];
#[cfg(not(target_os = "macos"))]
const KEYS: [&str; 6] = [
    "ctrl-shift-o",
    "ctrl-.",
    "ctrl-k",
    "alt-up",
    "alt-down",
    "ctrl-b",
];

/// The new-chat keystroke, as the search palette shows it.
pub(crate) const NEW_CHAT_KEY: &str = KEYS[0];

/// Registers Chat's key bindings. Call once at startup, after `ui::init`.
pub fn init(cx: &mut App) {
    let [new_chat, sidebar, search, previous, next, sidebar_too] = KEYS;
    cx.bind_keys([
        KeyBinding::new(new_chat, NewChat, Some(KEY_CONTEXT)),
        KeyBinding::new(sidebar, ToggleSidebar, Some(KEY_CONTEXT)),
        // Mod-. is the design's key, but on macOS Cmd-. also means cancel;
        // Mod-B toggles the sidebar too.
        KeyBinding::new(sidebar_too, ToggleSidebar, Some(KEY_CONTEXT)),
        KeyBinding::new(search, SearchChats, Some(KEY_CONTEXT)),
        KeyBinding::new(previous, PreviousChat, Some(KEY_CONTEXT)),
        KeyBinding::new(next, NextChat, Some(KEY_CONTEXT)),
    ]);
}
