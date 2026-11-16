//! The shell's chats when it has a backend: the list comes from the host,
//! pins from Chat's store, and opening, renaming and deleting go through
//! them. Without a backend the shell shows the static example chats.

use conversation_host::{ConversationHost, SessionEntry, SessionQuery};
use conversation_model::schema::SessionId;
use conversation_model::{Availability, Branch, newest_first};
use conversation_ui::{ConversationEvent, ConversationView};
use std::rc::Rc;
use ui::prelude::*;
use ui::{Entity, SharedString, Window};

use super::ChatShell;
use crate::Backend;
use crate::demo::SessionSummary;
use crate::{copy, metrics};

impl ChatShell {
    /// Load the chats and the pins, newest activity first, pinned on top.
    ///
    /// A load replaces the one before it (its task is dropped), and a chat
    /// deleted while it was on its way stays deleted.
    pub(super) fn load(&mut self, backend: Backend, cx: &mut Context<Self>) {
        self.reloading = true;
        let task = cx.spawn(async move |this, cx| {
            // Every session: a chat's versions count in the list, so a limit
            // could push chats out.
            let query = SessionQuery::default();
            let listed = backend.host.list(query).await;
            let pins = backend.store.pins().await.unwrap_or_default();
            let _ = this.update(cx, |this, cx| {
                match listed {
                    Ok(entries) => {
                        // A fork is a version inside its chat, not a chat;
                        // a chat is as new as its newest version.
                        let order = newest_first(&branches(&entries));
                        let mut entries: Vec<_> = entries
                            .into_iter()
                            .filter(|entry| entry.forked_from.is_none())
                            .collect();
                        entries.sort_by_key(|entry| {
                            order
                                .iter()
                                .position(|(id, _)| *id == entry.id)
                                .unwrap_or(usize::MAX)
                        });
                        let mut sessions: Vec<SessionSummary> = entries
                            .into_iter()
                            .map(|entry| {
                                let id = SharedString::from(entry.id.0.clone());
                                let pinned = pins.contains(&entry.id.0);
                                let title = entry.title.unwrap_or_else(|| copy::NEW_CHAT.into());
                                // The newest change in any of its versions.
                                let active = order
                                    .iter()
                                    .find(|(root, _)| *root == entry.id)
                                    .map_or(entry.updated_at, |(_, at)| *at);
                                SessionSummary::new(id, title, pinned).active_at(active)
                            })
                            .collect();
                        // Deleted here after the list was asked for.
                        sessions.retain(|s| !this.deleted.contains(&s.id));
                        // Pins keep their pin order; the rest stay newest first.
                        sessions.sort_by_key(|s| {
                            pins.iter()
                                .position(|p| *p == s.id.as_ref())
                                .unwrap_or(usize::MAX)
                        });
                        // Chats started before the list arrived stay, on top.
                        let listed: Vec<SharedString> =
                            sessions.iter().map(|s| s.id.clone()).collect();
                        let mut started: Vec<SessionSummary> = this
                            .sessions
                            .drain(..)
                            .filter(|s| !listed.contains(&s.id))
                            .collect();
                        let pinned = sessions.iter().take_while(|s| s.pinned).count();
                        let rest = sessions.split_off(pinned);
                        sessions.append(&mut started);
                        sessions.extend(rest);
                        this.sessions = sessions;
                        this.load_error = None;
                    }
                    Err(error) => this.load_error = Some(error.message.into()),
                }
                this.loaded = true;
                this.reloading = false;
                this.loads += 1;
                cx.notify();
            });
        });
        self._load = Some(task);
    }

    /// Ask the window to install a model, then watch it until it is ready.
    /// `false` if there is no window to ask, or it could not take the
    /// request now.
    pub fn install_model(&mut self, model: &str, cx: &mut Context<Self>) -> bool {
        let asked = self
            .backend
            .as_ref()
            .and_then(|b| b.install_model.as_ref())
            .is_some_and(|install| install(model));
        self.install_asked = asked.then(|| model.to_owned().into());
        self.asked_polls = 0;
        if asked {
            self.watch_models(cx);
        }
        asked
    }

