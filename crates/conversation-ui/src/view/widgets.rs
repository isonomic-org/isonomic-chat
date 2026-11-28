// Copyright 2026 Anud Inc.
// SPDX-License-Identifier: GPL-3.0-or-later

//! The display widgets a view draws, and how the person sorted them.

use super::ConversationView;
use crate::WidgetRegistry;
use crate::widgets::{Shelf, next_sort};
use conversation_model::schema::CallId;
use std::rc::Rc;
use ui::prelude::*;
use ui::{App, WeakEntity, Window};

impl ConversationView {
    /// Draw display calls with these widgets instead of the starter set.
    /// An empty registry shows every call's fallback text.
    pub fn set_widgets(&mut self, widgets: WidgetRegistry, cx: &mut Context<Self>) {
        self.widgets = Rc::new(widgets);
        self.rows_changed(cx);
    }

    /// Sort a display call's table by `column`: the same column again
    /// flips the order. It is the view's own state; nothing is sent.
    pub fn sort_widget(&mut self, call: CallId, column: usize, cx: &mut Context<Self>) {
        let next = next_sort(self.sorts.get(&call).copied(), column);
        Rc::make_mut(&mut self.sorts).insert(call, next);
        self.rows_changed(cx);
    }

    /// The sort the person chose for a display call's widget.
    pub fn widget_sort(&self, call: &CallId) -> Option<(usize, bool)> {
        self.sorts.get(call).copied()
    }

    pub(super) fn shelf(&self, view: &WeakEntity<Self>) -> Shelf {
        let view = view.clone();
        Shelf {
            registry: self.widgets.clone(),
            sorts: self.sorts.clone(),
            on_sort: Some(Rc::new(
                move |call, column, _: &mut Window, cx: &mut App| {
                    let _ = view.update(cx, |view, cx| view.sort_widget(call, column, cx));
                },
            )),
        }
    }
}
