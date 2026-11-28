// Copyright 2026 Anud Inc.
// SPDX-License-Identifier: GPL-3.0-or-later

//! The question the model asked the person: one card above the composer,
//! and the person's reply through the host. Nothing is sent without their
//! input; anything unclear is no answer.

use super::ConversationView;
use conversation_model::schema::QuestionId;
use conversation_model::{Ask, Reply};
use ui::extra::question_card::{
    Question, QuestionAnswer, QuestionCard, QuestionCardEvent, QuestionOption,
};
use ui::prelude::*;
use ui::{Entity, Subscription, Window};

/// The card showing an open question.
pub(super) struct AskCard {
    pub(super) id: QuestionId,
    pub(super) card: Entity<QuestionCard>,
    _events: Subscription,
}

/// The person's reply for what the card reports; `None` when it is not a
/// clear answer, which is never sent.
pub(crate) fn reply_for(ask: &Ask, answer: &QuestionAnswer) -> Option<Reply> {
    match answer {
        QuestionAnswer::Option(id) => ask
            .choices
            .iter()
            .any(|choice| choice.id == id.as_ref())
            .then(|| Reply::Answered {
                selected: vec![id.to_string()],
                text: None,
            }),
        QuestionAnswer::Text(text) if !text.trim().is_empty() => Some(Reply::Answered {
            selected: Vec::new(),
            text: Some(text.trim().to_owned()),
        }),
        // An ask never decides an approval, and empty words are no answer.
        _ => None,
    }
}

impl ConversationView {
    /// The question the last turn waits on, if any.
    pub fn open_ask(&self) -> Option<Ask> {
        let thread = self.thread.borrow();
        thread
            .turns()
            .last()
            .and_then(|turn| turn.open_ask())
            .cloned()
    }

    /// Reply to the open question, as the person chose. A reply to a
    /// question that is not open, or one already on its way, does nothing.
    pub fn answer(&mut self, reply: Reply, cx: &mut Context<Self>) {
        let (Some(session), Some(ask)) = (self.session(), self.open_ask()) else {
            return;
        };
        if self.answering.contains(&ask.id) {
            return;
        }
        self.answering.push(ask.id.clone());
        self.rows_changed(cx);
        let request = self.host.answer(&session, &ask.id, reply);
        let id = ask.id.clone();
        let task = cx.spawn(async move |this, cx| {
            let result = request.await;
            let _ = this.update(cx, |this, cx| {
                this.answering.retain(|a| a != &id);
                if let Err(error) = result {
                    // The card comes back to be answered again.
                    this.ask_card = None;
                    this.refused(error, cx);
                }
                this.flush_queued(cx);
                this.rows_changed(cx);
            });
        });
        self._decisions.push(task);
    }

    /// A message sent while a question is open: the question goes back as
    /// moved on, and the message is sent once the turn has ended.
    pub(super) fn move_on(&mut self, text: String, cx: &mut Context<Self>) {
        self.queued = Some(text);
        self.answer(Reply::MovedOn, cx);
    }

    /// Send a message held for the end of the turn, once it can go.
    pub(super) fn flush_queued(&mut self, cx: &mut Context<Self>) {
        if self.input_state().can_send()
            && let Some(text) = self.queued.take()
        {
            self.send(text, cx);
        }
    }

    /// The card for the open question: made when a question opens, gone
    /// when it is answered.
    pub(super) fn ask_card(
        &mut self,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Option<Entity<QuestionCard>> {
        let ask = self
            .open_ask()
            .filter(|ask| !self.answering.contains(&ask.id));
        let Some(ask) = ask else {
            self.ask_card = None;
            return None;
        };
        if self.ask_card.as_ref().is_some_and(|card| card.id == ask.id) {
            return self.ask_card.as_ref().map(|card| card.card.clone());
        }
        let mut question = Question::new(ask.id.as_str().to_owned(), ask.prompt.clone());
        for choice in &ask.choices {
            question =
                question.option(QuestionOption::new(choice.id.clone(), choice.label.clone()));
        }
        let card = cx.new(|cx| QuestionCard::new(window, cx).questions([question]));
        let events = cx.subscribe_in(&card, window, move |this, _, event, _, cx| {
            if let QuestionCardEvent::Answered { answer, .. } = event {
                match this.open_ask().and_then(|ask| reply_for(&ask, answer)) {
                    Some(reply) => this.answer(reply, cx),
                    // Not a clear answer: nothing is sent, and the card
                    // asks again.
                    None => {
                        this.ask_card = None;
                        cx.notify();
                    }
                }
            }
            // "Not now" folds the card and sends nothing.
        });
        self.ask_card = Some(AskCard {
            id: ask.id,
            card: card.clone(),
            _events: events,
        });
        Some(card)
    }
}

#[cfg(test)]
mod tests {
    use super::reply_for;
    use conversation_model::schema::QuestionId;
    use conversation_model::{Ask, AskChoice, AskState, Reply};
    use ui::extra::question_card::QuestionAnswer;

    fn ask() -> Ask {
        Ask {
            id: QuestionId::new("q1"),
            call: None,
            prompt: "Which room?".into(),
            choices: vec![AskChoice {
                id: "a".into(),
                label: "The small one".into(),
            }],
            multiple: false,
            free_text: true,
            state: AskState::Open,
        }
    }

    #[test]
    fn only_a_clear_answer_is_sent() {
        let ask = ask();
        assert_eq!(
            reply_for(&ask, &QuestionAnswer::Option("a".into())),
            Some(Reply::Answered {
                selected: vec!["a".into()],
                text: None
            })
        );
        assert_eq!(
            reply_for(&ask, &QuestionAnswer::Text("  the big one ".into())),
            Some(Reply::Answered {
                selected: Vec::new(),
                text: Some("the big one".into())
            })
        );
        // A choice it never offered, empty words, and an approval are not
        // answers.
        assert_eq!(reply_for(&ask, &QuestionAnswer::Option("z".into())), None);
        assert_eq!(reply_for(&ask, &QuestionAnswer::Text("   ".into())), None);
        assert_eq!(reply_for(&ask, &QuestionAnswer::Approve), None);
        assert_eq!(reply_for(&ask, &QuestionAnswer::Decline), None);
    }
}
