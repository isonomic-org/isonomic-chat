//! Chat's layout sizes, from the version 0.2 design reference
//! (`docs/design/v0.2/`). Colours, type and spacing come from the theme;
//! the few sizes that belong to Chat's layout live here and nowhere else.

use ui::{Edges, Pixels, px};

/// Width of the expanded app sidebar, its border included. Collapsed, it is
/// Isonomic UI's icon rail.
pub(crate) fn sidebar_width() -> Pixels {
    px(288.)
}

/// Width of the home column, the thread column and the composer.
pub(crate) fn content_max_width() -> Pixels {
    px(720.)
}

/// Home's bottom padding, which lifts its column above true centre.
pub(crate) fn home_lift() -> Pixels {
    px(140.)
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

/// Height of the row under home's composer, which holds the model picker.
pub(crate) fn footer_height() -> Pixels {
    px(28.)
}

/// Width of the model picker's menu (design B).
pub(crate) fn model_picker_width() -> Pixels {
    px(380.)
}

/// Width of an empty state.
pub(crate) fn empty_width() -> Pixels {
    px(360.)
}

/// Width an empty state's description wraps at.
pub(crate) fn empty_text_width() -> Pixels {
    px(300.)
}

/// An empty state's icon tile, and the icon in it.
pub(crate) fn empty_tile() -> Pixels {
    px(44.)
}

/// The icon in an empty state's tile.
pub(crate) fn empty_icon() -> Pixels {
    px(22.)
}

/// An empty state's padding above and below.
pub(crate) fn empty_padding_y() -> Pixels {
    px(28.)
}

/// An icon button: the sidebar toggle (design 0.2 `IconButton`, 28 × 28).
pub(crate) fn icon_button() -> Pixels {
    px(28.)
}

/// The box a row's status dot sits in.
pub(crate) fn status_box() -> Pixels {
    px(20.)
}

/// Width of a row's and the title's menu.
pub(crate) fn menu_width() -> Pixels {
    px(220.)
}

/// Width of the inline rename field.
pub(crate) fn rename_width() -> Pixels {
    px(300.)
}

/// How often a model on its way is asked for its state.
pub(crate) fn model_poll() -> std::time::Duration {
    std::time::Duration::from_secs(1)
}

/// Polls to wait for the window to start a download Chat asked for, before
/// offering Install again (one minute at `model_poll`).
pub(crate) const ASK_POLLS: u32 = 60;

/// A sidebar row's height (design 0.2 `SidebarNavItem`, `SidebarSessionRow`).
pub(crate) fn row_height() -> Pixels {
    px(32.)
}

/// The space between sidebar rows, and between its groups.
pub(crate) fn row_gap() -> Pixels {
    px(1.)
}

fn edges(top: f32, right: f32, bottom: f32, left: f32) -> Edges<Pixels> {
    Edges {
        top: px(top),
        right: px(right),
        bottom: px(bottom),
        left: px(left),
    }
}

/// The expanded sidebar's header and footer padding: 10 top, 12 sides.
pub(crate) fn header_padding() -> Edges<Pixels> {
    edges(10., 12., 8., 12.)
}

/// The rail's padding: 12 top, 8 sides.
pub(crate) fn rail_padding() -> Edges<Pixels> {
    edges(12., 8., 8., 8.)
}

/// The sidebar's content padding: 8 at the sides.
pub(crate) fn content_padding() -> Edges<Pixels> {
    edges(0., 8., 0., 8.)
}

/// A sidebar group label: 10 top, 4 bottom, 10 sides.
pub(crate) fn group_label_padding() -> Edges<Pixels> {
    edges(10., 10., 4., 10.)
}

/// A search palette row's height.
pub(crate) fn palette_row() -> Pixels {
    px(36.)
}

/// A model picker row's padding: 8 top and bottom, 10 sides.
pub(crate) fn picker_row_padding() -> Edges<Pixels> {
    edges(8., 10., 8., 10.)
}

/// The gap inside a model picker row.
pub(crate) fn picker_row_gap() -> Pixels {
    px(12.)
}
