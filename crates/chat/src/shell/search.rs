//! The search palette (design 0.2 `SearchPalette`): Isonomic UI's command
//! palette over the shell, listing the chats by title and the New chat
//! action. Typing narrows them; Enter opens; Escape closes.

use ui::component::ActiveTheme as _;
use ui::extra::command_palette::{CommandPalette, PaletteCommand, PaletteEvent};
use ui::prelude::*;
use ui::{AnyElement, Entity, Focusable as _, Subscription, Window, div};

use super::ChatShell;
use crate::{copy, metrics};

/// The id of the New chat command; every other id is a chat's.
const NEW_CHAT: &str = "action:new-chat";

/// The open palette.
pub(super) struct Search {
    palette: Entity<CommandPalette>,
    _events: Subscription,
}

impl ChatShell {
    /// Whether the search palette is open.
    pub fn is_searching(&self) -> bool {
        self.search.is_some()
    }

    /// Open the search palette over the shell, focused.
    pub fn open_search(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let mut commands: Vec<PaletteCommand> = self
            .sessions()
            .iter()
            .map(|session| {
                PaletteCommand::new(session.id.clone(), copy::CHATS, session.title.clone())
            })
            .collect();
        commands.push(PaletteCommand::new(NEW_CHAT, copy::ACTIONS, copy::NEW_CHAT));
        let palette = cx.new(|cx| CommandPalette::new(commands, window, cx));
        let events = cx.subscribe_in(&palette, window, |this, _, event, window, cx| match event {
            PaletteEvent::Run(id) if id.as_ref() == NEW_CHAT => {
                this.close_search(window, cx);
                this.new_chat(window, cx);
            }
            PaletteEvent::Run(id) => {
                this.close_search(window, cx);
                this.open_session(id.clone(), window, cx);
            }
            PaletteEvent::Dismiss => this.close_search(window, cx),
        });
        let focus = palette.read(cx).focus_handle(cx);
        window.focus(&focus, cx);
        self.search = Some(Search {
            palette,
            _events: events,
        });
        cx.notify();
    }

    /// Close the search palette and give focus back to the shell.
    pub fn close_search(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        if self.search.take().is_some() {
            window.focus(&self.focus_handle, cx);
            cx.notify();
        }
    }

    /// The palette over the whole shell, on a scrim that closes it.
    pub(super) fn render_search(&self, cx: &mut Context<Self>) -> Option<AnyElement> {
        let search = self.search.as_ref()?;
        Some(
            div()
                .id("search-scrim")
                .absolute()
                .inset_0()
                .flex()
                .justify_center()
                .pt(metrics::palette_top())
                .bg(cx.theme().overlay)
                .on_click(cx.listener(|this, _, window, cx| this.close_search(window, cx)))
                .child(
                    div()
                        .id("search-palette")
                        .w(metrics::palette_width())
                        .max_w_full()
                        .on_click(|_, _, cx| cx.stop_propagation())
                        .child(search.palette.clone()),
                )
                .into_any_element(),
        )
    }
}
