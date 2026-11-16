//! Everything Chat says on screen, in one place.

/// The title of a chat before it has one, and the sidebar's new-chat entry.
pub const NEW_CHAT: &str = "New chat";
/// The wordmark at the top of the sidebar.
pub const WORDMARK: &str = "Isonomic";
/// The nav item and palette title for searching chats.
pub const SEARCH: &str = "Search";
/// The sidebar group of pinned chats.
pub const PINNED: &str = "Pinned";
/// The sidebar group of the other chats, newest first.
pub const RECENTS: &str = "Recents";
/// The palette group of actions.
pub const ACTIONS: &str = "Actions";
/// A row's menu button.
pub const MORE: &str = "More";
/// Pin a chat to the top of the sidebar.
pub const PIN: &str = "Pin";
/// Unpin a chat.
pub const UNPIN: &str = "Unpin";
/// The sidebar group that lists chats.
pub const CHATS: &str = "Chats";
/// The composer's placeholder on home.
pub const COMPOSER_HOME: &str = "How can I help you today?";
/// The composer's placeholder under a conversation.
pub const COMPOSER_THREAD: &str = "Reply…";
/// Shown under the docked composer.
pub const DISCLAIMER: &str = "Isonomic Chat can make mistakes. Please double-check responses.";
/// The header title menu.
pub const RENAME: &str = "Rename";
/// The header title menu.
pub const DELETE: &str = "Delete";
/// Shown where the conversation view will be.
pub const NO_CHATS: &str = "Chats you start appear here.";
pub const LIST_FAILED: &str = "Chats couldn't load.";
pub const TRY_AGAIN: &str = "Try again";
pub const SAVE_FAILED: &str = "A change couldn't be saved.";

pub const NO_MODEL: &str = "Chat needs a model on this computer before it can reply.";

/// The button that asks the window to install a model.
pub fn install_model(name: &str) -> String {
    format!("Install {name}")
}

pub const THREAD_PENDING: &str = "This example chat has no conversation behind it.";

/// Chat's words for the kit's conversation view.
pub fn conversation() -> conversation_ui::ConversationCopy {
    conversation_ui::ConversationCopy {
        placeholder: COMPOSER_THREAD.into(),
        ..Default::default()
    }
}
/// Accessible names for icon-only buttons.
pub const TOGGLE_SIDEBAR: &str = "Expand or collapse the sidebar";

/// The part of the day a greeting names.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DayPart {
    /// 05:00 to 11:59.
    Morning,
    /// 12:00 to 17:59.
    Afternoon,
    /// 18:00 to 04:59.
    Evening,
}

impl DayPart {
    /// The part of the day for an hour from 0 to 23. Out-of-range hours wrap.
    pub fn from_hour(hour: u32) -> Self {
        match hour % 24 {
            5..=11 => Self::Morning,
            12..=17 => Self::Afternoon,
            _ => Self::Evening,
        }
    }
}

/// The home greeting, such as "Good morning, Omar". Without a name it is
/// just "Good morning".
pub fn greeting(part: DayPart, name: Option<&str>) -> String {
    let salutation = match part {
        DayPart::Morning => "Good morning",
        DayPart::Afternoon => "Good afternoon",
        DayPart::Evening => "Good evening",
    };
    match name.map(str::trim).filter(|name| !name.is_empty()) {
        Some(name) => format!("{salutation}, {name}"),
        None => salutation.to_string(),
    }
}

/// A starter category under the home composer and the prompts it offers.
pub struct Starter {
    /// The chip's label.
    pub label: &'static str,
    /// Example prompts; picking one fills the composer.
    pub prompts: &'static [&'static str],
}

/// The starter categories, in order.
pub const STARTERS: &[Starter] = &[
    Starter {
        label: "Write",
        prompts: &[
            "Draft a short, friendly reply declining a meeting",
            "Turn these notes into a clear project update",
            "Write a product description from a list of features",
        ],
    },
    Starter {
        label: "Learn",
        prompts: &[
            "Explain public-key cryptography in plain terms",
            "Teach me the basics of statistics with examples",
            "What should I know before reading about compilers?",
        ],
    },
    Starter {
        label: "Code",
        prompts: &[
            "Review this function for bugs and edge cases",
            "Write a SQL migration that adds an index safely",
            "Explain what this error message means",
        ],
    },
    Starter {
        label: "Life stuff",
        prompts: &[
            "Plan a relaxed weekend away within a small budget",
            "Help me prepare questions for a doctor's visit",
            "Suggest a weekly routine for learning a language",
        ],
    },
    Starter {
        label: "Brainstorm",
        prompts: &[
            "Give me ten names for a neighbourhood book club",
            "Ideas for a small team offsite that isn't awkward",
            "Ways to make a long report easier to read",
        ],
    },
];

/// A chat title from the first message: its first line, cut at a word
/// boundary to at most `max` characters with an ellipsis.
pub fn title_from(text: &str, max: usize) -> String {
    let line = text.lines().map(str::trim).find(|line| !line.is_empty());
    let Some(line) = line else {
        return NEW_CHAT.to_string();
    };
    if line.chars().count() <= max {
        return line.to_string();
    }
    let cut: String = line.chars().take(max.saturating_sub(1)).collect();
    let cut = match cut.rfind(char::is_whitespace) {
        Some(at) if at > 0 => cut[..at].trim_end().to_string(),
        _ => cut,
    };
    format!("{cut}…")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day_parts_cover_every_hour() {
        assert_eq!(DayPart::from_hour(4), DayPart::Evening);
        assert_eq!(DayPart::from_hour(5), DayPart::Morning);
        assert_eq!(DayPart::from_hour(11), DayPart::Morning);
        assert_eq!(DayPart::from_hour(12), DayPart::Afternoon);
        assert_eq!(DayPart::from_hour(17), DayPart::Afternoon);
        assert_eq!(DayPart::from_hour(18), DayPart::Evening);
        assert_eq!(DayPart::from_hour(23), DayPart::Evening);
        assert_eq!(DayPart::from_hour(29), DayPart::Morning);
    }

    #[test]
    fn greeting_names_the_person_when_known() {
        assert_eq!(
            greeting(DayPart::Morning, Some("Omar")),
            "Good morning, Omar"
        );
        assert_eq!(greeting(DayPart::Evening, Some("  ")), "Good evening");
        assert_eq!(greeting(DayPart::Afternoon, None), "Good afternoon");
    }

    #[test]
    fn titles_come_from_the_first_line_and_stop_at_a_word() {
        assert_eq!(title_from("\n  Hello there\nmore", 40), "Hello there");
        assert_eq!(title_from("   \n ", 40), NEW_CHAT);
        assert_eq!(
            title_from(
                "Explain how public-key cryptography works in plain terms",
                32
            ),
            "Explain how public-key…"
        );
        assert!(title_from(&"x".repeat(100), 10).chars().count() <= 10);
    }

    #[test]
    fn every_starter_offers_prompts() {
        assert_eq!(STARTERS.len(), 5);
        assert!(STARTERS.iter().all(|starter| !starter.prompts.is_empty()));
    }
}
