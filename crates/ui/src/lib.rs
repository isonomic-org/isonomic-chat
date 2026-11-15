//! The single re-export of Isonomic UI.
//!
//! Every other crate in this repository depends on `ui` for its interface and
//! never names Isonomic UI or GPUI directly, so moving the UI pin (the SDK's
//! `UI_REF`) is a change to this crate alone.
//!
//! - `ui::*` is GPUI, through the Isonomic UI facade, which apps reach
//!   through the SDK (`isonomic_app::ui`) so the SDK's `UI_REF` is the pin.
//! - [`component`], [`base`] and [`assets`] are Isonomic UI's layers.
//! - [`extra`] holds the conversation components (tool call card, approval
//!   card, message action bar and the rest of the queue).
//!
//! GPUI's macros expand to paths that start with `gpui::`. A crate that uses
//! them (derives, `#[ui::test]`) adds `extern crate ui as gpui;` at its root:
//! an alias for this crate, not a dependency on GPUI.

pub use isonomic_app::ui::*;

/// The conversation components and the other components queued on the
/// Isonomic UI foundation. The SDK does not re-export them yet (requests O8),
/// so this is the one direct dependency on isonomic-ui.
pub use isonomic_ui_extra as extra;
