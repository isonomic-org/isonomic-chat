//! The words a conversation view shows. The app that mounts it may replace
//! any of them.

use ui::SharedString;

/// Every string the conversation view shows.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ConversationCopy {
    /// The empty composer's placeholder.
    pub placeholder: SharedString,
    /// The status while the host works before the first words.
    pub thinking: SharedString,
    /// The continue button under a reply that stopped early.
    pub continue_label: SharedString,
    /// The note under a reply the person stopped.
    pub stopped: SharedString,
    /// The note under a reply the output limit cut short.
    pub truncated: SharedString,
    /// Shown for a failure the host gave no words for.
    pub failed: SharedString,
    /// Shown for a refusal the host gave no words for.
    pub refused: SharedString,
    /// The note under a turn a budget stopped.
    pub limited: SharedString,
    /// The label of a block this build cannot show.
    pub unknown: SharedString,
    /// The picked model is not one this app may use.
    pub model_denied: SharedString,
    /// The picked model does not exist.
    pub model_unknown: SharedString,
    /// The picked model is not installed yet.
    pub model_unavailable: SharedString,
}

impl Default for ConversationCopy {
    fn default() -> Self {
        Self {
            placeholder: "Reply…".into(),
            thinking: "Thinking".into(),
            continue_label: "Continue".into(),
            stopped: "Stopped".into(),
            truncated: "The reply reached its length limit.".into(),
            failed: "Something went wrong. Try again.".into(),
            refused: "The model declined to answer this.".into(),
            limited: "Stopped at this conversation's limit.".into(),
            unknown: "Content this version can't show".into(),
            model_denied: "This app isn't allowed to use that model.".into(),
            model_unknown: "That model isn't available any more. Pick another.".into(),
            model_unavailable: "That model isn't installed yet. Download it, or pick another."
                .into(),
        }
    }
}
