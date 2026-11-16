//! The sidebar: collapse and new-chat buttons, then the chats.
//!
//! The list is a static stand-in for the kit's `SessionList` binding, which
//! will own grouping, pinning, rename and search for every app that lists
//! sessions.

use ui::component::button::{Button, ButtonVariants as _};
use ui::component::sidebar::{Sidebar, SidebarGroup, SidebarMenu, SidebarMenuItem};
use ui::component::{ActiveTheme as _, Icon, Sizable as _, h_flex, v_flex};
use ui::prelude::*;
use ui::{SharedString, div};

use super::{ChatShell, Route};
use crate::{NewChat, ToggleSidebar, copy, icons, metrics};

pub(super) fn render(shell: &ChatShell, cx: &mut Context<ChatShell>) -> impl IntoElement {
    let header = h_flex()
        .w_full()
        .gap_1()
        .child(
            Button::new("sidebar-toggle")
                .ghost()
                .small()
                .icon(icons::toggle_sidebar())
                .tooltip(copy::TOGGLE_SIDEBAR)
                .on_click(|_, window, cx| window.dispatch_action(Box::new(ToggleSidebar), cx)),
        )
        .child(div().flex_1())
        .child(
            Button::new("new-chat")
                .ghost()
                .small()
                .icon(icons::new_chat())
                .tooltip(copy::NEW_CHAT)
                .on_click(|_, window, cx| window.dispatch_action(Box::new(NewChat), cx)),
        );

    let selected = match shell.route() {
        Route::Session(id) => Some(id.clone()),
        Route::Home => None,
    };
    let rows = shell.sessions().iter().map(|session| {
        let id = session.id.clone();
        SidebarMenuItem::new(session.title.clone())
            .active(selected.as_ref() == Some(&session.id))
            .when(session.pinned, |item| {
                item.suffix(|_, _| Icon::new(icons::pinned()).small())
            })
            .on_click(
                cx.listener(move |shell, _, window, cx| shell.open_session(id.clone(), window, cx)),
            )
    });

    let muted = cx.theme().muted_foreground;
    let note = |id: &'static str, text: SharedString| {
        div()
            .id(id)
            .px_2()
            .py_1()
            .text_sm()
            .text_color(muted)
            .child(text)
    };
    let status = if let Some(error) = shell.load_error() {
        Some(
            v_flex()
                .gap_2()
                .child(note(
                    "chats-failed",
                    format!("{} {error}", copy::LIST_FAILED).into(),
                ))
                .child(
                    div().px_2().child(
                        Button::new("chats-retry")
                            .outline()
                            .small()
                            .label(copy::TRY_AGAIN)
                            .on_click(cx.listener(|shell, _, _, cx| shell.reload(cx))),
                    ),
                )
                .into_any_element(),
        )
    } else if shell.is_loaded() && shell.sessions().is_empty() {
        Some(note("chats-empty", copy::NO_CHATS.into()).into_any_element())
    } else {
        None
    };

    Sidebar::new("chat_sidebar")
        .w(metrics::sidebar_width())
        .h_full()
        .border_r_1()
        .border_color(cx.theme().sidebar_border)
        .header(header)
        .child(SidebarGroup::new(copy::CHATS).child(SidebarMenu::new().children(rows)))
        .footer(
            v_flex().w_full().gap_2().children(status).children(
                shell
                    .save_error()
                    .map(|_| note("save-failed", copy::SAVE_FAILED.into())),
            ),
        )
}
