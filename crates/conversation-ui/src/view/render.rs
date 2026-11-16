//! Drawing a conversation: the column of turns, the status bar, the
//! composer.

use super::ConversationView;
use crate::metrics;
use crate::status::{OnClick, Status, StatusButton};
use crate::turn::{ReplyControls, ending, render_turn};
use conversation_model::InputState;
use conversation_model::TurnState;
use std::rc::Rc;
use ui::WeakEntity;
use ui::component::message_scroller::MessageScroller;
use ui::component::{ActiveTheme as _, v_flex};
use ui::extra::run_status_bar::RunStatus;
use ui::prelude::*;
use ui::{App, ElementId, SharedString, StyleRefinement, Window, div};

impl ConversationView {
    /// What the status bar says and which buttons it shows, left to right:
    /// a failed load; closed, reconnecting or an app away; a reply being
    /// written; else how the last turn ended, unless that was dismissed.
    pub(super) fn status_layout(&self) -> Option<(RunStatus, SharedString, Vec<StatusButton>)> {
        let copy = &self.copy;
        if self.load_failed.is_some() {
            let buttons = vec![StatusButton::Reload];
            return Some((RunStatus::Failed, copy.load_failed.clone(), buttons));
        }
        let thread = self.thread.borrow();
        if thread.input_state() == InputState::Closed {
            return Some((RunStatus::Closed, copy.closed.clone(), Vec::new()));
        } else if self.reconnecting {
            return Some((
                RunStatus::Reconnecting,
                copy.reconnecting.clone(),
                Vec::new(),
            ));
        } else if thread.awaiting_client() {
            return Some((
                RunStatus::Reconnecting,
                copy.awaiting_client.clone(),
                Vec::new(),
            ));
        }
        let index = thread.turns().len().checked_sub(1)?;
        let turn = &thread.turns()[index];
        if self.dismissed == Some((index, turn.revision)) {
            return None;
        }
        if turn.retrying.is_some() {
            return Some((RunStatus::Reconnecting, copy.retrying.clone(), Vec::new()));
        }
        if turn.state.is_open() {
            return Some((
                RunStatus::Running,
                copy.working.clone(),
                vec![StatusButton::Stop],
            ));
        }
        let message = ending(turn, copy)?;
        let can_continue = thread.input_state().can_continue();
        let can_retry = turn.state.can_retry()
            && !turn.prompt.is_empty()
            && turn.error_code.as_deref() != Some(conversation_model::TOO_LARGE);
        let state = turn.state.clone();
        drop(thread);
        let mut buttons = vec![StatusButton::Dismiss];
        let status = match &state {
            TurnState::Stopped => {
                if self.can_regenerate(index) {
                    buttons.push(StatusButton::Retry);
                }
                RunStatus::Stopped
            }
            TurnState::Truncated | TurnState::Limited => RunStatus::CutShort,
            _ => {
                if can_retry {
                    buttons.push(StatusButton::TryAgain);
                }
                RunStatus::Failed
            }
        };
        if can_continue {
            buttons.push(StatusButton::Continue);
        }
        Some((status, message, buttons))
    }

    /// The element id of one of the status bar's buttons, if it shows now.
    pub fn status_button(&self, which: StatusButton) -> Option<ElementId> {
        let (_, _, buttons) = self.status_layout()?;
        let position = buttons.iter().position(|b| *b == which)?;
        Some(crate::status::button_id(position))
    }

    /// Hide the status bar until the last turn changes.
    pub fn dismiss_status(&mut self, cx: &mut Context<Self>) {
        let thread = self.thread.borrow();
        if let Some(turn) = thread.turns().last() {
            self.dismissed = Some((thread.turns().len() - 1, turn.revision));
        }
        drop(thread);
        cx.notify();
    }

    /// The status bar, with a handler for each button.
    fn status(&self, cx: &mut Context<Self>) -> Option<Status> {
        let (status, message, layout) = self.status_layout()?;
        let view = cx.weak_entity();
        let stopped = status == RunStatus::Stopped;
        let index = self.thread.borrow().turns().len().saturating_sub(1);
        let buttons = layout
            .into_iter()
            .map(|which| {
                let view = view.clone();
                let label = match which {
                    StatusButton::Dismiss => self.copy.dismiss.clone(),
                    StatusButton::Retry => self.copy.retry.clone(),
                    StatusButton::TryAgain | StatusButton::Reload => self.copy.try_again.clone(),
                    StatusButton::Continue if stopped => self.copy.resume.clone(),
                    StatusButton::Continue => self.copy.continue_label.clone(),
                    StatusButton::Stop => self.copy.stop.clone(),
                };
                let click = Box::new(move |_: &mut Window, cx: &mut App| {
                    let _ = view.update(cx, |view, cx| match which {
                        StatusButton::Dismiss => view.dismiss_status(cx),
                        StatusButton::Retry => view.regenerate(index, cx),
                        StatusButton::TryAgain => view.retry(cx),
                        StatusButton::Continue => view.resume(cx),
                        StatusButton::Stop => view.stop(cx),
                        StatusButton::Reload => view.retry_load(cx),
                    });
                }) as OnClick;
                (label, which, click)
            })
            .collect();
        Some(Status {
            status,
            message,
            buttons,
        })
    }
}

