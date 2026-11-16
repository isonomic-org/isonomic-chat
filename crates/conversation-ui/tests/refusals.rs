//! A fork the host refuses, on try again or on an edited prompt: the thread
//! stays as it was, no version is added, the error is reported, and the
//! next attempt goes through.

extern crate ui as gpui;

use conversation_host::{ConversationHost, HostError, ScriptedHost, ScriptedReply};
use conversation_ui::{ConversationCopy, ConversationEvent, ConversationView};
use std::cell::RefCell;
use std::rc::Rc;
use ui::{Entity, TestAppContext, VisualTestContext};

type Events = Rc<RefCell<Vec<ConversationEvent>>>;

fn open(
    cx: &mut TestAppContext,
) -> (
    ScriptedHost,
    Entity<ConversationView>,
    VisualTestContext,
    Events,
) {
    cx.update(ui::init);
    let host = ScriptedHost::new();
    let shared: Rc<dyn ConversationHost> = Rc::new(host.clone());
    let (view, window) = cx.add_window_view(|window, cx| {
        ConversationView::new(shared, "test", ConversationCopy::default(), window, cx)
    });
    let window = window.clone();
    let events: Events = Rc::default();
    let log = events.clone();
    cx.update(|cx| {
        cx.subscribe(&view, move |_, event: &ConversationEvent, _| {
            log.borrow_mut().push(event.clone())
        })
        .detach()
    });
    cx.run_until_parked();
    (host, view, window, events)
}

fn say(
    cx: &mut TestAppContext,
    host: &ScriptedHost,
    view: &Entity<ConversationView>,
    prompt: &str,
    reply: &str,
) {
    host.script(ScriptedReply::text(reply));
    let prompt = prompt.to_owned();
    cx.update(|cx| view.update(cx, |view, cx| view.send(prompt, cx)));
    cx.run_until_parked();
    host.finish();
    cx.run_until_parked();
}

fn shown(cx: &mut TestAppContext, view: &Entity<ConversationView>) -> Vec<(String, String)> {
    cx.update(|cx| {
        let view = view.read(cx);
        let thread = view.thread();
        thread
            .turns()
            .iter()
            .map(|turn| (turn.prompt.clone(), turn.reply()))
            .collect()
    })
}

fn refusal() -> HostError {
    HostError::new("unavailable", "The engine could not branch this chat.")
}

fn reported(events: &Events) -> bool {
    events.borrow().iter().any(|event| {
        matches!(event, ConversationEvent::HostError(message)
            if message.as_ref() == "The engine could not branch this chat.")
    })
}

#[ui::test]
fn a_refused_try_again_keeps_the_reply_and_the_next_one_works(cx: &mut TestAppContext) {
    let (host, view, _window, events) = open(cx);
    say(cx, &host, &view, "Name a fruit", "Apple.");
    let before = shown(cx, &view);

    host.refuse_next_write(refusal());
    host.script(ScriptedReply::text("Pear."));
    cx.update(|cx| view.update(cx, |view, cx| view.regenerate(0, cx)));
    cx.run_until_parked();
    assert_eq!(shown(cx, &view), before);
    assert_eq!(cx.update(|cx| view.read(cx).versions(0)), None);
    assert_eq!(host.sessions().len(), 1, "no fork was made");
    assert!(
        reported(&events),
        "the error is reported: {:?}",
        events.borrow()
    );

    // The scripted reply is still queued; the next try goes through.
    cx.update(|cx| view.update(cx, |view, cx| view.regenerate(0, cx)));
    cx.run_until_parked();
    host.finish();
    cx.run_until_parked();
    assert_eq!(
        shown(cx, &view),
        [("Name a fruit".to_owned(), "Pear.".to_owned())]
    );
    assert_eq!(cx.update(|cx| view.read(cx).versions(0)), Some((2, 1)));
}

#[ui::test]
fn a_refused_edit_keeps_the_prompt_and_the_next_save_works(cx: &mut TestAppContext) {
    let (host, view, mut window, events) = open(cx);
    say(cx, &host, &view, "Name a fruit", "Apple.");
    let before = shown(cx, &view);

    let edit = |window: &mut VisualTestContext, text: &'static str| {
        window.update(|window, cx| view.update(cx, |view, cx| view.begin_edit(0, window, cx)));
        window.update(|window, cx| {
            let field = view.read(cx).edit_field().expect("an edit field");
            field.update(cx, |field, cx| field.set_value(text, window, cx));
        });
        window.update(|window, cx| view.update(cx, |view, cx| view.save_edit(window, cx)));
    };

    host.refuse_next_write(refusal());
    host.script(ScriptedReply::text("Cherry."));
    edit(&mut window, "Name a red fruit");
    cx.run_until_parked();
    assert_eq!(shown(cx, &view), before);
    assert_eq!(cx.update(|cx| view.read(cx).versions(0)), None);
    assert!(reported(&events));

    edit(&mut window, "Name a red fruit");
    cx.run_until_parked();
    host.finish();
    cx.run_until_parked();
    assert_eq!(
        shown(cx, &view),
        [("Name a red fruit".to_owned(), "Cherry.".to_owned())]
    );
    assert_eq!(cx.update(|cx| view.read(cx).versions(0)), Some((2, 1)));
}
