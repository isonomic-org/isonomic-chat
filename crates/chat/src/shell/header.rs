//! The header above the panel (design 0.2 `SessionHeader`): on a chat, its
//! title with a menu to rename, pin or delete it; on home, nothing. The
//! device indicator, side-panel toggle, Share and incognito are outside
//! version 1.

use ui::component::button::{Button, ButtonVariants as _};
use ui::component::input::Input;
use ui::component::menu::{DropdownMenu as _, PopupMenuItem};
use ui::component::{Sizable as _, h_flex};
use ui::prelude::*;
use ui::{AnyElement, div};

use super::ChatShell;
use crate::{copy, metrics};

pub(super) fn render(shell: &ChatShell, cx: &mut Context<ChatShell>) -> impl IntoElement {
    h_flex()
        .id("chat_header")
        .w_full()
        .h(metrics::header_height())
        .flex_shrink_0()
        .px_4()
        .gap_2()
        .child(
            h_flex()
                .flex_1()
                .min_w_0()
                .overflow_hidden()
                .children(title(shell, cx)),
        )
}

/// The open chat's title, whose menu renames, pins or deletes it. While
/// renaming, an inline field. Nothing on home.
fn title(shell: &ChatShell, cx: &mut Context<ChatShell>) -> Option<AnyElement> {
    if let Some(rename) = &shell.rename {
        return Some(
            div()
                .w_full()
                .max_w(metrics::content_max_width())
                .child(Input::new(&rename.field).id("chat-title-rename").small())
                .into_any_element(),
        );
    }
    let session = shell.current()?.clone();
    let view = cx.entity();
    Some(
        Button::new("chat-title")
            .ghost()
            .small()
            .label(session.title.clone())
            .dropdown_caret(true)
            .dropdown_menu(move |menu, _, _| {
                let (rename, pin, delete) = (view.clone(), view.clone(), view.clone());
                let (pin_id, delete_id) = (session.id.clone(), session.id.clone());
                let pinned = session.pinned;
                let pin_label = if pinned { copy::UNPIN } else { copy::PIN };
                menu.item(
                    PopupMenuItem::new(copy::RENAME).on_click(move |_, window, cx| {
                        rename.update(cx, |shell, cx| shell.begin_rename(window, cx))
                    }),
                )
                .item(PopupMenuItem::new(pin_label).on_click(move |_, _, cx| {
                    pin.update(cx, |shell, cx| shell.set_pinned(&pin_id, !pinned, cx))
                }))
                .separator()
                .item(
                    PopupMenuItem::new(copy::DELETE).on_click(move |_, window, cx| {
                        delete.update(cx, |shell, cx| shell.delete_session(&delete_id, window, cx))
                    }),
                )
            })
            .into_any_element(),
    )
}
