// Copyright 2026 Anud Inc.
// SPDX-License-Identifier: GPL-3.0-or-later

//! A session's events folded into turns.

use crate::Update;
use crate::input::InputState;
use crate::schema::{
    AttemptOutcome, Block, Cause, DeltaKind, Event, LiveDelta, Purpose, Seq, SessionEvent,
    SessionId, Status, StepId, StopReason, TurnId, TurnStop,
};
use crate::turn::{Notice, Turn, TurnState};
use std::collections::BTreeSet;

mod asks;
mod branch;
mod tools;

/// The error code of a turn that failed because the conversation no longer
/// fits the model.
pub const TOO_LARGE: &str = "context.too_large";

/// One session as a list of turns.
///
/// Feed it every [`Update`] the host reports, in order. Durable events are
/// the truth: a step's live deltas are dropped as soon as its `assistant`
/// event (or its failure) arrives, and an event at or below the last `seq`
/// seen is ignored, so resubscribing from [`Thread::last_seq`] is safe.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Thread {
    session: Option<SessionId>,
    title: Option<String>,
    turns: Vec<Turn>,
    notices: Vec<Notice>,
    last_seq: Seq,
    closed: bool,
    /// A divider before the first turn (a compaction with no turn yet).
    divider: Option<String>,
    awaiting_client: bool,
    committed: BTreeSet<StepId>,
    /// Steps that are not the reply (the title step, summaries): their text
    /// is never part of a turn.
    side_steps: BTreeSet<StepId>,
    /// The last reply's stop reason in the open turn.
    last_stop: Option<StopReason>,
    /// A refusal or error recorded by an attempt, for the turn's end.
    attempt_message: Option<String>,
    revision: u64,
    /// How many turns at the start came from the sessions this one was
    /// forked from; their `start_seq` is in those sessions' logs.
    carried: usize,
}

impl Thread {
    /// An empty thread, before any session exists.
    pub fn new() -> Self {
        Self::default()
    }

    /// A thread for an existing session.
    pub fn for_session(session: SessionId) -> Self {
        Self {
            session: Some(session),
            ..Self::default()
        }
    }

