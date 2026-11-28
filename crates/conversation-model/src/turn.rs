// Copyright 2026 Anud Inc.
// SPDX-License-Identifier: GPL-3.0-or-later

//! One turn as a view shows it.

use crate::asks::Ask;
use crate::schema::{CallId, DisplayOutcome, Seq, StepId, TurnId};
use crate::tools::{Approval, ToolUse};
use std::collections::BTreeMap;

/// Where a turn is.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum TurnState {
    /// Sent, and the host has not confirmed it yet.
    Sending,
    /// The host is working on it: thinking, calling tools or writing.
    Running,
    /// Finished.
    Done,
    /// The output limit cut the reply short; it can be continued.
    Truncated,
    /// Stopped by the person; the partial reply is kept and can be continued.
    Stopped,
    /// The model declined. The message says why; it cannot be regenerated.
    Refused(String),
    /// It failed. The message is readable.
    Failed(String),
    /// A budget stopped it.
    Limited,
    /// The host rejected the input before any step.
    Rejected(String),
}

impl TurnState {
    /// Whether more is still coming.
    pub fn is_open(&self) -> bool {
        matches!(self, TurnState::Sending | TurnState::Running)
    }

    /// Whether sending the same prompt again could succeed.
    pub fn can_retry(&self) -> bool {
        matches!(self, TurnState::Failed(_) | TurnState::Rejected(_))
    }

    /// Whether the reply can be continued from where it stopped.
    pub fn can_continue(&self) -> bool {
        matches!(self, TurnState::Stopped | TurnState::Truncated)
    }
}

/// Something in the log this build does not know how to show. It renders as
/// a generic collapsible block.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Notice {
    /// The event or block type name.
    pub kind: String,
}

/// One exchange: the prompt and everything the host did for it. A continued
/// reply stays in the turn it extends.
#[derive(Clone, Debug, PartialEq)]
pub struct Turn {
    /// The turns this exchange spans: the first, then each continuation.
    pub ids: Vec<TurnId>,
    /// The person's prompt, as text. Empty for a turn nobody typed.
    pub prompt: String,
    /// Reasoning text, committed and streaming.
    pub reasoning: String,
    /// The reply's committed text.
    pub(crate) text: String,
    /// Text streaming in for steps not yet committed, by step then block.
    pub(crate) live: BTreeMap<StepId, BTreeMap<u32, String>>,
    /// Live reasoning, likewise.
    pub(crate) live_reasoning: BTreeMap<StepId, BTreeMap<u32, String>>,
    /// Whether the next reply carries on the text before it (a continue).
    pub(crate) continuing: bool,
    /// Tools the reply used, in call order.
    pub tools: Vec<ToolUse>,
    /// Approvals the host asked for in this turn, in order.
    pub approvals: Vec<Approval>,
    /// Questions the model asked the person in this turn, in order.
    pub asks: Vec<Ask>,
    /// Things this build cannot show.
    pub notices: Vec<Notice>,
    /// The stable code of the error that ended the turn, if it failed.
    pub error_code: Option<String>,
    /// A quiet divider after this turn: older messages were summarized and
    /// the note says so. Never a message.
    pub divider_after: Option<String>,
    /// Why the host refused a request about this turn, such as continue.
    pub refusal: Option<String>,
    /// The host is retrying after a transient error; the readable reason.
    pub retrying: Option<String>,
    /// Where it is.
    pub state: TurnState,
    /// The `seq` of its first `turn.start` in its session's log; 0 until the
    /// host confirms it. A fork that shares this turn forks after it.
    pub start_seq: Seq,
    /// When the host started it, in milliseconds since the epoch.
    pub started_at: Option<u64>,
    /// How long it took from the start to the first reply that carried
    /// reasoning; `None` when the reply had no reasoning.
    pub thought_ms: Option<u64>,
    /// Bumped on every change, so a view re-renders only the turns that moved.
    pub revision: u64,
}

impl Turn {
    pub(crate) fn new(id: Option<TurnId>, prompt: String, state: TurnState) -> Self {
        Self {
            ids: id.into_iter().collect(),
            prompt,
            start_seq: 0,
            reasoning: String::new(),
            text: String::new(),
            live: BTreeMap::new(),
            live_reasoning: BTreeMap::new(),
            continuing: false,
            tools: Vec::new(),
            approvals: Vec::new(),
            asks: Vec::new(),
            notices: Vec::new(),
            error_code: None,
            divider_after: None,
            refusal: None,
            retrying: None,
            state,
            started_at: None,
            thought_ms: None,
            revision: 0,
        }
    }

    /// The reply as it stands: committed text, then anything streaming.
    pub fn reply(&self) -> String {
        let mut reply = self.text.clone();
        let streaming = self
            .live
            .values()
            .flat_map(|b| b.values())
            .any(|t| !t.is_empty());
        if streaming && !reply.is_empty() && !self.continuing && !reply.ends_with("\n\n") {
            reply.push_str("\n\n");
        }
        for blocks in self.live.values() {
            for text in blocks.values() {
                reply.push_str(text);
            }
        }
        reply
    }

    /// Reasoning as it stands, committed then streaming.
    pub fn thinking(&self) -> String {
        let mut thinking = self.reasoning.clone();
        for blocks in self.live_reasoning.values() {
            for text in blocks.values() {
                thinking.push_str(text);
            }
        }
        thinking
    }

    /// The display widgets to show under the reply, in call order: display
    /// calls that have ended, without those the model is retrying. A call
    /// still running shows nothing yet.
    pub fn widgets(&self) -> impl Iterator<Item = &ToolUse> {
        self.tools.iter().filter(|tool| {
            tool.is_display()
                && tool.outcome.is_some()
                && tool.display != Some(DisplayOutcome::Retry)
        })
    }

    /// The approval that gates a call, if one was asked for.
    pub fn approval_for(&self, call: &CallId) -> Option<&Approval> {
        self.approvals
            .iter()
            .rev()
            .find(|approval| approval.call.as_ref() == Some(call))
    }

    /// A question that still waits for the person, while the turn is open.
    pub fn open_ask(&self) -> Option<&Ask> {
        if !self.state.is_open() {
            return None;
        }
        self.asks.iter().find(|ask| ask.is_open())
    }

    /// The question a call asked, if it asked one.
    pub fn ask_for(&self, call: &CallId) -> Option<&Ask> {
        self.asks.iter().find(|ask| ask.call.as_ref() == Some(call))
    }

    /// An approval that still waits for an answer, while the turn is open.
    pub fn pending_approval(&self) -> Option<&Approval> {
        if !self.state.is_open() {
            return None;
        }
        self.approvals.iter().find(|approval| approval.is_pending())
    }

    /// Whether this turn includes the given host turn.
    pub fn spans(&self, id: &TurnId) -> bool {
        self.ids.contains(id)
    }
}
