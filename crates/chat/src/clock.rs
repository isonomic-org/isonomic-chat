//! The wall clock, for how long ago a chat was active.

use std::time::{SystemTime, UNIX_EPOCH};

/// Milliseconds since the epoch, now.
pub(crate) fn now_ms() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_or(0, |since| since.as_millis() as u64)
}
