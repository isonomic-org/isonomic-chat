// Copyright 2026 Anud Inc.
// SPDX-License-Identifier: GPL-3.0-or-later

//! Repository checks, run by `cargo xtask check` and by this crate's tests
//! (so `cargo test --workspace` runs them in CI):
//!
//! - **Boundary:** no kit crate (`crates/conversation-*`) depends on the
//!   chat crate, and the chat crate never folds events, matches on event
//!   variants, renders a tool call or decides an approval.
//! - **Product terms:** the kit's public names carry no product terms, so
//!   the kit moves to its own repository as a directory copy.

use std::fs;
use std::path::{Path, PathBuf};

/// Words a kit crate's public names may not contain.
const PRODUCT_TERMS: &[&str] = &["chat", "agent", "project"];

/// What the chat crate's sources may not use: these belong to the kit.
const CHAT_FORBIDDEN: &[(&str, &str)] = &[
    ("schema::Event", "matches on session events"),
    ("Event::", "matches on session event variants"),
    ("from_events", "folds events"),
    ("Thread::", "folds events"),
    ("ToolUse", "renders a tool call"),
    ("tool_row", "renders a tool call"),
    ("approval_card", "renders an approval"),
    ("Decision::", "decides an approval"),
    (".decide(", "decides an approval"),
];

/// Whether `code` uses `pattern` as a name of its own, not as the tail of a
/// longer one (`InputEvent::` is not `Event::`).
fn uses(code: &str, pattern: &str) -> bool {
    code.match_indices(pattern).any(|(at, _)| {
        !pattern.starts_with(|c: char| c.is_alphanumeric())
            || !code[..at]
                .chars()
                .next_back()
                .is_some_and(|c| c.is_alphanumeric() || c == '_')
    })
}

fn root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("the workspace root")
        .to_path_buf()
}

fn sources(dir: &Path, out: &mut Vec<PathBuf>) {
    let Ok(entries) = fs::read_dir(dir) else {
        return;
    };
    for entry in entries.flatten() {
        let path = entry.path();
        if path.is_dir() {
            sources(&path, out);
        } else if path.extension().is_some_and(|e| e == "rs") {
            out.push(path);
        }
    }
}

fn kit_crates(root: &Path) -> Vec<PathBuf> {
    let mut crates: Vec<PathBuf> = fs::read_dir(root.join("crates"))
        .expect("read crates/")
        .flatten()
        .map(|e| e.path())
        .filter(|p| {
            p.file_name()
                .and_then(|n| n.to_str())
                .is_some_and(|n| n.starts_with("conversation-"))
        })
        .collect();
    crates.sort();
    crates
}

/// Every dependency table entry of a manifest that names the chat crate.
fn boundary(root: &Path) -> Vec<String> {
    let mut problems = Vec::new();
    for krate in kit_crates(root) {
        let manifest = fs::read_to_string(krate.join("Cargo.toml")).expect("read a manifest");
        for line in manifest.lines() {
            let name = line.split(['=', '.']).next().unwrap_or_default().trim();
            if name == "isonomic-chat" || line.contains("crates/chat") {
                problems.push(format!(
                    "{}: a kit crate depends on the chat crate",
                    krate.display()
                ));
            }
        }
    }
    let mut files = Vec::new();
    sources(&root.join("crates/chat/src"), &mut files);
    for file in files {
        let text = fs::read_to_string(&file).expect("read a source file");
        for (number, line) in text.lines().enumerate() {
            let code = line.split("//").next().unwrap_or_default();
            for (pattern, why) in CHAT_FORBIDDEN {
                if uses(code, pattern) {
                    problems.push(format!(
                        "{}:{}: Chat {why} (`{pattern}`); that is the kit's job",
                        file.display(),
                        number + 1
                    ));
                }
            }
        }
    }
    problems
}

/// The name a `pub` item declares, if the line declares one.
fn public_name(line: &str) -> Option<&str> {
    let rest = line.trim_start().strip_prefix("pub ")?;
    let rest = rest.trim_start();
    for keyword in [
        "struct ", "enum ", "trait ", "fn ", "type ", "const ", "mod ", "static ",
    ] {
        if let Some(name) = rest.strip_prefix(keyword) {
            let end = name
                .find(|c: char| !(c.is_alphanumeric() || c == '_'))
                .unwrap_or(name.len());
            return Some(&name[..end]);
        }
    }
    None
}

fn product_terms(root: &Path) -> Vec<String> {
    let mut problems = Vec::new();
    for krate in kit_crates(root) {
        let mut files = Vec::new();
        sources(&krate.join("src"), &mut files);
        for file in files {
            let text = fs::read_to_string(&file).expect("read a source file");
            for (number, line) in text.lines().enumerate() {
                let Some(name) = public_name(line) else {
                    continue;
                };
                let lower = name.to_lowercase();
                if let Some(term) = PRODUCT_TERMS.iter().find(|t| lower.contains(*t)) {
                    problems.push(format!(
                        "{}:{}: public name `{name}` carries the product term \"{term}\"",
                        file.display(),
                        number + 1
                    ));
                }
            }
        }
    }
    problems
}

fn check() -> Vec<String> {
    let root = root();
    let mut problems = boundary(&root);
    problems.extend(product_terms(&root));
    problems
}

fn main() {
    match std::env::args().nth(1).as_deref() {
        Some("check") | None => {
            let problems = check();
            for problem in &problems {
                eprintln!("{problem}");
            }
            if !problems.is_empty() {
                std::process::exit(1);
            }
            println!("boundary and product-term checks pass");
        }
        Some(other) => {
            eprintln!("unknown task `{other}`; try `cargo xtask check`");
            std::process::exit(2);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_boundary_holds() {
        let problems = boundary(&root());
        assert!(problems.is_empty(), "{}", problems.join("\n"));
    }

    #[test]
    fn kit_names_carry_no_product_terms() {
        let problems = product_terms(&root());
        assert!(problems.is_empty(), "{}", problems.join("\n"));
    }

    #[test]
    fn a_pattern_matches_whole_names_only() {
        assert!(uses("match e { Event::Title(_) => 1 }", "Event::"));
        assert!(!uses("InputEvent::Blur => 1", "Event::"));
        assert!(uses("host.decide(a, b)", ".decide("));
    }

    #[test]
    fn public_names_are_read_from_declarations() {
        assert_eq!(public_name("pub struct ChatView {"), Some("ChatView"));
        assert_eq!(
            public_name("    pub fn agent_row(x: u8)"),
            Some("agent_row")
        );
        assert_eq!(public_name("pub(crate) fn hidden()"), None);
        assert_eq!(public_name("let x = 1;"), None);
    }
}
