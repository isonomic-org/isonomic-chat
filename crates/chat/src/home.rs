//! Home: the greeting, the composer and the starter chips (Figma
//! `ChatView/01_NewChat` and `02_Composing`).

use ui::component::button::{Button, ButtonVariants as _};
use ui::component::menu::{DropdownMenu as _, PopupMenuItem};
use ui::component::{ActiveTheme as _, Icon, Sizable as _, h_flex, v_flex};
use ui::prelude::*;
use ui::{AnyElement, App, Entity, TestSupportExt as _, Window, div, rems};

use crate::composer::Composer;
use crate::copy::{self, DayPart};
use crate::{demo, icons, metrics};

/// The greeting row: the mark and a time-of-day greeting.
pub(crate) fn greeting(part: DayPart, examples: bool, cx: &App) -> impl IntoElement {
    h_flex()
        .id("greeting")
        .test_support()
        .gap_3()
        .items_center()
        .child(Icon::new(icons::mark()).size(rems(1.75)))
        .child(
            div()
                .text_3xl()
                .text_color(cx.theme().foreground)
                .child(copy::greeting(part, examples.then_some(demo::FIRST_NAME))),
        )
}

/// One row of starter chips. Each opens its example prompts; picking one
/// fills the composer.
pub(crate) fn starters(composer: &Entity<Composer>) -> impl IntoElement {
    h_flex()
        .id("starters")
        .gap_2()
        .flex_wrap()
        .justify_center()
        .children(copy::STARTERS.iter().enumerate().map(|(index, starter)| {
            let composer = composer.clone();
            Button::new(("starter", index))
                .outline()
                .small()
                .icon(icons::starter(index))
                .label(starter.label)
                .dropdown_menu(move |mut menu, _, _| {
                    for prompt in starter.prompts {
                        let composer = composer.clone();
                        menu = menu.item(PopupMenuItem::new(*prompt).on_click(
                            move |_, window, cx| {
                                composer.update(cx, |composer, cx| {
                                    composer.set_text(prompt, window, cx)
                                });
                            },
                        ));
                    }
                    menu
                })
        }))
}

/// Home, centred in the panel below the header.
pub(crate) fn render(
    part: DayPart,
    examples: bool,
    composer: &Entity<Composer>,
    _: &mut Window,
    cx: &mut Context<impl Sized>,
) -> AnyElement {
    v_flex()
        .id("home")
        .size_full()
        .items_center()
        .justify_center()
        .gap_7()
        .px_6()
        .pb_24()
        .child(greeting(part, examples, cx))
        .child(
            div()
                .w_full()
                .max_w(metrics::content_max_width())
                .child(composer.clone()),
        )
        .child(starters(composer))
        .into_any_element()
}

/// Home when no chat model can answer: in place of a composer that cannot
/// reply, one button asks the window to install the recommended model.
pub(crate) fn render_install(
    model: &conversation_model::ModelOption,
    cx: &mut Context<crate::ChatShell>,
) -> AnyElement {
    let id = model.id.clone();
    let label = copy::install_model(&model.name);
    v_flex()
        .id("home-install")
        .size_full()
        .items_center()
        .justify_center()
        .gap_4()
        .px_6()
        .child(
            div()
                .text_color(cx.theme().muted_foreground)
                .child(copy::NO_MODEL),
        )
        .child(
            Button::new("install-model")
                .primary()
                .label(label)
                .on_click(cx.listener(move |shell, _, _, cx| {
                    shell.install_model(&id);
                    cx.notify();
                })),
        )
        .into_any_element()
}
