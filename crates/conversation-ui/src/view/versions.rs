//! Versions: trying a reply again forks the session before its turn and
//! writes the turn anew; the pager under a reply switches between them.
//!
//! What shows is one path through the conversation's sessions
//! ([`conversation_model::plan`]): each session along it is folded up to
//! where the next one forked, and the last is followed live.

use super::ConversationView;
use crate::metrics;
use conversation_host::{ConversationHost, ForkAt, SessionQuery};
use conversation_model::schema::{Event, ForkReason, Seq, SessionId, TurnId};
use conversation_model::{Branch, Path, Slot, Thread, Turn, Update, plan};
use futures::StreamExt as _;
use futures::future::{self, Either};
use ui::BackgroundExecutor;
use ui::prelude::*;

/// A turn with more than one version, where it shows.
#[derive(Clone, Debug)]
pub(super) struct SlotAt {
    pub(super) slot: Slot,
    /// The turn's index, once the session it is in has been folded; `None`
    /// while that session is the one followed live.
    pub(super) index: Option<usize>,
    /// The turn's id in `slot.parent`, the point every version forks at.
    pub(super) turn: Option<TurnId>,
}

/// Where to start a load: the session, the turns it carries on from, the
/// `seq` of its own log already among them, and the fork point whose other
/// versions to leave out (they are already in a slot).
#[derive(Clone)]
pub(super) struct Start {
    pub(super) session: SessionId,
    pub(super) carried: Vec<Turn>,
    pub(super) after: Seq,
    pub(super) skip: Option<(SessionId, Seq)>,
}

impl Start {
    /// A whole conversation from its first session.
    pub(super) fn root(session: SessionId) -> Self {
        Self {
            session,
            carried: Vec::new(),
            after: 0,
            skip: None,
        }
    }
}

impl ConversationView {
    /// Show a session and, under it, the versions most recently worked on;
    /// then follow the last session live.
    pub(super) fn load(&mut self, start: Start, cx: &mut Context<Self>) {
        let host = self.host.clone();
        self.load_failed = None;
        self._follow = None;
        *self.thread.borrow_mut() =
            Thread::with_turns(start.session.clone(), start.carried.clone())
                .starting_after(start.after);
        self.rows_changed(cx);
        self._load = Some(cx.spawn(async move |this, cx| {
            // Without a list there are no versions to find: show the session.
            let branches: Vec<Branch> = host
                .list(SessionQuery::default())
                .await
                .unwrap_or_default()
                .into_iter()
                .filter(|entry| entry.forked_from != start.skip || start.skip.is_none())
                .map(|entry| Branch {
                    id: entry.id,
                    forked_from: entry.forked_from,
                    created_at: entry.created_at,
                    updated_at: entry.updated_at,
                })
                .collect();
            let path = plan(&start.session, &branches);
            let executor = cx.background_executor().clone();
            let folded = fold(host.as_ref(), &path, start.clone(), &executor).await;
            let Ok((thread, slots, owners)) = folded else {
                let _ = this.update(cx, |this, cx| {
                    this.load_failed = Some(start);
                    this.rows_changed(cx);
                });
                return;
            };
            let _ = this.update(cx, |this, cx| {
                this.owners.extend(owners);
                for found in slots {
                    let known = this.slots.iter().any(|s| {
                        s.slot.parent == found.slot.parent && s.slot.seq == found.slot.seq
                    });
                    if !known {
                        this.slots.push(found);
                    }
                }
                let leaf = thread.session().cloned();
                *this.thread.borrow_mut() = thread;
                this.rows_changed(cx);
                if let Some(leaf) = leaf {
                    this.follow(leaf, cx);
                }
            });
        }));
    }

    /// Whether opening the conversation failed (a log that ended early or
    /// stalled); the status bar offers Try again.
    pub fn load_failed(&self) -> bool {
        self.load_failed.is_some()
    }

    /// Load the conversation again after a failure.
    pub fn retry_load(&mut self, cx: &mut Context<Self>) {
        if let Some(start) = self.load_failed.take() {
            self.load(start, cx);
        }
    }

    /// The versions of the turn at `index`, if it has more than one: how
    /// many, and which shows, counted from zero.
    pub fn versions(&self, index: usize) -> Option<(usize, usize)> {
        let at = self.slot_at(index)?;
        let slot = &self.slots[at].slot;
        Some((slot.sessions.len(), slot.current))
    }

