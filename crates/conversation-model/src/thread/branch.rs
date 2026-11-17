// Copyright 2026 Anud Inc.
// SPDX-License-Identifier: GPL-3.0-or-later

//! A thread that carries on from another: the turns a fork shares with the
//! session it was forked from come first, then its own.

use super::Thread;
use crate::schema::{Seq, SessionId};
use crate::turn::Turn;

impl Thread {
    /// A thread for `session` that starts with `turns`, the turns it shares
    /// with the sessions it was forked from. Its own events follow them.
    pub fn with_turns(session: SessionId, turns: Vec<Turn>) -> Self {
        Self {
            session: Some(session),
            carried: turns.len(),
            turns,
            ..Self::default()
        }
    }

    /// Skip this session's events up to and including `seq`: they are
    /// already among the carried turns.
    pub fn starting_after(mut self, seq: Seq) -> Self {
        self.last_seq = seq;
        self
    }

    /// How many turns at the start were carried over from the sessions this
    /// one was forked from.
    pub fn carried(&self) -> usize {
        self.carried
    }

    /// The turns a fork of this session at `seq` shares: the carried ones,
    /// then this session's own turns that started at or before `seq`.
    pub fn turns_through(&self, seq: Seq) -> Vec<Turn> {
        let own = self.turns[self.carried..]
            .iter()
            .take_while(|turn| turn.start_seq != 0 && turn.start_seq <= seq);
        self.turns[..self.carried]
            .iter()
            .chain(own)
            .cloned()
            .collect()
    }

    /// The index of the first turn after `seq` in this session's own log:
    /// the turn a fork at `seq` writes again.
    pub fn turn_after(&self, seq: Seq) -> usize {
        self.turns_through(seq).len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::TurnState;
    use crate::schema::{
        Cause, Event, SessionEvent, SessionId, TurnEnd, TurnId, TurnStart, TurnStop, User,
    };

    fn event(session: &SessionId, seq: Seq, turn: &str, event: Event) -> SessionEvent {
        SessionEvent {
            session: session.clone(),
            seq,
            at: 0,
            turn: Some(TurnId::new(turn)),
            step: None,
            event,
        }
    }

    /// A turn that starts at `seq` and ends two events later.
    fn turn(session: &SessionId, seq: Seq, id: &str, prompt: &str) -> Vec<SessionEvent> {
        let start = Event::TurnStart(TurnStart {
            cause: Cause::User,
            wake: None,
        });
        let user = Event::User(User {
            content: vec![crate::schema::Block::text(prompt)],
        });
        let end = Event::TurnEnd(TurnEnd {
            stop: TurnStop::Done,
            steps: 1,
            duration_ms: 0,
            usage: Default::default(),
            reason: None,
            error: None,
            budget: None,
        });
        vec![
            event(session, seq, id, start),
            event(session, seq + 1, id, user),
            event(session, seq + 2, id, end),
        ]
    }

    #[test]
    fn a_fork_shares_the_turns_before_its_point_and_adds_its_own() {
        let parent = SessionId::new("ses_a");
        let mut log = turn(&parent, 1, "t1", "one");
        log.extend(turn(&parent, 4, "t2", "two"));
        let thread = Thread::from_events(&log);
        assert_eq!(thread.turns()[1].start_seq, 4);

        // A fork before the second turn shares only the first.
        let shared = thread.turns_through(3);
        assert_eq!(shared.len(), 1);
        assert_eq!(thread.turn_after(3), 1);

        let child = SessionId::new("ses_b");
        let mut fork = Thread::with_turns(child.clone(), shared);
        for e in turn(&child, 3, "t3", "two again") {
            fork.apply_event(&e);
        }
        let prompts: Vec<_> = fork.turns().iter().map(|t| t.prompt.as_str()).collect();
        assert_eq!(prompts, ["one", "two again"]);
        assert_eq!(fork.carried(), 1);
        assert_eq!(fork.turns()[1].state, TurnState::Done);

        // A fork of the fork before its own turn keeps the carried one.
        assert_eq!(fork.turns_through(2).len(), 1);
        assert_eq!(fork.turn_after(2), 1);
    }
}
