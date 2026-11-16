//! The status bar above the composer: the one place that says the
//! conversation is reconnecting or closed, or how the last reply ended, with
//! the one thing to do about it (continue, or try again). Drawn by Isonomic
//! UI's run status bar; the kit picks the state, the words and the action.

use ui::extra::run_status_bar::{RunStatus, RunStatusBar};
use ui::prelude::*;
use ui::{AnyElement, App, ElementId, SharedString, Window};

use crate::metrics;

/// Element id of the bar when its action continues the reply. Its button
/// is the child `(CONTINUE_ID, "action")`.
pub const CONTINUE_ID: &str = "conversation-continue";

/// Element id of the bar when its action tries the prompt again. Its button
/// is the child `(RETRY_ID, "action")`.
pub const RETRY_ID: &str = "conversation-retry";

/// Element id of the bar when it has no action.
const STATUS_ID: &str = "conversation-status";

/// The button of a bar whose id is `bar`, such as [`CONTINUE_ID`].
pub fn action_button(bar: &'static str) -> ElementId {
    (ElementId::from(bar), "action").into()
}

/// A button's click handler.
pub(crate) type OnClick = Box<dyn Fn(&mut Window, &mut App) + 'static>;

/// What the action does.
pub(crate) enum Action {
    /// Continue the reply from where it stopped.
    Continue,
    /// Send the prompt again.
    Retry,
}

/// What the bar says and offers.
pub(crate) struct Status {
    pub(crate) status: RunStatus,
    /// The words.
    pub(crate) message: SharedString,
    /// The action, its label and its handler.
    pub(crate) action: Option<(Action, SharedString, OnClick)>,
}

pub(crate) fn render(status: Status) -> AnyElement {
    let id = match &status.action {
        Some((Action::Continue, ..)) => CONTINUE_ID,
        Some((Action::Retry, ..)) => RETRY_ID,
        None => STATUS_ID,
    };
    let mut bar = RunStatusBar::new(id, status.status)
        .message(status.message)
        .max_w(metrics::column_width());
    if let Some((_, label, on_click)) = status.action {
        bar = bar
            .action_label(label)
            .on_action(move |_, window, cx| on_click(window, cx));
    }
    bar.into_any_element()
}
