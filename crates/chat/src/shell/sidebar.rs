//! The sidebar: collapse and new-chat buttons, then the chats.
//!
//! The list is a static stand-in for the kit's `SessionList` binding, which
//! will own grouping, pinning, rename and search for every app that lists
//! sessions.

use ui::component::button::{Button, ButtonVariants as _};
use ui::component::sidebar::{Sidebar, SidebarGroup, SidebarMenu, SidebarMenuItem};
use ui::component::{ActiveTheme as _, Icon, Sizable as _, h_flex};
use ui::div;
use ui::prelude::*;

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

    Sidebar::new("chat_sidebar")
        .w(metrics::sidebar_width())
        .h_full()
        .border_r_1()
        .border_color(cx.theme().sidebar_border)
        .header(header)
        .child(SidebarGroup::new(copy::CHATS).child(SidebarMenu::new().children(rows)))
}
