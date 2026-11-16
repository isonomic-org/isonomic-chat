//! Static data for the shell and home while the kit and the host are not
//! wired. Development data only: none of it ships as a default.

use ui::SharedString;

/// One chat in the sidebar.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SessionSummary {
    /// Stable id.
    pub id: SharedString,
    /// Title shown in the sidebar and header.
    pub title: SharedString,
    /// Pinned chats sort first and show a pin.
    pub pinned: bool,
}

impl SessionSummary {
    /// A chat with an id and a title.
    pub fn new(id: impl Into<SharedString>, title: impl Into<SharedString>, pinned: bool) -> Self {
        Self {
            id: id.into(),
            title: title.into(),
            pinned,
        }
    }
}

/// The sidebar's chats: one pinned, then recent ones.
pub fn sessions() -> Vec<SessionSummary> {
    [
        ("s1", "Getting started with Isonomic Chat", true),
        ("s2", "Public-key cryptography explained", false),
        ("s3", "Refactor a slow loop", false),
        ("s4", "Explaining quantum entanglement", false),
        ("s5", "Summarize this paper", false),
        ("s6", "Ideas for a weekend away", false),
        ("s7", "Optimize a slow query", false),
        ("s8", "Write a SQL migration", false),
    ]
    .into_iter()
    .map(|(id, title, pinned)| SessionSummary::new(id, title, pinned))
    .collect()
}

/// The person's first name, for the greeting.
pub const FIRST_NAME: &str = "Omar";
