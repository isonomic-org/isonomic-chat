// Copyright 2026 Anud Inc.
// SPDX-License-Identifier: GPL-3.0-or-later

//! A host that plays a recorded session log, for the gallery and for tests.
//!
//! [`FixtureHost`] serves the sessions in a log as they were recorded: it
//! lists them, and following one replays its events. By default a follower
//! gets every event at once, as when a finished conversation is opened. In
//! replay mode ([`FixtureHost::replay`]) nothing is sent until
//! [`FixtureHost::advance`] releases it, and each reply arrives word by word
//! as live deltas before its durable event, as a model writing it would.
//!
//! The log is read only: every write is refused with `read_only`.

use crate::{
    ConversationHost, ForkAt, HostError, HostFuture, NewSession, SessionEntry, SessionQuery,
    UpdateStream,
};
use conversation_model::schema::{
    ApprovalId, Block, Decision, DeltaKind, Event, ForkReason, LiveDelta, QuestionId, Seq,
    SessionEvent, SessionId,
};
use conversation_model::{Availability, ModelCatalog, Reply, Update};
use futures::channel::mpsc::{UnboundedSender, unbounded};
use futures::{FutureExt as _, StreamExt as _, future};
use std::cell::RefCell;
use std::collections::{BTreeMap, VecDeque};
use std::rc::Rc;

#[derive(Default)]
struct Recorded {
    log: Vec<SessionEvent>,
    /// What replay has still to send, in order.
    pending: VecDeque<Update>,
    subscribers: Vec<UnboundedSender<Update>>,
}

#[derive(Default)]
struct Inner {
    sessions: BTreeMap<SessionId, Recorded>,
    /// Session ids in the order they first appear in the log.
    order: Vec<SessionId>,
    replay: bool,
}

/// A host over a recorded log. Cheap to clone; clones share state.
#[derive(Clone, Default)]
pub struct FixtureHost {
    inner: Rc<RefCell<Inner>>,
}

impl FixtureHost {
    /// A host serving every session in `events`, which may interleave.
    pub fn new(events: &[SessionEvent]) -> Self {
        let mut inner = Inner::default();
        for event in events {
            if !inner.sessions.contains_key(&event.session) {
                inner.order.push(event.session.clone());
            }
            let session = inner.sessions.entry(event.session.clone()).or_default();
            session.log.push(event.clone());
        }
        Self {
            inner: Rc::new(RefCell::new(inner)),
        }
    }

    /// From now on, a follower gets nothing until [`Self::advance`], and
    /// replies stream word by word.
    pub fn replay(&self) -> &Self {
        self.inner.borrow_mut().replay = true;
        self
    }

    /// The sessions in the log, in the order they first appear.
    pub fn sessions(&self) -> Vec<SessionId> {
        self.inner.borrow().order.clone()
    }

    /// A session's recorded events.
    pub fn events(&self, session: &SessionId) -> Vec<SessionEvent> {
        self.inner
            .borrow()
            .sessions
            .get(session)
            .map(|s| s.log.clone())
            .unwrap_or_default()
    }

    /// In replay mode, send each follower the next `count` updates (durable
    /// events and live deltas) of its session. Returns whether any session
    /// has more to send.
    pub fn advance(&self, count: usize) -> bool {
        let mut inner = self.inner.borrow_mut();
        let mut more = false;
        for session in inner.sessions.values_mut() {
            for _ in 0..count {
                let Some(update) = session.pending.pop_front() else {
                    break;
                };
                session
                    .subscribers
                    .retain(|s| s.unbounded_send(update.clone()).is_ok());
            }
            more |= !session.pending.is_empty();
        }
        more
    }

    /// In replay mode, send everything that is left.
    pub fn finish(&self) {
        while self.advance(usize::MAX) {}
    }

    fn refuse<T: 'static>() -> HostFuture<T> {
        future::ready(Err(HostError::new(
            "read_only",
            "This is a recorded conversation.",
        )))
        .boxed_local()
    }
}

/// The updates that replay one session: each reply's words as live deltas,
/// then its durable event.
fn replayed(log: &[SessionEvent], from_seq: Seq) -> VecDeque<Update> {
    let mut updates = VecDeque::new();
    let mut after = from_seq;
    for event in log.iter().filter(|e| e.seq > from_seq) {
        if let (Event::Assistant(reply), Some(turn), Some(step)) =
            (&event.event, &event.turn, &event.step)
        {
            for (index, block) in reply.content.iter().enumerate() {
                let (kind, text) = match block {
                    Block::Text { text } => (DeltaKind::Text, text),
                    Block::Reasoning { text, .. } => (DeltaKind::Reasoning, text),
                    _ => continue,
                };
                for word in text.split_inclusive(' ') {
                    updates.push_back(Update::Delta(LiveDelta {
                        session: event.session.clone(),
                        turn: turn.clone(),
                        step: step.clone(),
                        after_seq: after,
                        block: index as u32,
                        kind: kind.clone(),
                        text: word.to_owned(),
                    }));
                }
            }
        }
        after = event.seq;
        updates.push_back(Update::from(event.clone()));
    }
    updates
}

