//! The status bar against the scripted host: its buttons in each state, as
//! design 0.2 draws them, Dismiss, and a conversation whose log stalls
//! showing a plain error with Try again instead of loading for ever.

extern crate ui as gpui;

use conversation_host::{ConversationHost, ScriptedHost, ScriptedReply};
use conversation_model::schema::SessionId;
use conversation_ui::{ConversationCopy, ConversationView, StatusButton};
use std::rc::Rc;
use std::time::Duration;
use ui::{Entity, TestAppContext};

fn view(
    cx: &mut TestAppContext,
    host: &ScriptedHost,
    session: Option<SessionId>,
) -> Entity<ConversationView> {
    cx.update(ui::init);
    let shared: Rc<dyn ConversationHost> = Rc::new(host.clone());
    let (view, _) = cx.add_window_view(|window, cx| match session {
        Some(session) => {
            ConversationView::open(shared, session, ConversationCopy::default(), window, cx)
        }
        None => ConversationView::new(shared, "test", ConversationCopy::default(), window, cx),
    });
    cx.run_until_parked();
    view
}

/// The bar's buttons, left to right, among those a test asks about.
fn buttons(cx: &mut TestAppContext, view: &Entity<ConversationView>) -> Vec<StatusButton> {
    use StatusButton::*;
    let all = [Dismiss, Retry, TryAgain, Continue, Stop, Reload];
    cx.update(|cx| {
        let view = view.read(cx);
        let mut shown: Vec<(ui::ElementId, StatusButton)> = all
            .into_iter()
            .filter_map(|b| view.status_button(b).map(|id| (id, b)))
            .collect();
        shown.sort_by_key(|(id, _)| id.to_string());
        shown.into_iter().map(|(_, b)| b).collect()
    })
}

fn act(
    cx: &mut TestAppContext,
    view: &Entity<ConversationView>,
    f: impl FnOnce(&mut ConversationView, &mut ui::Context<ConversationView>),
) {
    cx.update(|cx| view.update(cx, f));
    cx.run_until_parked();
}

#[ui::test]
fn the_bar_offers_what_each_state_allows(cx: &mut TestAppContext) {
    use StatusButton::*;
    let host = ScriptedHost::new();
    let view = view(cx, &host, None);

    // While a reply is written: Stop.
    host.script(ScriptedReply::text("One two three four five six"));
    act(cx, &view, |view, cx| view.send("Count".into(), cx));
    host.pump(2);
    cx.run_until_parked();
    assert_eq!(buttons(cx, &view), [Stop]);

    // Stopped: Dismiss, Retry, Resume.
    act(cx, &view, |view, cx| view.stop(cx));
    assert_eq!(buttons(cx, &view), [Dismiss, Retry, Continue]);

    // Dismiss hides the bar until the turn changes.
    act(cx, &view, |view, cx| view.dismiss_status(cx));
    assert!(buttons(cx, &view).is_empty());
    act(cx, &view, |view, cx| view.resume(cx));
    host.finish();
    cx.run_until_parked();
    assert!(buttons(cx, &view).is_empty(), "a finished reply has no bar");

    // A failed send: Dismiss, Try again.
    act(cx, &view, |view, cx| {
        view.send("Nothing scripted".into(), cx)
    });
    assert_eq!(buttons(cx, &view), [Dismiss, TryAgain]);
}

#[ui::test]
fn a_stalled_log_fails_the_load_with_try_again_and_never_loads_for_ever(cx: &mut TestAppContext) {
    let host = ScriptedHost::new();
    let first = view(cx, &host, None);
    host.script(ScriptedReply::text("Apple."));
    act(cx, &first, |view, cx| view.send("Name a fruit".into(), cx));
    host.finish();
    cx.run_until_parked();
    host.script(ScriptedReply::text("Pear."));
    act(cx, &first, |view, cx| view.regenerate(0, cx));
    host.finish();
    cx.run_until_parked();
    let root = host.sessions()[0].clone();

    // The first session's log stalls: opening the chat must give up.
    host.stall(&root);
    let opened = view(cx, &host, Some(root.clone()));
    assert!(!cx.update(|cx| opened.read(cx).load_failed()));
    cx.executor().advance_clock(Duration::from_secs(11));
    cx.run_until_parked();
    assert!(cx.update(|cx| opened.read(cx).load_failed()));
    assert_eq!(buttons(cx, &opened), [StatusButton::Reload]);

    // Try again once the log answers.
    host.unstall(&root);
    act(cx, &opened, |view, cx| view.retry_load(cx));
    let shown = cx.update(|cx| {
        let view = opened.read(cx);
        (
            view.load_failed(),
            view.thread().turns().first().map(|t| t.reply()),
        )
    });
    assert_eq!(shown, (false, Some("Pear.".to_owned())));
}
