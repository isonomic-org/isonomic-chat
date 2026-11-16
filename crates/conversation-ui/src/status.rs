//! The status bar above the composer: the one place that says a reply is
//! being written, how the last one ended, or that the conversation is
//! reconnecting, closed or could not load, with what can be done about it.
//! Drawn by Isonomic UI's run status bar, as design 0.2 draws it: Stop while
//! running; Dismiss, Retry and Resume when stopped; Dismiss and Retry on an
//! error. Steer is outside version 1.

use ui::ElementId;
use ui::extra::run_status_bar::{RunStatus, RunStatusBar, RunStatusButton};
use ui::prelude::*;
use ui::{AnyElement, App, SharedString, Window};

use crate::metrics;

/// Element id of the bar. Its buttons are its children `"button-0"`,
/// `"button-1"` and so on, left to right; see
/// [`ConversationView::status_button`](crate::ConversationView::status_button).
pub const STATUS_ID: &str = "conversation-status";

/// One button the bar can show.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum StatusButton {
    /// Hide the bar until something changes.
    Dismiss,
    /// Write the stopped reply again, as a new version.
    Retry,
    /// Send a failed prompt again.
    TryAgain,
    /// Carry on with a reply that stopped or was cut short.
    Continue,
    /// Stop the reply being written.
    Stop,
    /// Open the conversation again after it failed to load.
    Reload,
}

impl StatusButton {
    /// Whether it is the one primary button: the one that carries on.
    fn primary(self) -> bool {
        !matches!(self, Self::Dismiss | Self::Retry)
    }
}

/// The element id of the button at `position` in the bar.
pub(crate) fn button_id(position: usize) -> ElementId {
    (ElementId::from(STATUS_ID), format!("button-{position}")).into()
}

/// A button's click handler.
pub(crate) type OnClick = Box<dyn Fn(&mut Window, &mut App) + 'static>;

/// What the bar says and offers.
pub(crate) struct Status {
    pub(crate) status: RunStatus,
    /// The words.
    pub(crate) message: SharedString,
    /// The buttons, left to right, with their labels and handlers.
    pub(crate) buttons: Vec<(SharedString, StatusButton, OnClick)>,
}

pub(crate) fn render(status: Status) -> AnyElement {
    let buttons = status.buttons.into_iter().map(|(label, which, on_click)| {
        let button =
            RunStatusButton::new(label).on_click(move |_, window, cx| on_click(window, cx));
        if which.primary() {
            button.primary()
        } else {
            button
        }
    });
    RunStatusBar::new(STATUS_ID, status.status)
        .message(status.message)
        .buttons(buttons)
        .max_w(metrics::column_width())
        .into_any_element()
}
