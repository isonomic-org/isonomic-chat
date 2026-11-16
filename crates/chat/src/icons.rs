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

/// The four-point star beside the greeting and in empty states: the
/// closest drawing in the set to the design's sparkle (`Sparkles` draws a
/// wand).
pub fn greeting() -> IconName {
    IconName::Beacon
}

/// Open a chat, in a row's menu.
pub fn open() -> IconName {
    IconName::ArrowUpRight
}

/// Pin a chat.
pub fn pin() -> IconName {
    IconName::Pin
}

/// Unpin a chat.
pub fn unpin() -> IconName {
    IconName::PinFilled
}

/// Rename a chat.
pub fn rename() -> IconName {
    IconName::Edit
}

/// Delete a chat.
pub fn delete() -> IconName {
    IconName::Delete
}

/// Install a model.
pub fn install() -> IconName {
    IconName::Download
}

/// Every icon Chat draws, for a host that registers only the icons in use.
pub const ALL: &[IconName] = &[
    IconName::PanelLeft,
    IconName::Plus,
    IconName::Search,
    IconName::Chat,
    IconName::Ellipsis,
    IconName::ChevronDown,
    IconName::Beacon,
    IconName::Download,
    IconName::ArrowUpRight,
    IconName::Pin,
    IconName::PinFilled,
    IconName::Delete,
    // Drawn by the conversation kit: copy and copied, try again and the
    // version pager, edit, the stopped note and the status bar's icons.
    IconName::Copy,
    IconName::RefreshCw,
    IconName::ChevronLeft,
    IconName::ChevronRight,
    IconName::Edit,
    IconName::Check,
    IconName::Stop,
    IconName::CircleAlert,
    IconName::Info,
    // The run status bar's: stopped, cut short, and the reconnecting spinner.
    IconName::Square,
    IconName::TriangleAlert,
    IconName::Loader,
];
