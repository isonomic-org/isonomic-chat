// Copyright 2026 Anud Inc.
// SPDX-License-Identifier: GPL-3.0-or-later

//! What the model asked the person, and how they answered: generic over
//! every ask, whatever component shows it.

use crate::schema::{CallId, QuestionId};

/// One choice a question offers.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct AskChoice {
    /// Its id, sent back when picked.
    pub id: String,
    /// Its words.
    pub label: String,
}

/// Where an ask stands.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum AskState {
    /// It waits for the person.
    Open,
    /// The person answered.
    Answered {
        /// The choices picked, by id.
        selected: Vec<String>,
        /// Their own words.
        text: Option<String>,
    },
    /// The person did not answer: they skipped it, or moved on.
    Unanswered,
}

/// A question the model asked the person.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Ask {
    /// The question.
    pub id: QuestionId,
    /// The tool call that asked it, if any.
    pub call: Option<CallId>,
    /// What is asked.
    pub prompt: String,
    /// The choices, if any.
    pub choices: Vec<AskChoice>,
    /// Whether more than one choice may be picked.
    pub multiple: bool,
    /// Whether an answer in the person's own words is welcome.
    pub free_text: bool,
    /// Where it stands.
    pub state: AskState,
}

impl Ask {
    /// Whether it still waits for the person.
    pub fn is_open(&self) -> bool {
        self.state == AskState::Open
    }

    /// The answer as the person reads it: the picked choices' words, then
    /// their own words; `None` while open or unanswered.
    pub fn answer_text(&self) -> Option<String> {
        let AskState::Answered { selected, text } = &self.state else {
            return None;
        };
        let mut parts: Vec<String> = selected
            .iter()
            .map(|id| {
                self.choices
                    .iter()
                    .find(|choice| &choice.id == id)
                    .map_or_else(|| id.clone(), |choice| choice.label.clone())
            })
            .collect();
        parts.extend(
            text.iter()
                .map(|t| t.trim().to_owned())
                .filter(|t| !t.is_empty()),
        );
        Some(parts.join(", "))
    }
}

/// The person's reply to an ask. Nothing is ever sent without their input.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Reply {
    /// They answered: the choices picked, by id, and their own words.
    Answered {
        /// The choices picked.
        selected: Vec<String>,
        /// Their own words.
        text: Option<String>,
    },
    /// They chose to skip it.
    Skipped,
    /// They sent a new message instead.
    MovedOn,
}
