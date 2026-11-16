//! Home's composer (design 0.2 `Composer`): Isonomic UI's prompt composer,
//! the same one the thread uses. The model is picked in the footer under
//! it, not in the composer.

use ui::extra::prompt_composer::{PromptComposer, PromptComposerEvent};
use ui::prelude::*;
use ui::{App, Entity, EventEmitter, SharedString, Subscription, Window, div};

use crate::{copy, metrics};

/// What the composer reports.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ComposerEvent {
    /// The person sent this text (trimmed, never empty).
    Submit(String),
}

/// Home's composer.
pub struct Composer {
    field: Entity<PromptComposer>,
    _subscription: Subscription,
}

impl EventEmitter<ComposerEvent> for Composer {}

impl Composer {
    /// A composer with the home placeholder.
    pub fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
        let field = cx.new(|cx| {
            let mut field = PromptComposer::new(copy::COMPOSER_HOME, window, cx);
            field.set_dictation(false, cx);
            // Version 1 has no attachments.
            field.set_attach(false, cx);
            // Design 0.2's size: 96 high, round 32 send and stop.
            field.set_comfortable(true, cx);
            // Design 0.2: the composer never turns blue; it holds the
            // keyboard whenever its screen is open.
            field.set_quiet_focus(true, cx);
            field
        });
        let subscription = cx.subscribe(&field, |_, _, event, cx| match event {
            PromptComposerEvent::Submit { text, .. } if !text.trim().is_empty() => {
                cx.emit(ComposerEvent::Submit(text.trim().to_owned()));
            }
            PromptComposerEvent::Changed(_) => cx.notify(),
            _ => {}
        });
        Self {
            field,
            _subscription: subscription,
        }
    }

    /// The current text.
    pub fn text(&self, cx: &App) -> SharedString {
        self.field.read(cx).text(cx)
    }

    /// Replaces the text and moves focus to the field.
    pub fn set_text(&mut self, text: &str, window: &mut Window, cx: &mut Context<Self>) {
        let text = text.to_owned();
        self.field
            .update(cx, |field, cx| field.set_text(text, window, cx));
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
        let field = self.field.clone();
        field.update(cx, |field, cx| field.focus(window, cx));
    }
}

impl Render for Composer {
    fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
        div()
            .id("composer")
            .w_full()
            .max_w(metrics::content_max_width())
            .child(self.field.clone())
    }
}
