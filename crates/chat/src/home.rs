//! Home (design 0.2 `HomeView/01_Home`): the greeting, the composer and,
//! under it, the footer with the model picker. A 720 column, centred, lifted
//! above true centre. No suggestion chips.

use ui::component::button::{Button, ButtonVariants as _};
use ui::component::empty::{
    Empty, EmptyContent, EmptyDescription, EmptyHeader, EmptyMedia, EmptyMediaVariant, EmptyTitle,
};
use ui::component::{ActiveTheme as _, Disableable as _, Icon, Sizable as _, h_flex, v_flex};
use ui::extra::model_picker::{ModelOption as PickerOption, ModelPicker};
use ui::prelude::*;
use ui::{AnyElement, App, Entity, SharedString, TestSupportExt as _, Window, div, rems};

use crate::composer::Composer;
use crate::copy::{self, DayPart};
use crate::{ChatShell, demo, icons, metrics};
use conversation_model::{Availability, ModelCatalog};

/// The greeting row: the sparkle and a time-of-day greeting.
pub(crate) fn greeting(part: DayPart, examples: bool, cx: &App) -> impl IntoElement {
    h_flex()
        .id("greeting")
        .test_support()
        .gap_3p5()
        .items_center()
        .child(Icon::new(icons::greeting()).size(rems(2.)))
        .child(
            div()
                .text_3xl()
                .text_color(cx.theme().foreground)
                .child(copy::greeting(part, examples.then_some(demo::FIRST_NAME))),
        )
}

/// Home, in the panel below the header.
pub(crate) fn render(
    shell: &ChatShell,
    part: DayPart,
    composer: &Entity<Composer>,
    _: &mut Window,
    cx: &mut Context<ChatShell>,
) -> AnyElement {
    v_flex()
        .id("home")
        .size_full()
        .items_center()
        .justify_center()
        .gap_5()
        .px_4()
        .pb(metrics::home_lift())
        .child(greeting(part, !shell.is_live(), cx))
        .child(
            div()
                .w_full()
                .max_w(metrics::content_max_width())
                .child(composer.clone()),
        )
        .child(
            // Always its full height, so the group sits where the design puts
            // it whether or not there is a picker.
            h_flex()
                .id("composer-footer")
                .w_full()
                .max_w(metrics::content_max_width())
                .h(metrics::footer_height())
                .px_2()
                .justify_end()
                .children(footer(shell, cx)),
        )
        .into_any_element()
}

/// The footer's control: the picker once there are models, or a way to list
/// them again when that failed.
fn footer(shell: &ChatShell, cx: &mut Context<ChatShell>) -> Option<AnyElement> {
    if shell.models_failed() {
        return Some(
            h_flex()
                .gap_2()
                .text_sm()
                .text_color(cx.theme().muted_foreground)
                .child(copy::MODELS_FAILED)
                .child(
                    Button::new("models-retry")
                        .ghost()
                        .small()
                        .label(copy::TRY_AGAIN)
                        .on_click(cx.listener(|shell, _, _, cx| shell.reload_models(cx))),
                )
                .into_any_element(),
        );
    }
    let catalog = shell.catalog().filter(|c| !c.options().is_empty())?;
    Some(picker(catalog, shell.model(), cx))
}

/// The model picker in the footer: the catalog's models, each noted as
/// recommended, not installed or downloading. Picking one that is not
/// installed asks the window to install it.
fn picker(catalog: &ModelCatalog, picked: Option<&str>, cx: &mut Context<ChatShell>) -> AnyElement {
    let selected = picked
        .filter(|id| catalog.get(id).is_some())
        .or_else(|| catalog.choose(None).map(|o| o.id.as_str()))
        .map(str::to_owned);
    let recommended = catalog.recommended().map(|o| o.id.clone());
    let mut picker = ModelPicker::new("model-picker")
        .align_end(true)
        .w(metrics::model_picker_width());
    for option in catalog.options() {
        let note = match option.availability {
            Availability::NotInstalled => Some(copy::MODEL_NOT_INSTALLED),
            Availability::Downloading => Some(copy::MODEL_DOWNLOADING),
            Availability::Unavailable => Some(copy::MODEL_UNAVAILABLE),
            _ if recommended.as_deref() == Some(option.id.as_str()) => {
                Some(copy::MODEL_RECOMMENDED)
            }
            _ => None,
        };
        let mut entry = PickerOption::new(option.id.clone(), option.name.clone());
        if let Some(note) = note {
            entry = entry.tagline(note);
        }
        picker = picker.model(entry);
    }
    if let Some(selected) = selected {
        picker = picker.value(selected);
    }
    let view = cx.entity();
    picker
        .on_change(move |id: &SharedString, _, cx| {
            view.update(cx, |shell, cx| shell.choose_model(id.to_string(), cx))
        })
        .into_any_element()
}

/// Home when no chat model can answer: in place of a composer that cannot
/// reply, an empty state whose one button asks the window to install the
/// recommended model.
pub(crate) fn render_install(
    model: &conversation_model::ModelOption,
    asked: bool,
    cx: &mut Context<ChatShell>,
) -> AnyElement {
    let id = model.id.clone();
    // Once asked, the button says the model is on its way and stands still.
    let label = if asked {
        copy::MODEL_DOWNLOADING.to_owned()
    } else {
        copy::install_model(&model.name)
    };
    v_flex()
        .id("home-install")
        .size_full()
        .items_center()
        .justify_center()
        .px_4()
        .pb(metrics::home_lift())
        .child(
            // Design 0.2 `EmptyState`: 360 wide, 28 / 24 padding, 8 gap.
            Empty::new()
                .w(metrics::empty_width())
                .gap_2()
                .py(metrics::empty_padding_y())
                .px_6()
                .header(
                    EmptyHeader::new()
                        .max_w(metrics::empty_text_width())
                        .media(
                            EmptyMedia::new()
                                .with_variant(EmptyMediaVariant::Icon)
                                .size(metrics::empty_tile())
                                .child(Icon::new(icons::greeting()).size(metrics::empty_icon())),
                        )
                        .title(EmptyTitle::new().child(copy::NO_MODEL_TITLE))
                        .description(EmptyDescription::new().child(copy::NO_MODEL)),
                )
                .content(
                    EmptyContent::new().child(
                        Button::new("install-model")
                            .primary()
                            .icon(icons::install())
                            .label(label)
                            .disabled(asked)
                            .on_click(cx.listener(move |shell, _, _, cx| {
                                shell.install_model(&id);
                                cx.notify();
                            })),
                    ),
                ),
        )
        .into_any_element()
}
