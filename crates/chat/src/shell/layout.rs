//! How the shell fits its window: the panels give way at narrow widths and
//! come back when the window grows, unless the person toggled them.

use ui::{Pixels, Window};

use super::ChatShell;
use crate::metrics;

/// Which panels the window's width leaves room for.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) struct Fit {
    pub(super) sidebar: bool,
    pub(super) context_panel: bool,
}

impl Fit {
    pub(super) fn for_width(width: Pixels) -> Self {
        Self {
            sidebar: width >= metrics::sidebar_breakpoint(),
            context_panel: width >= metrics::context_panel_breakpoint(),
        }
    }
}

impl ChatShell {
    /// Hide a panel when the window gets too narrow for it, and show it
    /// again when the window grows back. A toggle in between still wins until
    /// the next crossing.
    pub(super) fn fit_to(&mut self, window: &Window) {
        let fit = Fit::for_width(window.viewport_size().width);
        let Some(last) = self.fit.replace(fit) else {
            // The first frame: start from what fits.
            self.sidebar_collapsed |= !fit.sidebar;
            self.context_panel_open &= fit.context_panel;
            return;
        };
        if last.sidebar != fit.sidebar {
            self.sidebar_collapsed = !fit.sidebar;
        }
        if last.context_panel != fit.context_panel {
            self.context_panel_open = fit.context_panel;
        }
    }

    /// Whether the window is narrow: labels shorten in the header.
    pub fn is_narrow(&self) -> bool {
        self.fit.is_some_and(|fit| !fit.context_panel)
    }

    /// Whether Chat runs on a backend; without one it shows example data.
    pub(crate) fn is_live(&self) -> bool {
        self.backend.is_some()
    }
}
