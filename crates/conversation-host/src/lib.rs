// Copyright 2026 Anud Inc.
// SPDX-License-Identifier: GPL-3.0-or-later

//! The conversation kit's one way in.
//!
//! Every binding reaches the platform through [`ConversationHost`] and
//! nothing else: the kit never opens a socket. Its methods mirror the SDK's
//! agents interface (`sessions.create`, `send`, `continue`, `cancel`,
//! `rename`, `fork`, `subscribe`, `approvals.decide`), so the SDK adapter is a thin mapping. Until that
//! adapter lands, [`ScriptedHost`] plays scripted replies with real events.

mod scripted;

pub use scripted::{Failure, ScriptedHost, ScriptedReply, ScriptedTool};

use conversation_model::schema::{ApprovalId, Block, Decision, ForkReason, Seq, SessionId, TurnId};
use conversation_model::{Availability, ModelCatalog, Update};
use futures::future::LocalBoxFuture;
use futures::stream::LocalBoxStream;
use std::fmt;

/// A host's answer to a request.
pub type HostFuture<T> = LocalBoxFuture<'static, Result<T, HostError>>;

/// A session's updates: first the durable events after the requested `seq`,
/// then new events and live deltas as they happen, with no gap or duplicate.
pub type UpdateStream = LocalBoxStream<'static, Update>;

/// Why a host refused a request.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct HostError {
    /// A stable code, such as `not_found` or `continue_refused`.
    pub code: String,
    /// A short readable sentence.
    pub message: String,
}

impl HostError {
    /// An error with a code and a message.
    pub fn new(code: impl Into<String>, message: impl Into<String>) -> Self {
        Self {
            code: code.into(),
            message: message.into(),
        }
    }
}

impl fmt::Display for HostError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}: {}", self.code, self.message)
    }
}

impl std::error::Error for HostError {}

/// What a new session runs.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct NewSession {
    /// The preset's id, owned by the app that mounts the kit.
    pub preset: String,
    /// A title to start with, if any.
    pub title: Option<String>,
    /// The model to run on, overriding the preset's; `None` uses the
    /// preset's model.
    pub model: Option<String>,
}

/// One session in a list, newest activity first.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SessionEntry {
    /// The session.
    pub id: SessionId,
    /// Its title, once it has one.
    pub title: Option<String>,
    /// When it was created, in milliseconds since the epoch.
    pub created_at: u64,
    /// When it last changed, in milliseconds since the epoch.
    pub updated_at: u64,
    /// Whether it is archived.
    pub archived: bool,
    /// The model picked when it was created; `None` when the preset's runs.
    pub model: Option<String>,
    /// The session it was forked from and the last `seq` of that session it
    /// shares; `None` for a session that is not a fork.
    pub forked_from: Option<(SessionId, Seq)>,
}

/// Where a fork leaves its parent: at a turn boundary.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ForkAt {
    /// Share everything before this turn, not the turn itself.
    Before(TurnId),
    /// Share everything up to and including this turn.
    After(TurnId),
}

/// Which sessions to list.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct SessionQuery {
    /// Archived ones instead of the others.
    pub archived: bool,
    /// Only those whose title contains this text.
    pub text: Option<String>,
    /// At most this many; 0 means every one (the host pages through them).
    pub limit: u32,
}

/// The platform, as the conversation kit sees it.
///
/// Requests resolve on the foreground; what they cause arrives on the
/// session's [`UpdateStream`], never in the reply.
pub trait ConversationHost {
    /// Create a session.
    fn create(&self, request: NewSession) -> HostFuture<SessionId>;

    /// Send the person's input. Opens a turn, or queues behind the open one.
    fn send(&self, session: &SessionId, content: Vec<Block>) -> HostFuture<()>;

    /// Extend the last reply that stopped early (stopped or cut off).
    fn resume(&self, session: &SessionId) -> HostFuture<()>;

    /// Stop the open turn. The partial reply is kept. Safe to repeat.
    fn cancel(&self, session: &SessionId) -> HostFuture<()>;

    /// Rename the session.
    fn rename(&self, session: &SessionId, title: String) -> HostFuture<()>;

    /// The caller's sessions, most recently changed first.
    fn list(&self, query: SessionQuery) -> HostFuture<Vec<SessionEntry>>;

    /// A new session that shares this one's history up to `at`. Its own log
    /// starts with `forked`, naming the parent and the last `seq` shared;
    /// the shared turns stay in the parent's log.
    fn fork(&self, session: &SessionId, at: ForkAt, reason: ForkReason) -> HostFuture<SessionId>;

    /// Archive a session, or bring it back.
    fn archive(&self, session: &SessionId, archived: bool) -> HostFuture<()>;

    /// The chat models the caller may use, with whether each can answer now.
    fn models(&self) -> HostFuture<ModelCatalog>;

    /// One model's availability now. The window owns downloads; an app
    /// watches a model it asked for by asking this while it is on its way.
    fn availability(&self, model: &str) -> HostFuture<Availability>;

    /// Answer an approval the session asked for, as the person chose. Only a
    /// clear `Allow` allows; anything else is a denial. The answer arrives
    /// on the session's stream as `approval.decision`.
    fn decide(
        &self,
        session: &SessionId,
        approval: &ApprovalId,
        decision: Decision,
    ) -> HostFuture<()>;

    /// Follow a session from just after `from_seq` (0 for everything).
    fn subscribe(&self, session: &SessionId, from_seq: Seq) -> HostFuture<UpdateStream>;
}
