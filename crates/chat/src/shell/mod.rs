//! Chat's shell (design 0.2 app shell): the app sidebar, which collapses to
//! a rail, and the main panel with its header and the home or thread view.
//! The search palette opens over both.

mod header;
mod layout;
mod search;
mod sessions;
mod sidebar;

use ui::component::empty::{Empty, EmptyDescription, EmptyHeader};
use ui::component::input::{InputEvent, InputState};
use ui::component::{ActiveTheme as _, h_flex, v_flex};
use ui::prelude::*;
use ui::{
    AnyElement, App, Entity, FocusHandle, Focusable, SharedString, Subscription, Window, div,
};

use conversation_ui::ConversationView;
use std::collections::{HashMap, HashSet};

use crate::composer::{Composer, ComposerEvent};
use crate::copy::{self, DayPart};
use crate::demo::{self, SessionSummary};
use crate::{
    Backend, KEY_CONTEXT, NewChat, NextChat, PreviousChat, SearchChats, ToggleSidebar, home,
    metrics,
};

/// Where the panel is.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Route {
    /// A new chat: the greeting, the composer and the starters.
    Home,
    /// An existing chat, by id.
    Session(SharedString),
}

/// An inline rename of the open chat's title, in the header.
pub(crate) struct Rename {
    pub(crate) id: SharedString,
    pub(crate) field: Entity<InputState>,
    _subscription: Subscription,
}

/// Chat's root view.
pub struct ChatShell {
    focus_handle: FocusHandle,
    route: Route,
    sessions: Vec<SessionSummary>,
    next_id: usize,
    sidebar_collapsed: bool,
    search: Option<search::Search>,
    composer: Entity<Composer>,
    backend: Option<Backend>,
    threads: HashMap<SharedString, Entity<ConversationView>>,
    load_error: Option<SharedString>,
    fit: Option<layout::Fit>,
    loaded: bool,
    /// Whether a load of the list is on its way.
    reloading: bool,
    /// How many loads of the list have finished.
    loads: u64,
    /// Chats deleted here: a list answer never brings one back.
    deleted: HashSet<SharedString>,
    saving: usize,
    model: Option<String>,
    catalog: Option<conversation_model::ModelCatalog>,
    install_asked: Option<SharedString>,
    models_failed: bool,
    _models: Option<ui::Task<()>>,
    /// Watches the models on their way, while any is.
    _watch: Option<ui::Task<()>>,
    /// Polls since the window was asked for a model it has not started.
    asked_polls: u32,
    save_error: Option<SharedString>,
    _load: Option<ui::Task<()>>,
    rename: Option<Rename>,
    day_part: DayPart,
    _subscriptions: Vec<Subscription>,
}

impl ChatShell {
    /// Chat on home. With a backend, the sidebar lists the host's chats
    /// and every chat runs on it; without one the sidebar shows the static
    /// example chats and a new chat opens empty.
    pub fn new(backend: Option<Backend>, window: &mut Window, cx: &mut Context<Self>) -> Self {
        let composer = cx.new(|cx| Composer::new(window, cx));
        let subscription =
            cx.subscribe_in(
                &composer,
                window,
                |this, _, event, window, cx| match event {
                    ComposerEvent::Submit(text) => this.start_session(text, window, cx),
                },
            );
        let sessions = match backend {
            Some(_) => Vec::new(),
            None => demo::sessions(),
        };
        let mut shell = Self {
            focus_handle: cx.focus_handle(),
            route: Route::Home,
            sessions,
            next_id: 1,
            sidebar_collapsed: false,
            search: None,
            composer,
            backend: backend.clone(),
            threads: HashMap::new(),
            load_error: None,
            fit: None,
            loaded: backend.is_none(),
            reloading: false,
            loads: 0,
            deleted: HashSet::new(),
            saving: 0,
            model: None,
            catalog: None,
            install_asked: None,
            models_failed: false,
            _models: None,
            _watch: None,
            asked_polls: 0,
            save_error: None,
            _load: None,
            rename: None,
            day_part: DayPart::from_hour(local_hour()),
            _subscriptions: vec![subscription],
        };
        if let Some(backend) = backend {
            shell.load_models(&backend, cx);
            shell.load(backend, cx);
        }
        shell
    }

