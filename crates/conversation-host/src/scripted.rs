// Copyright 2026 Anud Inc.
// SPDX-License-Identifier: GPL-3.0-or-later

//! A host that plays scripted replies, word by word, as real session events.
//!
//! Tests and the gallery drive it by hand: [`ScriptedHost::pump`] streams the
//! next words of every open reply, [`ScriptedHost::finish`] streams the rest.
//! Stop seals the partial reply exactly as the engine does, and continue
//! streams the words that were left.

use events::{assistant, compacted, step_end, title_event, too_large, turn_end, words};

mod events;
mod fork;
mod list;
mod refuse;
mod reply;

pub use reply::ScriptedReply;

use crate::{
    ConversationHost, ForkAt, HostError, HostFuture, NewSession, SessionEntry, SessionQuery,
    UpdateStream,
};
use conversation_model::schema::{
    AppId, Attempt, AttemptOutcome, Block, Cause, DeltaKind, Event, ForkReason, LiveDelta, Params,
    PresetId, PresetRef, Principal, PrincipalKind, Purpose, Reasoning, Seq, SessionClass,
    SessionEvent, SessionId, SessionStart, StepId, StepOutcome, StepStart, StopReason, TitleSource,
    TurnId, TurnStart, TurnStop, User,
};
use conversation_model::{Availability, ModelCatalog, Update};
use futures::channel::mpsc::{UnboundedSender, unbounded};
use futures::{FutureExt as _, StreamExt as _, future};
use std::cell::RefCell;
use std::collections::{BTreeMap, VecDeque};
use std::rc::Rc;

/// The first event's time: 14 Nov 2026, 12:00 UTC. Each event adds 10 ms.
const EPOCH_MS: u64 = 1_794_657_600_000;

/// A reply being written.
struct Open {
    turn: TurnId,
    step: StepId,
    reasoning: VecDeque<String>,
    text: VecDeque<String>,
    /// Words held back by the output limit, for continue.
    cut: Option<VecDeque<String>>,
    written_reasoning: String,
    written_text: String,
    title: Option<String>,
    refuse: bool,
    continues: Option<Seq>,
}

/// What a stopped reply did not get to write.
struct Leftover {
    text: VecDeque<String>,
    /// The `seq` of the sealed partial reply it carries on.
    after: Seq,
    title: Option<String>,
}

#[derive(Default)]
struct Session {
    log: Vec<SessionEvent>,
    subscribers: Vec<UnboundedSender<Update>>,
    open: Option<Open>,
    /// Words a stopped reply did not get to, for continue.
    leftover: Option<Leftover>,
    titled: bool,
    archived: bool,
}

#[derive(Default)]
struct Inner {
    models: ModelCatalog,
    created: Vec<NewSession>,
    refuse_create: Option<HostError>,
    refuse_resume: Option<HostError>,
    refuse_list: Option<HostError>,
    /// The next rename, archive or fork fails with this.
    refuse_write: Option<HostError>,
    /// Sessions whose stream never sends anything.
    stalled: std::collections::BTreeSet<SessionId>,
    /// Whether list answers wait for `release_lists`, and those waiting.
    hold_lists: bool,
    held_lists: Vec<futures::channel::oneshot::Sender<()>>,
    script: VecDeque<ScriptedReply>,
    sessions: BTreeMap<SessionId, Session>,
    next_session: u32,
    next_turn: u32,
    next_step: u32,
    clock: u64,
}

/// A host that plays scripted replies. Cheap to clone; clones share state.
#[derive(Clone, Default)]
pub struct ScriptedHost {
    inner: Rc<RefCell<Inner>>,
}

impl ScriptedHost {
    /// A host with nothing scripted.
    pub fn new() -> Self {
        Self::default()
    }

    /// Queue a reply for the next prompt sent to any session.
    pub fn script(&self, reply: ScriptedReply) -> &Self {
        self.inner.borrow_mut().script.push_back(reply);
        self
    }

    /// The connection drops: every open stream ends. Sessions and their
    /// logs stay; subscribing again resumes them.
    pub fn disconnect(&self) {
        for session in self.inner.borrow_mut().sessions.values_mut() {
            session.subscribers.clear();
        }
    }

