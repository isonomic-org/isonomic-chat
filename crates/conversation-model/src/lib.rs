//! The conversation kit's headless models.
//!
//! [`Thread`] folds one session's durable events and live deltas into turns
//! a view can render: the prompt, the reply as it streams, the tools it used
//! and how the turn ended. [`InputState`] says what the composer offers next.
//!
//! Nothing here knows about a window, a socket or a product. Events reach the
//! model as [`Update`]s from whatever host the binding talks to.
//!
//! The event schema is harness-core's, re-exported as [`schema`]. When the
//! SDK's agents interface ships, it re-exports the same crate.

mod input;
mod models;
mod thread;
mod turn;
mod versions;

pub use harness_core as schema;
pub use input::InputState;
pub use models::{Availability, ModelCatalog, ModelOption};
pub use thread::{TOO_LARGE, Thread};
pub use turn::{Notice, ToolUse, Turn, TurnState};
pub use versions::{Branch, Path, Slot, plan};

use schema::{LiveDelta, SessionEvent};

/// One thing a host reports about a session.
#[derive(Clone, Debug, PartialEq)]
pub enum Update {
    /// A durable event, in `seq` order.
    Event(Box<SessionEvent>),
    /// A lossy live delta of a reply in progress.
    Delta(LiveDelta),
    /// The session is gone (purged): nothing more will come. Do not follow
    /// it again.
    Ended,
}

impl From<SessionEvent> for Update {
    fn from(event: SessionEvent) -> Self {
        Update::Event(Box::new(event))
    }
}

impl From<LiveDelta> for Update {
    fn from(delta: LiveDelta) -> Self {
        Update::Delta(delta)
    }
}
