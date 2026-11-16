//! The header above the panel (Figma `ChatHeader`, states `Untitled` and
//! `Titled`): the title with its menu, Share, and the panel toggles.

use ui::component::button::{Button, ButtonVariants as _};
use ui::component::input::Input;
use ui::component::menu::{DropdownMenu as _, PopupMenuItem};
use ui::component::{ActiveTheme as _, Disableable as _, Sizable as _, h_flex};
use ui::prelude::*;
use ui::{AnyElement, div};

use super::ChatShell;
use crate::{ToggleContextPanel, ToggleSidebar, copy, icons, metrics};

pub(super) fn render(shell: &ChatShell, cx: &mut Context<ChatShell>) -> impl IntoElement {
    let title = title(shell, cx);
    let titled = shell.current().is_some();
    let theme = cx.theme();

    h_flex()
        .id("chat_header")
        .w_full()
        .h(metrics::header_height())
        .flex_shrink_0()
        .px_4()
        .gap_2()
        .border_b_1()
        .border_color(theme.border)
        .when(!shell.sidebar_open(), |this| {
            this.child(
                Button::new("header-sidebar-toggle")
                    .ghost()
                    .small()
                    .icon(icons::toggle_sidebar())
                    .tooltip(copy::TOGGLE_SIDEBAR)
                    .on_click(|_, window, cx| window.dispatch_action(Box::new(ToggleSidebar), cx)),
            )
        })
        .child(h_flex().flex_1().min_w_0().overflow_hidden().child(title))
        .when(titled, |this| {
            let share = Button::new("share")
                .outline()
                .small()
                .icon(icons::share())
                .tooltip(copy::SHARE_LATER)
                .disabled(true);
            // Narrow, Share keeps its icon; the tooltip still names it.
            this.child(if shell.is_narrow() {
                share
            } else {
                share.label(copy::SHARE)
            })
        })
        .child(
            Button::new("context-toggle")
                .ghost()
                .small()
                .icon(icons::toggle_context())
                .tooltip(copy::TOGGLE_CONTEXT)
                .on_click(|_, window, cx| window.dispatch_action(Box::new(ToggleContextPanel), cx)),
        )
}

/// Untitled: a muted "New chat". Titled: the title, whose menu renames or
/// deletes the chat. While renaming, an inline field.
fn title(shell: &ChatShell, cx: &mut Context<ChatShell>) -> AnyElement {
    let theme = cx.theme();
    if let Some(rename) = &shell.rename {
        return div()
            .max_w(metrics::content_max_width())
            .child(Input::new(&rename.field).id("chat-title-rename").small())
            .into_any_element();
    }
    let Some(session) = shell.current() else {
        return div()
            .text_sm()
            .text_color(theme.muted_foreground)
            .child(copy::NEW_CHAT)
            .into_any_element();
    };
    let view = cx.entity();
    Button::new("chat-title")
        .ghost()
        .small()
        .label(session.title.clone())
        .dropdown_caret(true)
        .dropdown_menu(move |menu, _, _| {
            let rename = view.clone();
            let delete = view.clone();
            menu.item(
                PopupMenuItem::new(copy::RENAME).on_click(move |_, window, cx| {
                    rename.update(cx, |shell, cx| shell.begin_rename(window, cx))
                }),
            )
            .item(
                PopupMenuItem::new(copy::DELETE).on_click(move |_, window, cx| {
                    delete.update(cx, |shell, cx| shell.delete_current(window, cx))
                }),
            )
        })
        .into_any_element()
}
