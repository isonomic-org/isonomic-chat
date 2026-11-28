// Copyright 2026 Anud Inc.
// SPDX-License-Identifier: GPL-3.0-or-later

//! Display widgets: the registry against the recorded display fixture, a
//! call it cannot draw, and the view drawing them under the reply.

extern crate ui as gpui;

use conversation_host::{ConversationHost, FixtureHost};
use conversation_model::schema::{CallId, DisplayOutcome, ToolKind, ToolOutcome, Value};
use conversation_model::{Thread, ToolUse};
use conversation_ui::{
    ConversationCopy, ConversationView, Shown, WidgetRegistry, tool_row_id, widget_id,
};
use harness_testkit::fixtures;
use std::rc::Rc;
use ui::test::TestWindowExt as _;
use ui::{
    AppContext as _, Bounds, ElementId, Entity, TestAppContext, WindowBounds, WindowHandle,
    WindowOptions, base::Root, px, size,
};

fn display_thread() -> Thread {
    Thread::from_events(&fixtures::display().events)
}

fn shown(thread: &Thread, registry: &WidgetRegistry) -> Vec<Vec<(String, Shown)>> {
    thread
        .turns()
        .iter()
        .map(|turn| {
            turn.tools
                .iter()
                .filter(|tool| tool.is_display())
                .map(|tool| (tool.call.as_str().to_owned(), registry.resolve(tool)))
                .collect()
        })
        .collect()
}

fn widget(name: &str) -> Shown {
    Shown::Widget(name.into())
}

#[test]
fn the_fixture_shows_each_starter_widget_its_retry_its_fallback_and_an_unknown() {
    let thread = display_thread();
    let shown = shown(&thread, &WidgetRegistry::standard());
    assert_eq!(
        shown[0],
        [
            ("cal_0001".to_owned(), widget("show_table")),
            ("cal_0002".to_owned(), widget("show_steps")),
            ("cal_0003".to_owned(), widget("show_metrics")),
            ("cal_0004".to_owned(), widget("show_chart")),
        ]
    );
    assert_eq!(
        shown[1],
        [
            // Retried: nothing, the next call follows.
            ("cal_0005".to_owned(), Shown::Nothing),
            ("cal_0006".to_owned(), widget("show_table")),
            ("cal_0007".to_owned(), Shown::Nothing),
            // Failed twice: the input's own words.
            (
                "cal_0008".to_owned(),
                Shown::Fallback("Yearly rain: Lisbon 774 mm, Porto 1,254 mm.".into())
            ),
            // A widget this build does not know, with no text to fall back on.
            ("cal_0009".to_owned(), Shown::Generic("show_map".into())),
        ]
    );
    // The turn lists what it draws, in call order, without the retries.
    let drawn: Vec<_> = thread.turns()[1]
        .widgets()
        .map(|tool| tool.call.as_str().to_owned())
        .collect();
    assert_eq!(drawn, ["cal_0006", "cal_0008", "cal_0009"]);
}

fn call(tool: &str, input: &str, display: Option<DisplayOutcome>) -> ToolUse {
    ToolUse {
        call: CallId::new("call_1"),
        tool: tool.into(),
        kind: ToolKind::Display,
        input: serde_json::from_str::<Value>(input).expect("json"),
        input_raw: None,
        effects: Vec::new(),
        outcome: Some(ToolOutcome::Ok),
        output: "shown".into(),
        started_at: 0,
        duration_ms: Some(1),
        display,
    }
}

#[test]
fn bad_data_the_schema_let_through_shows_the_fallback_text() {
    let registry = WidgetRegistry::standard();
    let cases = [
        // A row short of a cell.
        (
            "show_table",
            r#"{"columns":[{"title":"A"},{"title":"B"}],"rows":[["1"]],"fallback_text":"A 1"}"#,
        ),
        // Two current steps.
        (
            "show_steps",
            r#"{"steps":[{"title":"A","state":"current"},{"title":"B","state":"current"}],"fallback_text":"A, B"}"#,
        ),
        // A label twice.
        (
            "show_chart",
            r#"{"kind":"bar","points":[{"label":"A","value":1},{"label":"A","value":2}],"fallback_text":"A 1, A 2"}"#,
        ),
        // A kind from a newer revision.
        (
            "show_chart",
            r#"{"kind":"pie","points":[{"label":"A","value":1},{"label":"B","value":2}],"fallback_text":"A 1, B 2"}"#,
        ),
        // Not even the shape.
        (
            "show_metrics",
            r#"{"metrics":"none","fallback_text":"none"}"#,
        ),
    ];
    for (tool, input) in cases {
        let shown = registry.resolve(&call(tool, input, Some(DisplayOutcome::Shown)));
        assert!(matches!(shown, Shown::Fallback(_)), "{tool}: {shown:?}");
    }
}

