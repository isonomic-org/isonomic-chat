//! The single re-export of Isonomic UI.
//!
//! Every other crate in this repository depends on `ui` for its interface and
//! never names Isonomic UI or GPUI directly, so moving the pinned commit in
//! `UI_REF` is a change to this crate alone.
//!
//! - `ui::*` is GPUI, through the Isonomic UI facade.
//! - [`component`], [`base`] and [`assets`] are Isonomic UI's layers.
//! - [`extra`] holds the conversation components (tool call card, approval
//!   card, message action bar and the rest of the queue).
//!
//! GPUI's macros expand to paths that start with `gpui::`. A crate that uses
//! them (derives, `#[ui::test]`) adds `extern crate ui as gpui;` at its root:
//! an alias for this crate, not a dependency on GPUI.

pub use isonomic_ui::*;

/// The conversation components and the other components queued on the
/// Isonomic UI foundation.
pub use isonomic_ui_extra as extra;
