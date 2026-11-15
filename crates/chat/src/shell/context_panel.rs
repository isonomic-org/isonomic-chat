//! The context panel on the right (Figma `ContextSidebar`): what is in scope
//! for this chat. Chat owns the layout and when it opens; the rows become the
//! kit's `ContextItem` binding (isonomic-ui #208 is the row it needs).

use ui::component::button::{Button, ButtonVariants as _};
use ui::component::tab::{Tab, TabBar};
use ui::component::{
    ActiveTheme as _, Disableable as _, Icon, Sizable as _, StyledExt as _, h_flex, v_flex,
};
use ui::prelude::*;
use ui::{AnyElement, App, SharedString, TestSupportExt as _, div};

use super::{ChatShell, ContextTab};
use crate::{copy, demo, icons, metrics};

pub(super) fn render(shell: &ChatShell, cx: &mut Context<ChatShell>) -> impl IntoElement {
    let theme = cx.theme();
    let selected = match shell.context_tab {
        ContextTab::Context => 0,
        ContextTab::Tools => 1,
    };
    let view = cx.entity();
    let body = match shell.context_tab {
        ContextTab::Context => context(cx),
        ContextTab::Tools => empty_line(copy::NO_TOOLS, cx),
    };

    v_flex()
        .id("context_sidebar")
        .test_support()
        .w(metrics::context_panel_width())
        .h_full()
        .flex_shrink_0()
        .border_l_1()
        .border_color(theme.border)
        .bg(theme.background)
        .child(
            div().px_4().pt_2().child(
                TabBar::new("context_navigation")
                    .underline()
                    .selected_index(selected)
                    .on_click(move |index, _, cx| {
                        view.update(cx, |shell, cx| {
                            shell.context_tab = if *index == 0 {
                                ContextTab::Context
                            } else {
                                ContextTab::Tools
                            };
                            cx.notify();
                        })
                    })
                    .child(tab(copy::CONTEXT, icons::context()))
                    .child(tab(copy::TOOLS, icons::tools())),
            ),
        )
        .child(div().p_4().child(body))
}

/// A labelled tab with a leading icon. Isonomic UI draws a tab with
/// `icon` as icon-only, so the icon goes in the prefix.
fn tab(label: &'static str, icon: ui::assets::IconName) -> Tab {
    Tab::new().label(label).prefix(Icon::new(icon).small())
}

fn context(cx: &App) -> AnyElement {
    let rows = demo::connections().into_iter().map(|connection| {
        row(
            Icon::new(connection.icon),
            connection.name,
            connection.detail,
            cx,
        )
    });
    v_flex()
        .gap_4()
        .child(section(
            "files",
            copy::FILES,
            empty_line(copy::NO_FILES, cx),
            cx,
        ))
        .child(section(
            "connections",
            copy::CONNECTIONS,
            v_flex().gap_1().children(rows).into_any_element(),
            cx,
        ))
        .into_any_element()
}

/// A titled section with an add button and a hairline under it.
fn section(id: &'static str, title: &'static str, body: AnyElement, cx: &App) -> AnyElement {
    let theme = cx.theme();
    v_flex()
        .id(SharedString::from(format!("{id}_section")))
        .gap_2()
        .pb_4()
        .border_b_1()
        .border_color(theme.border)
        .child(
            h_flex()
                .child(div().flex_1().text_sm().font_medium().child(title))
                .child(
                    Button::new(id)
                        .ghost()
                        .xsmall()
                        .icon(icons::add())
                        .tooltip(copy::ADD)
                        .disabled(true),
                ),
        )
        .child(body)
        .into_any_element()
}

/// One item in scope: an icon, a name and a muted detail line. A stand-in
/// for the kit's `ContextItem`.
fn row(icon: Icon, name: SharedString, detail: SharedString, cx: &App) -> impl IntoElement {
    let theme = cx.theme();
    h_flex()
        .gap_3()
        .py_1()
        .child(icon.text_color(theme.muted_foreground))
        .child(
            v_flex()
                .min_w_0()
                .text_sm()
                .child(div().font_medium().truncate().child(name))
                .child(
                    div()
                        .text_color(theme.muted_foreground)
                        .truncate()
                        .child(detail),
                ),
        )
}

fn empty_line(text: &'static str, cx: &App) -> AnyElement {
    div()
        .text_sm()
        .text_color(cx.theme().muted_foreground)
        .child(text)
        .into_any_element()
}