    /// Fold a whole log, for tests and replay.
    pub fn from_events<'a>(events: impl IntoIterator<Item = &'a SessionEvent>) -> Self {
        let mut thread = Self::new();
        for event in events {
            thread.apply_event(event);
        }
        thread
    }

    /// The session, once known.
    pub fn session(&self) -> Option<&SessionId> {
        self.session.as_ref()
    }

    /// The latest title.
    pub fn title(&self) -> Option<&str> {
        self.title.as_deref()
    }

    /// The turns, oldest first.
    pub fn turns(&self) -> &[Turn] {
        &self.turns
    }

    /// Unknown events from before the first turn.
    pub fn notices(&self) -> &[Notice] {
        &self.notices
    }

    /// The last durable `seq` folded in: subscribe from here to resume.
    pub fn last_seq(&self) -> Seq {
        self.last_seq
    }

    /// Bumped on every change.
    pub fn revision(&self) -> u64 {
        self.revision
    }

    /// What the composer offers next.
    pub fn input_state(&self) -> InputState {
        if self.closed {
            return InputState::Closed;
        }
        match self.turns.last().map(|turn| &turn.state) {
            Some(state) if state.is_open() => InputState::Busy,
            Some(state) if state.can_continue() => InputState::Continuable,
            _ => InputState::Ready,
        }
    }

    /// Show a prompt at once, before the host confirms it (an optimistic
    /// send). The host's `turn.start` and `user` events adopt it.
    pub fn begin_send(&mut self, prompt: impl Into<String>) {
        self.turns
            .push(Turn::new(None, prompt.into(), TurnState::Sending));
        self.touch_last();
    }

    /// A divider before the first turn, if a compaction came before any.
    pub fn divider(&self) -> Option<&str> {
        self.divider.as_deref()
    }

    /// Whether a client tool's app is gone and the session waits for it.
    pub fn awaiting_client(&self) -> bool {
        self.awaiting_client
    }

    /// The prompt to send again for the last turn, if it failed or was
    /// rejected. A rejected send was never opened by the host, so it is
    /// dropped here; a failed turn stays in the log and in the thread.
    pub fn take_retry(&mut self) -> Option<String> {
        let turn = self.turns.last()?;
        // The same prompt cannot fit a conversation that is too large.
        if turn.prompt.is_empty() || turn.error_code.as_deref() == Some(TOO_LARGE) {
            return None;
        }
        match turn.state {
            TurnState::Rejected(_) if turn.ids.is_empty() => {
                let prompt = self.turns.pop().map(|t| t.prompt);
                self.revision += 1;
                prompt
            }
            TurnState::Failed(_) | TurnState::Rejected(_) => Some(turn.prompt.clone()),
            _ => None,
        }
    }

    /// The host refused to continue the last turn; it stays as it was, with
    /// the reason shown.
    pub fn fail_continue(&mut self, message: impl Into<String>) {
        if let Some(turn) = self.turns.last_mut() {
            turn.refusal = Some(message.into());
            self.touch_last();
        }
    }

    /// The host refused an optimistic send before opening a turn.
    pub fn fail_send(&mut self, message: impl Into<String>) {
        if let Some(turn) = self.turns.last_mut()
            && turn.state == TurnState::Sending
        {
            turn.state = TurnState::Rejected(message.into());
            self.touch_last();
        }
    }

    /// Fold in one update.
    pub fn apply(&mut self, update: &Update) {
        match update {
            Update::Event(event) => self.apply_event(event),
            Update::Delta(delta) => self.apply_delta(delta),
            Update::Ended => {
                self.closed = true;
                self.revision += 1;
            }
        }
    }

    /// Fold in a live delta. Lossy: a dropped delta only delays text until
    /// the durable reply arrives.
    pub fn apply_delta(&mut self, delta: &LiveDelta) {
        if self.session.as_ref().is_some_and(|s| s != &delta.session)
            || self.committed.contains(&delta.step)
            || self.side_steps.contains(&delta.step)
        {
            return;
        }
        let Some(index) = self.turn_index(&delta.turn) else {
            return;
        };
        let turn = &mut self.turns[index];
        let target = match delta.kind {
            DeltaKind::Text => &mut turn.live,
            DeltaKind::Reasoning => &mut turn.live_reasoning,
            _ => return,
        };
        target
            .entry(delta.step.clone())
            .or_default()
            .entry(delta.block)
            .or_default()
            .push_str(&delta.text);
        self.touch(index);
    }

    /// Fold in one durable event.
    pub fn apply_event(&mut self, event: &SessionEvent) {
        match &self.session {
            Some(session) if session != &event.session => return,
            None => self.session = Some(event.session.clone()),
            _ => {}
        }
        if event.seq <= self.last_seq {
            return;
        }
        self.last_seq = event.seq;
        self.revision += 1;
        if let Some(step) = &event.step
            && self.side_steps.contains(step)
        {
            return;
        }
        match &event.event {
            Event::StepStart(start) if start.purpose != Purpose::Reply => {
                if let Some(step) = &event.step {
                    self.side_steps.insert(step.clone());
                }
            }
            Event::SessionStatus(status) => {
                self.closed = status.status == Status::Failed;
                self.awaiting_client = status.status == Status::AwaitingClient;
            }
            Event::Title(title) => self.title = Some(title.title.clone()),
            Event::TurnStart(start) => {
                self.start_turn(event.turn.clone(), &start.cause, event.seq);
                if let Some(turn) = self.turns.last_mut() {
                    turn.started_at.get_or_insert(event.at);
                }
            }
            Event::User(user) => {
                if let Some(index) = event.turn.as_ref().and_then(|t| self.turn_index(t)) {
                    self.turns[index].prompt = text_of(&user.content);
                    self.touch(index);
                }
            }
            Event::Assistant(reply) => {
                let Some(index) = self.event_turn(event) else {
                    return;
                };
                if let Some(step) = &event.step {
                    self.drop_live(index, step);
                }
                let turn = &mut self.turns[index];
                turn.retrying = None;
                let joins = reply.continues.is_some() || turn.continuing;
                commit(turn, &reply.content, joins);
                if turn.thought_ms.is_none() && !turn.reasoning.is_empty() {
                    turn.thought_ms = turn.started_at.map(|at| event.at.saturating_sub(at));
                }
                turn.continuing = false;
                self.last_stop = Some(reply.stop.clone());
                self.touch(index);
            }
            Event::Attempt(attempt) => {
                let Some(index) = self.event_turn(event) else {
                    return;
                };
                if let Some(step) = &event.step {
                    self.drop_live(index, step);
                }
                match attempt.outcome {
                    AttemptOutcome::Refused => {
                        self.attempt_message = Some(text_of(&attempt.content));
                    }
                    AttemptOutcome::Error if attempt.will_retry => {
                        let reason = attempt.error.as_ref().map(|e| e.message.clone());
                        self.turns[index].retrying = Some(reason.unwrap_or_default());
                    }
                    AttemptOutcome::Error => {
                        // The engine gave up: what streamed stays on show
                        // above the error (display only; it is not sent).
                        let turn = &mut self.turns[index];
                        let joins = turn.continuing;
                        commit(turn, &attempt.content, joins);
                        self.attempt_message =
                            attempt.error.as_ref().map(|error| error.message.clone());
                    }
                    _ => {}
                }
                self.touch(index);
            }
            Event::StepEnd(_) => {
                if let (Some(index), Some(step)) = (self.event_turn(event), &event.step) {
                    self.drop_live(index, step);
                    self.touch(index);
                }
            }
            Event::ToolCall(_)
            | Event::ToolResult(_)
            | Event::ApprovalRequest(_)
            | Event::ApprovalDecision(_) => {
                if let Some(index) = self.event_turn(event)
                    && tools::fold(&mut self.turns[index], event)
                {
                    self.touch(index);
                }
            }
            Event::Question(_) | Event::Answer(_) => {
                if let Some(index) = self.event_turn(event)
                    && asks::fold(&mut self.turns[index], event)
                {
                    self.touch(index);
                }
            }
            Event::TurnEnd(end) => {
                if let Some(index) = self.event_turn(event) {
                    let message = end
                        .reason
                        .clone()
                        .or_else(|| end.error.as_ref().map(|e| e.message.clone()));
                    let state = self.end_state(&end.stop, message);
                    let turn = &mut self.turns[index];
                    turn.error_code = end.error.as_ref().map(|e| e.code.clone());
                    turn.live.clear();
                    turn.live_reasoning.clear();
                    turn.retrying = None;
                    turn.continuing = false;
                    turn.state = state;
                    self.touch(index);
                }
                self.last_stop = None;
                self.attempt_message = None;
            }
            Event::Compacted(compacted) => {
                if let Some(summary) = &compacted.summary {
                    // After the last turn the host has confirmed: a send in
                    // flight comes after the compaction, not before it.
                    let confirmed = self.turns.iter().rposition(|t| !t.ids.is_empty());
                    match confirmed {
                        Some(last) => {
                            self.turns[last].divider_after = Some(summary.text.clone());
                            self.touch(last);
                        }
                        None => self.divider = Some(summary.text.clone()),
                    }
                }
            }
            Event::Unknown(unknown) => {
                let notice = Notice {
                    kind: unknown.kind.0.clone(),
                };
                match self.event_turn(event) {
                    Some(index) => {
                        self.turns[index].notices.push(notice);
                        self.touch(index);
                    }
                    None => self.notices.push(notice),
                }
            }
            // Artifacts, memory and the rest are shown by their
            // own bindings as they land; the turns do not change.
            _ => {}
        }
    }

    fn start_turn(&mut self, id: Option<TurnId>, cause: &Cause, seq: Seq) {
        self.last_stop = None;
        self.attempt_message = None;
        match cause {
            Cause::Continue => {
                if let Some(turn) = self.turns.last_mut() {
                    turn.refusal = None;
                    turn.ids.extend(id);
                    turn.state = TurnState::Running;
                    turn.continuing = true;
                    self.touch_last();
                    return;
                }
            }
            Cause::User => {
                if let Some(turn) = self.turns.last_mut()
                    && turn.state == TurnState::Sending
                    && turn.ids.is_empty()
                {
                    turn.ids.extend(id);
                    turn.state = TurnState::Running;
                    turn.start_seq = seq;
                    self.touch_last();
                    return;
                }
            }
            _ => {}
        }
        let mut turn = Turn::new(id, String::new(), TurnState::Running);
        turn.start_seq = seq;
        self.turns.push(turn);
        self.touch_last();
    }

    fn end_state(&self, stop: &TurnStop, message: Option<String>) -> TurnState {
        let attempt = self.attempt_message.clone();
        match stop {
            TurnStop::Done if self.last_stop == Some(StopReason::MaxTokens) => TurnState::Truncated,
            TurnStop::Cancelled | TurnStop::Interrupted => TurnState::Stopped,
            TurnStop::Failed => TurnState::Failed(attempt.or(message).unwrap_or_default()),
            TurnStop::Refused => TurnState::Refused(attempt.or(message).unwrap_or_default()),
            TurnStop::Budget => TurnState::Limited,
            TurnStop::Rejected => TurnState::Rejected(message.unwrap_or_default()),
            _ => TurnState::Done,
        }
    }

    fn turn_index(&self, id: &TurnId) -> Option<usize> {
        self.turns.iter().rposition(|turn| turn.spans(id))
    }

    fn event_turn(&self, event: &SessionEvent) -> Option<usize> {
        event.turn.as_ref().and_then(|id| self.turn_index(id))
    }

    fn drop_live(&mut self, index: usize, step: &StepId) {
        self.committed.insert(step.clone());
        let turn = &mut self.turns[index];
        turn.live.remove(step);
        turn.live_reasoning.remove(step);
    }

    fn touch(&mut self, index: usize) {
        self.revision += 1;
        self.turns[index].revision += 1;
    }

    fn touch_last(&mut self) {
        if let Some(last) = self.turns.len().checked_sub(1) {
            self.touch(last);
        }
    }
}

/// Add a reply's blocks to the turn. `joins` carries on the text before it
/// (a continuation); otherwise a later step starts a new paragraph.
fn commit(turn: &mut Turn, content: &[Block], joins: bool) {
    let mut first = true;
    for block in content {
        match block {
            Block::Text { text } => {
                let ends_with_break = turn.text.ends_with("\n\n");
                if first && !joins && !turn.text.is_empty() && !ends_with_break {
                    turn.text.push_str("\n\n");
                }
                turn.text.push_str(text);
                first = false;
            }
            Block::Reasoning { text, .. } => {
                if !turn.reasoning.is_empty() {
                    turn.reasoning.push_str("\n\n");
                }
                turn.reasoning.push_str(text);
            }
            Block::Unknown(unknown) => turn.notices.push(Notice {
                kind: unknown.kind.0.clone(),
            }),
            _ => {}
        }
    }
}

/// The text blocks of some content, joined.
fn text_of(content: &[Block]) -> String {
    content
        .iter()
        .filter_map(|block| match block {
            Block::Text { text } => Some(text.as_str()),
            _ => None,
        })
        .collect::<Vec<_>>()
        .join("\n\n")
}
