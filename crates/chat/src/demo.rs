// Copyright 2026 Anud Inc.
// SPDX-License-Identifier: GPL-3.0-or-later

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
    /// When anything in the chat last changed, in milliseconds since the
    /// epoch: the newest of its versions. `None` when unknown.
    pub active_at: Option<u64>,
}

impl SessionSummary {
    /// A chat with an id and a title.
    pub fn new(id: impl Into<SharedString>, title: impl Into<SharedString>, pinned: bool) -> Self {
        Self {
            id: id.into(),
            title: title.into(),
            pinned,
            active_at: None,
        }
    }

    /// The same chat, last active at `at` (milliseconds since the epoch).
    pub fn active_at(mut self, at: u64) -> Self {
        self.active_at = Some(at);
        self
    }
}

/// The sidebar's chats: one pinned, then recent ones.
pub fn sessions() -> Vec<SessionSummary> {
    const MINUTE: u64 = 60_000;
    const HOUR: u64 = 60 * MINUTE;
    const DAY: u64 = 24 * HOUR;
    let now = crate::clock::now_ms();
    [
        ("s1", "Getting started with Isonomic Chat", true, 3 * DAY),
        (
            "s2",
            "Public-key cryptography explained",
            false,
            20 * MINUTE,
        ),
        ("s3", "Refactor a slow loop", false, 2 * HOUR),
        ("s4", "Explaining quantum entanglement", false, 5 * HOUR),
        ("s5", "Summarize this paper", false, DAY),
        ("s6", "Ideas for a weekend away", false, 3 * DAY),
        ("s7", "Optimize a slow query", false, 8 * DAY),
        ("s8", "Write a SQL migration", false, 22 * DAY),
    ]
    .into_iter()
    .map(|(id, title, pinned, ago)| {
        SessionSummary::new(id, title, pinned).active_at(now.saturating_sub(ago))
    })
    .collect()
}

/// The person's first name, for the greeting.
pub const FIRST_NAME: &str = "Omar";
