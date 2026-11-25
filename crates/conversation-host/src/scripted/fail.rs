// Copyright 2026 Anud Inc.
// SPDX-License-Identifier: GPL-3.0-or-later

//! An engine error part way through a reply, written as the engine writes
//! it: an errored attempt carrying what streamed, then either a fresh try
//! from the start or a failed step and a failed turn.

use super::Inner;
use super::events::{step_end, words};
use conversation_model::schema::{
    Attempt, AttemptOutcome, Block, ErrorInfo, Event, SessionId, StepOutcome, TurnEnd, TurnStop,
    Usage,
};

impl Inner {
    /// Whether the open reply has written as many words as its failure
    /// waits for.
    pub(super) fn fail_due(&self, id: &SessionId) -> bool {
        let Some(open) = self.sessions.get(id).and_then(|s| s.open.as_ref()) else {
            return false;
        };
        open.fail.as_ref().is_some_and(|fail| {
            open.reasoning.is_empty() && words(&open.written_text).len() >= fail.after
        })
    }

    /// Fail the open reply. Returns whether it is still open (a retry).
    pub(super) fn fail_reply(&mut self, id: &SessionId) -> bool {
        let Some(mut open) = self.sessions.get_mut(id).and_then(|s| s.open.take()) else {
            return false;
        };
        let Some(fail) = open.fail.take() else {
            return false;
        };
        let mut error = ErrorInfo::new(fail.code, fail.message.clone());
        error.transient = fail.retry;
        let content = if open.written_text.is_empty() {
            Vec::new()
        } else {
            vec![Block::text(open.written_text.clone())]
        };
        let (turn, step) = (open.turn.clone(), open.step.clone());
        let attempt = Event::Attempt(Attempt {
            outcome: AttemptOutcome::Error,
            error: Some(error.clone()),
            content,
            usage: None,
            retry: 0,
            will_retry: fail.retry,
        });
        self.emit(id, Some(&turn), Some(&step), attempt);
        if fail.retry {
            // The engine asks again; the answer starts over.
            let mut text = words(&open.written_text);
            text.extend(open.text.drain(..));
            open.text = text;
            open.written_text.clear();
            if let Some(session) = self.sessions.get_mut(id) {
                session.open = Some(open);
            }
            return true;
        }
        self.emit(id, Some(&turn), Some(&step), step_end(StepOutcome::Failed));
        let end = Event::TurnEnd(TurnEnd {
            stop: TurnStop::Failed,
            steps: 1,
            duration_ms: 0,
            usage: Usage::default(),
            reason: Some(fail.message),
            error: Some(error),
            budget: None,
        });
        self.emit(id, Some(&turn), None, end);
        false
    }
}
