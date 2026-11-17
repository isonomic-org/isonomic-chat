// Copyright 2026 Anud Inc.
// SPDX-License-Identifier: GPL-3.0-or-later

//! Forks, as the engine makes them: the child's own log is its
//! `session.start` and a `forked` naming the parent and the last `seq`
//! shared; the shared turns stay in the parent's log.

use super::{Inner, Session};
use crate::{ForkAt, HostError};
use conversation_model::schema::{Event, ForkReason, Forked, Seq, SessionEvent, SessionId, TurnId};

impl Inner {
    pub(super) fn fork(
        &mut self,
        parent: &SessionId,
        at: ForkAt,
        reason: ForkReason,
    ) -> Result<SessionId, HostError> {
        let log = &self.session(parent)?.log;
        let (seq, after_turn) = fork_seq(log, &at).ok_or_else(|| {
            HostError::new("not_at_turn_boundary", "That is not a turn boundary.")
        })?;
        let start = log
            .iter()
            .find_map(|e| match &e.event {
                Event::SessionStart(start) => Some(start.clone()),
                _ => None,
            })
            .ok_or_else(|| HostError::new("not_found", "No such session."))?;
        self.next_session += 1;
        let child = SessionId::new(format!("ses_{:04}", self.next_session));
        self.sessions.insert(
            child.clone(),
            Session {
                titled: true,
                ..Session::default()
            },
        );
        self.emit(&child, None, None, Event::SessionStart(start));
        let forked = Forked {
            from: parent.clone(),
            at_seq: seq,
            after_turn,
            reason,
        };
        self.emit(&child, None, None, Event::Forked(forked));
        Ok(child)
    }
}

/// The parent's last shared `seq` for a fork point, and the turn it ends
/// after, if any.
fn fork_seq(log: &[SessionEvent], at: &ForkAt) -> Option<(Seq, Option<TurnId>)> {
    match at {
        ForkAt::After(turn) => log
            .iter()
            .find(|e| e.turn.as_ref() == Some(turn) && matches!(e.event, Event::TurnEnd(_)))
            .map(|e| (e.seq, Some(turn.clone()))),
        ForkAt::Before(turn) => log
            .iter()
            .find(|e| e.turn.as_ref() == Some(turn) && matches!(e.event, Event::TurnStart(_)))
            .map(|e| (e.seq - 1, None)),
    }
}

/// A session's title: its own latest, else the one it was forked from.
pub(super) fn title(inner: &Inner, id: &SessionId) -> Option<String> {
    let log = &inner.sessions.get(id)?.log;
    log.iter()
        .rev()
        .find_map(|e| match &e.event {
            Event::Title(title) => Some(title.title.clone()),
            _ => None,
        })
        .or_else(|| forked_from(log).and_then(|(parent, _)| title(inner, &parent)))
}

/// The parent and shared `seq` a session's log says it was forked from.
pub(super) fn forked_from(log: &[SessionEvent]) -> Option<(SessionId, Seq)> {
    log.iter().find_map(|e| match &e.event {
        Event::Forked(forked) => Some((forked.from.clone(), forked.at_seq)),
        _ => None,
    })
}
