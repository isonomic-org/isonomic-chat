//! One turn as a row of the conversation list: the prompt's bubble on the
//! trailing side, then the reply, unboxed, with its actions.
//!
//! How the last turn ended (stopped, cut short, failed) is said by the status
//! bar above the composer, with its continue and try again; earlier turns say
//! it in a quiet note under their reply.

use conversation_model::{Turn, TurnState};
use std::hash::{Hash as _, Hasher as _};
use std::rc::Rc;
use ui::assets::IconName;
use ui::component::bubble::{Bubble, BubbleContent, BubbleVariant};
use ui::component::button::{Button, ButtonVariants as _};
use ui::component::clipboard::Clipboard;
use ui::component::input::{Textarea, TextareaState};
use ui::component::message::{Message, MessageAlignment, MessageContent};
use ui::component::text::{TextView, TextViewState, TextViewStyle};
use ui::component::{ActiveTheme as _, Icon, Sizable as _, Size, h_flex, v_flex};
use ui::extra::message_action_bar::{MessageAction, MessageActionBar, MessageActionLabel};
use ui::extra::thinking_indicator::{OrbState, ThinkingIndicator};
use ui::prelude::*;
use ui::{
    AnyElement, App, ElementId, Entity, FocusHandle, KeyDownEvent, SharedString, Window, div,
};

use crate::{ConversationCopy, metrics};

/// A click handler.
pub(crate) type Handler = Rc<dyn Fn(&mut Window, &mut App)>;

/// A handler for picking a version, counted from zero.
pub(crate) type VersionHandler = Rc<dyn Fn(usize, &mut Window, &mut App)>;

/// What a reply offers besides copy: trying it again, and its versions.
#[derive(Default)]
pub(crate) struct ReplyControls {
    /// Write the reply again, as a new version.
    pub(crate) on_regenerate: Option<Handler>,
    /// How many versions there are and which shows, with a handler to
    /// pick another; only with more than one.
    pub(crate) versions: Option<(usize, usize, VersionHandler)>,
    /// Open the edit field on the prompt.
    pub(crate) on_edit: Option<Handler>,
    /// The prompt's edit field, open, with save and cancel.
    pub(crate) editing: Option<EditBox>,
    /// Shows the prompt's actions while one of them has focus.
    pub(crate) prompt_focus: Option<FocusHandle>,
}

/// An open edit field on a prompt.
pub(crate) struct EditBox {
    pub(crate) field: Entity<TextareaState>,
    pub(crate) on_save: Handler,
    pub(crate) on_cancel: Handler,
}

