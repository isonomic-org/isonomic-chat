//! The conversation kit's bindings to Isonomic UI.
//!
//! [`ConversationView`] is one conversation: its turns in a virtualized list,
//! each reply streaming in as Markdown, and Isonomic UI's prompt composer
//! below, whose send turns into stop while a reply is written. A reply that
//! stopped early offers continue; a finished one can be tried again, and
//! the pager under it switches between its versions. The view talks to the platform only
//! through a [`ConversationHost`](conversation_host::ConversationHost).
//!
//! The app that mounts it supplies the words ([`ConversationCopy`]) and the
//! preset; the kit carries no product terms.

extern crate ui as gpui;

mod copy;
mod metrics;
mod status;
mod turn;
mod view;

pub use copy::ConversationCopy;
pub use status::{STATUS_ID, StatusButton};
pub use view::{ConversationEvent, ConversationView};
