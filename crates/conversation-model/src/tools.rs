// Copyright 2026 Anud Inc.
// SPDX-License-Identifier: GPL-3.0-or-later

//! The tools a reply used and the approvals they waited on, as a view shows
//! them: generic over every tool, whatever its name.

use crate::schema::{ApprovalId, Basis, Block, CallId, Decision, Effect, ToolOutcome, Value};

/// Input keys that name the connection a call goes through, best first.
const CONNECTION_KEYS: [&str; 5] = ["connection_label", "label", "connection", "app", "server"];
/// Input keys that name the operation a call runs, best first.
const OPERATION_KEYS: [&str; 4] = ["operation", "op", "action", "method"];
/// Input keys that hold the operation's own arguments, best first.
const ARGUMENT_KEYS: [&str; 4] = ["input", "arguments", "args", "params"];

/// A tool the reply used.
#[derive(Clone, Debug, PartialEq)]
pub struct ToolUse {
    /// The call.
    pub call: CallId,
    /// The tool's name.
    pub tool: String,
    /// The input, parsed; `Null` when it did not parse.
    pub input: Value,
    /// The model's raw input, only when it did not parse.
    pub input_raw: Option<String>,
    /// The tool's declared effects.
    pub effects: Vec<Effect>,
    /// How it ended; `None` while it runs or waits.
    pub outcome: Option<ToolOutcome>,
    /// What it returned, or the readable reason it failed or was denied.
    pub output: String,
    /// When the call was made, in milliseconds since the epoch.
    pub started_at: u64,
    /// How long it ran, once it ended.
    pub duration_ms: Option<u64>,
}

/// Where a call goes and what it asks, read from its input when the input
/// carries them.
#[derive(Clone, Debug, PartialEq)]
pub struct Target {
    /// The connection's label or handle.
    pub connection: Option<String>,
    /// The operation run through it.
    pub operation: Option<String>,
    /// The arguments: the operation's own, or the whole input.
    pub arguments: Value,
}

impl ToolUse {
    /// Where the call goes, from its input.
    pub fn target(&self) -> Target {
        let Value::Object(fields) = &self.input else {
            return Target {
                connection: None,
                operation: None,
                arguments: self.input.clone(),
            };
        };
        let text = |keys: &[&str]| {
            keys.iter().find_map(|key| match fields.get(*key) {
                Some(Value::String(s)) if !s.trim().is_empty() => Some(s.trim().to_owned()),
                _ => None,
            })
        };
        let (connection, operation) = (text(&CONNECTION_KEYS), text(&OPERATION_KEYS));
        let nested = ARGUMENT_KEYS
            .iter()
            .find_map(|key| fields.get(*key))
            .cloned();
        let arguments = match nested {
            Some(arguments) if connection.is_some() || operation.is_some() => arguments,
            _ => self.input.clone(),
        };
        Target {
            connection,
            operation,
            arguments,
        }
    }

    /// The input as the person reads it: laid-out JSON, or the raw text the
    /// model wrote when it did not parse.
    pub fn input_text(&self) -> String {
        match &self.input_raw {
            Some(raw) => raw.clone(),
            None => pretty(&self.input),
        }
    }

    /// Whether it ended without a result: failed, denied, timed out or
    /// stopped.
    pub fn failed(&self) -> bool {
        self.outcome
            .as_ref()
            .is_some_and(|outcome| *outcome != ToolOutcome::Ok)
    }
}

/// The arguments of a call, short, on one line per field: `name: value`.
/// Nested values are written as compact JSON.
pub fn summary_lines(arguments: &Value) -> Vec<(String, String)> {
    match arguments {
        Value::Null => Vec::new(),
        Value::Object(fields) => fields
            .iter()
            .map(|(key, value)| (key.clone(), scalar(value)))
            .collect(),
        other => vec![(String::new(), scalar(other))],
    }
}

fn scalar(value: &Value) -> String {
    match value {
        Value::String(s) => s.clone(),
        other => serde_json::to_string(other).unwrap_or_default(),
    }
}

/// A value as laid-out JSON.
pub fn pretty(value: &Value) -> String {
    serde_json::to_string_pretty(value).unwrap_or_default()
}

/// An approval the host asked for.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Approval {
    /// The approval.
    pub id: ApprovalId,
    /// The call it gates, if any.
    pub call: Option<CallId>,
    /// What will happen, in the host's words.
    pub summary: String,
    /// The effects at stake.
    pub effects: Vec<Effect>,
    /// How it was decided; `None` while it waits.
    pub decision: Option<(Decision, Basis)>,
}

impl Approval {
    /// Whether it still waits for an answer.
    pub fn is_pending(&self) -> bool {
        self.decision.is_none()
    }

    /// Whether it was allowed. Anything but a clear allow is a denial.
    pub fn allowed(&self) -> bool {
        matches!(self.decision, Some((Decision::Allow, _)))
    }
}

/// The text of a result's content: its text blocks, and any structured data
/// as JSON.
pub(crate) fn output_of(content: &[Block]) -> String {
    content
        .iter()
        .filter_map(|block| match block {
            Block::Text { text } => Some(text.clone()),
            Block::Json { value } => Some(pretty(value)),
            _ => None,
        })
        .collect::<Vec<_>>()
        .join("\n\n")
}
