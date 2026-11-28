// Copyright 2026 Anud Inc.
// SPDX-License-Identifier: GPL-3.0-or-later

//! The conversation kit's host on the SDK's agents interface.
//!
//! [`SdkHost`] maps each [`ConversationHost`] method one to one onto the
//! SDK's `sessions.*` calls, under the app's own token, and turns a session
//! stream into the kit's [`Update`]s. The event types are the SDK's
//! re-export of the harness schema, the same crate the kit folds, so events
//! pass through unchanged.

use conversation_host::{
    ConversationHost, ForkAt, HostError, HostFuture, NewSession, SessionEntry, SessionQuery,
    UpdateStream,
};
use conversation_model::schema::{
    ApprovalId, Block, Decision, ForkReason, QuestionId, Seq, SessionId,
};
use conversation_model::{Availability, ModelCatalog, ModelOption, Reply, Update, display_name};
use futures::future::LocalBoxFuture;
use futures::{FutureExt as _, StreamExt as _, stream};
use isonomic_client::{Agents, ClientError, Models, SessionUpdate};
use isonomic_proto::agents::{
    AnswerQuestion, CreateSession, DecisionInput, ForkPoint, ListSessions, PresetId,
};
use isonomic_proto::models::ModelState;
use std::rc::Rc;

/// The window's user-action token for the person's decision on an
/// approval in a session: what `approvals.decide` needs. The host calls it
/// at once, inside the person's input; only the future it returns is
/// awaited later.
pub type UserActions =
    Rc<dyn Fn(SessionId, ApprovalId) -> LocalBoxFuture<'static, Result<String, HostError>>>;

/// A conversation host on the SDK's agents interface.
#[derive(Clone)]
pub struct SdkHost {
    agents: Agents,
    models: Option<Models>,
    user_actions: Option<UserActions>,
}

impl SdkHost {
    /// A host over the agents interface of the app's scoped client.
    pub fn new(agents: Agents) -> Self {
        Self {
            agents,
            models: None,
            user_actions: None,
        }
    }

    /// Ask for the window's user-action token when the person decides an
    /// approval. Without it a decision goes without one, and the engine
    /// refuses it (`missing_user_action`).
    pub fn with_user_actions(mut self, user_actions: UserActions) -> Self {
        self.user_actions = Some(user_actions);
        self
    }

    /// Also list models, through the app's `models` interface.
    pub fn with_models(mut self, models: Models) -> Self {
        self.models = Some(models);
        self
    }
}

/// A model's state as the kit shows it.
pub fn availability(state: ModelState) -> Availability {
    match state {
        ModelState::Ready | ModelState::Stopped | ModelState::Starting => Availability::Installed,
        ModelState::Downloading => Availability::Downloading,
        ModelState::NotInstalled => Availability::NotInstalled,
        ModelState::Degraded => Availability::Unavailable,
        _ => Availability::Unknown,
    }
}

/// An SDK error as the kit shows it: the wire's stable code and message.
pub fn host_error(error: ClientError) -> HostError {
    match error {
        ClientError::Engine(wire) => {
            let code = serde_json::to_value(wire.code)
                .ok()
                .and_then(|code| code.as_str().map(str::to_owned))
                .unwrap_or_else(|| "engine".to_owned());
            HostError::new(code, wire.message)
        }
        ClientError::Disconnected => HostError::new("disconnected", "Not connected to the engine."),
        ClientError::UnexpectedReply => HostError::new(
            "unexpected_reply",
            "The engine sent a reply of the wrong kind.",
        ),
    }
}

/// An SDK session update as a kit update, or `None` to end the stream. A
/// gap ends it, so the view follows again from its last `seq` and reads the
/// events that were skipped; a purge ends it for good.
pub fn update(update: SessionUpdate) -> Option<Update> {
    match update {
        SessionUpdate::Event(event) => Some(Update::Event(event)),
        SessionUpdate::Delta(delta) => Some(Update::Delta(delta)),
        SessionUpdate::Ended(_) => Some(Update::Ended),
        SessionUpdate::Gap { .. } => None,
    }
}

impl ConversationHost for SdkHost {
    fn create(&self, request: NewSession) -> HostFuture<SessionId> {
        let agents = self.agents.clone();
        async move {
            // `model` overrides the preset's for this session; `None` keeps it.
            let request = CreateSession {
                preset: PresetId::new(request.preset),
                title: request.title,
                location_opt_in: false,
                model: request.model,
            };
            agents.create_with(request).await.map_err(host_error)
        }
        .boxed_local()
    }

    fn send(&self, session: &SessionId, content: Vec<Block>) -> HostFuture<()> {
        let (agents, session) = (self.agents.clone(), session.clone());
        async move {
            agents
                .send(&session, content)
                .await
                .map(|_queued| ())
                .map_err(host_error)
        }
        .boxed_local()
    }

    fn resume(&self, session: &SessionId) -> HostFuture<()> {
        let (agents, session) = (self.agents.clone(), session.clone());
        async move { agents.continue_reply(&session).await.map_err(host_error) }.boxed_local()
    }

    fn cancel(&self, session: &SessionId) -> HostFuture<()> {
        let (agents, session) = (self.agents.clone(), session.clone());
        async move { agents.cancel(&session).await.map_err(host_error) }.boxed_local()
    }

    fn rename(&self, session: &SessionId, title: String) -> HostFuture<()> {
        let (agents, session) = (self.agents.clone(), session.clone());
        async move { agents.rename(&session, &title).await.map_err(host_error) }.boxed_local()
    }

