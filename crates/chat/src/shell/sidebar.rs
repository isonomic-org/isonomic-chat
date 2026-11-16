//! The app sidebar (design 0.2 `AppSidebar`, `AppSidebarRail`): the toggle
//! and the wordmark, the nav (New chat, Search), then the Pinned and Recents
//! groups of chats. Collapsed, it is a rail of icons: the toggle and the nav
//! stay, the chats and the wordmark go.

use ui::component::button::{Button, ButtonVariants as _};
use ui::component::menu::{DropdownMenu as _, PopupMenuItem};
use ui::component::sidebar::{
    Sidebar, SidebarCollapsible, SidebarGroup, SidebarItem as _, SidebarMenu, SidebarMenuItem,
};
use ui::component::{ActiveTheme as _, Collapsible as _, Sizable as _, h_flex, v_flex};
use ui::extra::wordmark::Wordmark;
use ui::prelude::*;
use ui::{Entity, FontWeight, SharedString, StyleRefinement, Window, div};

use super::{ChatShell, Route};
use crate::demo::SessionSummary;
use crate::{NewChat, SearchChats, ToggleSidebar, copy, icons, metrics};

pub(super) fn render(
    shell: &ChatShell,
    window: &mut Window,
    cx: &mut Context<ChatShell>,
) -> impl IntoElement {
    let collapsed = shell.sidebar_collapsed;
    let toggle_name = if collapsed {
        copy::EXPAND_SIDEBAR
    } else {
        copy::COLLAPSE_SIDEBAR
    };
    let toggle = Button::new("sidebar-toggle")
        .ghost()
        .small()
        .size(metrics::icon_button())
        .icon(icons::toggle_sidebar())
        .tooltip(toggle_name)
        .on_click(|_, window, cx| window.dispatch_action(Box::new(ToggleSidebar), cx));
    let top = h_flex()
        .w_full()
        .gap_2()
        .when(collapsed, |this| this.justify_center())
        .child(toggle)
        .when(!collapsed, |this| {
            // The brand's wordmark: "Isonomic" in the sans, regular, then
            // the product's name in light, at the design's 20.
            this.child(
                div()
                    .id("wordmark")
                    .child(Wordmark::product(copy::PRODUCT).size(metrics::wordmark_size())),
            )
        });

    // New chat stays selected on home, under the palette too; the rail
    // shows no selection.
    let home = shell.route() == &Route::Home && !collapsed;
    let nav_item = |label: &'static str| {
        let item = SidebarMenuItem::new(label).focusable(true);
        if collapsed {
            item.p_1p5().height(metrics::icon_button())
        } else {
            row_padding(item)
        }
    };
    let nav = SidebarMenu::new()
        .child(
            nav_item(copy::NEW_CHAT)
                .icon(icons::new_chat())
                .active(home)
                .on_click(|_, window, cx| window.dispatch_action(Box::new(NewChat), cx)),
        )
        .child(
            nav_item(copy::SEARCH)
                .icon(icons::search())
                .on_click(|_, window, cx| window.dispatch_action(Box::new(SearchChats), cx)),
        )
        .collapsed(collapsed);
    let nav = if collapsed {
        nav.gap_1()
    } else {
        nav.gap(metrics::row_gap())
    };
    let header = v_flex()
        .w_full()
        .map(|this| {
            if collapsed {
                this.gap_4()
            } else {
                this.gap_2()
            }
        })
        .child(top)
        .child(nav.render("chat-nav", window, cx));

    let mut sidebar = Sidebar::new("chat_sidebar")
        .collapsible(SidebarCollapsible::Icon)
        .collapsed(collapsed)
        .w(metrics::sidebar_width())
        .h_full()
        .border_r_1()
        .border_color(cx.theme().sidebar_border)
        // Design 0.2: header pt 10, px 12 (rail pt 12, px 8); content px 8,
        // groups 1 apart.
        .header_padding(if collapsed {
            metrics::rail_padding()
        } else {
            metrics::header_padding()
        })
        .content_padding(metrics::content_padding())
        .footer_padding(metrics::header_padding())
        .content_gap(metrics::row_gap())
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
        let title = session.title.clone();
        let name = if running {
            SharedString::from(copy::running(&title))
        } else {
            title.clone()
        };
        row_padding(SidebarMenuItem::new(session.title.clone()))
            .icon(icons::chat())
            .accessibility_label(name)
            // A selected chat keeps its regular weight; only nav rows go medium.
            .label_style(StyleRefinement::default().font_weight(FontWeight::NORMAL))
            .active(selected.as_ref() == Some(&session.id))
            .suffix(move |_, cx| {
                h_flex()
                    .gap_2p5()
                    .when(running, |this| {
                        this.child(
                            div()
                                .size(metrics::status_box())
                                .flex()
                                .items_center()
                                .justify_center()
                                .child(
                                    div()
                                        .id("running")
                                        .size(metrics::status_dot())
                                        .rounded_full()
                                        .bg(cx.theme().muted_foreground),
                                ),
                        )
                    })
                    .child(row_menu(view.clone(), row.clone(), title.clone(), pinned))
            })
            .on_click(
                cx.listener(move |shell, _, window, cx| shell.open_session(id.clone(), window, cx)),
            )
    });
    let group = SidebarGroup::new(label)
        .padding(metrics::group_label_padding())
        .child(SidebarMenu::new().gap(metrics::row_gap()).children(rows));
    // Pinned folds away with its caret, as the design draws it.
    if label == copy::PINNED {
        group.foldable(true).default_open(true)
    } else {
        group
    }
}

