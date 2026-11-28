// Copyright 2026 Anud Inc.
// SPDX-License-Identifier: GPL-3.0-or-later

//! Questions and their answers, folded into their turn.

use crate::asks::{Ask, AskChoice, AskState};
use crate::schema::{Event, SessionEvent};
use crate::turn::Turn;

/// Fold a question or answer event into its turn. Returns whether the turn
/// changed.
pub(super) fn fold(turn: &mut Turn, event: &SessionEvent) -> bool {
    match &event.event {
        Event::Question(question) => {
            if turn.asks.iter().any(|ask| ask.id == question.question) {
                return false;
            }
            turn.asks.push(Ask {
                id: question.question.clone(),
                call: question.call.clone(),
                prompt: question.prompt.clone(),
                choices: question
                    .options
                    .iter()
                    .map(|choice| AskChoice {
                        id: choice.id.clone(),
                        label: choice.label.clone(),
                    })
                    .collect(),
                multiple: question.multiple,
                free_text: question.free_text,
                state: AskState::Open,
            });
            true
        }
        Event::Answer(answer) => {
            let Some(ask) = turn.asks.iter_mut().find(|a| a.id == answer.question) else {
                return false;
            };
            let empty = answer.selected.is_empty()
                && answer.text.as_deref().is_none_or(|t| t.trim().is_empty());
            // Deny-safe: a skip, or an answer with nothing in it, is no
            // answer.
            ask.state = if answer.skipped || empty {
                AskState::Unanswered
            } else {
                AskState::Answered {
                    selected: answer.selected.clone(),
                    text: answer.text.clone(),
                }
            };
            true
        }
        _ => false,
    }
}