/// Render one turn. `last` is whether it is the newest, whose ending the
/// status bar shows instead of a note.
pub(crate) fn render_turn(
    index: usize,
    turn: &Turn,
    last: bool,
    copy: &ConversationCopy,
    mut controls: ReplyControls,
    text: Option<Entity<TextViewState>>,
    cx: &App,
) -> AnyElement {
    let mut row = v_flex()
        .id(ElementId::NamedInteger("turn".into(), index as u64))
        .w_full()
        .min_w_0()
        .gap_6();
    if !turn.prompt.is_empty() {
        let (on_edit, editing) = (controls.on_edit.take(), controls.editing.take());
        let focus = controls.prompt_focus.take();
        row = row.child(match editing {
            Some(editing) => edit_box(index, editing, copy, cx).into_any_element(),
            None => prompt(index, turn, copy, on_edit, focus).into_any_element(),
        });
    }
    let open = turn.state.is_open();
    let said =
        !shown(&turn.reply()).is_empty() || !turn.notices.is_empty() || controls.versions.is_some();
    let noted = turn.refusal.is_some()
        || matches!(turn.state, TurnState::Refused(_))
        || (!last && ending(turn, copy).is_some());
    if open || said || noted {
        row = row.child(reply(index, turn, last, copy, controls, text, cx));
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
fn prompt(
    index: usize,
    turn: &Turn,
    copy: &ConversationCopy,
    on_edit: Option<Handler>,
    focus: Option<FocusHandle>,
) -> impl IntoElement {
    let group = SharedString::from(format!("prompt-{index}"));
    v_flex()
        .id(ElementId::NamedInteger("prompt".into(), index as u64))
        .group(group.clone())
        .w_full()
        .items_end()
        .gap_1p5()
        .child(
            Message::new().alignment(MessageAlignment::End).content(
                // Design 0.2 UserMessage: 16 / 24 text, 16 / 10 padding
                // (less the bubble's 1 px border), wraps at 560.
                MessageContent::new().bubble(
                    Bubble::new()
                        .with_variant(BubbleVariant::Muted)
                        .max_w(metrics::bubble_width())
                        .content(
                            BubbleContent::new()
                                .text_base()
                                .line_height(metrics::text_line())
                                .px(metrics::bubble_pad_x())
                                .py(metrics::bubble_pad_y())
                                .child(turn.prompt.clone()),
                        ),
                ),
            ),
        )
        .child(
            // Hidden until pointed at or focused, but still laid out and
            // painted, so its buttons are Tab stops.
            h_flex()
                .id(ElementId::NamedInteger(
                    "prompt-actions".into(),
                    index as u64,
                ))
                .gap_0p5()
                .opacity(0.)
                .group_hover(group, |this| this.opacity(1.))
                .when_some(focus, |this, focus| {
                    this.track_focus(&focus).in_focus(|this| this.opacity(1.))
                })
                .child(
                    Clipboard::new(ElementId::NamedInteger("prompt-copy".into(), index as u64))
                        .value(turn.prompt.clone())
                        .tooltip(copy.copy_prompt.clone())
                        .accessibility_label(copy.copy_prompt.clone())
                        .with_size(Size::Size(metrics::icon_button()))
                        .icon_size(Size::Size(metrics::icon_size())),
                )
                .children(on_edit.map(|on_edit| {
                    Button::new(ElementId::NamedInteger("prompt-edit".into(), index as u64))
                        .ghost()
                        .with_size(Size::Size(metrics::icon_button()))
                        .icon_size(Size::Size(metrics::icon_size()))
                        .icon(IconName::Edit)
                        .tooltip(copy.edit.clone())
                        .on_click(move |_, window, cx| on_edit(window, cx))
                })),
        )
}

/// The edit field in place of a prompt's bubble, with Cancel and Save.
fn edit_box(index: usize, editing: EditBox, copy: &ConversationCopy, cx: &App) -> impl IntoElement {
    let EditBox {
        field,
        on_save,
        on_cancel,
    } = editing;
    h_flex().w_full().justify_end().child(
        v_flex()
            .id(ElementId::NamedInteger(
                "prompt-editing".into(),
                index as u64,
            ))
            .w_full()
            .max_w(metrics::edit_width())
            .gap_3()
            .px_3p5()
            .py_3()
            .border_2()
            .rounded_2xl()
            .border_color(cx.theme().foreground)
            .bg(cx.theme().background)
            .shadow_xs()
            // Escape cancels, as the Cancel button does.
            .on_key_down({
                let on_cancel = on_cancel.clone();
                move |event: &KeyDownEvent, window, cx| {
                    if event.keystroke.key == "escape" {
                        cx.stop_propagation();
                        on_cancel(window, cx);
                    }
                }
            })
            .child(
                Textarea::new(&field)
                    .appearance(false)
                    .large()
                    .p_0()
                    .aria_label(copy.edit_field.clone()),
            )
            .child(
                h_flex()
                    .justify_end()
                    .gap_2()
                    .child(
                        Button::new("prompt-edit-cancel")
                            .outline()
                            .label(copy.cancel.clone())
                            .on_click(move |_, window, cx| on_cancel(window, cx)),
                    )
                    .child(
                        Button::new("prompt-edit-save")
                            .primary()
                            .label(copy.save.clone())
                            .on_click(move |_, window, cx| on_save(window, cx)),
                    ),
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
    controls: ReplyControls,
    shown: Option<Entity<TextViewState>>,
    cx: &App,
) -> impl IntoElement {
    let muted = cx.theme().muted_foreground;
    let mut column = v_flex()
        .id(ElementId::NamedInteger("reply".into(), index as u64))
        .w_full()
        .min_w_0()
        .gap_3();
    let text = turn.reply();
    if turn.state.is_open() && shown.is_none() {
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
    let copy_code = copy.copy_code.clone();
    if let Some(shown) = &shown {
        column = column.child(
            div().w_full().min_w_0().child(
                TextView::new(shown)
                    // Design 0.2 has one heading level, 18 / 28 semibold; blocks
                    // 12 apart (the text view's own paragraph gap).
                    .style(
                        TextViewStyle::default().heading_font_size(|_, _| metrics::heading_size()),
                    )
                    // Lists: text 28 in, the marker in that column.
                    .list_indent(metrics::list_indent())
                    .list_marker_gap(metrics::list_marker_gap())
                    .code_block_actions(move |block, _, _| {
                        let code = block.code();
                        Clipboard::new(ElementId::Name(
                            format!("code-copy-{}", fingerprint(&code)).into(),
                        ))
                        .tooltip(copy_code.clone())
                        .accessibility_label(copy_code.clone())
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
    let ended = !turn.state.is_open();
    if ended && (shown.is_some() || controls.versions.is_some()) {
        let mut actions = vec![MessageAction::Copy];
        let mut bar = MessageActionBar::new(ElementId::NamedInteger(
            "reply-actions".into(),
            index as u64,
        ))
        .quiet(false)
        // Design 0.2: 28 buttons with 16 icons, the pager first, the app's
        // own words.
        .button_size(Size::Size(metrics::icon_button()))
        .icon_size(Size::Size(metrics::icon_size()))
        .pager_first(true)
        .label(MessageActionLabel::Regenerate, copy.try_again.clone())
        .label(MessageActionLabel::Copy, copy.copy.clone())
        .text(text);
        if controls.on_regenerate.is_some() || controls.versions.is_some() {
            actions.push(MessageAction::Regenerate);
        }
        if let Some(on_regenerate) = controls.on_regenerate {
            bar = bar.on_regenerate(move |window, cx| on_regenerate(window, cx));
        }
        if let Some((count, current, on_version)) = controls.versions {
            bar = bar
                .versions(count)
                .version(current)
                .on_version_change(move |version, window, cx| on_version(version, window, cx));
        }
        column = column.child(bar.actions(actions));
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

/// The reply as drawn: without an emphasis or code marker at its very end
/// that can only open a span, such as a reply stopped at `**`, which would
/// otherwise show as raw markup.
pub(crate) fn shown(text: &str) -> &str {
    let text = text.trim_end();
    let markers = text
        .bytes()
        .rev()
        .take_while(|byte| matches!(byte, b'*' | b'_' | b'~' | b'`'))
        .count();
    if markers == 0 {
        return text;
    }
    let rest = &text[..text.len() - markers];
    // A marker after a word can close a span; one at the start or after a
    // space can only open one.
    if rest.chars().next_back().is_none_or(char::is_whitespace) {
        rest.trim_end()
    } else {
        text
    }
}

#[cfg(test)]
mod tests {
    use super::shown;

    #[test]
    fn an_opening_marker_at_the_end_is_not_drawn() {
        assert_eq!(shown("**"), "");
        assert_eq!(shown("Here is **"), "Here is");
        assert_eq!(shown("Run `"), "Run");
        assert_eq!(shown("a list\n\n*"), "a list");
        assert_eq!(shown("** "), "");
    }

    #[test]
    fn a_closing_marker_and_plain_text_are_kept() {
        assert_eq!(shown("**bold**"), "**bold**");
        assert_eq!(shown("Plain text."), "Plain text.");
        assert_eq!(shown(""), "");
    }
}
