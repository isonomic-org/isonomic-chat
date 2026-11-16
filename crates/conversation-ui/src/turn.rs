//! One turn as a row of the conversation list: the prompt's bubble on the
//! trailing side, then the reply, unboxed, with its actions.
//!
//! How the last turn ended (stopped, cut short, failed) is said by the status
//! bar above the composer, with its continue and try again; earlier turns say
//! it in a quiet note under their reply.

use conversation_model::{Turn, TurnState};
use std::hash::{Hash as _, Hasher as _};
use ui::assets::IconName;
use ui::component::bubble::{Bubble, BubbleVariant};
use ui::component::clipboard::Clipboard;
use ui::component::message::{Message, MessageAlignment, MessageContent};
use ui::component::text::TextView;
use ui::component::{ActiveTheme as _, Icon, Sizable as _, h_flex, v_flex};
use ui::extra::message_action_bar::{MessageAction, MessageActionBar};
use ui::extra::thinking_indicator::{OrbState, ThinkingIndicator};
use ui::prelude::*;
use ui::{AnyElement, App, ElementId, SharedString, div};

use crate::ConversationCopy;

/// Render one turn. `last` is whether it is the newest, whose ending the
/// status bar shows instead of a note.
pub(crate) fn render_turn(
    index: usize,
    turn: &Turn,
    last: bool,
    copy: &ConversationCopy,
    cx: &App,
) -> AnyElement {
    let mut row = v_flex()
        .id(ElementId::NamedInteger("turn".into(), index as u64))
        .w_full()
        .min_w_0()
        .gap_6();
    if !turn.prompt.is_empty() {
        row = row.child(prompt(index, turn, copy));
    }
    let open = turn.state.is_open();
    let said = !turn.reply().is_empty() || !turn.notices.is_empty();
    let noted = turn.refusal.is_some()
        || matches!(turn.state, TurnState::Refused(_))
        || (!last && ending(turn, copy).is_some());
    if open || said || noted {
        row = row.child(reply(index, turn, last, copy, cx));
    }
    if let Some(note) = &turn.divider_after {
        row = row.child(
            div()
                .id("turn-divider")
                .w_full()
                .pt_2()
                .border_t_1()
                .border_color(cx.theme().border)
                .text_xs()
                .text_color(cx.theme().muted_foreground)
                .child(note.clone()),
        );
    }
    row.into_any_element()
}

/// The prompt: a muted bubble on the trailing side, and under it a copy
/// button that shows while the prompt is pointed at.
fn prompt(index: usize, turn: &Turn, copy: &ConversationCopy) -> impl IntoElement {
    let group = SharedString::from(format!("prompt-{index}"));
    v_flex()
        .id(ElementId::NamedInteger("prompt".into(), index as u64))
        .group(group.clone())
        .w_full()
        .items_end()
        .gap_1p5()
        .child(
            Message::new().alignment(MessageAlignment::End).content(
                MessageContent::new().bubble(
                    Bubble::new()
                        .with_variant(BubbleVariant::Muted)
                        .child(turn.prompt.clone()),
                ),
            ),
        )
        .child(
            h_flex()
                .invisible()
                .group_hover(group, |this| this.visible())
                .child(
                    Clipboard::new(ElementId::NamedInteger("prompt-copy".into(), index as u64))
                        .value(turn.prompt.clone())
                        .tooltip(copy.copy.clone())
                        .accessibility_label(copy.copy.clone())
                        .small(),
                ),
        )
}

/// The reply: thinking while nothing is written, the Markdown, notes, and
/// the action row once it has ended.
fn reply(
    index: usize,
    turn: &Turn,
    last: bool,
    copy: &ConversationCopy,
    cx: &App,
) -> impl IntoElement {
    let muted = cx.theme().muted_foreground;
    let mut column = v_flex()
        .id(ElementId::NamedInteger("reply".into(), index as u64))
        .w_full()
        .min_w_0()
        .gap_3();
    let text = turn.reply();
    if turn.state.is_open() && text.is_empty() {
        let label = if turn.thinking().is_empty() {
            copy.thinking.clone()
        } else {
            SharedString::from(last_line(&turn.thinking()))
        };
        column = column.child(
            div()
                .id("thinking")
                .child(ThinkingIndicator::new("thinking-orb", OrbState::Reasoning).label(label)),
        );
    }
    if !text.is_empty() {
        column = column.child(
            div().w_full().min_w_0().child(
                TextView::markdown(
                    ElementId::NamedInteger("reply-text".into(), index as u64),
                    text.clone(),
                )
                .code_block_actions(|block, _, _| {
                    let code = block.code();
                    Clipboard::new(ElementId::Name(
                        format!("code-copy-{}", fingerprint(&code)).into(),
                    ))
                    .value(code)
                    .small()
                }),
            ),
        );
    }
    for notice in &turn.notices {
        column = column.child(
            div()
                .text_sm()
                .text_color(muted)
                .child(format!("{} ({})", copy.unknown, notice.kind)),
        );
    }
    if let Some(refusal) = &turn.refusal {
        column = column.child(note("turn-note", None, refusal.clone().into(), muted));
    } else if let TurnState::Refused(message) = &turn.state {
        column = column.child(note("turn-note", None, or(message, &copy.refused), muted));
    } else if turn.state == TurnState::Stopped && !last {
        let stopped = copy.stopped.clone();
        column = column.child(note("turn-note", Some(IconName::Stop), stopped, muted));
    } else if !last && let Some(ending) = ending(turn, copy) {
        column = column.child(note("turn-note", None, ending, muted));
    }
    if !turn.state.is_open() && !text.is_empty() {
        column = column.child(
            MessageActionBar::new(ElementId::NamedInteger(
                "reply-actions".into(),
                index as u64,
            ))
            .actions([MessageAction::Copy])
            .quiet(false)
            .text(text),
        );
    }
    column
}

/// How a turn ended, in words, when it did not simply finish. The status
/// bar uses it for the last turn.
pub(crate) fn ending(turn: &Turn, copy: &ConversationCopy) -> Option<SharedString> {
    match &turn.state {
        _ if turn.retrying.is_some() => Some(copy.retrying.clone()),
        TurnState::Stopped => Some(copy.stopped.clone()),
        TurnState::Truncated => Some(copy.truncated.clone()),
        TurnState::Limited => Some(copy.limited.clone()),
        TurnState::Failed(message) | TurnState::Rejected(message) => {
            Some(or(message, &copy.failed))
        }
        _ => None,
    }
}

fn note(
    id: &'static str,
    icon: Option<IconName>,
    text: SharedString,
    color: ui::Hsla,
) -> impl IntoElement {
    h_flex()
        .id(id)
        .gap_1p5()
        .items_center()
        .text_sm()
        .text_color(color)
        .children(icon.map(|icon| Icon::new(icon).small().text_color(color)))
        .child(text)
}

fn or(message: &str, fallback: &SharedString) -> SharedString {
    if message.is_empty() {
        fallback.clone()
    } else {
        SharedString::from(message.to_owned())
    }
}

/// A short, stable key for a code block's copy button.
fn fingerprint(text: &str) -> u64 {
    let mut hasher = std::collections::hash_map::DefaultHasher::new();
    text.hash(&mut hasher);
    hasher.finish()
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