    /// Watch the models on their way (downloading, or asked of the window)
    /// until each settles: home leaves Downloading the moment the model is
    /// ready. The SDK has no models watch, so this asks each one's state on
    /// a short interval while any is pending, and stops when none is.
    pub(super) fn watch_models(&mut self, cx: &mut Context<Self>) {
        let Some(backend) = self.backend.clone() else {
            return;
        };
        if self._watch.is_some() || self.pending_models().is_empty() {
            return;
        }
        self._watch = Some(cx.spawn(async move |this, cx| {
            loop {
                cx.background_executor().timer(metrics::model_poll()).await;
                let pending = this
                    .update(cx, |this, _| this.pending_models())
                    .unwrap_or_default();
                if pending.is_empty() {
                    break;
                }
                for id in pending {
                    let state = backend.host.availability(&id).await;
                    if this
                        .update(cx, |this, cx| this.model_changed(&id, state.ok(), cx))
                        .is_err()
                    {
                        return;
                    }
                }
            }
            let _ = this.update(cx, |this, _| this._watch = None);
        }));
    }

    /// The models still on their way. The one asked of the window counts
    /// until it is ready or failed, for at most `metrics::ASK_POLLS` polls
    /// while the window has not started it.
    fn pending_models(&mut self) -> Vec<String> {
        let mut pending: Vec<String> = self
            .catalog
            .iter()
            .flat_map(|catalog| catalog.downloading())
            .map(|option| option.id.clone())
            .collect();
        if let Some(asked) = self.install_asked.clone() {
            let state = self
                .catalog
                .as_ref()
                .and_then(|c| c.get(&asked))
                .map(|o| o.availability);
            let waiting = state != Some(Availability::Downloading);
            if waiting {
                self.asked_polls += 1;
            }
            if waiting && self.asked_polls > metrics::ASK_POLLS {
                // The window never started it: offer Install again.
                self.install_asked = None;
            } else if !pending.contains(&asked.to_string()) {
                pending.push(asked.to_string());
            }
        }
        pending
    }

    /// A model's state as the host reported it.
    fn model_changed(&mut self, id: &str, state: Option<Availability>, cx: &mut Context<Self>) {
        let Some(state) = state else {
            return;
        };
        if let Some(catalog) = self.catalog.as_mut() {
            catalog.set_availability(id, state);
        }
        let settled = matches!(state, Availability::Installed | Availability::Unavailable);
        if settled && self.install_asked.as_deref() == Some(id) {
            self.install_asked = None;
        }
        cx.notify();
    }

    /// The model Chat last asked the window to install, if it took it.
    pub fn install_asked(&self) -> Option<&SharedString> {
        self.install_asked.as_ref()
    }

    /// The model to install when no chat model can answer: the recommended
    /// one, else the first listed. `None` while one can answer, or before
    /// the catalog has loaded.
    pub fn missing_model(&self) -> Option<&conversation_model::ModelOption> {
        let catalog = self.catalog.as_ref()?;
        if catalog.choose(None).is_some() {
            return None;
        }
        catalog.recommended().or_else(|| catalog.options().first())
    }

    /// Load the models, to know whether any chat model can answer.
    pub(super) fn load_models(&mut self, backend: &Backend, cx: &mut Context<Self>) {
        let request = backend.host.models();
        self.models_failed = false;
        let task = cx.spawn(async move |this, cx| {
            let listed = request.await;
            let _ = this.update(cx, |this, cx| {
                this.models_failed = listed.is_err();
                this.catalog = Some(listed.unwrap_or_default());
                this.watch_models(cx);
                cx.notify();
            });
        });
        self._models = Some(task);
    }

    /// Whether listing the models failed; home offers to try again.
    pub fn models_failed(&self) -> bool {
        self.models_failed
    }

    /// List the models again, after a failure.
    pub fn reload_models(&mut self, cx: &mut Context<Self>) {
        if let Some(backend) = self.backend.clone() {
            self.load_models(&backend, cx);
        }
    }