fn title(log: &[SessionEvent]) -> Option<String> {
    log.iter().rev().find_map(|e| match &e.event {
        Event::Title(t) => Some(t.title.clone()),
        _ => None,
    })
}

impl ConversationHost for FixtureHost {
    fn create(&self, _: NewSession) -> HostFuture<SessionId> {
        Self::refuse()
    }

    fn send(&self, _: &SessionId, _: Vec<Block>) -> HostFuture<()> {
        Self::refuse()
    }

    fn resume(&self, _: &SessionId) -> HostFuture<()> {
        Self::refuse()
    }

    fn cancel(&self, _: &SessionId) -> HostFuture<()> {
        future::ready(Ok(())).boxed_local()
    }

    fn rename(&self, _: &SessionId, _: String) -> HostFuture<()> {
        Self::refuse()
    }

    fn list(&self, query: SessionQuery) -> HostFuture<Vec<SessionEntry>> {
        let inner = self.inner.borrow();
        let mut entries: Vec<SessionEntry> = if query.archived {
            Vec::new()
        } else {
            inner
                .order
                .iter()
                .filter_map(|id| {
                    let log = &inner.sessions.get(id)?.log;
                    Some(SessionEntry {
                        id: id.clone(),
                        title: title(log),
                        created_at: log.first().map(|e| e.at).unwrap_or_default(),
                        updated_at: log.last().map(|e| e.at).unwrap_or_default(),
                        archived: false,
                        model: log.iter().find_map(|e| match &e.event {
                            Event::SessionStart(start) => start.model.clone(),
                            _ => None,
                        }),
                        forked_from: log.iter().find_map(|e| match &e.event {
                            Event::Forked(f) => Some((f.from.clone(), f.at_seq)),
                            _ => None,
                        }),
                    })
                })
                .filter(|entry| match &query.text {
                    Some(text) => entry
                        .title
                        .as_deref()
                        .is_some_and(|t| t.to_lowercase().contains(&text.to_lowercase())),
                    None => true,
                })
                .collect()
        };
        entries.sort_by_key(|entry| std::cmp::Reverse(entry.updated_at));
        if query.limit > 0 {
            entries.truncate(query.limit as usize);
        }
        future::ready(Ok(entries)).boxed_local()
    }

    fn fork(&self, _: &SessionId, _: ForkAt, _: ForkReason) -> HostFuture<SessionId> {
        Self::refuse()
    }

    fn archive(&self, _: &SessionId, _: bool) -> HostFuture<()> {
        Self::refuse()
    }

    fn models(&self) -> HostFuture<ModelCatalog> {
        future::ready(Ok(ModelCatalog::default())).boxed_local()
    }

    fn availability(&self, _: &str) -> HostFuture<Availability> {
        future::ready(Ok(Availability::Unknown)).boxed_local()
    }

    fn decide(&self, _: &SessionId, _: &ApprovalId, _: Decision) -> HostFuture<()> {
        Self::refuse()
    }

    fn answer(&self, _: &SessionId, _: &QuestionId, _: Reply) -> HostFuture<()> {
        Self::refuse()
    }

    fn subscribe(&self, session: &SessionId, from_seq: Seq) -> HostFuture<UpdateStream> {
        let mut inner = self.inner.borrow_mut();
        let replay = inner.replay;
        let Some(recorded) = inner.sessions.get_mut(session) else {
            return future::ready(Err(HostError::new("not_found", "No such session.")))
                .boxed_local();
        };
        let (sender, receiver) = unbounded();
        if replay {
            recorded.pending = replayed(&recorded.log, from_seq);
            recorded.subscribers.push(sender);
        } else {
            for event in recorded.log.iter().filter(|e| e.seq > from_seq) {
                let _ = sender.unbounded_send(Update::from(event.clone()));
            }
            // Kept open, as a live session's stream is: a stream that ends
            // reads as a lost connection.
            recorded.subscribers.push(sender);
        }
        future::ready(Ok(receiver.boxed_local())).boxed_local()
    }
}
