//! The app sidebar (design 0.2 `AppSidebar`, `AppSidebarRail`): the toggle
//! and the wordmark, the nav (New chat, Search), then the Pinned and Recents
//! groups of chats. Collapsed, it is a rail of icons: the toggle and the nav
//! stay, the chats and the wordmark go.

use ui::component::button::{Button, ButtonVariants as _};
use ui::component::menu::{DropdownMenu as _, PopupMenuItem};
use ui::component::sidebar::{
    Sidebar, SidebarCollapsible, SidebarGroup, SidebarItem as _, SidebarMenu, SidebarMenuItem,
};
use ui::component::{
    ActiveTheme as _, Collapsible as _, Sizable as _, StyledExt as _, h_flex, v_flex,
};
use ui::prelude::*;
use ui::{Entity, SharedString, Window, div};

use super::{ChatShell, Route};
use crate::demo::SessionSummary;
use crate::{NewChat, SearchChats, ToggleSidebar, copy, icons, metrics};

pub(super) fn render(
    shell: &ChatShell,
    window: &mut Window,
    cx: &mut Context<ChatShell>,
) -> impl IntoElement {
    let collapsed = shell.sidebar_collapsed;
    let toggle = Button::new("sidebar-toggle")
        .ghost()
        .small()
        .icon(icons::toggle_sidebar())
        .tooltip(copy::TOGGLE_SIDEBAR)
        .on_click(|_, window, cx| window.dispatch_action(Box::new(ToggleSidebar), cx));
    let top = h_flex()
        .w_full()
        .gap_2()
        .child(toggle)
        .when(!collapsed, |this| {
            this.child(
                div()
                    .id("wordmark")
                    .text_lg()
                    .font_semibold()
                    .text_color(cx.theme().foreground)
                    .child(copy::WORDMARK),
            )
        });

    let home = shell.route() == &Route::Home && !shell.is_searching();
    let nav = SidebarMenu::new()
        .child(
            SidebarMenuItem::new(copy::NEW_CHAT)
                .icon(icons::new_chat())
                .active(home)
                .on_click(|_, window, cx| window.dispatch_action(Box::new(NewChat), cx)),
        )
        .child(
            SidebarMenuItem::new(copy::SEARCH)
                .icon(icons::search())
                .active(shell.is_searching())
                .on_click(|_, window, cx| window.dispatch_action(Box::new(SearchChats), cx)),
        )
        .collapsed(collapsed);
    let header = v_flex()
        .w_full()
        .gap_2()
        .child(top)
        .child(nav.render("chat-nav", window, cx));

    let mut sidebar = Sidebar::new("chat_sidebar")
        .collapsible(SidebarCollapsible::Icon)
        .collapsed(collapsed)
        .w(metrics::sidebar_width())
        .h_full()
        .border_r_1()
        .border_color(cx.theme().sidebar_border)
        .header(header);
    if !collapsed {
        let (pinned, recents): (Vec<_>, Vec<_>) =
            shell.sessions().iter().partition(|session| session.pinned);
        if !pinned.is_empty() {
            sidebar = sidebar.child(group(copy::PINNED, &pinned, shell, cx));
        }
        if !recents.is_empty() {
            sidebar = sidebar.child(group(copy::RECENTS, &recents, shell, cx));
        }
        sidebar = sidebar.footer(status(shell, cx));
    }
    sidebar
}

/// One group of chat rows.
fn group(
    label: &'static str,
    sessions: &[&SessionSummary],
    shell: &ChatShell,
    cx: &mut Context<ChatShell>,
) -> SidebarGroup<SidebarMenu> {
    let selected = match shell.route() {
        Route::Session(id) => Some(id.clone()),
        Route::Home => None,
    };
    let rows = sessions.iter().map(|session| {
        let id = session.id.clone();
        let running = shell.is_running(&session.id, cx);
        let (view, row, pinned) = (cx.entity(), session.id.clone(), session.pinned);
        SidebarMenuItem::new(session.title.clone())
            .icon(icons::chat())
            .active(selected.as_ref() == Some(&session.id))
            .suffix(move |_, cx| {
                h_flex()
                    .gap_1()
                    .when(running, |this| {
                        this.child(
                            div()
                                .id("running")
                                .size(metrics::status_dot())
                                .rounded_full()
                                .bg(cx.theme().muted_foreground),
                        )
                    })
                    .child(row_menu(view.clone(), row.clone(), pinned))
            })
            .on_click(
                cx.listener(move |shell, _, window, cx| shell.open_session(id.clone(), window, cx)),
            )
    });
    SidebarGroup::new(label).child(SidebarMenu::new().children(rows))
}

/// A row's menu: pin or unpin, rename, delete.
fn row_menu(view: Entity<ChatShell>, id: SharedString, pinned: bool) -> impl IntoElement {
    Button::new(SharedString::from(format!("more-{id}")))
        .ghost()
        .xsmall()
        .icon(icons::more())
        .tooltip(copy::MORE)
        .dropdown_menu(move |menu, _, _| {
            let (pin, rename, delete) = (view.clone(), view.clone(), view.clone());
            let (pin_id, rename_id, delete_id) = (id.clone(), id.clone(), id.clone());
            let pin_label = if pinned { copy::UNPIN } else { copy::PIN };
            menu.item(PopupMenuItem::new(pin_label).on_click(move |_, _, cx| {
                pin.update(cx, |shell, cx| shell.set_pinned(&pin_id, !pinned, cx))
            }))
            .item(
                PopupMenuItem::new(copy::RENAME).on_click(move |_, window, cx| {
                    rename.update(cx, |shell, cx| {
                        shell.open_session(rename_id.clone(), window, cx);
                        shell.begin_rename(window, cx);
                    })
                }),
            )
            .separator()
            .item(
                PopupMenuItem::new(copy::DELETE).on_click(move |_, window, cx| {
                    delete.update(cx, |shell, cx| shell.delete_session(&delete_id, window, cx))
                }),
            )
        })
}

/// The foot of the expanded sidebar: an empty list, a failed list, or a
/// failed write, in words.
fn status(shell: &ChatShell, cx: &mut Context<ChatShell>) -> impl IntoElement {
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
    let list = if let Some(error) = shell.load_error() {
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
    v_flex().w_full().gap_2().children(list).children(
        shell
            .save_error()
            .map(|_| note("save-failed", copy::SAVE_FAILED.into())),
    )
}
