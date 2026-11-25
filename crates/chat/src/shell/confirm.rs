// Copyright 2026 Anud Inc.
// SPDX-License-Identifier: GPL-3.0-or-later

//! Delete asks first (design 0.2 `SessionMenu/05_DeleteConfirm`, decided for
//! version 1 in issue #49): Isonomic UI's alert dialog, 440 wide, titled
//! "Delete chat?", naming the chat, with Cancel and a destructive Delete.
//! Escape, Cancel or the close button keep the chat.

use ui::component::WindowExt as _;
use ui::component::button::ButtonVariant;
use ui::prelude::*;
use ui::{SharedString, Window};

use super::ChatShell;
use crate::{copy, metrics};

impl ChatShell {
    /// Ask before deleting a chat; it is deleted only on Delete.
    pub fn confirm_delete(
        &mut self,
        id: &SharedString,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let Some(session) = self.sessions.iter().find(|s| &s.id == id) else {
            return;
        };
        let body: SharedString = copy::delete_body(&session.title).into();
        let shell = cx.entity().downgrade();
        let id = id.clone();
        self.confirming = Some(id.clone());
        let cleared = shell.clone();
        window.open_alert_dialog(cx, move |alert, _, _| {
            let shell = shell.clone();
            let id = id.clone();
            let cleared = cleared.clone();
            alert
                .width(metrics::confirm_width())
                .centered()
                .title(copy::DELETE_TITLE)
                .description(body.clone())
                .show_cancel(true)
                .cancel_text(copy::CANCEL)
                .ok_text(copy::DELETE)
                .ok_variant(ButtonVariant::Danger)
                .close_button(true)
                .on_ok(move |_, window, cx| {
                    let id = id.clone();
                    shell
                        .update(cx, |shell, cx| shell.delete_session(&id, window, cx))
                        .ok();
                    true
                })
                .on_close(move |_, _, cx| {
                    cleared
                        .update(cx, |shell, cx| {
                            shell.confirming = None;
                            cx.notify();
                        })
                        .ok();
                })
        });
        cx.notify();
    }

    /// The chat whose delete is being asked about, if any.
    pub fn confirming_delete(&self) -> Option<&SharedString> {
        self.confirming.as_ref()
    }
}
