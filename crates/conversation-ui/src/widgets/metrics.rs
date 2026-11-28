// Copyright 2026 Anud Inc.
// SPDX-License-Identifier: GPL-3.0-or-later

//! `show_metrics`: headline figures side by side, on StatGrid.

use conversation_model::schema::Value;
use serde::Deserialize;
use ui::extra::stat_grid::{Stat, StatGrid};
use ui::prelude::*;
use ui::{AnyElement, App};

use super::{BindError, WidgetBinding, WidgetContext, WidgetProps, parse};

pub(super) struct Metrics;

#[derive(Deserialize)]
struct Input {
    title: Option<String>,
    metrics: Vec<Metric>,
}

/// One figure, as the model wrote it.
#[derive(Clone, Debug, PartialEq, Deserialize)]
pub struct Metric {
    /// What it measures.
    pub label: String,
    /// The figure, written out.
    pub value: String,
    /// The change, in percent.
    pub change_percent: Option<f32>,
    /// Whether a fall is good news.
    #[serde(default)]
    pub lower_is_better: bool,
    /// Earlier values, oldest first.
    #[serde(default)]
    pub trend: Vec<f32>,
    /// A line under it.
    pub hint: Option<String>,
}

/// The grid's props.
#[derive(Clone, Debug, PartialEq)]
pub struct MetricsProps {
    /// Its title.
    pub title: Option<String>,
    /// Its figures.
    pub metrics: Vec<Metric>,
}

impl WidgetBinding for Metrics {
    fn name(&self) -> &'static str {
        "show_metrics"
    }

    fn props(&self, input: &Value) -> Result<WidgetProps, BindError> {
        let input: Input = parse(input)?;
        if input.metrics.is_empty() {
            return Err(BindError("no metrics".into()));
        }
        Ok(WidgetProps::new(MetricsProps {
            title: input.title,
            metrics: input.metrics,
        }))
    }

    fn render(&self, props: &WidgetProps, cx: &WidgetContext, _: &App) -> AnyElement {
        let Some(props) = props.get::<MetricsProps>() else {
            return ui::Empty.into_any_element();
        };
        let stats = props
            .metrics
            .iter()
            .enumerate()
            .map(|(index, metric)| {
                let mut stat = Stat::new(
                    format!("m{index}"),
                    metric.label.clone(),
                    metric.value.clone(),
                );
                if let Some(change) = metric.change_percent {
                    stat = stat.delta(change);
                }
                if metric.lower_is_better {
                    stat = stat.lower_is_better();
                }
                if metric.trend.len() >= 2 {
                    stat = stat.trend(metric.trend.iter().copied());
                }
                if let Some(hint) = &metric.hint {
                    stat = stat.hint(hint.clone());
                }
                stat
            })
            .collect();
        let columns = props.metrics.len().min(3) as u16;
        let mut grid = StatGrid::new(cx.id.clone(), stats).columns(columns);
        if let Some(title) = &props.title {
            grid = grid.title(title.clone());
        }
        if props.metrics.iter().all(|metric| metric.trend.len() < 2) {
            grid = grid.without_trends();
        }
        grid.into_any_element()
    }
}
