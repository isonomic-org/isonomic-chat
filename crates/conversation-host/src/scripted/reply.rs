// Copyright 2026 Anud Inc.
// SPDX-License-Identifier: GPL-3.0-or-later

//! The replies a scripted host gives, in order.

/// One reply the host will give, in order.
#[derive(Clone, Debug, Default, PartialEq)]
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
    /// The engine fails part way through the answer.
    pub fail: Option<Failure>,
    /// Tool calls made before the answer, in order.
    pub tools: Vec<super::ScriptedTool>,
}

/// An engine error part way through a reply.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Failure {
    /// Words of the answer written before it fails.
    pub after: usize,
    /// The error's code and readable message.
    pub code: String,
    pub message: String,
    /// The engine tries the request again once, from the start.
    pub retry: bool,
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

    /// The engine fails after `words` words of the answer, with `message`;
    /// the turn ends failed.
    pub fn fail_after(mut self, words: usize, message: impl Into<String>) -> Self {
        self.fail = Some(Failure {
            after: words,
            code: "model.error".into(),
            message: message.into(),
            retry: false,
        });
        self
    }

    /// The engine fails after `words` words with a passing error, then tries
    /// again and writes the whole answer.
    pub fn retry_after(mut self, words: usize, message: impl Into<String>) -> Self {
        self.fail = Some(Failure {
            after: words,
            code: "model.overloaded".into(),
            message: message.into(),
            retry: true,
        });
        self
    }

    /// Make this tool call before answering.
    pub fn with_tool(mut self, tool: super::ScriptedTool) -> Self {
        self.tools.push(tool);
        self
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