    /// Where the panel is.
    pub fn route(&self) -> &Route {
        &self.route
    }

    /// The chats in the sidebar, in order.
    pub fn sessions(&self) -> &[SessionSummary] {
        &self.sessions
    }

    /// Whether the sidebar is expanded (otherwise it is the rail).
    pub fn sidebar_open(&self) -> bool {
        !self.sidebar_collapsed
    }

    /// Fixes the greeting's part of the day (for screenshots and tests).
    pub fn set_day_part(&mut self, part: DayPart, cx: &mut Context<Self>) {
        self.day_part = part;
        cx.notify();
    }

    /// Puts text in the composer, as if typed or picked from a starter.
    pub fn set_draft(&mut self, text: &str, window: &mut Window, cx: &mut Context<Self>) {
        self.composer
            .update(cx, |composer, cx| composer.set_text(text, window, cx));
    }

    /// The composer's text.
    pub fn draft(&self, cx: &App) -> SharedString {
        self.composer.read(cx).text(cx)
    }

    /// Expands the sidebar, or collapses it to the rail.
    pub fn set_sidebar_open(&mut self, open: bool, cx: &mut Context<Self>) {
        self.sidebar_collapsed = !open;
        cx.notify();
    }

    /// The open chat, if any.
    pub(crate) fn current(&self) -> Option<&SessionSummary> {
        match &self.route {
            Route::Home => None,
            Route::Session(id) => self.sessions.iter().find(|session| &session.id == id),
        }
    }

    /// Goes home and focuses the composer.
    pub fn new_chat(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        self.rename = None;
        self.route = Route::Home;
        self.composer.update(cx, |composer, cx| {
            composer.set_placeholder(copy::COMPOSER_HOME, window, cx);
            composer.focus(window, cx);
        });
        cx.notify();
    }

    /// Opens a chat from the sidebar.
    pub fn open_session(&mut self, id: SharedString, window: &mut Window, cx: &mut Context<Self>) {
        if !self.sessions.iter().any(|session| session.id == id) {
            return;
        }
        self.rename = None;
        self.ensure_thread(&id, window, cx);
        self.route = Route::Session(id);
        self.composer.update(cx, |composer, cx| {
            composer.set_placeholder(copy::COMPOSER_THREAD, window, cx)
        });
        cx.notify();
    }

    /// The conversation behind a chat, if it has one.
    pub fn thread(&self, id: &SharedString) -> Option<&Entity<ConversationView>> {
        self.threads.get(id)
    }

    /// Sending from home starts a chat titled from the text, opens its
    /// conversation and sends the text to the host.
    fn start_session(&mut self, text: &str, window: &mut Window, cx: &mut Context<Self>) {
        if self.route != Route::Home {
            return;
        }
        let id: SharedString = format!("new-{}", self.next_id).into();
        self.next_id += 1;
        let title = copy::title_from(text, metrics::TITLE_MAX_CHARS);
        let at = self.sessions.iter().take_while(|s| s.pinned).count();
        self.sessions.insert(
            at,
            SessionSummary::new(id.clone(), title, false).active_at(crate::clock::now_ms()),
        );
        let Some(backend) = self.backend.clone() else {
            self.open_session(id, window, cx);
            return;
        };
        let thread = self.new_thread(id.clone(), backend.host, window, cx);
        let text = text.to_owned();
        thread.update(cx, |thread, cx| thread.send(text, cx));
        thread.update(cx, |thread, cx| thread.focus_composer(window, cx));
        self.open_session(id, window, cx);
    }