    /// Pick the model new chats run on.
    pub fn pick_model(&mut self, model: String, cx: &mut Context<Self>) {
        self.model = Some(model);
        cx.notify();
    }

    /// The models Chat can run on, once loaded.
    pub fn catalog(&self) -> Option<&conversation_model::ModelCatalog> {
        self.catalog.as_ref()
    }

    /// Choose a model in home's picker: one that can answer is picked for
    /// new chats; one that is not installed is asked of the window.
    pub fn choose_model(&mut self, id: String, cx: &mut Context<Self>) {
        let availability = self
            .catalog
            .as_ref()
            .and_then(|catalog| catalog.get(&id))
            .map(|option| option.availability);
        match availability {
            Some(Availability::Installed) => self.pick_model(id, cx),
            Some(Availability::NotInstalled) => {
                self.install_model(&id, cx);
                cx.notify();
            }
            // Downloading, unavailable or unknown: nothing to do yet.
            _ => {}
        }
    }

    /// The model new chats run on, once one has been picked.
    pub fn model(&self) -> Option<&str> {
        self.model.as_deref()
    }

    /// Load the chat list again, after it failed.
    pub fn reload(&mut self, cx: &mut Context<Self>) {
        if let Some(backend) = self.backend.clone() {
            self.load_error = None;
            self.load(backend, cx);
            cx.notify();
        }
    }

    /// Whether the first load of the list has finished.
    pub fn is_loaded(&self) -> bool {
        self.loaded
    }

    /// Whether a load of the list is on its way: the first, or a reload.
    pub fn is_reloading(&self) -> bool {
        self.reloading
    }

    /// How many loads of the list have finished, failed ones included. A
    /// reload has finished once this passes the count read when it began.
    pub fn loads_finished(&self) -> u64 {
        self.loads
    }

    /// Why the list could not load, if it could not.
    pub fn load_error(&self) -> Option<&SharedString> {
        self.load_error.as_ref()
    }

    /// The conversation for a stored chat, opened on first visit.
    pub(super) fn ensure_thread(
        &mut self,
        id: &SharedString,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if self.threads.contains_key(id) {
            return;
        }
        let Some(backend) = self.backend.clone() else {
            return;
        };
        let session = SessionId::new(id.to_string());
        let host = backend.host.clone();
        let thread = cx.new(|cx| {
            let mut view = ConversationView::open(host, session, copy::conversation(), window, cx);
            view.set_model_menu(false, cx);
            view
        });
        self.watch(id.clone(), &thread, cx);
        self.threads.insert(id.clone(), thread);
    }

