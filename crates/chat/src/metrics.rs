//! Chat's layout sizes. Colours, type and spacing come from the theme; the
//! few sizes that belong to Chat's layout live here and nowhere else.

use ui::{Pixels, px};

/// Width of the sidebar (the desktop's rail sits to its left).
pub(crate) fn sidebar_width() -> Pixels {
    px(232.)
}

/// Width of the context panel on the right.
pub(crate) fn context_panel_width() -> Pixels {
    px(320.)
}

/// Widest the composer, greeting and messages grow.
pub(crate) fn content_max_width() -> Pixels {
    px(800.)
}

/// Height of the header above the conversation.
pub(crate) fn header_height() -> Pixels {
    px(54.)
}

/// Longest generated title, in characters.
pub(crate) const TITLE_MAX_CHARS: usize = 48;

/// Narrower than this, the context panel gives its width to the conversation.
pub(crate) fn context_panel_breakpoint() -> Pixels {
    px(1100.)
}

/// Narrower than this, the sidebar collapses too; the header's toggle opens it.
pub(crate) fn sidebar_breakpoint() -> Pixels {
    px(860.)
}
