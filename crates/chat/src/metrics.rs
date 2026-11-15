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

/// Lines the composer grows to before it scrolls.
pub(crate) const COMPOSER_ROWS: (usize, usize) = (3, 10);

/// Longest generated title, in characters.
pub(crate) const TITLE_MAX_CHARS: usize = 48;