    /// The host compacts the session: older messages are no longer shown to
    /// the model, and `note` says so.
    pub fn compact(&self, session: &SessionId, note: &str) {
        let mut inner = self.inner.borrow_mut();
        let through = inner
            .sessions
            .get(session)
            .map_or(0, |s| s.log.len() as Seq);
        inner.emit(session, None, None, compacted(through, note));
    }

    /// Every create request so far, oldest first.
    pub fn created(&self) -> Vec<NewSession> {
        self.inner.borrow().created.clone()
    }

    /// Refuse the next list with this error.
    pub fn refuse_list(&self, error: HostError) {
        self.inner.borrow_mut().refuse_list = Some(error);
    }

    /// Refuse the next continue with this error.
    pub fn refuse_resume(&self, error: HostError) {
        self.inner.borrow_mut().refuse_resume = Some(error);
    }

    /// Refuse the next create with this error.
    pub fn refuse_create(&self, error: HostError) {
        self.inner.borrow_mut().refuse_create = Some(error);
    }

    /// Offer these models.
    pub fn set_models(&self, models: ModelCatalog) {
        self.inner.borrow_mut().models = models;
    }

    /// Stream up to `words` more words of every open reply. Returns whether
    /// any reply is still open.
    pub fn pump(&self, words: usize) -> bool {
        let ids: Vec<SessionId> = self.inner.borrow().sessions.keys().cloned().collect();
        for id in ids {
            for _ in 0..words {
                if !self.inner.borrow_mut().write_word(&id) {
                    break;
                }
            }
        }
        self.is_busy()
    }

    /// Stream every open reply to its end.
    pub fn finish(&self) {
        while self.pump(64) {}
    }

    /// Whether any reply is open.
    pub fn is_busy(&self) -> bool {
        let inner = self.inner.borrow();
        inner
            .sessions
            .values()
            .any(|session| session.open.is_some())
    }

    /// A session's durable log so far.
    pub fn events(&self, session: &SessionId) -> Vec<SessionEvent> {
        let inner = self.inner.borrow();
        inner
            .sessions
            .get(session)
            .map(|s| s.log.clone())
            .unwrap_or_default()
    }

    /// The sessions created so far, oldest first.
    pub fn sessions(&self) -> Vec<SessionId> {
        self.inner.borrow().sessions.keys().cloned().collect()
    }

    fn call<T: 'static>(
        &self,
        f: impl FnOnce(&mut Inner) -> Result<T, HostError>,
    ) -> HostFuture<T> {
        let result = f(&mut self.inner.borrow_mut());
        future::ready(result).boxed_local()
    }
}

impl ConversationHost for ScriptedHost {
    fn create(&self, request: NewSession) -> HostFuture<SessionId> {
        self.call(|inner| {
            inner.created.push(request.clone());
            if let Some(error) = inner.refuse_create.take() {
                return Err(error);
            }
            inner.next_session += 1;
            let id = SessionId::new(format!("ses_{:04}", inner.next_session));
            inner.sessions.insert(id.clone(), Session::default());
            let start = Event::SessionStart(SessionStart {
                class: SessionClass::Interactive,
                app: AppId::new("scripted"),
                principal: Principal {
                    kind: PrincipalKind::User,
                    id: None,
                },
                preset: PresetRef {
                    id: PresetId::new(request.preset),
                    version: 1,
                },
                parent: None,
                profile: None,
                model: request.model.clone(),
            });
            inner.emit(&id, None, None, start);
            if let Some(title) = request.title {
                inner.emit(&id, None, None, title_event(title, TitleSource::User));
                inner.session(&id)?.titled = true;
            }
            Ok(id)
        })
    }

