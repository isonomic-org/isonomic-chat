// Copyright 2026 Anud Inc.
// SPDX-License-Identifier: GPL-3.0-or-later

//! The end of a scripted reply: sealed, refused or cut at the output limit.

use super::events::{assistant, step_end, title_event, turn_end};
use super::{Inner, Leftover};
use conversation_model::schema::{
    Attempt, AttemptOutcome, Block, Event, SessionId, StepOutcome, StopReason, TitleSource,
    TurnStop,
};

impl Inner {
    pub(super) fn end_reply(&mut self, id: &SessionId) {
        let Some(session) = self.sessions.get_mut(id) else {
            return;
        };
        let Some(open) = session.open.take() else {
            return;
        };
        let give_title = !session.titled && open.title.is_some();
        session.titled |= give_title;
        let (turn, step) = (Some(&open.turn), Some(&open.step));
        if open.refuse {
            let attempt = Event::Attempt(Attempt {
                outcome: AttemptOutcome::Refused,
                error: None,
                content: vec![Block::text(open.written_text.clone())],
                usage: None,
                retry: 0,
                will_retry: false,
            });
            self.emit(id, turn, step, attempt);
            self.emit(id, turn, step, step_end(StepOutcome::Refused));
            self.emit(id, turn, None, turn_end(TurnStop::Refused, None));
            return;
        }
        let cut = open.cut.clone().filter(|rest| !rest.is_empty());
        let stop = if cut.is_some() {
            StopReason::MaxTokens
        } else {
            StopReason::End
        };
        let sealed = self.emit(id, turn, step, assistant(&open, stop));
        self.emit(id, turn, step, step_end(StepOutcome::Complete));
        self.emit(id, turn, None, turn_end(TurnStop::Done, None));
        if let (Some(rest), Some(session)) = (cut, self.sessions.get_mut(id)) {
            session.leftover = Some(Leftover {
                text: rest,
                after: sealed,
                title: None,
            });
        }
        if let Some(title) = open.title.filter(|_| give_title) {
            self.emit(id, None, None, title_event(title, TitleSource::Model));
        }
    }
}
