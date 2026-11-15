//! A plain stand-in for the composer.
//!
//! Isonomic UI's Prompt composer (isonomic-ui #206) and the kit's
//! `PromptComposer` binding over composer-model replace this module. It holds
//! no draft logic of its own beyond what the stand-in needs to be usable:
//! a field, a disabled attach button, the model label and Send.

use ui::component::button::{Button, ButtonVariants as _};
use ui::component::input::{InputEvent, Textarea, TextareaState};
use ui::component::{ActiveTheme as _, Disableable as _, Sizable as _, h_flex, v_flex};
use ui::prelude::*;
use ui::{App, Entity, EventEmitter, SharedString, Subscription, Window, div};

use crate::{copy, demo, icons, metrics};

/// What the composer reports.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ComposerEvent {
    /// The person sent this text (trimmed, never empty).
    Submit(String),
}

/// The stand-in composer.
pub struct Composer {
    field: Entity<TextareaState>,
    _subscription: Subscription,
}

impl EventEmitter<ComposerEvent> for Composer {}

impl Composer {
    /// A composer with the home placeholder.
    pub fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
        let (min_rows, max_rows) = metrics::COMPOSER_ROWS;
        let field = cx.new(|cx| {
            TextareaState::new(window, cx)
                .auto_grow(min_rows, max_rows)
                .submit_on_enter(true)
                .placeholder(copy::COMPOSER_HOME)
        });
        let subscription =
            cx.subscribe_in(&field, window, |this, _, event, window, cx| match event {
                InputEvent::PressEnter { shift: false, .. } => this.submit(window, cx),
                InputEvent::Change => cx.notify(),
                _ => {}
            });
        Self {
            field,
            _subscription: subscription,
        }
    }

    /// The current text.
    pub fn text(&self, cx: &App) -> SharedString {
        self.field.read(cx).value()
    }

    /// Whether Send is enabled: the text has something besides whitespace.
    pub fn can_send(&self, cx: &App) -> bool {
        !self.text(cx).trim().is_empty()
    }

    /// Replaces the text and moves focus to the field.
    pub fn set_text(&mut self, text: &str, window: &mut Window, cx: &mut Context<Self>) {
        self.field.update(cx, |field, cx| {
            field.set_value(text.to_string(), window, cx)
        });
        self.focus(window, cx);
        cx.notify();
    }

    /// Changes the placeholder (home and thread differ).
    pub fn set_placeholder(
        &mut self,
        text: &'static str,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.field
            .update(cx, |field, cx| field.set_placeholder(text, window, cx));
    }

    /// Moves focus to the field.
    pub fn focus(&self, window: &mut Window, cx: &mut Context<Self>) {
        self.field.update(cx, |field, cx| field.focus(window, cx));
    }

    fn submit(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let text = self.text(cx).trim().to_string();
        if text.is_empty() {
            return;
        }
        self.field
            .update(cx, |field, cx| field.set_value(String::new(), window, cx));
        cx.emit(ComposerEvent::Submit(text));
        cx.notify();
    }
}

impl Render for Composer {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = cx.theme();
        let can_send = self.can_send(cx);
        v_flex()
            .id("composer")
            .w_full()
            .max_w(metrics::content_max_width())
            .gap_2()
            .pb_3()
            .rounded(theme.radius_lg)
            .border_1()
            .border_color(theme.input)
            .bg(theme.background)
            .shadow_sm()
            .child(
                div()
                    .px_1()
                    .pt_1()
                    .child(Textarea::new(&self.field).appearance(false)),
            )
            .child(
                h_flex()
                    .px_3()
                    .gap_2()
                    .child(
                        Button::new("composer-attach")
                            .outline()
                            .small()
                            .icon(icons::attach())
                            .label(copy::ADD_FILES)
                            .disabled(true),
                    )
                    .child(div().flex_1())
                    .child(
                        Button::new("composer-model")
                            .outline()
                            .small()
                            .label(demo::MODEL)
                            .dropdown_caret(true)
                            .disabled(true),
                    )
                    .child(
                        Button::new("composer-send")
                            .primary()
                            .small()
                            .label(copy::SEND)
                            .disabled(!can_send)
                            .when(can_send, |button| {
                                button.on_click(
                                    cx.listener(|this, _, window, cx| this.submit(window, cx)),
                                )
                            }),
                    ),
            )
    }
}
