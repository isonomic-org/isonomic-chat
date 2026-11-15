//! One conversation: the turns, the composer and continue.

use conversation_host::{ConversationHost, HostError, NewSession};
use conversation_model::schema::{Block, SessionId};
use conversation_model::{InputState, ModelCatalog, Thread, Update};
use futures::StreamExt as _;
use std::cell::RefCell;
use std::rc::Rc;
use ui::component::message_scroller::{MessageScroller, MessageScrollerState};
use ui::component::{ActiveTheme as _, v_flex};
use ui::extra::prompt_composer::{ComposerChoice, PromptComposer, PromptComposerEvent};
use ui::prelude::*;
use ui::{App, Entity, EventEmitter, FocusHandle, Focusable, SharedString, Task, Window, div};

use crate::ConversationCopy;
use crate::turn::{OnClick, RowActions, render_turn};

/// What a conversation view reports to the app that mounts it.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ConversationEvent {
    /// The first send created the session.
    SessionStarted(SessionId),
    /// The session's title changed.
    TitleChanged(SharedString),
    /// The host refused a request; the message is readable.
    HostError(SharedString),
    /// A model was picked. It applies to the next new conversation.
    ModelSelected(SharedString),
    /// The picked model is not installed; the app may offer its download.
    ModelUnavailable(SharedString),
}

/// One conversation, bound to a host.
pub struct ConversationView {
    host: Rc<dyn ConversationHost>,
    preset: String,
    copy: Rc<ConversationCopy>,
    thread: Rc<RefCell<Thread>>,
    scroller: Entity<MessageScrollerState>,
    composer: Entity<PromptComposer>,
    focus: FocusHandle,
    title: Option<String>,
    catalog: ModelCatalog,
    wanted: Option<String>,
    _models: Option<Task<()>>,
    _follow: Option<Task<()>>,
    _request: Option<Task<()>>,
    _composer_events: ui::Subscription,
}

impl EventEmitter<ConversationEvent> for ConversationView {}

impl Focusable for ConversationView {
    fn focus_handle(&self, _: &App) -> FocusHandle {
        self.focus.clone()
    }
}