    /// Whether the reply of the turn at `index` can be written again.
    pub fn can_regenerate(&self, index: usize) -> bool {
        let thread = self.thread.borrow();
        let busy = thread.input_state().is_busy();
        thread.turns().get(index).is_some_and(|turn| {
            !busy && !turn.state.is_open() && !turn.prompt.is_empty() && !turn.ids.is_empty()
        })
    }

    /// Write the reply of the turn at `index` again, as a new version: fork
    /// the session the turn was written in, before it, and send its prompt.
    pub fn regenerate(&mut self, index: usize, cx: &mut Context<Self>) {
        self.rewrite(index, None, ForkReason::Regenerate, cx);
    }

    /// Send an edited prompt in place of the one at `index`, as a new
    /// version of that turn. An empty or unchanged prompt does nothing.
    pub fn edit(&mut self, index: usize, prompt: String, cx: &mut Context<Self>) {
        let same = self
            .thread
            .borrow()
            .turns()
            .get(index)
            .is_none_or(|turn| turn.prompt == prompt);
        if prompt.trim().is_empty() || same {
            return;
        }
        self.rewrite(index, Some(prompt), ForkReason::Edit, cx);
    }

    /// Fork before the turn at `index` and send `prompt` (or the turn's own)
    /// in the fork, which becomes the version that shows.
    fn rewrite(
        &mut self,
        index: usize,
        prompt: Option<String>,
        reason: ForkReason,
        cx: &mut Context<Self>,
    ) {
        if !self.can_regenerate(index) {
            return;
        }
        let (prompt, before, start_seq, own_turn) = {
            let thread = self.thread.borrow();
            let turn = &thread.turns()[index];
            (
                prompt.unwrap_or_else(|| turn.prompt.clone()),
                thread.turns()[..index].to_vec(),
                turn.start_seq,
                turn.ids.first().cloned(),
            )
        };
        let (parent, turn) = match self.slot_at(index) {
            // A slot on the session followed live shows that session, the
            // parent, so the turn shown is the parent's own.
            Some(at) => (
                Some(self.slots[at].slot.parent.clone()),
                self.slots[at].turn.clone().or(own_turn),
            ),
            None => (self.owner(index), own_turn),
        };
        let (Some(parent), Some(turn)) = (parent, turn) else {
            return;
        };
        let request = self
            .host
            .fork(&parent, ForkAt::Before(turn.clone()), reason);
        self._request = Some(cx.spawn(async move |this, cx| match request.await {
            Ok(child) => {
                let _ = this.update(cx, |this, cx| {
                    let seq = start_seq.saturating_sub(1);
                    this.add_version(index, parent, seq, turn, child.clone());
                    this.owners.retain(|(_, start)| *start < index);
                    this.owners.push((child.clone(), index));
                    this._follow = None;
                    *this.thread.borrow_mut() = Thread::with_turns(child.clone(), before);
                    this.rows_changed(cx);
                    this.follow(child, cx);
                    this.send(prompt, cx);
                });
            }
            Err(error) => {
                let _ = this.update(cx, |this, cx| this.refused(error, cx));
            }
        }));
    }

    /// Show another version of the turn at `index`, counted from zero.
    pub fn show_version(&mut self, index: usize, version: usize, cx: &mut Context<Self>) {
        let Some(at) = self.slot_at(index) else {
            return;
        };
        let slot = self.slots[at].slot.clone();
        let Some(session) = slot.sessions.get(version).cloned() else {
            return;
        };
        if slot.current == version || self.input_state().is_busy() {
            return;
        }
        let carried = self.thread.borrow().turns()[..index].to_vec();
        self.slots[at].slot.current = version;
        self.slots[at].index = Some(index);
        self.slots.truncate(at + 1);
        self.owners.retain(|(_, start)| *start < index);
        // The first version is the parent: it carries on from the fork point.
        let after = if version == 0 { slot.seq } else { 0 };
        let start = Start {
            session,
            carried,
            after,
            skip: Some((slot.parent, slot.seq)),
        };
        self.load(start, cx);
    }

