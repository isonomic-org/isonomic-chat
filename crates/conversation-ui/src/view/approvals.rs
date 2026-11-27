// Copyright 2026 Anud Inc.
// SPDX-License-Identifier: GPL-3.0-or-later

//! The person's answer to an approval a reply waits on.

use super::ConversationView;
use conversation_model::schema::{ApprovalId, Decision};
use ui::prelude::*;

impl ConversationView {
    /// Answer an approval the session waits on. Only a clear allow allows.
    /// The card stays, busy, until the host records the answer; if the
    /// answer cannot be sent, the card asks again and the error is
    /// reported.
    pub fn decide(&mut self, approval: ApprovalId, decision: Decision, cx: &mut Context<Self>) {
        let Some(session) = self.session() else {
            return;
        };
        if self.deciding.contains(&approval) {
            return;
        }
        let decision = match decision {
            Decision::Allow => Decision::Allow,
            _ => Decision::Deny,
        };
        self.deciding.push(approval.clone());
        self.rows_changed(cx);
        let request = self.host.decide(&session, &approval, decision);
        let task = cx.spawn(async move |this, cx| {
            let result = request.await;
            let _ = this.update(cx, |this, cx| {
                this.deciding.retain(|id| id != &approval);
                if let Err(error) = result {
                    this.refused(error, cx);
                }
                this.rows_changed(cx);
            });
        });
        self._decisions.push(task);
    }

    /// Approvals whose answer is on its way.
    pub(super) fn deciding(&self) -> &[ApprovalId] {
        &self.deciding
    }
}
