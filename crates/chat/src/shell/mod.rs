//! Chat's shell: the sidebar, the header, the panel between them and the
//! context panel (Figma `ChatView/01`–`05`, layers `chat_sidebar`,
//! `chat_panel` and `context_sidebar`).
//!
//! The rail of app icons to the left belongs to the desktop, not to Chat.

mod context_panel;
mod header;
mod sidebar;

use ui::component::empty::{Empty, EmptyDescription, EmptyHeader};
use ui::component::input::{InputEvent, InputState};
use ui::component::{ActiveTheme as _, h_flex, v_flex};
use ui::prelude::*;
use ui::{
    AnyElement, App, Entity, FocusHandle, Focusable, SharedString, Subscription, Window, div,
};

use crate::composer::{Composer, ComposerEvent};
use crate::copy::{self, DayPart};
use crate::demo::{self, SessionSummary};
use crate::{KEY_CONTEXT, NewChat, ToggleContextPanel, ToggleSidebar, home, metrics};

/// Where the panel is.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Route {
    /// A new chat: the greeting, the composer and the starters.
    Home,
    /// An existing chat, by id.
    Session(SharedString),
}

/// The context panel's tabs.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum ContextTab {
    Context,
    Tools,
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
    context_panel_open: bool,
    context_tab: ContextTab,
    composer: Entity<Composer>,
    rename: Option<Rename>,
    day_part: DayPart,
    _subscriptions: Vec<Subscription>,
}

impl ChatShell {
    /// Chat with the static demo data, on home.
    pub fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
        let composer = cx.new(|cx| Composer::new(window, cx));
        let subscription = cx.subscribe_in(&composer, window, |this, _, event, window, cx| {
            let ComposerEvent::Submit(text) = event;
            this.start_session(text, window, cx);
        });
        Self {
            focus_handle: cx.focus_handle(),
            route: Route::Home,
            sessions: demo::sessions(),
            next_id: 1,
            sidebar_collapsed: false,
            context_panel_open: true,
            context_tab: ContextTab::Context,
            composer,
            rename: None,
            day_part: DayPart::from_hour(local_hour()),
            _subscriptions: vec![subscription],
        }
    }

    /// Where the panel is.
    pub fn route(&self) -> &Route {
        &self.route
    }

    /// The chats in the sidebar, in order.
    pub fn sessions(&self) -> &[SessionSummary] {
        &self.sessions
    }

    /// Whether the sidebar is shown.
    pub fn sidebar_open(&self) -> bool {
        !self.sidebar_collapsed
    }

    /// Whether the context panel is shown.
    pub fn context_panel_open(&self) -> bool {
        self.context_panel_open
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

    /// Shows or hides the sidebar and the context panel.
    pub fn set_panels(&mut self, sidebar: bool, context: bool, cx: &mut Context<Self>) {
        self.sidebar_collapsed = !sidebar;
        self.context_panel_open = context;
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
        self.route = Route::Session(id);
        self.composer.update(cx, |composer, cx| {
            composer.set_placeholder(copy::COMPOSER_THREAD, window, cx)
        });
        cx.notify();
    }

    /// Sending from home starts a chat titled from the text. With static
    /// data nothing is sent anywhere; the conversation view comes with the kit.
    fn start_session(&mut self, text: &str, window: &mut Window, cx: &mut Context<Self>) {
        if self.route != Route::Home {
            return;
        }
        let id: SharedString = format!("new-{}", self.next_id).into();
        self.next_id += 1;
        let title = copy::title_from(text, metrics::TITLE_MAX_CHARS);
        let at = self.sessions.iter().take_while(|s| s.pinned).count();
        self.sessions
            .insert(at, SessionSummary::new(id.clone(), title, false));
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
        {
            session.title = title.into();
        }
        window.focus(&self.focus_handle, cx);
        cx.notify();
    }

    /// Drops the open chat and goes home.
    pub fn delete_current(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        if let Route::Session(id) = &self.route {
            let id = id.clone();
            self.sessions.retain(|session| session.id != id);
            self.new_chat(window, cx);
        }
    }

    fn toggle_sidebar(&mut self, _: &ToggleSidebar, _: &mut Window, cx: &mut Context<Self>) {
        self.sidebar_collapsed = !self.sidebar_collapsed;
        cx.notify();
    }

    fn toggle_context_panel(
        &mut self,
        _: &ToggleContextPanel,
        _: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.context_panel_open = !self.context_panel_open;
        cx.notify();
    }

    fn on_new_chat(&mut self, _: &NewChat, window: &mut Window, cx: &mut Context<Self>) {
        self.new_chat(window, cx);
    }

    fn render_body(&mut self, window: &mut Window, cx: &mut Context<Self>) -> AnyElement {
        match self.route {
            Route::Home => home::render(self.day_part, &self.composer, window, cx),
            Route::Session(_) => self.render_thread(cx),
        }
    }

    /// The thread is the kit's `ConversationView`; until it exists the
    /// panel shows an empty state above the docked composer.
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
        let body = self.render_body(window, cx);
        let theme = cx.theme();
        h_flex()
            .id("chat")
            .key_context(KEY_CONTEXT)
            .track_focus(&self.focus_handle)
            .on_action(cx.listener(Self::on_new_chat))
            .on_action(cx.listener(Self::toggle_sidebar))
            .on_action(cx.listener(Self::toggle_context_panel))
            .size_full()
            .items_start()
            .bg(theme.background)
            .text_color(theme.foreground)
            .when(!self.sidebar_collapsed, |this| {
                this.child(sidebar::render(self, cx))
            })
            .child(
                v_flex()
                    .id("chat_panel")
                    .flex_1()
                    .min_w_0()
                    .h_full()
                    .child(header::render(self, cx))
                    .child(div().flex_1().min_h_0().w_full().child(body)),
            )
            .when(self.context_panel_open, |this| {
                this.child(context_panel::render(self, cx))
            })
    }
}

/// The local hour, for the greeting.
fn local_hour() -> u32 {
    use chrono::Timelike as _;
    chrono::Local::now().hour()
}