    /// A new conversation, before the host has named its session.
    pub(super) fn new_thread(
        &mut self,
        id: SharedString,
        host: Rc<dyn ConversationHost>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Entity<ConversationView> {
        let model = self.model.clone();
        let thread = cx.new(|cx| {
            let mut view =
                ConversationView::new(host, crate::PRESET, copy::conversation(), window, cx);
            // Home's footer picks the model; the thread's composer has none.
            view.set_model_menu(false, cx);
            // The model picked on home, or in an earlier chat, runs this one.
            if let Some(model) = model {
                view.select_model(model, cx);
            }
            view
        });
        self.watch(id.clone(), &thread, cx);
        self.threads.insert(id, thread.clone());
        thread
    }

    /// Keep the sidebar in step with a conversation: its title, and its id
    /// once the host creates the session.
    fn watch(
        &mut self,
        id: SharedString,
        thread: &Entity<ConversationView>,
        cx: &mut Context<Self>,
    ) {
        let local = Rc::new(std::cell::RefCell::new(id));
        let subscription = cx.subscribe(thread, move |this, thread, event, cx| {
            let current = local.borrow().clone();
            match event {
                ConversationEvent::TitleChanged(title) => {
                    if let Some(session) = this.sessions.iter_mut().find(|s| s.id == current) {
                        session.title = title.clone();
                    }
                }
                ConversationEvent::SessionStarted(session) => {
                    let named = SharedString::from(session.0.clone());
                    if let Some(summary) = this.sessions.iter_mut().find(|s| s.id == current) {
                        summary.id = named.clone();
                    }
                    this.threads.remove(&current);
                    this.threads.insert(named.clone(), thread);
                    if this.route == super::Route::Session(current.clone()) {
                        this.route = super::Route::Session(named.clone());
                    }
                    *local.borrow_mut() = named;
                }
                ConversationEvent::ModelSelected(model) => this.model = Some(model.to_string()),
                // Downloads belong to the window: ask it to offer this one.
                ConversationEvent::ModelUnavailable(model) => {
                    this.install_model(model, cx);
                }
                ConversationEvent::HostError(_) => {}
            }
            cx.notify();
        });
        self._subscriptions.push(subscription);
    }

    /// Pin or unpin a chat. Pinned chats sort first, in pin order.
    pub fn set_pinned(&mut self, id: &SharedString, pinned: bool, cx: &mut Context<Self>) {
        let Some(index) = self.sessions.iter().position(|s| &s.id == id) else {
            return;
        };
        let mut session = self.sessions.remove(index);
        session.pinned = pinned;
        let at = self.sessions.iter().take_while(|s| s.pinned).count();
        self.sessions.insert(at, session);
        if let Some(backend) = &self.backend {
            let request = backend.store.set_pinned(id, pinned);
            self.save(
                async move { request.await.map_err(|e| (e.to_string(), None)) },
                cx,
            );
        }
        cx.notify();
    }

    /// Tell the host about a rename or a delete of a stored chat. A deleted
    /// chat is archived in the host and loses its pin.
    pub(super) fn sync(&mut self, id: &SharedString, change: Change, cx: &mut Context<Self>) {
        let Some(backend) = &self.backend else {
            return;
        };
        let session = SessionId::new(id.to_string());
        // A delete the host refused is undone; one archived but not unpinned
        // stays done (an archived chat is not listed, pinned or not).
        let restore = matches!(change, Change::Delete).then(|| id.clone());
        let (request, unpin) = match change {
            Change::Rename(title) => (backend.host.rename(&session, title), None),
            Change::Delete => (
                backend.host.archive(&session, true),
                Some(backend.store.set_pinned(id, false)),
            ),
        };
        self.save(
            async move {
                request.await.map_err(|e| (e.message, restore))?;
                if let Some(unpin) = unpin {
                    unpin.await.map_err(|e| (e, None))?;
                }
                Ok(())
            },
            cx,
        );
    }

    /// Whether a change is still being written.
    pub fn is_saving(&self) -> bool {
        self.saving > 0
    }

    /// The last write that failed, if any; cleared by the next success.
    pub fn save_error(&self) -> Option<&SharedString> {
        self.save_error.as_ref()
    }

    /// Run a write and keep count of the ones in flight. A write that fails
    /// says why, and the list loads again, so the sidebar shows what the
    /// host and the store hold rather than the change that did not land; a
    /// chat whose delete failed comes back.
    fn save(
        &mut self,
        write: impl std::future::Future<Output = Result<(), (String, Option<SharedString>)>> + 'static,
        cx: &mut Context<Self>,
    ) {
        self.saving += 1;
        cx.spawn(async move |this, cx| {
            let result = write.await;
            let _ = this.update(cx, |this, cx| {
                this.saving -= 1;
                match result {
                    Ok(()) => this.save_error = None,
                    Err((message, restore)) => {
                        this.save_error = Some(message.into());
                        if let Some(id) = restore {
                            this.deleted.remove(&id);
                        }
                        this.reload(cx);
                    }
                }
                cx.notify();
            });
        })
        .detach();
    }
}

/// A change to a stored chat.
pub(super) enum Change {
    Rename(String),
    Delete,
}

/// The host's list as the version planner reads it.
fn branches(entries: &[SessionEntry]) -> Vec<Branch> {
    entries
        .iter()
        .map(|entry| Branch {
            id: entry.id.clone(),
            forked_from: entry.forked_from.clone(),
            created_at: entry.created_at,
            updated_at: entry.updated_at,
        })
        .collect()
}
