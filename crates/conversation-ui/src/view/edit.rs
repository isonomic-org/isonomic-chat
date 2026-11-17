// Copyright 2026 Anud Inc.
// SPDX-License-Identifier: GPL-3.0-or-later

//! Editing a sent prompt: a field in place of its bubble, with Cancel and
//! Save. Saving sends the new prompt as a new version of that turn (a fork
//! with reason edit); the pager under the reply switches back.

use super::ConversationView;
use ui::component::input::{InputEvent, TextareaState};
use ui::prelude::*;
use ui::{Entity, Subscription, Window};

/// A prompt being edited.
pub(super) struct Editing {
    pub(super) index: usize,
    pub(super) field: Entity<TextareaState>,
    _events: Subscription,
}

impl ConversationView {
    /// Open the edit field on the prompt of the turn at `index`, with its
    /// text, focused. Only a turn that could be tried again can be edited.
    pub fn begin_edit(&mut self, index: usize, window: &mut Window, cx: &mut Context<Self>) {
        if !self.can_regenerate(index) {
            return;
        }
        let Some(prompt) = self
            .thread
            .borrow()
            .turns()
            .get(index)
            .map(|turn| turn.prompt.clone())
        else {
            return;
        };
        let field = cx.new(|cx| {
            let mut field = TextareaState::new(window, cx).auto_grow(1, 8);
            field.set_value(prompt, window, cx);
            field
        });
        field.update(cx, |field, cx| field.focus(window, cx));
        // Mod-Enter saves; a plain Enter is a new line.
        let events = cx.subscribe_in(&field, window, |this, _, event, window, cx| {
            if let InputEvent::PressEnter {
                secondary: true, ..
            } = event
            {
                this.save_edit(window, cx);
            }
        });
        self.editing = Some(Editing {
            index,
            field,
            _events: events,
        });
        self.rows_changed(cx);
    }

    /// The turn whose prompt is being edited.
    pub fn editing(&self) -> Option<usize> {
        self.editing.as_ref().map(|editing| editing.index)
    }

    /// The open edit field, for tests and keyboard hosts.
    pub fn edit_field(&self) -> Option<Entity<TextareaState>> {
        self.editing.as_ref().map(|editing| editing.field.clone())
    }

    /// Close the edit field and keep the prompt as it was.
    pub fn cancel_edit(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        if self.editing.take().is_some() {
            self.rows_changed(cx);
            self.focus_composer(window, cx);
        }
    }

    /// Send the edited prompt as a new version, and close the field.
    pub fn save_edit(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let Some(editing) = self.editing.take() else {
            return;
        };
        let text = editing.field.read(cx).value().to_string();
        self.rows_changed(cx);
        self.edit(editing.index, text, cx);
        self.focus_composer(window, cx);
    }
}
