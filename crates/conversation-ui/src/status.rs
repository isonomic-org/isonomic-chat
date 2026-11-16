//! The status bar above the composer: the one place that says the
//! conversation is reconnecting or closed, or how the last reply ended, with
//! continue and try again beside it.
//!
//! A plain placeholder until Isonomic UI has a run status bar (requested;
//! see `docs/name-map.md`): an icon, the words, the buttons, on the theme's
//! muted or danger colours.

use ui::assets::IconName;
use ui::component::button::{Button, ButtonVariants as _};
use ui::component::{ActiveTheme as _, Icon, Sizable as _, StyledExt as _, h_flex};
use ui::prelude::*;
use ui::{AnyElement, App, SharedString, Window, div};

use crate::metrics;

/// Element id of the continue button.
pub(crate) const CONTINUE_ID: &str = "conversation-continue";

/// Element id of the try-again button.
pub(crate) const RETRY_ID: &str = "conversation-retry";

/// A button's click handler.
pub(crate) type OnClick = Box<dyn Fn(&mut Window, &mut App) + 'static>;

/// What the bar says and offers.
pub(crate) struct Status {
    /// The words.
    pub(crate) message: SharedString,
    /// An error, drawn in the danger colours.
    pub(crate) error: bool,
    /// The person stopped the reply.
    pub(crate) stopped: bool,
    /// Continue the reply, labelled.
    pub(crate) on_continue: Option<(SharedString, OnClick)>,
    /// Send the prompt again, labelled.
    pub(crate) on_retry: Option<(SharedString, OnClick)>,
}

pub(crate) fn render(status: Status, cx: &App) -> AnyElement {
    let theme = cx.theme();
    let (bg, border, fg) = if status.error {
        (
            theme.danger.opacity(0.08),
            theme.danger.opacity(0.3),
            theme.danger,
        )
    } else {
        (theme.muted, theme.border, theme.foreground)
    };
    let icon = if status.error {
        IconName::CircleAlert
    } else if status.stopped {
        IconName::Stop
    } else {
        IconName::Info
    };
    let primary = status.on_continue.is_some();
    h_flex()
        .id("conversation-status")
        .w_full()
        .max_w(metrics::column_width())
        .gap_2()
        .py_1p5()
        .pl_3()
        .pr_1p5()
        .items_center()
        .border_1()
        .rounded_lg()
        .bg(bg)
        .border_color(border)
        .text_color(fg)
        .child(Icon::new(icon).small().text_color(fg))
        .child(
            div()
                .flex_1()
                .min_w_0()
                .text_sm()
                .font_medium()
                .child(status.message),
        )
        .children(status.on_retry.map(|(label, on_click)| {
            let button = Button::new(RETRY_ID).small().label(label);
            let button = if primary {
                button.outline()
            } else {
                button.primary()
            };
            button.on_click(move |_, window, cx| on_click(window, cx))
        }))
        .children(status.on_continue.map(|(label, on_click)| {
            Button::new(CONTINUE_ID)
                .primary()
                .small()
                .label(label)
                .on_click(move |_, window, cx| on_click(window, cx))
        }))
        .into_any_element()
}
