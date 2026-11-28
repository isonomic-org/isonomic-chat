// Copyright 2026 Anud Inc.
// SPDX-License-Identifier: GPL-3.0-or-later

//! Where a tool call goes, read from its input whatever the tool's name.

use conversation_model::schema::{CallId, ToolKind, Value};
use conversation_model::{ToolUse, summary_lines};

fn call(tool: &str, input: &str) -> ToolUse {
    ToolUse {
        call: CallId::new("call_1"),
        tool: tool.into(),
        kind: ToolKind::Engine,
        input: serde_json::from_str::<Value>(input).expect("json"),
        input_raw: None,
        effects: Vec::new(),
        outcome: None,
        output: String::new(),
        started_at: 0,
        duration_ms: None,
        display: None,
    }
}

#[test]
fn a_call_through_a_connection_names_it_and_its_operation() {
    let tool = call(
        "connection_call",
        r#"{"connection":"Design files","operation":"list_files","input":{"folder":"Q4"}}"#,
    );
    let target = tool.target();
    assert_eq!(target.connection.as_deref(), Some("Design files"));
    assert_eq!(target.operation.as_deref(), Some("list_files"));
    assert_eq!(
        summary_lines(&target.arguments),
        [("folder".to_owned(), "Q4".to_owned())]
    );
}

#[test]
fn any_other_tool_shows_its_whole_input() {
    let tool = call("lookup", r#"{"query":"tides","limit":3}"#);
    let target = tool.target();
    assert_eq!(target.connection, None);
    assert_eq!(target.operation, None);
    assert_eq!(
        summary_lines(&target.arguments),
        [
            ("limit".to_owned(), "3".to_owned()),
            ("query".to_owned(), "tides".to_owned())
        ]
    );
}
