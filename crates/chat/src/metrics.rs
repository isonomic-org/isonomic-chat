//! Chat's layout sizes, from the version 0.2 design reference
//! (`docs/design/v0.2/`). Colours, type and spacing come from the theme;
//! the few sizes that belong to Chat's layout live here and nowhere else.

use ui::{Pixels, px};

/// Width of the expanded app sidebar, its border included. Collapsed, it is
/// Isonomic UI's icon rail.
pub(crate) fn sidebar_width() -> Pixels {
    px(288.)
}

/// Width of the home column, the thread column and the composer.
pub(crate) fn content_max_width() -> Pixels {
    px(720.)
}

/// Height of the header above home and the thread.
pub(crate) fn header_height() -> Pixels {
    px(52.)
}

/// A session row's status dot.
pub(crate) fn status_dot() -> Pixels {
    px(6.)
}

/// Width of the search palette.
pub(crate) fn palette_width() -> Pixels {
    px(640.)
}

/// Distance of the search palette from the top of the window.
pub(crate) fn palette_top() -> Pixels {
    px(96.)
}

/// Longest generated title, in characters.
pub(crate) const TITLE_MAX_CHARS: usize = 48;

/// Narrower than this, the sidebar collapses to its rail: the expanded
/// sidebar and the 720 column, with 16 on each side, no longer fit. The
/// design does not show narrow windows; this follows its collapse rule.
pub(crate) fn sidebar_breakpoint() -> Pixels {
    px(1040.)
}
