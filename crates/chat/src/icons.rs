//! The icons Chat draws, all from Isonomic UI's catalog. The window must
//! register an asset source that contains them (see `ALL`).

use ui::assets::IconName;

/// Show or hide the sidebar.
pub fn toggle_sidebar() -> IconName {
    IconName::PanelLeft
}

/// Start a new chat.
pub fn new_chat() -> IconName {
    IconName::Pen
}

/// A pinned chat.
pub fn pinned() -> IconName {
    IconName::Pin
}

/// Show or hide the context panel.
pub fn toggle_context() -> IconName {
    IconName::Layers
}

/// Share a chat.
pub fn share() -> IconName {
    IconName::Share
}

/// The Context tab.
pub fn context() -> IconName {
    IconName::Layers
}

/// The Tools tab.
pub fn tools() -> IconName {
    IconName::Tools
}

/// Add to a context section.
pub fn add() -> IconName {
    IconName::Plus
}

/// Attach files in the composer.
pub fn attach() -> IconName {
    IconName::Attachment
}

/// Send from the composer.
pub fn send() -> IconName {
    IconName::Send
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
    IconName::Pen,
    IconName::Pin,
    IconName::Layers,
    IconName::Share,
    IconName::Tools,
    IconName::Plus,
    IconName::Attachment,
    IconName::Send,
    IconName::ChevronDown,
    IconName::Isonomic,
    IconName::TextCreation,
    IconName::Education,
    IconName::Code,
    IconName::Cafe,
    IconName::Idea,
    IconName::Folder,
    IconName::Notebook,
    IconName::Chat,
];
