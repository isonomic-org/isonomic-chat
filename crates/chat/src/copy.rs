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
/// The search palette's placeholder.
pub const SEARCH_PLACEHOLDER: &str = "Search or ask…";
/// The search palette's footer hints.
pub const PALETTE_HINTS: [&str; 2] = ["↑↓ Navigate", "↵ Open"];
/// The search palette when nothing matches.
pub const NO_MATCHES: &str = "No chats match.";
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
pub const COMPOSER_THREAD: &str = "Reply to Isonomic…";
/// Shown under the docked composer.
pub const DISCLAIMER: &str = "Isonomic is AI and can make mistakes. Please double-check responses.";
/// The header title menu.
pub const RENAME: &str = "Rename";
/// The header title menu.
pub const DELETE: &str = "Delete";
/// Shown where the conversation view will be.
pub const NO_CHATS: &str = "Start a chat and Isonomic will keep track of it here.";
pub const LIST_FAILED: &str = "Chats couldn't load.";
pub const TRY_AGAIN: &str = "Try again";
pub const SAVE_FAILED: &str = "A change couldn't be saved.";

pub const NO_MODEL: &str = "Chat needs a model on this computer before it can reply.";
/// The title of home's empty state when no chat model is installed.
pub const NO_MODEL_TITLE: &str = "Install a model to start";
/// The model picker's header.
pub const MODEL: &str = "Model";
/// A model's note in the picker: the engine recommends it.
pub const MODEL_RECOMMENDED: &str = "Recommended";
/// A model's note in the picker: its file is not on this computer.
pub const MODEL_NOT_INSTALLED: &str = "Not installed";
/// A model's note in the picker: its file is being fetched.
pub const MODEL_DOWNLOADING: &str = "Downloading";
/// A model's note in the picker: the engine stopped trying it.
pub const MODEL_UNAVAILABLE: &str = "Unavailable";

/// Home's line when the models could not be listed.
pub const MODELS_FAILED: &str = "Models couldn't load.";

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
/// The sidebar toggle's name while the sidebar is expanded.
pub const COLLAPSE_SIDEBAR: &str = "Collapse sidebar";
/// The sidebar toggle's name while the sidebar is a rail.
pub const EXPAND_SIDEBAR: &str = "Expand sidebar";
/// A row's menu item that opens the chat.
pub const OPEN: &str = "Open";
/// The rename field's name.
pub const CHAT_TITLE: &str = "Chat title";
/// The sidebar's Try again, named for what it retries.
pub const RETRY_CHATS: &str = "Try loading chats again";

/// A row's More button, named for its chat.
pub fn more_for(title: &str) -> String {
    format!("More actions for {title}")
}

/// A running chat's row, named so its state is heard.
pub fn running(title: &str) -> String {
    format!("{title}, writing a reply")
}

/// The header's title button, named for the menu it opens.
pub fn chat_options(title: &str) -> String {
    format!("{title}, chat options")
}

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
}
