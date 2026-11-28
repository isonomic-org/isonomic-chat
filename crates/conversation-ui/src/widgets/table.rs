// Copyright 2026 Anud Inc.
// SPDX-License-Identifier: GPL-3.0-or-later

//! `show_table`: a table the person can sort, on SortableTable.

use conversation_model::schema::Value;
use serde::Deserialize;
use ui::extra::sortable_table::{SortableTable, TableColumn, TableRow};
use ui::prelude::*;
use ui::{AnyElement, App};

use super::{BindError, WidgetBinding, WidgetContext, WidgetProps, parse, titled};

pub(super) struct Table;

#[derive(Deserialize)]
struct Input {
    title: Option<String>,
    columns: Vec<Column>,
    rows: Vec<Vec<String>>,
    sort: Option<Sort>,
}

#[derive(Deserialize)]
struct Column {
    title: String,
    #[serde(default)]
    numeric: bool,
    #[serde(default)]
    total: bool,
}

#[derive(Deserialize)]
struct Sort {
    column: usize,
    ascending: Option<bool>,
}

/// The table's props.
#[derive(Clone, Debug, PartialEq)]
pub struct TableProps {
    /// Its title.
    pub title: Option<String>,
    /// Its columns.
    pub columns: Vec<TableColumn>,
    /// Its rows, each as many cells as there are columns.
    pub rows: Vec<TableRow>,
    /// The sort the model asked for.
    pub sort: Option<(usize, bool)>,
}

impl WidgetBinding for Table {
    fn name(&self) -> &'static str {
        "show_table"
    }

    fn props(&self, input: &Value) -> Result<WidgetProps, BindError> {
        let input: Input = parse(input)?;
        let count = input.columns.len();
        if count == 0 {
            return Err(BindError("no columns".into()));
        }
        if let Some(row) = input.rows.iter().position(|row| row.len() != count) {
            return Err(BindError(format!(
                "rows[{row}] has the wrong number of cells"
            )));
        }
        if input.sort.as_ref().is_some_and(|sort| sort.column >= count) {
            return Err(BindError("sort.column is past the last column".into()));
        }
        let columns = input
            .columns
            .iter()
            .map(|column| {
                let mut out = if column.numeric {
                    TableColumn::number(column.title.clone())
                } else {
                    TableColumn::text(column.title.clone())
                };
                // A total only on a number column.
                if column.total && column.numeric {
                    out = out.totalled();
                }
                out
            })
            .collect();
        let rows = input
            .rows
            .iter()
            .enumerate()
            .map(|(index, cells)| {
                TableRow::new(
                    format!("r{index}"),
                    cells.iter().map(String::as_str).collect(),
                )
            })
            .collect();
        Ok(WidgetProps::new(TableProps {
            title: input.title,
            columns,
            rows,
            sort: input
                .sort
                .map(|sort| (sort.column, sort.ascending.unwrap_or(true))),
        }))
    }

    fn render(&self, props: &WidgetProps, cx: &WidgetContext, app: &App) -> AnyElement {
        let Some(props) = props.get::<TableProps>() else {
            return ui::Empty.into_any_element();
        };
        // The person's sort, else the model's. Sorting is the view's own
        // state: nothing goes back to the model.
        let mut table =
            SortableTable::new(cx.id.clone(), props.columns.clone(), props.rows.clone())
                .sort(cx.sort.or(props.sort))
                .empty(cx.copy.empty_table.clone());
        if let Some(on_sort) = cx.on_sort.clone() {
            table = table.on_sort(move |column, window, app| on_sort(column, window, app));
        }
        titled(props.title.as_deref(), table.into_any_element(), app)
    }
}
