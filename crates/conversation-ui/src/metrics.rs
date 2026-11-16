//! The kit's few timings and layout sizes. Colours, type and spacing come
//! from the theme.

use std::time::Duration;
use ui::{Pixels, px};

/// The longest a session's log is read when opening a conversation, before
/// the load fails with Try again.
pub(crate) fn fold_limit() -> Duration {
    Duration::from_secs(10)
}

/// How long to wait for the event after a fork point, which may not exist.
pub(crate) fn glance() -> Duration {
    Duration::from_millis(250)
}

/// Widest the column of turns, the status bar and the composer grow.
pub(crate) fn column_width() -> Pixels {
    px(720.)
}

/// Widest a prompt's bubble grows before its text wraps.
pub(crate) fn bubble_width() -> Pixels {
    px(560.)
}

/// A prompt bubble's corner radius (design 0.2: 18).
pub(crate) fn bubble_radius() -> Pixels {
    px(18.)
}

/// A prompt bubble's side padding (design 0.2: 16, less its 1 px border).
pub(crate) fn bubble_pad_x() -> Pixels {
    px(15.)
}

/// See [`bubble_pad_x`].
pub(crate) fn bubble_pad_y() -> Pixels {
    px(9.)
}

/// Line height of body text (16 / 24).
pub(crate) fn text_line() -> Pixels {
    px(24.)
}

/// Every heading in a reply (design 0.2 has one level, 18 / 28).
pub(crate) fn heading_size() -> Pixels {
    px(18.)
}

/// How far a bulleted item's text is set in (design 0.2: 21).
pub(crate) fn bullet_indent() -> Pixels {
    px(21.)
}

/// The space between a bullet and its text: with the bullet's own width,
/// it starts 4 in (design 0.2: 4, a 7 wide bullet, 10).
pub(crate) fn bullet_gap() -> Pixels {
    px(10.)
}

/// How far a numbered item's text is set in (design 0.2: 28).
pub(crate) fn ordered_indent() -> Pixels {
    px(28.)
}

/// The space between a number and its text (design 0.2: 10).
pub(crate) fn ordered_gap() -> Pixels {
    px(10.)
}

/// An icon button under a prompt or a reply (28 x 28).
pub(crate) fn icon_button() -> Pixels {
    px(28.)
}

/// The icon in such a button.
pub(crate) fn icon_size() -> Pixels {
    px(16.)
}

/// Width of the field that edits a sent prompt.
pub(crate) fn edit_width() -> Pixels {
    px(560.)
}

/// How long to wait before following a session again after its stream
/// ended: a quarter second, doubling with each try, at most eight seconds.
pub(crate) fn reconnect_wait(tries: u32) -> Duration {
    let millis = 250u64.saturating_mul(1 << tries.min(5));
    Duration::from_millis(millis.min(8_000))
}

#[cfg(test)]
mod tests {
    use super::reconnect_wait;
    use std::time::Duration;

    #[test]
    fn the_wait_doubles_and_stops_growing() {
        assert_eq!(reconnect_wait(0), Duration::from_millis(250));
        assert_eq!(reconnect_wait(1), Duration::from_millis(500));
        assert_eq!(reconnect_wait(5), Duration::from_millis(8_000));
        assert_eq!(reconnect_wait(30), Duration::from_millis(8_000));
    }
}