    /// Starts renaming the open chat in the header.
    pub fn begin_rename(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let Some(session) = self.current().cloned() else {
            return;
        };
        let field = cx.new(|cx| InputState::new(window, cx).default_value(session.title.clone()));
        field.update(cx, |field, cx| field.focus(window, cx));
        let subscription =
            cx.subscribe_in(&field, window, |this, _, event, window, cx| match event {
                InputEvent::PressEnter { .. } => this.commit_rename(window, cx),
                InputEvent::Blur => this.commit_rename(window, cx),
                _ => {}
            });
        self.rename = Some(Rename {
            id: session.id,
            field,
            _subscription: subscription,
        });
        cx.notify();
    }

    /// Close the rename field and keep the title as it was.
    pub fn cancel_rename(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        if self.rename.take().is_some() {
            window.focus(&self.focus_handle, cx);
            cx.notify();
        }
    }

    /// Whether the header shows the rename field.
    pub fn is_renaming(&self) -> bool {
        self.rename.is_some()
    }

    /// Keeps a non-empty new title; an empty one leaves the title as it was.
    fn commit_rename(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let Some(rename) = self.rename.take() else {
            return;
        };
        let title = rename.field.read(cx).value().trim().to_string();
        if let Some(session) = self.sessions.iter_mut().find(|s| s.id == rename.id)
            && !title.is_empty()
            && session.title.as_ref() != title
        {
            session.title = title.clone().into();
            self.sync(&rename.id, sessions::Change::Rename(title), cx);
        }
        window.focus(&self.focus_handle, cx);
        cx.notify();
    }

    /// Drops the open chat and goes home.
    pub fn delete_current(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        if let Route::Session(id) = self.route.clone() {
            self.delete_session(&id, window, cx);
        }
    }

    /// Drops a chat; if it was open, goes home.
    pub fn delete_session(
        &mut self,
        id: &SharedString,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.sessions.retain(|session| &session.id != id);
        self.deleted.insert(id.clone());
        self.threads.remove(id);
        self.sync(id, sessions::Change::Delete, cx);
        if self.route == Route::Session(id.clone()) {
            self.new_chat(window, cx);
        }
        cx.notify();
    }

    /// Whether a chat's reply is being written.
    pub fn is_running(&self, id: &SharedString, cx: &App) -> bool {
        self.threads
            .get(id)
            .is_some_and(|thread| thread.read(cx).input_state().is_busy())
    }

    fn toggle_sidebar(&mut self, _: &ToggleSidebar, _: &mut Window, cx: &mut Context<Self>) {
        self.sidebar_collapsed = !self.sidebar_collapsed;
        cx.notify();
    }

    fn on_search(&mut self, _: &SearchChats, window: &mut Window, cx: &mut Context<Self>) {
        self.open_search(window, cx);
    }

    fn on_previous_chat(&mut self, _: &PreviousChat, window: &mut Window, cx: &mut Context<Self>) {
        self.step_chat(-1, window, cx);
    }

    fn on_next_chat(&mut self, _: &NextChat, window: &mut Window, cx: &mut Context<Self>) {
        self.step_chat(1, window, cx);
    }

    /// Open the chat `step` rows from the open one, in sidebar order (pinned
    /// first). From home, down opens the first chat; up stays.
    pub fn step_chat(&mut self, step: isize, window: &mut Window, cx: &mut Context<Self>) {
        let ids: Vec<SharedString> = self.sidebar_order();
        let at = match &self.route {
            Route::Session(id) => ids.iter().position(|s| s == id),
            Route::Home => None,
        };
        let next = match at {
            Some(at) => at.checked_add_signed(step).filter(|n| *n < ids.len()),
            None if step > 0 => (!ids.is_empty()).then_some(0),
            None => None,
        };
        if let Some(next) = next {
            self.open_session(ids[next].clone(), window, cx);
        }
    }

