//! Static data for the shell and home while the kit and the host are not
//! wired. Development data only: none of it ships as a default.

use ui::SharedString;
use ui::assets::IconName;

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

/// A connection shown in the context panel.
#[derive(Clone, Debug)]
pub struct Connection {
    /// Its name.
    pub name: SharedString,
    /// What is connected.
    pub detail: SharedString,
    /// Its icon.
    pub icon: IconName,
}

/// The context panel's connections.
pub fn connections() -> Vec<Connection> {
    [
        ("Team drive", "Connected · 3 folders", IconName::Folder),
        (
            "Notes workspace",
            "Connected · Growth strategy",
            IconName::Notebook,
        ),
        (
            "Analytics database",
            "Connected · Read only",
            IconName::Layers,
        ),
        ("Issue tracker", "Connected · 2 projects", IconName::Chat),
    ]
    .into_iter()
    .map(|(name, detail, icon)| Connection {
        name: name.into(),
        detail: detail.into(),
        icon,
    })
    .collect()
}

/// The person's first name, for the greeting.
pub const FIRST_NAME: &str = "Omar";

/// The model label in the composer.
pub const MODEL: &str = "Local model";