    fn send(&self, session: &SessionId, content: Vec<Block>) -> HostFuture<()> {
        let session = session.clone();
        self.call(move |inner| {
            if inner.session(&session)?.open.is_some() {
                return Err(HostError::new("busy", "A reply is still being written."));
            }
            let Some(reply) = inner.script.pop_front() else {
                return Err(HostError::new("no_script", "Nothing is scripted to reply."));
            };
            let turn = inner.turn_id();
            let start = Event::TurnStart(TurnStart {
                cause: Cause::User,
                wake: None,
            });
            inner.emit(&session, Some(&turn), None, start);
            inner.emit(&session, Some(&turn), None, Event::User(User { content }));
            if let Some(reason) = reply.too_large {
                inner.emit(&session, Some(&turn), None, too_large(reason));
                return Ok(());
            }
            let step = inner.start_step(&session, &turn);
            let mut text = words(&reply.text);
            let cut = reply.cut_after.map(|n| text.split_off(n.min(text.len())));
            let open = Open {
                turn,
                step,
                reasoning: words(reply.reasoning.as_deref().unwrap_or_default()),
                text,
                cut,
                written_reasoning: String::new(),
                written_text: String::new(),
                title: reply.title,
                refuse: reply.refuse,
                continues: None,
            };
            let state = inner.session(&session)?;
            state.leftover = None;
            state.open = Some(open);
            Ok(())
        })
    }

    fn resume(&self, session: &SessionId) -> HostFuture<()> {
        let session = session.clone();
        self.call(move |inner| {
            if let Some(error) = inner.refuse_resume.take() {
                return Err(error);
            }
            let state = inner.session(&session)?;
            let Some(leftover) = state.leftover.take().filter(|_| state.open.is_none()) else {
                return Err(HostError::new("continue_refused", "Nothing to continue."));
            };
            let turn = inner.turn_id();
            let start = Event::TurnStart(TurnStart {
                cause: Cause::Continue,
                wake: None,
            });
            inner.emit(&session, Some(&turn), None, start);
            let step = inner.start_step(&session, &turn);
            inner.session(&session)?.open = Some(Open {
                turn,
                step,
                reasoning: VecDeque::new(),
                text: leftover.text,
                cut: None,
                written_reasoning: String::new(),
                written_text: String::new(),
                title: leftover.title,
                refuse: false,
                continues: Some(leftover.after),
            });
            Ok(())
        })
    }

    fn cancel(&self, session: &SessionId) -> HostFuture<()> {
        let session = session.clone();
        self.call(move |inner| {
            let Some(open) = inner.session(&session)?.open.take() else {
                return Ok(());
            };
            let (turn, step) = (Some(&open.turn), Some(&open.step));
            let mut sealed = None;
            if !open.written_text.is_empty() || !open.written_reasoning.is_empty() {
                let reply = assistant(&open, StopReason::Cancelled);
                sealed = Some(inner.emit(&session, turn, step, reply));
            }
            let attempt = Event::Attempt(Attempt {
                outcome: AttemptOutcome::Cancelled,
                error: None,
                content: Vec::new(),
                usage: None,
                retry: 0,
                will_retry: false,
            });
            inner.emit(&session, turn, step, attempt);
            inner.emit(&session, turn, step, step_end(StepOutcome::Cancelled));
            let end = turn_end(TurnStop::Cancelled, Some("Stopped by the person.".into()));
            inner.emit(&session, turn, None, end);
            let state = inner.session(&session)?;
            state.leftover = sealed
                .filter(|_| !open.text.is_empty())
                .map(|after| Leftover {
                    text: open.text,
                    after,
                    title: open.title,
                });
            Ok(())
        })
    }

    fn rename(&self, session: &SessionId, title: String) -> HostFuture<()> {
        let session = session.clone();
        self.call(move |inner| {
            inner.write()?;
            inner.session(&session)?.titled = true;
            inner.emit(&session, None, None, title_event(title, TitleSource::User));
            Ok(())
        })
    }

    fn list(&self, query: SessionQuery) -> HostFuture<Vec<SessionEntry>> {
        self.held_list(query)
    }

    fn fork(&self, session: &SessionId, at: ForkAt, reason: ForkReason) -> HostFuture<SessionId> {
        let session = session.clone();
        self.call(move |inner| {
            inner.write()?;
            inner.fork(&session, at, reason)
        })
    }