impl Render for ConversationView {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let status = self.status(cx);
        let thread = self.thread.clone();
        let copy = self.copy.clone();
        let view = cx.weak_entity();
        let list = MessageScroller::new(
            "conversation-turns",
            self.scroller.clone(),
            move |index, window, cx| {
                let thread = thread.borrow();
                let Some(turn) = thread.turns().get(index) else {
                    return div().into_any_element();
                };
                let count = thread.turns().len();
                let last = index + 1 == count;
                let mut controls = controls(&view, index, cx);
                // The prompt's actions show while one of them has focus, so a
                // keyboard reaches them as a pointer does.
                let focus = window.use_keyed_state(
                    ElementId::NamedInteger("prompt-actions".into(), index as u64),
                    cx,
                    |_, cx| cx.focus_handle().tab_stop(false),
                );
                controls.prompt_focus = Some(focus.read(cx).clone());
                // The column: centred, at most 720 wide, 32 above the first
                // turn and below the last, 24 between turns.
                div()
                    .w_full()
                    .flex()
                    .justify_center()
                    .px_8()
                    .when(index == 0, |this| this.pt_8())
                    .map(|this| if last { this.pb_8() } else { this.pb_6() })
                    .child(
                        div()
                            .w_full()
                            .max_w(metrics::column_width())
                            .child(render_turn(index, turn, last, &copy, controls, cx)),
                    )
                    .into_any_element()
            },
        )
        // The kit sets every gap and inset itself (design 0.2 thread.md):
        // none from the scroller.
        .with_list_style(StyleRefinement::default().pt_0().pb_0())
        .with_row_style(StyleRefinement::default().px_0().pb_0())
        .with_bottom_fade(cx.theme().background);

        v_flex()
            .id("conversation")
            .track_focus(&self.focus)
            .size_full()
            .min_h_0()
            .child(div().flex_1().min_h_0().w_full().child(list))
            .child(
                v_flex()
                    .id("conversation-dock")
                    .w_full()
                    .items_center()
                    .gap_2()
                    .px_8()
                    .children(status.map(crate::status::render))
                    .child(
                        div()
                            .w_full()
                            .max_w(metrics::column_width())
                            .child(self.composer.clone()),
                    ),
            )
    }
}

/// Try again and the version pager for the reply of the turn at `index`.
fn controls(view: &WeakEntity<ConversationView>, index: usize, cx: &App) -> ReplyControls {
    let Some(entity) = view.upgrade() else {
        return ReplyControls::default();
    };
    let this = entity.read(cx);
    let on_regenerate = this.can_regenerate(index).then(|| {
        let view = view.clone();
        Rc::new(move |_: &mut Window, cx: &mut App| {
            let _ = view.update(cx, |view, cx| view.regenerate(index, cx));
        }) as crate::turn::Handler
    });
    let versions = this.versions(index).map(|(count, current)| {
        let view = view.clone();
        let pick = Rc::new(move |version: usize, _: &mut Window, cx: &mut App| {
            let _ = view.update(cx, |view, cx| view.show_version(index, version, cx));
        }) as crate::turn::VersionHandler;
        (count, current, pick)
    });
    let on_edit = (this.can_regenerate(index) && this.editing().is_none()).then(|| {
        let view = view.clone();
        Rc::new(move |window: &mut Window, cx: &mut App| {
            let _ = view.update(cx, |view, cx| view.begin_edit(index, window, cx));
        }) as crate::turn::Handler
    });
    let editing = this
        .editing
        .as_ref()
        .filter(|editing| editing.index == index)
        .map(|editing| {
            let (save, cancel) = (view.clone(), view.clone());
            crate::turn::EditBox {
                field: editing.field.clone(),
                on_save: Rc::new(move |window: &mut Window, cx: &mut App| {
                    let _ = save.update(cx, |view, cx| view.save_edit(window, cx));
                }),
                on_cancel: Rc::new(move |window: &mut Window, cx: &mut App| {
                    let _ = cancel.update(cx, |view, cx| view.cancel_edit(window, cx));
                }),
            }
        });
    ReplyControls {
        on_regenerate,
        versions,
        on_edit,
        editing,
        prompt_focus: None,
    }
}