#[test]
fn no_usable_text_shows_the_generic_block() {
    let registry = WidgetRegistry::standard();
    let blank = call(
        "show_table",
        r#"{"columns":[],"rows":[],"fallback_text":"  "}"#,
        Some(DisplayOutcome::Shown),
    );
    assert_eq!(
        registry.resolve(&blank),
        Shown::Generic("show_table".into())
    );
    let fallback = call(
        "show_chart",
        r#"{"kind":"pie"}"#,
        Some(DisplayOutcome::Fallback),
    );
    assert_eq!(
        registry.resolve(&fallback),
        Shown::Generic("show_chart".into())
    );
}

#[test]
fn anything_but_a_clear_shown_falls_back() {
    let registry = WidgetRegistry::standard();
    let input = r#"{"kind":"bar","points":[{"label":"A","value":1},{"label":"B","value":2}],"fallback_text":"A 1, B 2"}"#;
    for display in [
        None,
        Some(DisplayOutcome::Fallback),
        Some(DisplayOutcome::Other("later".into())),
    ] {
        let shown = registry.resolve(&call("show_chart", input, display.clone()));
        assert_eq!(shown, Shown::Fallback("A 1, B 2".into()), "{display:?}");
    }
    let mut running = call("show_chart", input, None);
    running.outcome = None;
    assert_eq!(registry.resolve(&running), Shown::Nothing);
}

#[test]
fn an_empty_registry_shows_every_call_as_text() {
    let thread = display_thread();
    for (_, shown) in shown(&thread, &WidgetRegistry::empty())[0].iter() {
        assert!(matches!(shown, Shown::Fallback(_)), "{shown:?}");
    }
    assert_eq!(
        WidgetRegistry::standard().names(),
        ["show_chart", "show_metrics", "show_steps", "show_table"]
    );
}

fn open(
    cx: &mut TestAppContext,
    host: &FixtureHost,
) -> (WindowHandle<Root>, Entity<ConversationView>) {
    cx.update(ui::init);
    let shared: Rc<dyn ConversationHost> = Rc::new(host.clone());
    let session = host.sessions()[0].clone();
    let (window, view) = cx.update(|cx| {
        let (window, view) = ui::open_window(
            WindowOptions {
                window_bounds: Some(WindowBounds::Windowed(Bounds {
                    origin: Default::default(),
                    size: size(px(900.), px(6000.)),
                })),
                ..Default::default()
            },
            cx,
            move |window, cx| {
                cx.new(|cx| {
                    ConversationView::open(shared, session, ConversationCopy::default(), window, cx)
                })
            },
        )
        .expect("open the test window");
        (window.downcast::<Root>().expect("a root view"), view)
    });
    cx.run_until_parked();
    (window, view)
}

fn finds(cx: &mut TestAppContext, window: WindowHandle<Root>, id: ElementId) -> bool {
    cx.run_until_parked();
    cx.update_window(window.into(), |_, window, cx| {
        window.render_frame(cx);
        window.render_frame(cx);
        window.try_find(id).is_some()
    })
    .expect("the window is open")
}

#[ui::test]
fn the_view_draws_widgets_under_the_reply_and_no_tool_rows_for_them(cx: &mut TestAppContext) {
    let fixture = fixtures::display();
    let host = FixtureHost::new(&fixture.events);
    let (window, _) = open(cx, &host);
    for id in [
        "cal_0001", "cal_0002", "cal_0003", "cal_0004", "cal_0006", "cal_0008",
    ] {
        let call = CallId::new(id);
        assert!(finds(cx, window, widget_id(&call)), "no widget for {id}");
        let row = ElementId::NamedChild(tool_row_id(&call).into(), "head".into());
        assert!(!finds(cx, window, row), "a tool row for {id}");
    }
    // Retried calls draw nothing.
    for id in ["cal_0005", "cal_0007"] {
        assert!(
            !finds(cx, window, widget_id(&CallId::new(id))),
            "{id} drawn"
        );
    }
}

#[ui::test]
fn a_sort_is_the_views_own_and_flips_on_the_same_column(cx: &mut TestAppContext) {
    let fixture = fixtures::display();
    let host = FixtureHost::new(&fixture.events);
    let logged = host.events(&host.sessions()[0]).len();
    let (_, view) = open(cx, &host);
    let table = CallId::new("cal_0001");
    let sort = |cx: &mut TestAppContext| cx.update(|cx| view.read(cx).widget_sort(&table));
    assert_eq!(sort(cx), None);
    cx.update(|cx| view.update(cx, |view, cx| view.sort_widget(table.clone(), 2, cx)));
    assert_eq!(sort(cx), Some((2, true)));
    cx.update(|cx| view.update(cx, |view, cx| view.sort_widget(table.clone(), 2, cx)));
    assert_eq!(sort(cx), Some((2, false)));
    cx.update(|cx| view.update(cx, |view, cx| view.sort_widget(table.clone(), 0, cx)));
    assert_eq!(sort(cx), Some((0, true)));
    cx.run_until_parked();
    // Nothing went to the host.
    assert_eq!(host.events(&host.sessions()[0]).len(), logged);
}
