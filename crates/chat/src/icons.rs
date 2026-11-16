//! The icons Chat draws, all from Isonomic UI's catalog. The window must
//! register an asset source that contains them (see `ALL`).

use ui::assets::IconName;

/// Expand or collapse the sidebar.
pub fn toggle_sidebar() -> IconName {
    IconName::PanelLeft
}

/// Start a new chat.
pub fn new_chat() -> IconName {
    IconName::Plus
}

/// Search chats.
pub fn search() -> IconName {
    IconName::Search
}

/// A chat in the sidebar.
pub fn chat() -> IconName {
    IconName::Chat
}

/// A row's menu.
pub fn more() -> IconName {
    IconName::Ellipsis
}

/// The sparkle beside the greeting and in empty states.
pub fn greeting() -> IconName {
    IconName::Sparkles
}

/// Every icon Chat draws, for a host that registers only the icons in use.
pub const ALL: &[IconName] = &[
    IconName::PanelLeft,
    IconName::Plus,
    IconName::Search,
    IconName::Chat,
    IconName::Ellipsis,
    IconName::ChevronDown,
    IconName::Sparkles,
];
