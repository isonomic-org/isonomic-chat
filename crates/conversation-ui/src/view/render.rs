//! Drawing a conversation: the column of turns, the status bar, the
//! composer.

use super::ConversationView;
use crate::metrics;
use crate::status::{OnClick, Status};
use crate::turn::{ReplyControls, ending, render_turn};
use conversation_model::InputState;
use std::rc::Rc;
use ui::WeakEntity;
use ui::component::message_scroller::MessageScroller;
use ui::component::{ActiveTheme as _, v_flex};
use ui::prelude::*;
use ui::{App, SharedString, Window, div};

impl ConversationView {
    /// The status bar: closed, reconnecting or an app away; else how the
    /// last turn ended, with continue or try again where they apply.
    fn status(&self, cx: &mut Context<Self>) -> Option<Status> {
        let thread = self.thread.borrow();
        let plain = |message: &SharedString| Status {
            message: message.clone(),
            error: false,
            stopped: false,
            on_continue: None,
            on_retry: None,
        };
        if thread.input_state() == InputState::Closed {
            return Some(plain(&self.copy.closed));
        } else if self.reconnecting {
            return Some(plain(&self.copy.reconnecting));
        } else if thread.awaiting_client() {
            return Some(plain(&self.copy.awaiting_client));
        }
        let turn = thread.turns().last()?;
        let message = ending(turn, &self.copy)?;
        let view = cx.weak_entity();
        let on_continue = thread.input_state().can_continue().then(|| {
            let view = view.clone();
            let click = Box::new(move |_: &mut Window, cx: &mut App| {
                let _ = view.update(cx, |view, cx| view.resume(cx));
            }) as OnClick;
            (self.copy.continue_label.clone(), click)
        });
        let retry = turn.state.can_retry()
            && !turn.prompt.is_empty()
            && turn.error_code.as_deref() != Some(conversation_model::TOO_LARGE);
        let on_retry = retry.then(|| {
            let click = Box::new(move |_: &mut Window, cx: &mut App| {
                let _ = view.update(cx, |view, cx| view.retry(cx));
            }) as OnClick;
            (self.copy.try_again.clone(), click)
        });
        Some(Status {
            message,
            error: turn.state.can_retry(),
            stopped: turn.state == conversation_model::TurnState::Stopped,
            on_continue,
            on_retry,
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
            move |index, _, cx| {
                let thread = thread.borrow();
                let Some(turn) = thread.turns().get(index) else {
                    return div().into_any_element();
                };
                let count = thread.turns().len();
                let last = index + 1 == count;
                let controls = controls(&view, index, cx);
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
                    .children(status.map(|status| crate::status::render(status, cx)))
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
    }
}
