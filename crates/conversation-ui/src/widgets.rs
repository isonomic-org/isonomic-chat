// Copyright 2026 Anud Inc.
// SPDX-License-Identifier: GPL-3.0-or-later

//! Display widgets: components the model chooses to show, by calling a
//! display tool with JSON. The host validates and records the call; the
//! [`WidgetRegistry`] maps the tool's name to a [`WidgetBinding`], which
//! turns the input into a component's props and renders it.
//!
//! A widget never shows broken. Bad data, a fallback the host chose, or a
//! name the registry does not know show the input's `fallback_text` as
//! Markdown; with no usable text, the generic block that unknown content
//! gets. A call the model is retrying shows nothing.

mod chart;
mod metrics;
mod steps;
mod table;

use conversation_model::schema::{CallId, DisplayOutcome, Value};
use conversation_model::{ToolUse, Turn};
use serde::de::DeserializeOwned;
use std::any::Any;
use std::collections::HashMap;
use std::rc::Rc;
use ui::component::text::TextView;
use ui::component::{ActiveTheme as _, v_flex};
use ui::prelude::*;
use ui::{AnyElement, App, ElementId, SharedString, TestSupportExt as _, Window, div};

use crate::ConversationCopy;

/// A handler for a sort the person asks of a widget: the column, counted
/// from zero.
pub type SortHandler = Rc<dyn Fn(usize, &mut Window, &mut App)>;

/// A component's props, as a binding made them.
pub struct WidgetProps(Box<dyn Any>);

impl WidgetProps {
    /// Wrap a binding's own props.
    pub fn new<T: Any>(props: T) -> Self {
        Self(Box::new(props))
    }

    /// The props as the binding's own type.
    pub fn get<T: Any>(&self) -> Option<&T> {
        self.0.downcast_ref()
    }
}

/// Why a binding could not make props from an input.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct BindError(pub String);

/// What a binding is given to render besides its props.
pub struct WidgetContext<'a> {
    /// A stable id for this call's component.
    pub id: ElementId,
    /// The column the person sorted by, and whether ascending.
    pub sort: Option<(usize, bool)>,
    /// Sort by another column.
    pub on_sort: Option<SortHandler>,
    /// Whether the widget was drawn before: replayed from history, it
    /// appears without its entrance motion.
    pub replayed: bool,
    /// The app's words.
    pub copy: &'a ConversationCopy,
}

/// One display widget: its tool's name, and how its input becomes a
/// component.
pub trait WidgetBinding {
    /// The display tool's name, such as `show_table`.
    fn name(&self) -> &'static str;
    /// Validated JSON in, the component's props out. `Err` means bad data.
    fn props(&self, input: &Value) -> Result<WidgetProps, BindError>;
    /// The component for its props.
    fn render(&self, props: &WidgetProps, cx: &WidgetContext, app: &App) -> AnyElement;
}

/// The display widgets a view knows, by tool name.
#[derive(Clone, Default)]
pub struct WidgetRegistry {
    bindings: HashMap<&'static str, Rc<dyn WidgetBinding>>,
}

impl WidgetRegistry {
    /// No widgets: every call shows its fallback text.
    pub fn empty() -> Self {
        Self::default()
    }

    /// The starter set: `show_table`, `show_steps`, `show_metrics` and
    /// `show_chart`.
    pub fn standard() -> Self {
        Self::empty()
            .with(table::Table)
            .with(steps::Steps)
            .with(metrics::Metrics)
            .with(chart::Chart)
    }

    /// Add a binding, or replace the one with its name.
    pub fn with(mut self, binding: impl WidgetBinding + 'static) -> Self {
        self.bindings.insert(binding.name(), Rc::new(binding));
        self
    }

