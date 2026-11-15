//! Everything Chat says on screen, in one place.

/// The title of a chat before it has one, and the sidebar's new-chat entry.
pub const NEW_CHAT: &str = "New chat";
/// The sidebar group that lists chats.
pub const CHATS: &str = "Chats";
/// The composer's placeholder on home.
pub const COMPOSER_HOME: &str = "How can I help you today?";
/// The composer's placeholder under a conversation.
pub const COMPOSER_THREAD: &str = "Reply…";
/// The composer's send button.
pub const SEND: &str = "Send";
/// The composer's attach button.
pub const ADD_FILES: &str = "Add files";
/// Shown under the docked composer.
pub const DISCLAIMER: &str = "Isonomic Chat can make mistakes. Please double-check responses.";
/// The header's share button.
pub const SHARE: &str = "Share";
/// Why Share is disabled.
pub const SHARE_LATER: &str = "Sharing comes in a later release";
/// The header title menu.
pub const RENAME: &str = "Rename";
/// The header title menu.
pub const DELETE: &str = "Delete";
/// The context panel's tabs.
pub const CONTEXT: &str = "Context";
/// The context panel's tabs.
pub const TOOLS: &str = "Tools";
/// A context panel section.
pub const FILES: &str = "Files";
/// A context panel section.
pub const CONNECTIONS: &str = "Connections";
/// The Files section with nothing in it.
pub const NO_FILES: &str = "Files you add to this chat appear here.";
/// The Tools tab with nothing in it.
pub const NO_TOOLS: &str = "Tools this chat can use appear here.";
/// Shown where the conversation view will be.
pub const THREAD_PENDING: &str = "This example chat has no conversation behind it.";

/// Chat's words for the kit's conversation view.
pub fn conversation() -> conversation_ui::ConversationCopy {
    conversation_ui::ConversationCopy {
        placeholder: COMPOSER_THREAD.into(),
        ..Default::default()
    }
}
/// Accessible names for icon-only buttons.
pub const TOGGLE_SIDEBAR: &str = "Show or hide the sidebar";
/// Accessible names for icon-only buttons.
pub const TOGGLE_CONTEXT: &str = "Show or hide the context panel";
/// Accessible names for icon-only buttons.
pub const ADD: &str = "Add";

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
