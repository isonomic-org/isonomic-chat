// Copyright 2026 Anud Inc.
// SPDX-License-Identifier: GPL-3.0-or-later

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
    /// The reasoning row once the reply is written, when its time is unknown.
    pub thought: SharedString,
    /// The reasoning row after about one second.
    pub thought_second: SharedString,
    /// The reasoning row after some seconds; `{n}` is the count.
    pub thought_seconds: SharedString,
    /// The reasoning row after about one minute.
    pub thought_minute: SharedString,
    /// The reasoning row after some minutes; `{n}` is the count.
    pub thought_minutes: SharedString,
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
    /// A conversation whose log could not be read when it was opened.
    pub load_failed: SharedString,
    /// The status bar while a reply is written.
    pub working: SharedString,
    /// Hide the status bar.
    pub dismiss: SharedString,
    /// Write a stopped reply again.
    pub retry: SharedString,
    /// Carry on with a stopped reply.
    pub resume: SharedString,
    /// Stop the reply being written.
    pub stop: SharedString,
    /// A tool call's row while it waits for the person's approval.
    pub tool_waiting: SharedString,
    /// A tool call's row once the person denied it.
    pub tool_denied: SharedString,
    /// A tool call's row once the reply was stopped before it ran.
    pub tool_stopped: SharedString,
    /// The status bar while a call waits for the person's approval.
    pub waiting_approval: SharedString,
    /// An approval card's title; `{name}` is the operation or tool.
    pub approval_title: SharedString,
    /// The title of an approval that gates no tool call.
    pub approval_lone: SharedString,
    /// The label of the connection on an approval card.
    pub approval_app: SharedString,
    /// The label of the operation on an approval card.
    pub approval_tool: SharedString,
    /// Allow the call.
    pub allow: SharedString,
    /// Deny the call.
    pub deny: SharedString,
    /// The picked model is not one this app may use.
    pub model_denied: SharedString,
    /// The picked model does not exist.
    pub model_unknown: SharedString,
    /// The picked model is not installed yet.
    pub model_unavailable: SharedString,
    /// The status while a question waits for the person.
    pub waiting_answer: SharedString,
    /// An asked question's row, when the person gave no answer.
    pub not_answered: SharedString,
    /// A table the model showed with no rows.
    pub empty_table: SharedString,
}

impl Default for ConversationCopy {
    fn default() -> Self {
        Self {
            placeholder: "Reply…".into(),
            thinking: "Thinking".into(),
            thought: "Thought".into(),
            thought_second: "Thought for 1 second".into(),
            thought_seconds: "Thought for {n} seconds".into(),
            thought_minute: "Thought for 1 minute".into(),
            thought_minutes: "Thought for {n} minutes".into(),
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
            load_failed: "This chat couldn't load.".into(),
            working: "Working".into(),
            dismiss: "Dismiss".into(),
            retry: "Retry".into(),
            resume: "Resume".into(),
            stop: "Stop".into(),
            tool_waiting: "Waiting for your approval".into(),
            tool_denied: "Denied".into(),
            tool_stopped: "Stopped before it ran".into(),
            waiting_approval: "Waiting for your approval".into(),
            approval_title: "Allow {name}?".into(),
            approval_lone: "Allow this action?".into(),
            approval_app: "App".into(),
            approval_tool: "Action".into(),
            allow: "Allow".into(),
            deny: "Deny".into(),
            model_denied: "This app isn't allowed to use that model.".into(),
            model_unknown: "That model isn't available any more. Pick another.".into(),
            empty_table: "Nothing to show.".into(),
            waiting_answer: "Waiting for your answer".into(),
            not_answered: "Not answered".into(),
            model_unavailable: "That model isn't installed yet. Download it, or pick another."
                .into(),
        }
    }
}

impl ConversationCopy {
    /// An approval card's title for an operation or tool.
    pub fn approve_title(&self, name: &str) -> SharedString {
        self.approval_title.replace("{name}", name).into()
    }

    /// The reasoning row's words for a reply that took `ms` to arrive:
    /// whole seconds (at least one) under a minute, then whole minutes.
    pub fn thought_for(&self, ms: Option<u64>) -> SharedString {
        let Some(ms) = ms else {
            return self.thought.clone();
        };
        let seconds = ms.div_ceil(1000).max(1);
        let (one, many, n) = if seconds < 60 {
            (&self.thought_second, &self.thought_seconds, seconds)
        } else {
            (
                &self.thought_minute,
                &self.thought_minutes,
                (seconds + 30) / 60,
            )
        };
        if n == 1 {
            one.clone()
        } else {
            many.replace("{n}", &n.to_string()).into()
        }
    }
}
