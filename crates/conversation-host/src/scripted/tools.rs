// Copyright 2026 Anud Inc.
// SPDX-License-Identifier: GPL-3.0-or-later

//! Tool calls a scripted reply makes before it answers, written as the
//! engine writes them: `tool.call`, then `tool.result` at once, or first an
//! `approval.request` that waits for [`decide`](crate::ConversationHost::decide)
//! or stop.

use super::Inner;
use crate::HostError;
use conversation_model::schema::{
    ApprovalDecision, ApprovalId, ApprovalRequest, ApprovalRule, Basis, Block, CallId, Decider,
    DeciderScope, Decision, Effect, ErrorInfo, Event, SessionId, ToolCall, ToolKind, ToolOutcome,
    ToolResult, Value,
};

/// One tool call a scripted reply makes before its answer, in order.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct ScriptedTool {
    /// The tool's name.
    pub tool: String,
    /// Its input.
    pub input: Value,
    /// Its declared effects.
    pub effects: Vec<Effect>,
    /// What it returns.
    pub output: String,
    /// Fail with `output` as the readable error.
    pub fails: bool,
    /// Ask the person first, with this summary.
    pub approval: Option<String>,
}

impl ScriptedTool {
    /// A call that runs without asking and returns `output`.
    pub fn new(tool: impl Into<String>, input: Value, output: impl Into<String>) -> Self {
        Self {
            tool: tool.into(),
            input,
            output: output.into(),
            ..Self::default()
        }
    }

    /// The call acts outward, so the host asks the person first, with
    /// `summary` as what will happen.
    pub fn asking(mut self, summary: impl Into<String>) -> Self {
        self.effects = vec![Effect::ActsOutward];
        self.approval = Some(summary.into());
        self
    }

    /// The call fails, with `output` as the reason.
    pub fn failing(mut self) -> Self {
        self.fails = true;
        self
    }
}

/// A call that waits for the person.
pub(super) struct Waiting {
    pub(super) approval: ApprovalId,
    pub(super) call: CallId,
    pub(super) tool: ScriptedTool,
}

const DENIED: &str = "The person denied this action.";
const STOPPED: &str = "The turn was stopped before this was approved.";

impl Inner {
    /// Make the open reply's next tool call. Returns whether there was one
    /// to make, or one still waits.
    pub(super) fn write_tool(&mut self, id: &SessionId) -> bool {
        let Some(open) = self.sessions.get_mut(id).and_then(|s| s.open.as_mut()) else {
            return false;
        };
        if open.waiting.is_some() {
            return true;
        }
        let Some(tool) = open.tools.pop_front() else {
            return false;
        };
        let (turn, step) = (open.turn.clone(), open.step.clone());
        self.next_call += 1;
        let call = CallId::new(format!("call_{:04}", self.next_call));
        let event = Event::ToolCall(ToolCall {
            call: call.clone(),
            tool: tool.tool.clone(),
            kind: ToolKind::Engine,
            input: tool.input.clone(),
            input_raw: None,
            effects: tool.effects.clone(),
            batch: None,
            child: None,
        });
        self.emit(id, Some(&turn), Some(&step), event);
        let Some(summary) = tool.approval.clone() else {
            let result = result(&call, &tool, None);
            self.emit(id, Some(&turn), Some(&step), result);
            return true;
        };
        self.next_call += 1;
        let approval = ApprovalId::new(format!("apv_{:04}", self.next_call));
        let request = Event::ApprovalRequest(ApprovalRequest {
            approval: approval.clone(),
            call: Some(call.clone()),
            summary,
            rule: ApprovalRule::ActsOutward,
            effects: tool.effects.clone(),
            expires_at: None,
        });
        self.emit(id, Some(&turn), Some(&step), request);
        if let Some(open) = self.sessions.get_mut(id).and_then(|s| s.open.as_mut()) {
            open.waiting = Some(Waiting {
                approval,
                call,
                tool,
            });
        }
        true
    }

    /// Whether a session's open reply waits for the person.
    pub(super) fn waits(&self, id: &SessionId) -> bool {
        self.sessions
            .get(id)
            .and_then(|s| s.open.as_ref())
            .is_some_and(|open| open.waiting.is_some())
    }

    /// The person answered: allow runs the call, anything else denies it.
    pub(super) fn decide(
        &mut self,
        id: &SessionId,
        approval: &ApprovalId,
        decision: Decision,
    ) -> Result<(), HostError> {
        let open = self.session(id)?.open.as_mut();
        let Some(open) =
            open.filter(|o| o.waiting.as_ref().is_some_and(|w| &w.approval == approval))
        else {
            return Err(HostError::new("not_found", "No such approval waits."));
        };
        let Some(waiting) = open.waiting.take() else {
            return Ok(());
        };
        let (turn, step) = (open.turn.clone(), open.step.clone());
        let allow = decision == Decision::Allow;
        let decided = Event::ApprovalDecision(ApprovalDecision {
            approval: waiting.approval,
            decision: if allow {
                Decision::Allow
            } else {
                Decision::Deny
            },
            basis: Basis::User,
            by: Decider {
                scope: DeciderScope::App,
                app: None,
            },
            answer: None,
            reason: None,
        });
        self.emit(id, Some(&turn), Some(&step), decided);
        let outcome = (!allow).then_some((ToolOutcome::Denied, DENIED));
        let result = result(&waiting.call, &waiting.tool, outcome);
        self.emit(id, Some(&turn), Some(&step), result);
        Ok(())
    }

    /// Stop while a call waits: the host denies it as cancelled.
    pub(super) fn cancel_waiting(&mut self, id: &SessionId, open: &mut super::Open) {
        let Some(waiting) = open.waiting.take() else {
            return;
        };
        let (turn, step) = (Some(&open.turn), Some(&open.step));
        let decided = Event::ApprovalDecision(ApprovalDecision {
            approval: waiting.approval,
            decision: Decision::Deny,
            basis: Basis::Cancelled,
            by: Decider {
                scope: DeciderScope::Host,
                app: None,
            },
            answer: None,
            reason: None,
        });
        self.emit(id, turn, step, decided);
        let outcome = Some((ToolOutcome::Cancelled, STOPPED));
        self.emit(
            id,
            turn,
            step,
            result(&waiting.call, &waiting.tool, outcome),
        );
    }
}

/// A call's result: what the tool returns, its failure, or `ended` (a
/// denial or a stop) with the reason the model is told.
fn result(call: &CallId, tool: &ScriptedTool, ended: Option<(ToolOutcome, &str)>) -> Event {
    let (outcome, text, error) = match ended {
        Some((outcome, reason)) => (outcome, reason.to_owned(), None),
        None if tool.fails => (
            ToolOutcome::Error,
            tool.output.clone(),
            Some(ErrorInfo::new("tool.error", tool.output.clone())),
        ),
        None => (ToolOutcome::Ok, tool.output.clone(), None),
    };
    Event::ToolResult(ToolResult {
        call: call.clone(),
        outcome,
        content: vec![Block::text(text)],
        error,
        truncated: None,
        duration_ms: Some(120),
    })
}
