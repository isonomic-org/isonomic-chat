// Copyright 2026 Anud Inc.
// SPDX-License-Identifier: GPL-3.0-or-later

//! A reply's tool calls, one compact row each, and the approval a call
//! waits on: generic over every tool, whatever its name. The row names the
//! connection and the operation when the call's input carries them, and
//! folds open to the arguments and the result.

use conversation_model::schema::{ApprovalId, CallId, Decision, Effect, ToolOutcome};
use conversation_model::{Approval, ToolUse, Turn, pretty, summary_lines};
use std::rc::Rc;
use std::time::Duration;
use ui::extra::approval_card::{ApprovalCard, ApprovalRisk, ApprovalStatus};
use ui::extra::tool_call_card::{ToolCallCard, ToolCallStatus};
use ui::prelude::*;
use ui::{AnyElement, App, ElementId, SharedString, Window, div};

use crate::{ConversationCopy, metrics};

/// A handler for the person's answer to an approval.
pub(crate) type DecideHandler = Rc<dyn Fn(ApprovalId, Decision, &mut Window, &mut App)>;

/// The element id of a call's row.
pub fn tool_row_id(call: &CallId) -> ElementId {
    ElementId::Name(format!("tool-{}", call.as_str()).into())
}

/// The element id of an approval's card. Its buttons are its children
/// `"approve"` and `"deny"`.
pub fn approval_card_id(approval: &ApprovalId) -> ElementId {
    ElementId::Name(format!("approval-{}", approval.as_str()).into())
}

/// The rows of a turn's tool calls, each followed by the card of an
/// approval it waits on. `deciding` are the approvals whose answer is on
/// its way.
pub(crate) fn render_tools(
    turn: &Turn,
    copy: &ConversationCopy,
    on_decide: Option<&DecideHandler>,
    deciding: &[ApprovalId],
) -> Vec<AnyElement> {
    let mut rows = Vec::new();
    for tool in &turn.tools {
        let approval = turn
            .approval_for(&tool.call)
            .filter(|a| a.is_pending() && turn.state.is_open());
        rows.push(row(tool, approval.is_some(), copy).into_any_element());
        if let Some(approval) = approval {
            let busy = deciding.contains(&approval.id);
            rows.push(card(tool, approval, copy, on_decide, busy).into_any_element());
        }
    }
    // An approval for no call of this turn, such as one the host asks
    // about the whole session.
    let orphans = turn.approvals.iter().filter(|a| {
        a.is_pending()
            && turn.state.is_open()
            && !turn.tools.iter().any(|t| a.call.as_ref() == Some(&t.call))
    });
    for approval in orphans {
        let busy = deciding.contains(&approval.id);
        rows.push(lone_card(approval, copy, on_decide, busy).into_any_element());
    }
    rows
}

/// The name a call goes by: its operation, else the tool's name.
fn name(tool: &ToolUse) -> String {
    tool.target().operation.unwrap_or_else(|| tool.tool.clone())
}

fn row(tool: &ToolUse, waiting: bool, copy: &ConversationCopy) -> impl IntoElement {
    let target = tool.target();
    let status = match &tool.outcome {
        None => ToolCallStatus::Running,
        Some(ToolOutcome::Ok) => ToolCallStatus::Success,
        Some(_) => ToolCallStatus::Failed,
    };
    let summary = match (&tool.outcome, target.connection) {
        (None, _) if waiting => Some(copy.tool_waiting.clone()),
        (Some(ToolOutcome::Denied), _) => Some(copy.tool_denied.clone()),
        (Some(ToolOutcome::Cancelled), _) => Some(copy.tool_stopped.clone()),
        (_, Some(connection)) => Some(SharedString::from(connection)),
        _ => None,
    };
    let mut card = ToolCallCard::new(tool_row_id(&tool.call), name(tool), status)
        // Arguments and results stay folded until the person opens them,
        // a failed call too.
        .open_on_error(false)
        .arguments(match tool.input_raw.clone() {
            Some(raw) => raw,
            None => pretty(&target.arguments),
        });
    if let Some(summary) = summary {
        card = card.summary(summary);
    }
    if let Some(ms) = tool.duration_ms {
        card = card.duration(Duration::from_millis(ms));
    }
    if !tool.output.is_empty() {
        card = if tool.failed() {
            card.error(tool.output.clone())
        } else {
            card.result(tool.output.clone())
        };
    }
    div().w_full().child(card)
}

/// How risky an approval is, from the effects at stake.
fn risk(effects: &[Effect]) -> ApprovalRisk {
    if effects.contains(&Effect::ExecutesCode) {
        ApprovalRisk::High
    } else if effects.contains(&Effect::ActsOutward) {
        ApprovalRisk::Medium
    } else {
        ApprovalRisk::Low
    }
}

/// The card for an approval a call waits on: which connection, which
/// operation, and its arguments, one per line.
fn card(
    tool: &ToolUse,
    approval: &Approval,
    copy: &ConversationCopy,
    on_decide: Option<&DecideHandler>,
    busy: bool,
) -> impl IntoElement {
    let target = tool.target();
    let mut card = base(
        approval,
        copy.approve_title(&name(tool)),
        copy,
        on_decide,
        busy,
    );
    if let Some(connection) = target.connection {
        card = card.detail(copy.approval_app.clone(), connection);
    }
    card = card.detail(copy.approval_tool.clone(), name(tool));
    for (field, value) in summary_lines(&target.arguments) {
        card = card.detail(field, metrics::clip(&value));
    }
    div().w_full().child(card)
}

/// The card for an approval that gates no call: the host's words alone.
fn lone_card(
    approval: &Approval,
    copy: &ConversationCopy,
    on_decide: Option<&DecideHandler>,
    busy: bool,
) -> impl IntoElement {
    let title = copy.approval_lone.clone();
    let card =
        base(approval, title, copy, on_decide, busy).description(metrics::clip(&approval.summary));
    div().w_full().child(card)
}

fn base(
    approval: &Approval,
    title: SharedString,
    copy: &ConversationCopy,
    on_decide: Option<&DecideHandler>,
    busy: bool,
) -> ApprovalCard {
    let mut card = ApprovalCard::new(approval_card_id(&approval.id), title)
        .risk(risk(&approval.effects))
        // Held at pending: the card goes when the host records the answer,
        // and comes back if the answer could not be sent.
        .status(ApprovalStatus::Pending)
        .busy(busy)
        .allow_always(false)
        .approve_label(copy.allow.clone())
        .deny_label(copy.deny.clone());
    if let Some(on_decide) = on_decide {
        let (allow, deny) = (on_decide.clone(), on_decide.clone());
        let (a, d) = (approval.id.clone(), approval.id.clone());
        card = card
            .on_approve(move |_, window, cx| allow(a.clone(), Decision::Allow, window, cx))
            // Deny, Escape and an expired request all deny.
            .on_deny(move |_, window, cx| deny(d.clone(), Decision::Deny, window, cx));
    }
    card
}
