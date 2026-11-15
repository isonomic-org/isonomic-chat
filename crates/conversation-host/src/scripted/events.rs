//! The events the scripted host writes, built as the engine builds them.

use super::Open;
use conversation_model::schema::{
    Assistant, Block, Event, StepEnd, StepOutcome, StopReason, Title, TitleSource, TurnEnd,
    TurnStop, Usage,
};
use std::collections::VecDeque;

/// Split text into words, each keeping the space after it.
pub(super) fn words(text: &str) -> VecDeque<String> {
    text.split_inclusive(' ').map(str::to_owned).collect()
}

pub(super) fn assistant(open: &Open, stop: StopReason) -> Event {
    let mut content = Vec::new();
    if !open.written_reasoning.is_empty() {
        content.push(Block::Reasoning {
            text: open.written_reasoning.clone(),
            elapsed_ms: None,
        });
    }
    if !open.written_text.is_empty() {
        content.push(Block::text(open.written_text.clone()));
    }
    Event::Assistant(Assistant {
        content,
        stop,
        stream: Vec::new(),
        usage: None,
        continues: open.continues,
    })
}

pub(super) fn step_end(outcome: StepOutcome) -> Event {
    Event::StepEnd(StepEnd {
        outcome,
        duration_ms: 0,
        usage: None,
    })
}

pub(super) fn turn_end(stop: TurnStop, reason: Option<String>) -> Event {
    Event::TurnEnd(TurnEnd {
        stop,
        steps: 1,
        duration_ms: 0,
        usage: Usage::default(),
        reason,
        error: None,
        budget: None,
    })
}

pub(super) fn title_event(title: String, source: TitleSource) -> Event {
    Event::Title(Title { title, source })
}