    /// The chats in the order the sidebar shows them: pinned, then recents.
    fn sidebar_order(&self) -> Vec<SharedString> {
        let pinned = self.sessions.iter().filter(|s| s.pinned);
        let rest = self.sessions.iter().filter(|s| !s.pinned);
        pinned.chain(rest).map(|s| s.id.clone()).collect()
    }

    fn on_new_chat(&mut self, _: &NewChat, window: &mut Window, cx: &mut Context<Self>) {
        self.new_chat(window, cx);
    }

    fn render_body(&mut self, window: &mut Window, cx: &mut Context<Self>) -> AnyElement {
        match self.route {
            Route::Home => match self.missing_model().cloned() {
                Some(model) => {
                    let asked = self
                        .install_asked()
                        .is_some_and(|asked| asked.as_ref() == model.id);
                    home::render_install(&model, asked, cx)
                }
                None => home::render(self, self.day_part, &self.composer.clone(), window, cx),
            },
            Route::Session(ref id) => match self.threads.get(id) {
                Some(thread) => self.render_conversation(thread.clone(), cx),
                None => self.render_thread(cx),
            },
        }
    }

    /// A chat with a conversation: the kit's view, then the disclaimer.
    fn render_conversation(
        &self,
        thread: Entity<ConversationView>,
        cx: &mut Context<Self>,
    ) -> AnyElement {
        let theme = cx.theme();
        v_flex()
            .id("thread")
            .size_full()
            .items_center()
            .pb_3()
            .gap_2()
            .child(div().flex_1().min_h_0().w_full().child(thread))
            .child(
                div()
                    .text_xs()
                    .text_color(theme.muted_foreground)
                    .child(copy::DISCLAIMER),
            )
            .into_any_element()
    }

    /// An example chat with no conversation behind it: an empty state above
    /// the docked composer.
    fn render_thread(&self, cx: &mut Context<Self>) -> AnyElement {
        let theme = cx.theme();
        v_flex()
            .id("thread")
            .size_full()
            .child(
                div()
                    .flex_1()
                    .flex()
                    .items_center()
                    .justify_center()
                    .px_6()
                    .child(
                        Empty::new().header(
                            EmptyHeader::new()
                                .description(EmptyDescription::new().child(copy::THREAD_PENDING)),
                        ),
                    ),
            )
            .child(
                v_flex()
                    .id("composer_dock")
                    .w_full()
                    .items_center()
                    .gap_2()
                    .px_6()
                    .pb_3()
                    .child(self.composer.clone())
                    .child(
                        div()
                            .text_xs()
                            .text_color(theme.muted_foreground)
                            .child(copy::DISCLAIMER),
                    ),
            )
            .into_any_element()
    }
}

impl Focusable for ChatShell {
    fn focus_handle(&self, _: &App) -> FocusHandle {
        self.focus_handle.clone()
    }
}

impl Render for ChatShell {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        self.fit_to(window);
        let body = self.render_body(window, cx);
        let theme = cx.theme();
        h_flex()
            .id("chat")
            .key_context(KEY_CONTEXT)
            .track_focus(&self.focus_handle)
            .on_action(cx.listener(Self::on_new_chat))
            .on_action(cx.listener(Self::toggle_sidebar))
            .on_action(cx.listener(Self::on_search))
            .on_action(cx.listener(Self::on_previous_chat))
            .on_action(cx.listener(Self::on_next_chat))
            .size_full()
            .items_start()
            .bg(theme.background)
            .text_color(theme.foreground)
            .relative()
            .child(sidebar::render(self, window, cx))
            .child(
                v_flex()
                    .id("chat_panel")
                    .flex_1()
                    .min_w_0()
                    .h_full()
                    .child(header::render(self, cx))
                    .child(div().flex_1().min_h_0().w_full().child(body)),
            )
            .children(self.render_search(cx))
    }
}

/// The local hour, for the greeting.
fn local_hour() -> u32 {
    use chrono::Timelike as _;
    chrono::Local::now().hour()
}
