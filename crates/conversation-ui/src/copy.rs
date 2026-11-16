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
    /// The button that sends a failed prompt again.
    pub try_again: SharedString,
    /// The copy button's name, under a prompt and a reply.
    pub copy: SharedString,
    /// The copy button under a prompt.
    pub copy_prompt: SharedString,
    /// The copy button in a code block.
    pub copy_code: SharedString,
    /// The edit button under a prompt.
    pub edit: SharedString,
    /// The name of the field that edits a prompt.
    pub edit_field: SharedString,
    /// Keep the edited prompt and send it as a new version.
    pub save: SharedString,
    /// Close the edit field and keep the prompt.
    pub cancel: SharedString,
    /// The note while the host retries after a transient error.
    pub retrying: SharedString,
    /// The note while the connection to the host is being restored.
    pub reconnecting: SharedString,
    /// The note while the app a tool needs is away.
    pub awaiting_client: SharedString,
    /// The note when the session takes no more input.
    pub closed: SharedString,
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
            stopped: "You stopped this response".into(),
            truncated: "The reply reached its length limit.".into(),
            failed: "Something went wrong. Try again.".into(),
            refused: "The model declined to answer this.".into(),
            limited: "Stopped at this conversation's limit.".into(),
            unknown: "Content this version can't show".into(),
            try_again: "Try again".into(),
            copy: "Copy".into(),
            copy_prompt: "Copy prompt".into(),
            copy_code: "Copy code".into(),
            edit: "Edit".into(),
            edit_field: "Edit message".into(),
            save: "Save".into(),
            cancel: "Cancel".into(),
            retrying: "Having trouble reaching the model. Trying again…".into(),
            reconnecting: "Reconnecting…".into(),
            awaiting_client: "Waiting for an app this chat needs to come back.".into(),
            closed: "This conversation can't continue. Start a new one.".into(),
            model_denied: "This app isn't allowed to use that model.".into(),
            model_unknown: "That model isn't available any more. Pick another.".into(),
            model_unavailable: "That model isn't installed yet. Download it, or pick another."
                .into(),
        }
    }
}
