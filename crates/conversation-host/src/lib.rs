//! The conversation kit's one way in.
//!
//! Every binding reaches the platform through [`ConversationHost`] and
//! nothing else: the kit never opens a socket. Its methods mirror the SDK's
//! agents interface (`sessions.create`, `send`, `continue`, `cancel`,
//! `rename`, `subscribe`), so the SDK adapter is a thin mapping. Until that
//! adapter lands, [`ScriptedHost`] plays scripted replies with real events.

mod scripted;

pub use scripted::{ScriptedHost, ScriptedReply};

use conversation_model::schema::{Block, Seq, SessionId};
use conversation_model::{ModelCatalog, Update};
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
    /// When it last changed, in milliseconds since the epoch.
    pub updated_at: u64,
    /// Whether it is archived.
    pub archived: bool,
}

/// Which sessions to list.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct SessionQuery {
    /// Archived ones instead of the others.
    pub archived: bool,
    /// Only those whose title contains this text.
    pub text: Option<String>,
    /// At most this many; 0 means the host's default.
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

    /// Archive a session, or bring it back.
    fn archive(&self, session: &SessionId, archived: bool) -> HostFuture<()>;

    /// The chat models the caller may use, with whether each can answer now.
    fn models(&self) -> HostFuture<ModelCatalog>;

    /// Follow a session from just after `from_seq` (0 for everything).
    fn subscribe(&self, session: &SessionId, from_seq: Seq) -> HostFuture<UpdateStream>;
}
