// Copyright 2026 Anud Inc.
// SPDX-License-Identifier: GPL-3.0-or-later

//! Tool calls, their results and the approvals they wait on, folded into
//! their turn.

use crate::schema::{Event, SessionEvent};
use crate::tools::{Approval, ToolUse, output_of};
use crate::turn::Turn;

/// Fold one tool or approval event into its turn. Returns whether the turn
/// changed.
pub(super) fn fold(turn: &mut Turn, event: &SessionEvent) -> bool {
    match &event.event {
        Event::ToolCall(call) => {
            if turn.tools.iter().any(|tool| tool.call == call.call) {
                return false;
            }
            turn.tools.push(ToolUse {
                call: call.call.clone(),
                tool: call.tool.clone(),
                input: call.input.clone(),
                input_raw: call.input_raw.clone(),
                effects: call.effects.clone(),
                outcome: None,
                output: String::new(),
                started_at: event.at,
                duration_ms: None,
            });
            true
        }
        Event::ToolResult(result) => {
            let Some(tool) = turn.tools.iter_mut().find(|t| t.call == result.call) else {
                return false;
            };
            tool.outcome = Some(result.outcome.clone());
            let mut output = output_of(&result.content);
            if output.is_empty()
                && let Some(error) = &result.error
            {
                output = error.message.clone();
            }
            tool.output = output;
            tool.duration_ms = result
                .duration_ms
                .or_else(|| Some(event.at.saturating_sub(tool.started_at)));
            true
        }
        Event::ApprovalRequest(request) => {
            if turn.approvals.iter().any(|a| a.id == request.approval) {
                return false;
            }
            turn.approvals.push(Approval {
                id: request.approval.clone(),
                call: request.call.clone(),
                summary: request.summary.clone(),
                effects: request.effects.clone(),
                decision: None,
            });
            true
        }
        Event::ApprovalDecision(decision) => {
            let Some(approval) = turn
                .approvals
                .iter_mut()
                .find(|a| a.id == decision.approval)
            else {
                return false;
            };
            approval.decision = Some((decision.decision.clone(), decision.basis.clone()));
            true
        }
        _ => false,
    }
}