    fn add_version(
        &mut self,
        index: usize,
        parent: SessionId,
        seq: Seq,
        turn: TurnId,
        child: SessionId,
    ) {
        match self.slot_at(index) {
            Some(at) => {
                let slot = &mut self.slots[at].slot;
                slot.sessions.push(child);
                slot.current = slot.sessions.len() - 1;
                self.slots[at].index = Some(index);
                self.slots.truncate(at + 1);
            }
            None => {
                self.slots.retain(|s| s.index.is_some_and(|at| at < index));
                self.slots.push(SlotAt {
                    slot: Slot {
                        parent: parent.clone(),
                        seq,
                        sessions: vec![parent, child],
                        current: 1,
                    },
                    index: Some(index),
                    turn: Some(turn),
                });
            }
        }
    }

    /// Where in `slots` the turn at `index` is.
    fn slot_at(&self, index: usize) -> Option<usize> {
        let thread = self.thread.borrow();
        self.slots.iter().position(|s| match s.index {
            Some(at) => at == index,
            None => {
                thread.session() == Some(&s.slot.parent) && thread.turn_after(s.slot.seq) == index
            }
        })
    }

    /// The session the turn at `index` was written in.
    fn owner(&self, index: usize) -> Option<SessionId> {
        self.owners
            .iter()
            .rev()
            .find(|(_, start)| *start <= index)
            .map(|(session, _)| session.clone())
            .or_else(|| self.thread.borrow().session().cloned())
    }
}

/// A session's log did not reach the point the next version forked at in
/// time: it ended early, or stalled.
pub(super) struct FoldFailed;

/// Fold the sessions of `path`: each up to where the next forked, the last
/// not at all (it is followed live). Returns the thread to follow, the
/// versions found, and where each session's own turns start.
///
/// Each session is read for at most `metrics::fold_limit()`; one that ends
/// before its fork point, or stalls, fails the load instead of leaving the
/// thread loading.
async fn fold(
    host: &dyn ConversationHost,
    path: &Path,
    start: Start,
    executor: &BackgroundExecutor,
) -> Result<(Thread, Vec<SlotAt>, Vec<(SessionId, usize)>), FoldFailed> {
    let (mut carried, mut after) = (start.carried, start.after);
    let mut slots = Vec::new();
    let mut owners = Vec::new();
    let mut last = Thread::new();
    for (session, leave) in &path.sessions {
        owners.push((session.clone(), carried.len()));
        let thread = Thread::with_turns(session.clone(), carried).starting_after(after);
        let Some(leave) = *leave else {
            for slot in path.slots.iter().filter(|s| &s.parent == session) {
                slots.push(SlotAt {
                    slot: slot.clone(),
                    index: None,
                    turn: None,
                });
            }
            return Ok((thread, slots, owners));
        };
        let read = Box::pin(read_to(host, session, after, leave, thread, executor));
        let limit = Box::pin(executor.timer(metrics::fold_limit()));
        let (thread, next) = match future::select(read, limit).await {
            Either::Left((Some(read), _)) => read,
            _ => return Err(FoldFailed),
        };
        for slot in path.slots.iter().filter(|s| &s.parent == session) {
            let index = thread.turn_after(slot.seq);
            let turn = if slot.seq == leave {
                next.clone()
            } else {
                thread
                    .turns()
                    .get(index)
                    .and_then(|t| t.ids.first().cloned())
            };
            slots.push(SlotAt {
                slot: slot.clone(),
                index: Some(index),
                turn,
            });
        }
        carried = thread.turns_through(leave);
        after = 0;
        last = thread;
    }
    Ok((last, slots, owners))
}

/// Read a session's log into `thread` up to and including `leave`, then
/// glance at the event after it for the turn a fork there writes again.
/// `None` if the log ends before `leave`.
async fn read_to(
    host: &dyn ConversationHost,
    session: &SessionId,
    after: Seq,
    leave: Seq,
    mut thread: Thread,
    executor: &BackgroundExecutor,
) -> Option<(Thread, Option<TurnId>)> {
    let mut updates = host.subscribe(session, after).await.ok()?;
    while thread.last_seq() < leave {
        let update = updates.next().await?;
        if let Update::Event(event) = &update
            && event.seq > leave
        {
            // The fork point itself was never sent: a gap in the log.
            return None;
        }
        thread.apply(&update);
    }
    // The fork point is reached. Nothing may follow it (a fork after the
    // last turn), so wait only a moment for the next event.
    let glance = Box::pin(executor.timer(metrics::glance()));
    let next = match future::select(updates.next(), glance).await {
        Either::Left((Some(Update::Event(event)), _))
            if matches!(event.event, Event::TurnStart(_)) =>
        {
            event.turn.clone()
        }
        _ => None,
    };
    Some((thread, next))
}
