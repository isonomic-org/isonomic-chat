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

/// The mark beside the greeting.
pub fn mark() -> IconName {
    IconName::Isonomic
}

/// The icon of a starter chip, by its position in `copy::STARTERS`.
pub fn starter(index: usize) -> IconName {
    match index {
        0 => IconName::TextCreation,
        1 => IconName::Education,
        2 => IconName::Code,
        3 => IconName::Cafe,
        _ => IconName::Idea,
    }
}

/// Every icon Chat draws, for a host that registers only the icons in use.
pub const ALL: &[IconName] = &[
    IconName::PanelLeft,
    IconName::Plus,
    IconName::Search,
    IconName::Chat,
    IconName::Ellipsis,
    IconName::ChevronDown,
    IconName::Isonomic,
    IconName::TextCreation,
    IconName::Education,
    IconName::Code,
    IconName::Cafe,
    IconName::Idea,
];
