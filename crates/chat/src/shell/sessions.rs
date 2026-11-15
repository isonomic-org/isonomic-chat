//! The shell's chats when it has a backend: the list comes from the host,
//! pins from Chat's store, and opening, renaming and deleting go through
//! them. Without a backend the shell shows the static example chats.

use conversation_host::{ConversationHost, SessionQuery};
use conversation_model::schema::SessionId;
use conversation_ui::{ConversationEvent, ConversationView};
use std::rc::Rc;
use ui::prelude::*;
use ui::{Entity, SharedString, Window};

use super::ChatShell;
use crate::Backend;
use crate::copy;
use crate::demo::SessionSummary;

/// How many chats the sidebar loads.
const LIST_LIMIT: u32 = 200;

impl ChatShell {
    /// Load the chats and the pins, newest activity first, pinned on top.
    pub(super) fn load(&mut self, backend: Backend, cx: &mut Context<Self>) {
        let task = cx.spawn(async move |this, cx| {
            let query = SessionQuery {
                limit: LIST_LIMIT,
                ..Default::default()
            };
            let listed = backend.host.list(query).await;
            let pins = backend.store.pins().await.unwrap_or_default();
            let _ = this.update(cx, |this, cx| {
                match listed {
                    Ok(entries) => {
                        let mut sessions: Vec<SessionSummary> = entries
                            .into_iter()
                            .map(|entry| {
                                let id = SharedString::from(entry.id.0.clone());
                                let pinned = pins.contains(&entry.id.0);
                                let title = entry.title.unwrap_or_else(|| copy::NEW_CHAT.into());
                                SessionSummary::new(id, title, pinned)
                            })
                            .collect();
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
                cx.notify();
            });
        });
        self._load = Some(task);
    }

    /// Whether the first load of the list has finished.
    pub fn is_loaded(&self) -> bool {
        self.loaded
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
        let thread =
            cx.new(|cx| ConversationView::open(host, session, copy::conversation(), window, cx));
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
        let thread = cx
            .new(|cx| ConversationView::new(host, crate::PRESET, copy::conversation(), window, cx));
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
            self.save(async move { request.await.map_err(|e| e.to_string()) }, cx);
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
        let (request, unpin) = match change {
            Change::Rename(title) => (backend.host.rename(&session, title), None),
            Change::Delete => (
                backend.host.archive(&session, true),
                Some(backend.store.set_pinned(id, false)),
            ),
        };
        self.save(
            async move {
                request.await.map_err(|e| e.message)?;
                if let Some(unpin) = unpin {
                    unpin.await?;
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

    /// Run a write and keep count of the ones in flight.
    fn save(
        &mut self,
        write: impl std::future::Future<Output = Result<(), String>> + 'static,
        cx: &mut Context<Self>,
    ) {
        self.saving += 1;
        cx.spawn(async move |this, cx| {
            let result = write.await;
            let _ = this.update(cx, |this, cx| {
                this.saving -= 1;
                this.save_error = result.err().map(SharedString::from);
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