    /// The names it knows, sorted.
    pub fn names(&self) -> Vec<&'static str> {
        let mut names: Vec<_> = self.bindings.keys().copied().collect();
        names.sort_unstable();
        names
    }

    /// What a display call shows, decided without drawing.
    pub fn resolve(&self, call: &ToolUse) -> Shown {
        if call.outcome.is_none() || call.display == Some(DisplayOutcome::Retry) {
            return Shown::Nothing;
        }
        let fallback = || match fallback_text(&call.input) {
            Some(text) => Shown::Fallback(text),
            None => Shown::Generic(call.tool.clone()),
        };
        // Anything but a clear "shown" falls back: the host's fallback, a
        // value from a newer revision, or no verdict at all.
        if call.display != Some(DisplayOutcome::Shown) {
            return fallback();
        }
        match self.bindings.get(call.tool.as_str()) {
            Some(binding) => match binding.props(&call.input) {
                Ok(_) => Shown::Widget(call.tool.clone()),
                Err(_) => fallback(),
            },
            None => fallback(),
        }
    }

    /// Draw a display call: its component, its fallback text or the
    /// generic block. A call still running, or being retried, draws
    /// nothing.
    pub fn render(&self, call: &ToolUse, cx: &WidgetContext, app: &App) -> Option<AnyElement> {
        let element = match self.resolve(call) {
            Shown::Nothing => return None,
            Shown::Widget(name) => {
                let binding = self.bindings.get(name.as_str())?;
                let props = binding.props(&call.input).ok()?;
                binding.render(&props, cx, app)
            }
            Shown::Fallback(text) => TextView::markdown(cx.id.clone(), text)
                .selectable(true)
                .into_any_element(),
            Shown::Generic(name) => div()
                .text_sm()
                .text_color(app.theme().muted_foreground)
                .child(format!("{} ({name})", cx.copy.unknown))
                .into_any_element(),
        };
        Some(
            v_flex()
                .id(cx.id.clone())
                .w_full()
                .min_w_0()
                .child(element)
                .test_support()
                .into_any_element(),
        )
    }
}

/// What a display call shows.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Shown {
    /// Nothing: still running, or being retried.
    Nothing,
    /// The named widget's component.
    Widget(String),
    /// The input's fallback text, as Markdown.
    Fallback(String),
    /// The generic block, naming the widget.
    Generic(String),
}

/// The input's `fallback_text`, when it is usable text.
fn fallback_text(input: &Value) -> Option<String> {
    input
        .get("fallback_text")
        .and_then(Value::as_str)
        .map(str::trim)
        .filter(|text| !text.is_empty())
        .map(str::to_owned)
}

/// Read an input into a binding's own type.
pub(crate) fn parse<T: DeserializeOwned>(input: &Value) -> Result<T, BindError> {
    serde_json::to_value(input)
        .and_then(serde_json::from_value)
        .map_err(|error| BindError(error.to_string()))
}

/// A widget's title, above it, when it has one.
pub(crate) fn titled(title: Option<&str>, body: AnyElement, app: &App) -> AnyElement {
    let Some(title) = title.map(str::trim).filter(|t| !t.is_empty()) else {
        return body;
    };
    v_flex()
        .w_full()
        .min_w_0()
        .gap_2()
        .child(
            div()
                .text_sm()
                .font_weight(ui::FontWeight::MEDIUM)
                .text_color(app.theme().foreground)
                .child(SharedString::from(title.to_owned())),
        )
        .child(body)
        .into_any_element()
}

/// The element id of a display call's component.
pub fn widget_id(call: &CallId) -> ElementId {
    ElementId::Name(format!("widget-{}", call.as_str()).into())
}

/// A handler for a sort the person asks of one call's widget.
pub(crate) type CallSortHandler = Rc<dyn Fn(CallId, usize, &mut Window, &mut App)>;

/// What a reply needs to draw its widgets: the registry, each widget's
/// sort, and where a new sort goes.
#[derive(Clone)]
pub(crate) struct Shelf {
    pub(crate) registry: Rc<WidgetRegistry>,
    pub(crate) sorts: Rc<HashMap<CallId, (usize, bool)>>,
    pub(crate) on_sort: Option<CallSortHandler>,
}

impl Shelf {
    /// The widgets of the turn at `index`, in call order. Only the newest
    /// turn's widgets make their entrance.
    pub(crate) fn render(
        &self,
        turn: &Turn,
        last: bool,
        copy: &ConversationCopy,
        app: &App,
    ) -> Vec<AnyElement> {
        turn.widgets()
            .filter_map(|call| {
                let on_sort = self.on_sort.clone().map(|on_sort| {
                    let id = call.call.clone();
                    Rc::new(move |column, window: &mut Window, app: &mut App| {
                        on_sort(id.clone(), column, window, app)
                    }) as SortHandler
                });
                let cx = WidgetContext {
                    id: widget_id(&call.call),
                    sort: self.sorts.get(&call.call).copied(),
                    on_sort,
                    replayed: !last,
                    copy,
                };
                self.registry.render(call, &cx, app)
            })
            .collect()
    }
}

/// The sort after the person clicks `column`: the same column flips, a new
/// one sorts ascending.
pub(crate) fn next_sort(current: Option<(usize, bool)>, column: usize) -> (usize, bool) {
    match current {
        Some((at, ascending)) if at == column => (column, !ascending),
        _ => (column, true),
    }
}
