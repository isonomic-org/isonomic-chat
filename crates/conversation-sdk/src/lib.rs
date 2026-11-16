//! The conversation kit's host on the SDK's agents interface.
//!
//! [`SdkHost`] maps each [`ConversationHost`] method one to one onto the
//! SDK's `sessions.*` calls, under the app's own token, and turns a session
//! stream into the kit's [`Update`]s. The event types are the SDK's
//! re-export of the harness schema, the same crate the kit folds, so events
//! pass through unchanged.

use conversation_host::{
    ConversationHost, HostError, HostFuture, NewSession, SessionEntry, SessionQuery, UpdateStream,
};
use conversation_model::schema::{Block, Seq, SessionId};
use conversation_model::{Availability, ModelCatalog, ModelOption, Update};
use futures::{FutureExt as _, StreamExt as _, stream};
use isonomic_client::{Agents, ClientError, Models, SessionUpdate};
use isonomic_proto::agents::{CreateSession, ListSessions, PresetId};
use isonomic_proto::models::ModelState;

/// A conversation host on the SDK's agents interface.
#[derive(Clone)]
pub struct SdkHost {
    agents: Agents,
    models: Option<Models>,
}

impl SdkHost {
    /// A host over the agents interface of the app's scoped client.
    pub fn new(agents: Agents) -> Self {
        Self {
            agents,
            models: None,
        }
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

/// An SDK session update as a kit update.
pub fn update(update: SessionUpdate) -> Update {
    match update {
        SessionUpdate::Event(event) => Update::Event(event),
        SessionUpdate::Delta(delta) => Update::Delta(delta),
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
            let filter = ListSessions {
                archived: Some(query.archived),
                text: query.text,
                limit: query.limit,
                ..Default::default()
            };
            let (items, _next) = agents.list(filter).await.map_err(host_error)?;
            Ok(items
                .into_iter()
                .map(|summary| SessionEntry {
                    id: summary.id,
                    title: summary.title,
                    updated_at: summary.updated_at,
                    archived: summary.archived,
                })
                .collect())
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
                    name: model.name,
                    remote: model.remote,
                    availability: state.map_or(Availability::Unknown, availability),
                });
            }
            Ok(ModelCatalog::new(options, recommended))
        }
        .boxed_local()
    }

    fn subscribe(&self, session: &SessionId, from_seq: Seq) -> HostFuture<UpdateStream> {
        let (agents, session) = (self.agents.clone(), session.clone());
        async move {
            let stream = agents
                .subscribe(&session, from_seq)
                .await
                .map_err(host_error)?;
            let updates = stream::unfold(stream, |mut stream| async move {
                let next = stream.next().await?;
                Some((update(next), stream))
            });
            Ok(updates.boxed_local())
        }
        .boxed_local()
    }
}