    fn list(&self, query: SessionQuery) -> HostFuture<Vec<SessionEntry>> {
        let agents = self.agents.clone();
        async move {
            // Page through until the list is complete, or `limit` is reached:
            // the engine sends at most a page at a time.
            let wanted = (query.limit > 0).then_some(query.limit as usize);
            let mut items = Vec::new();
            let mut cursor = None;
            loop {
                let filter = ListSessions {
                    archived: Some(query.archived),
                    text: query.text.clone(),
                    cursor: cursor.take(),
                    ..Default::default()
                };
                let (page, next) = agents.list(filter).await.map_err(host_error)?;
                let empty = page.is_empty();
                items.extend(page);
                let full = wanted.is_some_and(|wanted| items.len() >= wanted);
                match next {
                    Some(next) if !full && !empty => cursor = Some(next),
                    _ => break,
                }
            }
            if let Some(wanted) = wanted {
                items.truncate(wanted);
            }
            Ok(items
                .into_iter()
                .map(|summary| SessionEntry {
                    id: summary.id,
                    title: summary.title,
                    created_at: summary.created_at,
                    updated_at: summary.updated_at,
                    archived: summary.archived,
                    model: summary.model,
                    forked_from: summary.forked_from.map(|f| (f.session, f.seq)),
                })
                .collect())
        }
        .boxed_local()
    }

    fn fork(&self, session: &SessionId, at: ForkAt, reason: ForkReason) -> HostFuture<SessionId> {
        let (agents, session) = (self.agents.clone(), session.clone());
        let at = match at {
            ForkAt::Before(turn) => ForkPoint::BeforeTurn(turn),
            ForkAt::After(turn) => ForkPoint::AfterTurn(turn),
        };
        async move {
            agents
                .fork(&session, at, Some(reason))
                .await
                .map_err(host_error)
        }
        .boxed_local()
    }

    fn availability(&self, model: &str) -> HostFuture<Availability> {
        let (models, model) = (self.models.clone(), model.to_owned());
        async move {
            let Some(models) = models else {
                return Ok(Availability::Unknown);
            };
            let detail = models.info(&model).await.map_err(host_error)?;
            Ok(availability(detail.state))
        }
        .boxed_local()
    }

    fn archive(&self, session: &SessionId, archived: bool) -> HostFuture<()> {
        let (agents, session) = (self.agents.clone(), session.clone());
        async move { agents.archive(&session, archived).await.map_err(host_error) }.boxed_local()
    }

    fn models(&self) -> HostFuture<ModelCatalog> {
        let models = self.models.clone();
        async move {
            let Some(models) = models else {
                return Ok(ModelCatalog::default());
            };
            let listed = models.list().await.map_err(host_error)?;
            let recommended = listed
                .iter()
                .find(|m| m.recommended && m.supports("chat"))
                .map(|m| m.id.clone());
            let mut options = Vec::new();
            for model in listed.into_iter().filter(|m| m.supports("chat")) {
                // A model whose state cannot be read is offered as unknown,
                // never as installed.
                let state = models.info(&model.id).await.map(|d| d.state).ok();
                options.push(ModelOption {
                    id: model.id,
                    name: display_name(&model.name),
                    remote: model.remote,
                    availability: state.map_or(Availability::Unknown, availability),
                });
            }
            Ok(ModelCatalog::new(options, recommended))
        }
        .boxed_local()
    }

    fn decide(
        &self,
        session: &SessionId,
        approval: &ApprovalId,
        decision: Decision,
    ) -> HostFuture<()> {
        let (agents, session, approval) = (self.agents.clone(), session.clone(), approval.clone());
        let user_actions = self.user_actions.clone();
        // Deny-safe: only a clear allow allows.
        let decision = match decision {
            Decision::Allow => DecisionInput::Allow,
            _ => DecisionInput::Deny,
        };
        // Ask for the token now, while the person's click or key press is
        // being handled: the window refuses a token asked for later.
        let token = user_actions.map(|mint| mint(session.clone(), approval.clone()));
        async move {
            let token = match token {
                Some(token) => Some(token.await?),
                None => None,
            };
            agents
                .decide(&session, &approval, decision, token.as_deref())
                .await
                .map_err(host_error)
        }
        .boxed_local()
    }

    fn answer(&self, session: &SessionId, question: &QuestionId, reply: Reply) -> HostFuture<()> {
        let (selected, text, skipped) = match reply {
            Reply::Answered { selected, text } => (selected, text, false),
            // A skip and a new message are no answer, never a guess.
            Reply::Skipped | Reply::MovedOn => (Vec::new(), None, true),
        };
        let answer = AnswerQuestion {
            session: session.clone(),
            question: question.clone(),
            selected,
            text,
            skipped,
        };
        let agents = self.agents.clone();
        async move { agents.answer(answer).await.map_err(host_error) }.boxed_local()
    }

    fn subscribe(&self, session: &SessionId, from_seq: Seq) -> HostFuture<UpdateStream> {
        let (agents, session) = (self.agents.clone(), session.clone());
        async move {
            let stream = agents
                .subscribe(&session, from_seq)
                .await
                .map_err(host_error)?;
            let updates = stream::unfold(Some(stream), |stream| async move {
                let mut stream = stream?;
                let next = update(stream.next().await?)?;
                // Nothing follows an end.
                let rest = (next != Update::Ended).then_some(stream);
                Some((next, rest))
            });
            Ok(updates.boxed_local())
        }
        .boxed_local()
    }
}
