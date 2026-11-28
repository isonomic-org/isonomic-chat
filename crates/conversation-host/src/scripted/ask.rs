// Copyright 2026 Anud Inc.
// SPDX-License-Identifier: GPL-3.0-or-later

//! A scripted reply that asks the person something, as the engine's
//! `ask_user` does: `tool.call`, then `question`, waiting for
//! [`answer`](crate::ConversationHost::answer) or stop, then `answer` and the
//! call's result.

use super::Inner;
use super::tools::{ScriptedTool, Waiting, result};
use crate::HostError;
use conversation_model::Reply;
use conversation_model::schema::{
    Answer, CallId, Choice, Decider, DeciderScope, Event, Question, QuestionId, SessionId, Value,
};

/// A question a scripted reply puts to the person.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct ScriptedQuestion {
    /// What is asked.
    pub prompt: String,
    /// The choices, as (id, label).
    pub choices: Vec<(String, String)>,
    /// More than one choice may be picked.
    pub multiple: bool,
    /// An answer in the person's own words is welcome.
    pub free_text: bool,
}

impl ScriptedTool {
    /// A call to `ask_user` that puts `prompt` to the person, with these
    /// choices as (id, label), and returns their answer.
    pub fn question(prompt: impl Into<String>, choices: &[(&str, &str)]) -> Self {
        let prompt = prompt.into();
        Self {
            tool: "ask_user".into(),
            input: Value::object([("prompt", Value::String(prompt.clone()))]),
            question: Some(ScriptedQuestion {
                prompt,
                choices: choices
                    .iter()
                    .map(|(id, label)| ((*id).to_owned(), (*label).to_owned()))
                    .collect(),
                multiple: false,
                free_text: true,
            }),
            ..Self::default()
        }
    }
}

impl Inner {
    /// Put a call's question to the person, and wait.
    pub(super) fn ask(&mut self, id: &SessionId, call: CallId, tool: ScriptedTool) {
        let Some(asked) = tool.question.clone() else {
            return;
        };
        let Some(open) = self.sessions.get(id).and_then(|s| s.open.as_ref()) else {
            return;
        };
        let (turn, step) = (open.turn.clone(), open.step.clone());
        self.next_call += 1;
        let question = QuestionId::new(format!("qst_{:04}", self.next_call));
        let event = Event::Question(Question {
            question: question.clone(),
            call: Some(call.clone()),
            prompt: asked.prompt,
            options: asked
                .choices
                .into_iter()
                .map(|(id, label)| Choice { id, label })
                .collect(),
            multiple: asked.multiple,
            free_text: asked.free_text,
        });
        self.emit(id, Some(&turn), Some(&step), event);
        if let Some(open) = self.sessions.get_mut(id).and_then(|s| s.open.as_mut()) {
            open.waiting = Some(Waiting {
                approval: None,
                question: Some(question),
                call,
                tool,
            });
        }
    }

    /// The person's reply to an open question: recorded, then the call's
    /// result carries it to the model.
    pub(super) fn answer(
        &mut self,
        id: &SessionId,
        question: &QuestionId,
        reply: Reply,
    ) -> Result<(), HostError> {
        self.answers
            .push((id.clone(), question.clone(), reply.clone()));
        let open = self.session(id)?.open.as_mut();
        let Some(open) = open.filter(|o| {
            o.waiting
                .as_ref()
                .is_some_and(|w| w.question.as_ref() == Some(question))
        }) else {
            return Err(HostError::new("not_found", "No such question waits."));
        };
        let Some(mut waiting) = open.waiting.take() else {
            return Ok(());
        };
        let (turn, step) = (open.turn.clone(), open.step.clone());
        let (selected, text, skipped) = match reply {
            Reply::Answered { selected, text } => (selected, text, false),
            Reply::Skipped | Reply::MovedOn => (Vec::new(), None, true),
        };
        waiting.tool.output = if skipped {
            "The person did not answer.".to_owned()
        } else {
            let mut said = selected.clone();
            said.extend(text.clone());
            said.join(", ")
        };
        let answered = Event::Answer(Answer {
            question: question.clone(),
            selected,
            text,
            skipped,
            by: Decider {
                scope: DeciderScope::App,
                app: None,
            },
        });
        self.emit(id, Some(&turn), Some(&step), answered);
        let result = result(&waiting.call, &waiting.tool, None);
        self.emit(id, Some(&turn), Some(&step), result);
        Ok(())
    }
}

impl super::ScriptedHost {
    /// Every reply the person sent to a question, oldest first.
    pub fn answers(&self) -> Vec<(SessionId, QuestionId, Reply)> {
        self.inner.borrow().answers.clone()
    }
}
