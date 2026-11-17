// Copyright 2026 Anud Inc.
// SPDX-License-Identifier: GPL-3.0-or-later

//! How the shell fits its window: the sidebar collapses to its rail at
//! narrow widths and expands again when the window grows, unless the person
//! toggled it in between.

use ui::{Pixels, Window};

use super::ChatShell;
use crate::metrics;

/// Whether the window's width leaves room for the expanded sidebar.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) struct Fit {
    pub(super) sidebar: bool,
}

impl Fit {
    pub(super) fn for_width(width: Pixels) -> Self {
        Self {
            sidebar: width >= metrics::sidebar_breakpoint(),
        }
    }
}

impl ChatShell {
    /// Collapse the sidebar when the window gets too narrow for it, and
    /// expand it again when the window grows back. A toggle in between still
    /// wins until the next crossing.
    pub(super) fn fit_to(&mut self, window: &Window) {
        let fit = Fit::for_width(window.viewport_size().width);
        let Some(last) = self.fit.replace(fit) else {
            // The first frame: start from what fits.
            self.sidebar_collapsed |= !fit.sidebar;
            return;
        };
        if last.sidebar != fit.sidebar {
            self.sidebar_collapsed = !fit.sidebar;
        }
    }

    /// Whether the window is too narrow for the expanded sidebar.
    pub fn is_narrow(&self) -> bool {
        self.fit.is_some_and(|fit| !fit.sidebar)
    }

    /// Whether Chat runs on a backend; without one it shows example data.
    pub(crate) fn is_live(&self) -> bool {
        self.backend.is_some()
    }
}
