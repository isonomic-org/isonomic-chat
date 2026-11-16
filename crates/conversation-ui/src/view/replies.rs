//! The text view state behind each reply, owned by the conversation view.
//!
//! A reply is drawn from a state the view keeps per turn, not one the text
//! view finds by its element id each frame, so a row that leaves the list's
//! layout, or moves when a later turn arrives, keeps the text it shows. A
//! reply that has ended is drawn from a state parsed once from its whole
//! text, so an earlier answer never shows only the pieces it streamed in.

use conversation_model::Thread;
use ui::component::text::TextViewState;
use ui::{AppContext as _, Context, Entity};

use crate::ConversationView;
use crate::turn::shown;

struct Reply {
    state: Entity<TextViewState>,
    /// The text the state was last given.
    text: String,
    /// Whether the state was parsed from the reply once it had ended.
    ended: bool,
}

/// One state per turn, by index.
#[derive(Default)]
pub(crate) struct Replies {
    replies: Vec<Option<Reply>>,
}

impl Replies {
    /// Brings every reply's state in step with `thread` and returns them by
    /// turn index (`None` for a turn with nothing to draw yet).
    pub(crate) fn sync(
        &mut self,
        thread: &Thread,
        cx: &mut Context<ConversationView>,
    ) -> Vec<Option<Entity<TextViewState>>> {
        let turns = thread.turns();
        self.replies.truncate(turns.len());
        self.replies.resize_with(turns.len(), || None);
        for (slot, turn) in self.replies.iter_mut().zip(turns) {
            let text = shown(&turn.reply()).to_owned();
            let ended = !turn.state.is_open();
            if text.is_empty() {
                *slot = None;
                continue;
            }
            match slot {
                Some(reply) if reply.text == text && reply.ended == ended => {}
                // Still being written: extend the state, as the text view
                // streams.
                Some(reply) if !ended && !reply.ended => {
                    reply
                        .state
                        .update(cx, |state, cx| state.set_text(&text, cx));
                    reply.text = text;
                }
                // New, or ended: one parse of the whole text.
                _ => {
                    let state = cx.new(|cx| TextViewState::markdown(&text, cx));
                    *slot = Some(Reply { state, text, ended });
                }
            }
        }
        self.replies
            .iter()
            .map(|reply| reply.as_ref().map(|reply| reply.state.clone()))
            .collect()
    }
}