    fn archive(&self, session: &SessionId, archived: bool) -> HostFuture<()> {
        let session = session.clone();
        self.call(move |inner| {
            inner.write()?;
            inner.session(&session)?.archived = archived;
            Ok(())
        })
    }

    fn models(&self) -> HostFuture<ModelCatalog> {
        self.call(|inner| Ok(inner.models.clone()))
    }

    fn availability(&self, model: &str) -> HostFuture<Availability> {
        let model = model.to_owned();
        self.call(move |inner| {
            Ok(inner
                .models
                .get(&model)
                .map_or(Availability::Unknown, |option| option.availability))
        })
    }

    fn subscribe(&self, session: &SessionId, from_seq: Seq) -> HostFuture<UpdateStream> {
        let session = session.clone();
        self.call(move |inner| {
            if inner.stalled.contains(&session) {
                return Ok(futures::stream::pending().boxed_local());
            }
            let state = inner.session(&session)?;
            let (sender, receiver) = unbounded();
            for event in state.log.iter().filter(|e| e.seq > from_seq) {
                let _ = sender.unbounded_send(Update::from(event.clone()));
            }
            state.subscribers.push(sender);
            Ok(receiver.boxed_local())
        })
    }
}

impl Inner {
    fn session(&mut self, id: &SessionId) -> Result<&mut Session, HostError> {
        self.sessions
            .get_mut(id)
            .ok_or_else(|| HostError::new("not_found", "No such session."))
    }

    fn turn_id(&mut self) -> TurnId {
        self.next_turn += 1;
        TurnId::new(format!("trn_{:04}", self.next_turn))
    }

    fn start_step(&mut self, session: &SessionId, turn: &TurnId) -> StepId {
        self.next_step += 1;
        let step = StepId::new(format!("stp_{:04}", self.next_step));
        let start = Event::StepStart(StepStart {
            purpose: Purpose::Reply,
            model: "scripted".into(),
            params: Params {
                max_output_tokens: 2048,
                reasoning: Reasoning::Off,
                options: BTreeMap::new(),
            },
            tools: None,
            memory_policy: None,
        });
        self.emit(session, Some(turn), Some(&step), start);
        step
    }

    /// Append an event to the log and send it to every subscriber.
    fn emit(
        &mut self,
        id: &SessionId,
        turn: Option<&TurnId>,
        step: Option<&StepId>,
        event: Event,
    ) -> Seq {
        self.clock += 10;
        let at = EPOCH_MS + self.clock;
        let Some(session) = self.sessions.get_mut(id) else {
            return 0;
        };
        let seq = session.log.len() as Seq + 1;
        let event = SessionEvent {
            session: id.clone(),
            seq,
            at,
            turn: turn.cloned(),
            step: step.cloned(),
            event,
        };
        session.log.push(event.clone());
        let update = Update::from(event);
        session
            .subscribers
            .retain(|sender| sender.unbounded_send(update.clone()).is_ok());
        seq
    }

    /// Write one word of the session's open reply, or end it. Returns whether
    /// the reply is still open.
    fn write_word(&mut self, id: &SessionId) -> bool {
        let Some(session) = self.sessions.get_mut(id) else {
            return false;
        };
        let after_seq = session.log.len() as Seq;
        let Some(open) = session.open.as_mut() else {
            return false;
        };
        let has_reasoning = !open.written_reasoning.is_empty() || !open.reasoning.is_empty();
        let (kind, block, word) = if let Some(word) = open.reasoning.pop_front() {
            open.written_reasoning.push_str(&word);
            (DeltaKind::Reasoning, 0, word)
        } else if let Some(word) = open.text.pop_front() {
            open.written_text.push_str(&word);
            (DeltaKind::Text, u32::from(has_reasoning), word)
        } else {
            self.end_reply(id);
            return false;
        };
        let update = Update::from(LiveDelta {
            session: id.clone(),
            turn: open.turn.clone(),
            step: open.step.clone(),
            after_seq,
            block,
            kind,
            text: word,
        });
        session
            .subscribers
            .retain(|sender| sender.unbounded_send(update.clone()).is_ok());
        true
    }

    fn end_reply(&mut self, id: &SessionId) {
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
