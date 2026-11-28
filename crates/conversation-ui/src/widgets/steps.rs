// Copyright 2026 Anud Inc.
// SPDX-License-Identifier: GPL-3.0-or-later

//! `show_steps`: where each step of a plan stands, on StepTrail. Display
//! only: never a gate, an ask or an approval.

use conversation_model::schema::Value;
use serde::Deserialize;
use ui::extra::step_trail::{StepTrail, SubstepState, TrailState, TrailStep, TrailSubstep};
use ui::prelude::*;
use ui::{AnyElement, App};

use super::{BindError, WidgetBinding, WidgetContext, WidgetProps, parse};

pub(super) struct Steps;

#[derive(Clone, Deserialize)]
struct Input {
    title: Option<String>,
    steps: Vec<Step>,
}

/// One step, as the model wrote it.
#[derive(Clone, Debug, PartialEq, Deserialize)]
pub struct Step {
    /// What the step is.
    pub title: String,
    /// done, current, pending (the default) or failed.
    pub state: Option<String>,
    /// A line about it.
    pub detail: Option<String>,
    /// Its sub-steps.
    #[serde(default)]
    pub substeps: Vec<String>,
}

/// The trail's props.
#[derive(Clone, Debug, PartialEq)]
pub struct StepsProps {
    /// Its title.
    pub title: Option<String>,
    /// Its steps.
    pub steps: Vec<Step>,
}

fn state(state: Option<&str>) -> Result<TrailState, BindError> {
    match state.unwrap_or("pending") {
        "done" => Ok(TrailState::Done),
        "current" => Ok(TrailState::Running),
        "pending" => Ok(TrailState::Pending),
        "failed" => Ok(TrailState::Failed),
        other => Err(BindError(format!("unknown state {other:?}"))),
    }
}

impl WidgetBinding for Steps {
    fn name(&self) -> &'static str {
        "show_steps"
    }

    fn props(&self, input: &Value) -> Result<WidgetProps, BindError> {
        let input: Input = parse(input)?;
        if input.steps.is_empty() {
            return Err(BindError("no steps".into()));
        }
        for step in &input.steps {
            state(step.state.as_deref())?;
        }
        let current = input
            .steps
            .iter()
            .filter(|step| step.state.as_deref() == Some("current"))
            .count();
        if current > 1 {
            return Err(BindError("more than one current step".into()));
        }
        Ok(WidgetProps::new(StepsProps {
            title: input.title,
            steps: input.steps,
        }))
    }

    fn render(&self, props: &WidgetProps, cx: &WidgetContext, _: &App) -> AnyElement {
        let Some(props) = props.get::<StepsProps>() else {
            return ui::Empty.into_any_element();
        };
        let mut trail = StepTrail::new(cx.id.clone());
        if let Some(title) = &props.title {
            trail = trail.title(title.clone());
        }
        for (index, step) in props.steps.iter().enumerate() {
            let state = state(step.state.as_deref()).unwrap_or(TrailState::Pending);
            let mut out = TrailStep::new(format!("s{index}"), step.title.clone()).state(state);
            // The trail's status line needs a start time for its live
            // timer, which a display widget has not got: the detail shows
            // as the step's first line inside it instead.
            if let Some(detail) = step.detail.as_ref().filter(|d| !d.trim().is_empty()) {
                out = out.substep(TrailSubstep::new(detail.clone()).state(SubstepState::Pending));
            }
            for substep in &step.substeps {
                out = out.substep(TrailSubstep::new(substep.clone()));
            }
            trail = trail.step(out);
        }
        trail.into_any_element()
    }
}
