//! The replies a scripted host gives, in order.

/// One reply the host will give, in order.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct ScriptedReply {
    /// Reasoning streamed before the answer.
    pub reasoning: Option<String>,
    /// The answer.
    pub text: String,
    /// A title set after the first turn.
    pub title: Option<String>,
    /// Decline instead of answering; `text` is the refusal.
    pub refuse: bool,
    /// Stop at the output limit after this many words; continue writes the
    /// rest.
    pub cut_after: Option<usize>,
    /// Fail before any step because the conversation no longer fits the
    /// model; the reason is written for the person.
    pub too_large: Option<String>,
}

impl ScriptedReply {
    /// A plain answer.
    pub fn text(text: impl Into<String>) -> Self {
        Self {
            text: text.into(),
            ..Self::default()
        }
    }

    /// Stream this reasoning first.
    pub fn with_reasoning(mut self, reasoning: impl Into<String>) -> Self {
        self.reasoning = Some(reasoning.into());
        self
    }

    /// Set the session's title after this reply, if it is the first.
    pub fn with_title(mut self, title: impl Into<String>) -> Self {
        self.title = Some(title.into());
        self
    }

    /// Stop at the output limit after `words` words.
    pub fn cut_after(mut self, words: usize) -> Self {
        self.cut_after = Some(words);
        self
    }

    /// A turn the host fails before any step: the conversation is too large.
    pub fn too_large(reason: impl Into<String>) -> Self {
        Self {
            too_large: Some(reason.into()),
            ..Self::default()
        }
    }

    /// A refusal with this message.
    pub fn refusal(text: impl Into<String>) -> Self {
        Self {
            text: text.into(),
            refuse: true,
            ..Self::default()
        }
    }
}
