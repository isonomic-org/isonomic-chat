//! One turn as a row of the conversation list.

use conversation_model::{Turn, TurnState};
use ui::component::bubble::{Bubble, BubbleVariant};
use ui::component::button::Button;
use ui::component::message::{Message, MessageAlignment, MessageContent};
use ui::component::text::TextView;
use ui::component::{ActiveTheme as _, Sizable as _, h_flex, v_flex};
use ui::extra::thinking_indicator::{OrbState, ThinkingIndicator};
use ui::prelude::*;
use ui::{AnyElement, App, ElementId, SharedString, Window, div};

use crate::ConversationCopy;

/// Element id of the continue button under the last turn.
pub(crate) const CONTINUE_ID: &str = "conversation-continue";

/// Element id of the try-again button under a failed last turn.
pub(crate) const RETRY_ID: &str = "conversation-retry";

/// A row's click handler.
pub(crate) type OnClick = Box<dyn Fn(&mut Window, &mut App) + 'static>;

/// What the last row may offer.
pub(crate) struct RowActions {
    /// Called when continue is pressed, on the last turn only.
    pub(crate) on_continue: Option<OnClick>,
    /// Called when try again is pressed, on a failed last turn only.
    pub(crate) on_retry: Option<OnClick>,
}

/// Render one turn: the prompt on the trailing side, the reply below it, and
/// a status line for how it ended.
pub(crate) fn render_turn(
    index: usize,
    turn: &Turn,
    copy: &ConversationCopy,
    actions: RowActions,
    cx: &App,
) -> AnyElement {
    let mut row = v_flex()
        .id(ElementId::NamedInteger("turn".into(), index as u64))
        .w_full()
        .min_w_0()
        .gap_3()
        .pb_6();

    if !turn.prompt.is_empty() {
        let bubble = Bubble::new()
            .with_variant(BubbleVariant::Muted)
            .child(turn.prompt.clone());
        row = row.child(
            Message::new()
                .alignment(MessageAlignment::End)
                .content(MessageContent::new().bubble(bubble)),
        );
    }

    let reply = turn.reply();
    let waiting = turn.state.is_open() && reply.is_empty();
    if waiting {
        let label = if turn.thinking().is_empty() {
            copy.thinking.clone()
        } else {
            SharedString::from(last_line(&turn.thinking()))
        };
        row = row.child(
            div()
                .id("thinking")
                .child(ThinkingIndicator::new("thinking-orb", OrbState::Reasoning).label(label)),
        );
    }
    if !reply.is_empty() {
        row = row.child(
            div()
                .id(ElementId::NamedInteger("reply".into(), index as u64))
                .w_full()
                .min_w_0()
                .child(TextView::markdown(
                    ElementId::NamedInteger("reply-text".into(), index as u64),
                    reply,
                )),
        );
    }
    for notice in &turn.notices {
        row = row.child(
            div()
                .text_sm()
                .text_color(cx.theme().muted_foreground)
                .child(format!("{} ({})", copy.unknown, notice.kind)),
        );
    }

    let note = match &turn.state {
        _ if turn.retrying.is_some() => Some(copy.retrying.clone()),
        TurnState::Stopped => Some(copy.stopped.clone()),
        TurnState::Truncated => Some(copy.truncated.clone()),
        TurnState::Limited => Some(copy.limited.clone()),
        TurnState::Failed(message) | TurnState::Rejected(message) => {
            Some(or(message, &copy.failed))
        }
        TurnState::Refused(message) => Some(or(message, &copy.refused)),
        _ => None,
    };
    let mut status = h_flex().gap_3().items_center();
    let mut has_status = false;
    if let Some(note) = note {
        has_status = true;
        status = status.child(
            div()
                .id("turn-note")
                .text_sm()
                .text_color(cx.theme().muted_foreground)
                .child(note),
        );
    }
    if let Some(on_continue) = actions.on_continue {
        has_status = true;
        status = status.child(
            Button::new(CONTINUE_ID)
                .outline()
                .small()
                .label(copy.continue_label.clone())
                .on_click(move |_, window, cx| on_continue(window, cx)),
        );
    }
    if let Some(on_retry) = actions.on_retry {
        has_status = true;
        status = status.child(
            Button::new(RETRY_ID)
                .outline()
                .small()
                .label(copy.try_again.clone())
                .on_click(move |_, window, cx| on_retry(window, cx)),
        );
    }
    if has_status {
        row = row.child(status);
    }
    row.into_any_element()
}

fn or(message: &str, fallback: &SharedString) -> SharedString {
    if message.is_empty() {
        fallback.clone()
    } else {
        SharedString::from(message.to_owned())
    }
}

/// The last non-empty line of some text, for a compact status.
fn last_line(text: &str) -> String {
    text.lines()
        .rev()
        .find(|line| !line.trim().is_empty())
        .unwrap_or_default()
        .trim()
        .to_owned()
}