impl ConversationView {
    /// A new conversation. The first send creates a session from `preset`.
    pub fn new(
        host: Rc<dyn ConversationHost>,
        preset: impl Into<String>,
        copy: ConversationCopy,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Self {
        let composer = cx.new(|cx| {
            let mut composer = PromptComposer::new(copy.placeholder.clone(), window, cx);
            composer.set_dictation(false, cx);
            composer
        });
        let composer_events =
            cx.subscribe_in(&composer, window, |this, _, event, _, cx| match event {
                PromptComposerEvent::Submit { text, .. } => this.send(text.to_string(), cx),
                PromptComposerEvent::Stop => this.stop(cx),
                PromptComposerEvent::SelectModel(id) => this.select_model(id.to_string(), cx),
                _ => {}
            });
        let mut view = Self {
            host,
            preset: preset.into(),
            copy: Rc::new(copy),
            thread: Rc::new(RefCell::new(Thread::new())),
            scroller: cx.new(|cx| MessageScrollerState::new(0, cx)),
            composer,
            focus: cx.focus_handle(),
            title: None,
            catalog: ModelCatalog::default(),
            wanted: None,
            _models: None,
            _follow: None,
            _request: None,
            _composer_events: composer_events,
        };
        view.load_models(cx);
        view
    }

    /// The models on offer, in order.
    pub fn catalog(&self) -> &ModelCatalog {
        &self.catalog
    }

    /// The model a new session runs on: the picked one if the host listed
    /// it, else the one the catalog would choose, else the preset's.
    pub fn model(&self) -> Option<&str> {
        match self.wanted.as_deref() {
            Some(id) if self.catalog.get(id).is_some() => Some(id),
            _ => self.catalog.choose(None).map(|o| o.id.as_str()),
        }
    }

    /// Prefer this model for the session this view creates.
    pub fn select_model(&mut self, id: String, cx: &mut Context<Self>) {
        self.wanted = Some(id.clone());
        self.show_models(cx);
        cx.emit(ConversationEvent::ModelSelected(id.into()));
    }

    fn load_models(&mut self, cx: &mut Context<Self>) {
        let request = self.host.models();
        self._models = Some(cx.spawn(async move |this, cx| {
            // No catalog is not an error: the preset's model is used.
            let catalog = request.await.unwrap_or_default();
            let _ = this.update(cx, |this, cx| {
                this.catalog = catalog;
                this.show_models(cx);
            });
        }));
    }

    fn show_models(&mut self, cx: &mut Context<Self>) {
        let choices = self
            .catalog
            .options()
            .iter()
            .map(|option| ComposerChoice::new(option.id.clone(), option.name.clone()))
            .collect();
        let selected = self.model().map(|id| SharedString::from(id.to_owned()));
        self.composer.update(cx, |composer, cx| {
            composer.set_models(choices, selected, cx)
        });
        cx.notify();
    }

    /// An existing session: replays its log, then follows it.
    pub fn open(
        host: Rc<dyn ConversationHost>,
        session: SessionId,
        copy: ConversationCopy,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Self {
        let mut view = Self::new(host, String::new(), copy, window, cx);
        *view.thread.borrow_mut() = Thread::for_session(session.clone());
        view.follow(session, cx);
        view
    }

    /// The folded thread.
    pub fn thread(&self) -> std::cell::Ref<'_, Thread> {
        self.thread.borrow()
    }

    /// The session, once created.
    pub fn session(&self) -> Option<SessionId> {
        self.thread.borrow().session().cloned()
    }

    /// What the composer offers now.
    pub fn input_state(&self) -> InputState {
        self.thread.borrow().input_state()
    }

    /// Focus the composer.
    pub fn focus_composer(&self, window: &mut Window, cx: &mut App) {
        self.composer
            .update(cx, |composer, cx| composer.focus(window, cx));
    }

    /// Send a prompt: shown at once, then confirmed by the host.
    pub fn send(&mut self, text: String, cx: &mut Context<Self>) {
        if text.trim().is_empty() || !self.input_state().can_send() {
            return;
        }
        self.thread.borrow_mut().begin_send(text.clone());
        self.rows_changed(cx);
        let host = self.host.clone();
        let session = self.session();
        let preset = self.preset.clone();
        let model = self.model().map(str::to_owned);
        self._request = Some(cx.spawn(async move |this, cx| {
            let session = match session {
                Some(session) => session,
                None => {
                    let request = NewSession {
                        preset,
                        title: None,
                        model,
                    };
                    match host.create(request).await {
                        Ok(session) => {
                            let started = session.clone();
                            let _ = this.update(cx, |this, cx| {
                                this.follow(started.clone(), cx);
                                cx.emit(ConversationEvent::SessionStarted(started));
                            });
                            session
                        }
                        Err(error) => {
                            let _ = this.update(cx, |this, cx| this.refused(error, cx));
                            return;
                        }
                    }
                }
            };
            if let Err(error) = host.send(&session, vec![Block::text(text)]).await {
                let _ = this.update(cx, |this, cx| this.refused(error, cx));
            }
        }));
    }

    /// Stop the reply being written. The partial reply is kept.
    pub fn stop(&mut self, cx: &mut Context<Self>) {
        let Some(session) = self.session() else {
            return;
        };
        let request = self.host.cancel(&session);
        self.request(request, cx);
    }

    /// Continue the last reply from where it stopped.
    pub fn resume(&mut self, cx: &mut Context<Self>) {
        if !self.input_state().can_continue() {
            return;
        }
        let Some(session) = self.session() else {
            return;
        };
        let request = self.host.resume(&session);
        self.request(request, cx);
    }

    fn request(&mut self, request: conversation_host::HostFuture<()>, cx: &mut Context<Self>) {
        self._request = Some(cx.spawn(async move |this, cx| {
            if let Err(error) = request.await {
                let _ = this.update(cx, |this, cx| this.refused(error, cx));
            }
        }));
    }

    fn refused(&mut self, error: HostError, cx: &mut Context<Self>) {
        let pending = self
            .thread
            .borrow()
            .turns()
            .last()
            .is_some_and(|turn| turn.state == conversation_model::TurnState::Sending);
        let message = match error.code.as_str() {
            "capability_denied" if self.session().is_none() => self.copy.model_denied.to_string(),
            "unknown_model" => self.copy.model_unknown.to_string(),
            "model_unavailable" => {
                if let Some(model) = self.model() {
                    cx.emit(ConversationEvent::ModelUnavailable(model.to_owned().into()));
                }
                self.copy.model_unavailable.to_string()
            }
            _ => error.message.clone(),
        };
        if pending {
            self.thread.borrow_mut().fail_send(message.clone());
            self.rows_changed(cx);
        }
        cx.emit(ConversationEvent::HostError(message.into()));
    }

    /// Follow the session's updates from the last `seq` seen.
    fn follow(&mut self, session: SessionId, cx: &mut Context<Self>) {
        let from = self.thread.borrow().last_seq();
        let subscribe = self.host.subscribe(&session, from);
        self._follow = Some(cx.spawn(async move |this, cx| {
            let mut updates = match subscribe.await {
                Ok(updates) => updates,
                Err(error) => {
                    let _ = this.update(cx, |this, cx| this.refused(error, cx));
                    return;
                }
            };
            while let Some(update) = updates.next().await {
                if this.update(cx, |this, cx| this.apply(update, cx)).is_err() {
                    break;
                }
            }
        }));
    }

    /// Fold one update in and re-render only what moved.
    fn apply(&mut self, update: Update, cx: &mut Context<Self>) {
        self.thread.borrow_mut().apply(&update);
        let title = self.thread.borrow().title().map(str::to_owned);
        if title.is_some() && title != self.title {
            self.title = title.clone();
            cx.emit(ConversationEvent::TitleChanged(
                title.unwrap_or_default().into(),
            ));
        }
        self.rows_changed(cx);
    }

    fn rows_changed(&mut self, cx: &mut Context<Self>) {
        let count = self.thread.borrow().turns().len();
        let busy = self.input_state().is_busy();
        self.scroller.update(cx, |scroller, cx| {
            let known = scroller.item_count();
            if count > known {
                scroller.append(count - known, cx);
            } else if count < known {
                scroller.reset(count, cx);
            }
            if count > 0 {
                scroller.remeasure_items(count - 1..count, cx);
            }
        });
        self.composer
            .update(cx, |composer, cx| composer.set_busy(busy, cx));
        cx.notify();
    }
}

impl Render for ConversationView {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let thread = self.thread.clone();
        let copy = self.copy.clone();
        let view = cx.weak_entity();
        let list = MessageScroller::new(
            "conversation-turns",
            self.scroller.clone(),
            move |index, _, cx| {
                let thread = thread.borrow();
                let Some(turn) = thread.turns().get(index) else {
                    return div().into_any_element();
                };
                let last = index + 1 == thread.turns().len();
                let on_continue = (last && thread.input_state().can_continue()).then(|| {
                    let view = view.clone();
                    Box::new(move |_: &mut Window, cx: &mut App| {
                        let _ = view.update(cx, |view, cx| view.resume(cx));
                    }) as OnClick
                });
                render_turn(index, turn, &copy, RowActions { on_continue }, cx)
            },
        )
        .with_bottom_fade(cx.theme().background);

        v_flex()
            .id("conversation")
            .track_focus(&self.focus)
            .size_full()
            .min_h_0()
            .child(div().flex_1().min_h_0().w_full().child(list))
            .child(div().w_full().child(self.composer.clone()))
    }
}
