//! What the composer offers next.

/// The composer's mode, derived from the thread.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum InputState {
    /// Ready for a new prompt.
    Ready,
    /// A turn is open: send becomes stop.
    Busy,
    /// The last reply stopped early: the person may continue it or send a
    /// new prompt.
    Continuable,
    /// The session takes no more input.
    Closed,
}

impl InputState {
    /// Whether send is shown as stop.
    pub fn is_busy(self) -> bool {
        self == InputState::Busy
    }

    /// Whether continue is offered.
    pub fn can_continue(self) -> bool {
        self == InputState::Continuable
    }

    /// Whether a new prompt can be sent.
    pub fn can_send(self) -> bool {
        matches!(self, InputState::Ready | InputState::Continuable)
    }
}