/// A row's padding: 10 on the left, 6 on the right, 10 between icon and
/// label (design 0.2 `SidebarNavItem`, `SidebarSessionRow`).
fn row_padding(item: SidebarMenuItem) -> SidebarMenuItem {
    item.pl_2p5()
        .pr_1p5()
        .gap_x_2p5()
        .height(metrics::row_height())
        .focusable(true)
}

/// A row's menu: open, pin or unpin, rename, delete.
fn row_menu(
    view: Entity<ChatShell>,
    id: SharedString,
    title: SharedString,
    pinned: bool,
) -> impl IntoElement {
    Button::new(SharedString::from(format!("chat-row-menu-{id}")))
        .ghost()
        .xsmall()
        .icon(icons::more())
        .tooltip(copy::MORE)
        .accessibility_label(copy::more_for(&title))
        .dropdown_menu(move |menu, _, _| {
            let (open, pin, rename, delete) =
                (view.clone(), view.clone(), view.clone(), view.clone());
            let (open_id, pin_id, rename_id, delete_id) =
                (id.clone(), id.clone(), id.clone(), id.clone());
            let (pin_label, pin_icon) = if pinned {
                (copy::UNPIN, icons::unpin())
            } else {
                (copy::PIN, icons::pin())
            };
            menu.min_w(metrics::menu_width())
                .item(PopupMenuItem::new(copy::OPEN).icon(icons::open()).on_click(
                    move |_, window, cx| {
                        open.update(cx, |shell, cx| {
                            shell.open_session(open_id.clone(), window, cx)
                        })
                    },
                ))
                .item(
                    PopupMenuItem::new(pin_label)
                        .icon(pin_icon)
                        .on_click(move |_, _, cx| {
                            pin.update(cx, |shell, cx| shell.set_pinned(&pin_id, !pinned, cx))
                        }),
                )
                .item(
                    PopupMenuItem::new(copy::RENAME)
                        .icon(icons::rename())
                        .on_click(move |_, window, cx| {
                            rename.update(cx, |shell, cx| {
                                shell.open_session(rename_id.clone(), window, cx);
                                shell.begin_rename(window, cx);
                            })
                        }),
                )
                .separator()
                .item(
                    PopupMenuItem::new(copy::DELETE)
                        .destructive()
                        .icon(icons::delete())
                        .on_click(move |_, window, cx| {
                            delete.update(cx, |shell, cx| {
                                shell.delete_session(&delete_id, window, cx)
                            })
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
                    h_flex().px_2().child(
                        Button::new("chats-retry")
                            .outline()
                            .small()
                            .label(copy::TRY_AGAIN)
                            .accessibility_label(copy::RETRY_CHATS)
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
