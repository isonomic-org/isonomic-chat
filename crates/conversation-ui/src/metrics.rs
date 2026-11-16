//! The kit's few timings and layout sizes. Colours, type and spacing come
//! from the theme.

use std::time::Duration;
use ui::{Pixels, px};

/// Widest the column of turns, the status bar and the composer grow.
pub(crate) fn column_width() -> Pixels {
    px(720.)
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
