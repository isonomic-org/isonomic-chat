//! Listing sessions, and test hooks for a stream that stalls and a list
//! answer that arrives late.

use super::{Inner, ScriptedHost, fork};
use crate::{HostError, HostFuture, SessionEntry, SessionQuery};
use conversation_model::schema::{Event, SessionId};
use futures::FutureExt as _;

impl ScriptedHost {
    /// From now on, following `session` gives a stream that never sends
    /// anything, as a host whose log stalls would.
    pub fn stall(&self, session: &SessionId) {
        self.inner.borrow_mut().stalled.insert(session.clone());
    }

    /// Let `session`'s stream send again.
    pub fn unstall(&self, session: &SessionId) {
        self.inner.borrow_mut().stalled.remove(session);
    }

    /// From now on, a list is answered as the sessions stand when it is
    /// asked for, but the answer arrives only at [`Self::release_lists`],
    /// as a slow host's would.
    pub fn hold_lists(&self) {
        self.inner.borrow_mut().hold_lists = true;
    }

    /// Deliver every held list answer, and answer later ones at once.
    pub fn release_lists(&self) {
        let mut inner = self.inner.borrow_mut();
        inner.hold_lists = false;
        for gate in inner.held_lists.drain(..) {
            let _ = gate.send(());
        }
    }

    pub(super) fn held_list(&self, query: SessionQuery) -> HostFuture<Vec<SessionEntry>> {
        let mut inner = self.inner.borrow_mut();
        let answer = inner.list(query);
        if !inner.hold_lists {
            return futures::future::ready(answer).boxed_local();
        }
        let (gate, released) = futures::channel::oneshot::channel();
        inner.held_lists.push(gate);
        async move {
            let _ = released.await;
            answer
        }
        .boxed_local()
    }
}

impl Inner {
    pub(super) fn list(&mut self, query: SessionQuery) -> Result<Vec<SessionEntry>, HostError> {
        let inner = self;
        if let Some(error) = inner.refuse_list.take() {
            return Err(error);
        }
        let mut entries: Vec<SessionEntry> = inner
            .sessions
            .iter()
            .filter(|(_, s)| s.archived == query.archived)
            .map(|(id, s)| SessionEntry {
                id: id.clone(),
                title: fork::title(inner, id),
                created_at: s.log.first().map(|e| e.at).unwrap_or_default(),
                updated_at: s.log.last().map(|e| e.at).unwrap_or_default(),
                archived: s.archived,
                model: s.log.iter().find_map(|e| match &e.event {
                    Event::SessionStart(start) => start.model.clone(),
                    _ => None,
                }),
                forked_from: fork::forked_from(&s.log),
            })
            .filter(|entry| match &query.text {
                Some(text) => entry
                    .title
                    .as_deref()
                    .is_some_and(|t| t.contains(text.as_str())),
                None => true,
            })
            .collect();
        entries.sort_by(|a, b| b.updated_at.cmp(&a.updated_at).then(b.id.cmp(&a.id)));
        if query.limit > 0 {
            entries.truncate(query.limit as usize);
        }
        Ok(entries)
    }
}
