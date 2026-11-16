//! A test hook for a host that refuses a write.

use super::{Inner, ScriptedHost};
use crate::HostError;

impl ScriptedHost {
    /// The next rename, archive or fork fails with `error`, as a host that
    /// cannot write would; the one after goes through.
    pub fn refuse_next_write(&self, error: HostError) {
        self.inner.borrow_mut().refuse_write = Some(error);
    }
}

impl Inner {
    /// Whether a write may go ahead: the refusal, once, if one is set.
    pub(super) fn write(&mut self) -> Result<(), HostError> {
        match self.refuse_write.take() {
            Some(error) => Err(error),
            None => Ok(()),
        }
    }
}
