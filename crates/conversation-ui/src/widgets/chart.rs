// Copyright 2026 Anud Inc.
// SPDX-License-Identifier: GPL-3.0-or-later

//! `show_chart`: one series of labelled values, as a bar, line or area
//! chart.

use conversation_model::schema::Value;
use serde::Deserialize;
use ui::component::chart::{AreaChart, BarChart, LineChart};
use ui::prelude::*;
use ui::{AnyElement, App, SharedString, div};

use super::{BindError, WidgetBinding, WidgetContext, WidgetProps, parse, titled};
use crate::metrics::widget_chart_height;

pub(super) struct Chart;

#[derive(Deserialize)]
struct Input {
    title: Option<String>,
    kind: String,
    series: Option<String>,
    points: Vec<Point>,
    unit: Option<String>,
}

/// One labelled value.
#[derive(Clone, Debug, PartialEq, Deserialize)]
pub struct Point {
    /// Its label.
    pub label: String,
    /// Its value.
    pub value: f64,
}

/// How the series is drawn.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ChartKind {
    /// Bars.
    Bar,
    /// A line.
    Line,
    /// A filled line.
    Area,
}

/// The chart's props.
#[derive(Clone, Debug, PartialEq)]
pub struct ChartProps {
    /// Its title.
    pub title: Option<String>,
    /// How it is drawn.
    pub kind: ChartKind,
    /// The series' name.
    pub series: Option<String>,
    /// Its points, labels unique.
    pub points: Vec<Point>,
    /// The values' unit.
    pub unit: Option<String>,
}

impl WidgetBinding for Chart {
    fn name(&self) -> &'static str {
        "show_chart"
    }

    fn props(&self, input: &Value) -> Result<WidgetProps, BindError> {
        let input: Input = parse(input)?;
        let kind = match input.kind.as_str() {
            "bar" => ChartKind::Bar,
            "line" => ChartKind::Line,
            "area" => ChartKind::Area,
            other => return Err(BindError(format!("unknown kind {other:?}"))),
        };
        if input.points.len() < 2 {
            return Err(BindError("fewer than two points".into()));
        }
        let mut labels: Vec<&str> = input.points.iter().map(|p| p.label.as_str()).collect();
        labels.sort_unstable();
        if labels.windows(2).any(|pair| pair[0] == pair[1]) {
            return Err(BindError("labels repeat".into()));
        }
        if input.points.iter().any(|p| !p.value.is_finite()) {
            return Err(BindError("a value is not a number".into()));
        }
        Ok(WidgetProps::new(ChartProps {
            title: input.title,
            kind,
            series: input.series,
            points: input.points,
            unit: input.unit,
        }))
    }

    fn render(&self, props: &WidgetProps, cx: &WidgetContext, app: &App) -> AnyElement {
        let Some(props) = props.get::<ChartProps>() else {
            return ui::Empty.into_any_element();
        };
        let unit = props.unit.clone().unwrap_or_default();
        let format = move |value: f64| -> SharedString {
            let value = if value.fract() == 0.0 {
                format!("{value:.0}")
            } else {
                format!("{value:.1}")
            };
            if unit.is_empty() {
                value.into()
            } else {
                format!("{value} {unit}").into()
            }
        };
        let name = props.series.clone().unwrap_or_default();
        let points = props.points.clone();
        let appear = !cx.replayed;
        let chart = match props.kind {
            ChartKind::Bar => BarChart::new(points)
                .id(cx.id.clone())
                .band(|p: &Point| SharedString::from(p.label.clone()))
                .value(|p: &Point| p.value)
                .name(name)
                .value_axis(true)
                .value_tick_format(format)
                .appear(appear)
                .interactive(true)
                .into_any_element(),
            ChartKind::Line => LineChart::new(points)
                .id(cx.id.clone())
                .x(|p: &Point| SharedString::from(p.label.clone()))
                .y(|p: &Point| p.value)
                .name(name)
                .y_axis(true)
                .y_tick_format(format)
                .appear(appear)
                .interactive(true)
                .into_any_element(),
            ChartKind::Area => AreaChart::new(points)
                .id(cx.id.clone())
                .x(|p: &Point| SharedString::from(p.label.clone()))
                .y(|p: &Point| p.value)
                .name(name)
                .y_axis(true)
                .y_tick_format(format)
                .appear(appear)
                .interactive(true)
                .into_any_element(),
        };
        let body = div()
            .w_full()
            .h(widget_chart_height())
            .child(chart)
            .into_any_element();
        titled(props.title.as_deref(), body, app)
    }
}
