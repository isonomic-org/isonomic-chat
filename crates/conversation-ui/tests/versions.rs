// Copyright 2026 Anud Inc.
// SPDX-License-Identifier: GPL-3.0-or-later

//! Versions against the scripted host: trying a reply again forks the
//! session before its turn, the pager switches between the versions, and a
//! conversation opened again shows the version last worked on.

extern crate ui as gpui;

use conversation_host::{ConversationHost, ScriptedHost, ScriptedReply, SessionQuery};
use conversation_model::schema::SessionId;
use conversation_ui::{ConversationCopy, ConversationView};
use futures::FutureExt as _;
use std::rc::Rc;
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

/// Every turn's prompt and reply, in order.
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

fn turn(prompt: &str, reply: &str) -> (String, String) {
    (prompt.to_owned(), reply.to_owned())
}

fn again(
    cx: &mut TestAppContext,
    host: &ScriptedHost,
    view: &Entity<ConversationView>,
    index: usize,
    reply: &str,
) {
    host.script(ScriptedReply::text(reply));
    cx.update(|cx| view.update(cx, |view, cx| view.regenerate(index, cx)));
    cx.run_until_parked();
    host.finish();
    cx.run_until_parked();
}

fn pick(cx: &mut TestAppContext, view: &Entity<ConversationView>, index: usize, version: usize) {
    cx.update(|cx| view.update(cx, |view, cx| view.show_version(index, version, cx)));
    cx.run_until_parked();
}

fn versions(
    cx: &mut TestAppContext,
    view: &Entity<ConversationView>,
    index: usize,
) -> Option<(usize, usize)> {
    cx.update(|cx| view.read(cx).versions(index))
}

#[ui::test]
fn trying_the_last_reply_again_adds_a_version_and_the_pager_switches(cx: &mut TestAppContext) {
    let host = ScriptedHost::new();
    let view = view(cx, &host, None);
    say(cx, &host, &view, "Name a fruit", "Apple.");
    say(cx, &host, &view, "Name a colour", "Red.");
    assert_eq!(versions(cx, &view, 1), None);

    again(cx, &host, &view, 1, "Blue.");
    assert_eq!(
        shown(cx, &view),
        [
            turn("Name a fruit", "Apple."),
            turn("Name a colour", "Blue.")
        ]
    );
    assert_eq!(versions(cx, &view, 1), Some((2, 1)));

    again(cx, &host, &view, 1, "Green.");
    assert_eq!(versions(cx, &view, 1), Some((3, 2)));

    pick(cx, &view, 1, 0);
    assert_eq!(
        shown(cx, &view),
        [
            turn("Name a fruit", "Apple."),
            turn("Name a colour", "Red.")
        ]
    );
    assert_eq!(versions(cx, &view, 1), Some((3, 0)));
    pick(cx, &view, 1, 1);
    assert_eq!(shown(cx, &view)[1], turn("Name a colour", "Blue."));

    // Every version forks the first session at the same point.
    let root = cx.update(|cx| view.read(cx).thread().turns()[0].ids.clone());
    assert!(!root.is_empty());
    let listed = host
        .list(SessionQuery::default())
        .now_or_never()
        .unwrap()
        .unwrap();
    let points: Vec<_> = listed
        .iter()
        .filter_map(|e| e.forked_from.clone())
        .collect();
    assert_eq!(points.len(), 2);
    assert_eq!(points[0], points[1]);
}

#[ui::test]
fn an_earlier_reply_tried_again_drops_the_turns_after_it_until_switched_back(
    cx: &mut TestAppContext,
) {
    let host = ScriptedHost::new();
    let view = view(cx, &host, None);
    say(cx, &host, &view, "Name a fruit", "Apple.");
    say(cx, &host, &view, "Name a colour", "Red.");

    again(cx, &host, &view, 0, "Pear.");
    assert_eq!(shown(cx, &view), [turn("Name a fruit", "Pear.")]);
    assert_eq!(versions(cx, &view, 0), Some((2, 1)));

    // The new version goes on from there.
    say(cx, &host, &view, "Name a tree", "Oak.");
    assert_eq!(shown(cx, &view)[1], turn("Name a tree", "Oak."));

    pick(cx, &view, 0, 0);
    assert_eq!(
        shown(cx, &view),
        [
            turn("Name a fruit", "Apple."),
            turn("Name a colour", "Red.")
        ]
    );
    pick(cx, &view, 0, 1);
    assert_eq!(
        shown(cx, &view),
        [turn("Name a fruit", "Pear."), turn("Name a tree", "Oak.")]
    );
}

#[ui::test]
fn a_conversation_opened_again_shows_the_version_last_worked_on(cx: &mut TestAppContext) {
    let host = ScriptedHost::new();
    let first = view(cx, &host, None);
    say(cx, &host, &first, "Name a fruit", "Apple.");
    say(cx, &host, &first, "Name a colour", "Red.");
    again(cx, &host, &first, 1, "Blue.");
    // A version of a later turn, inside that version.
    say(cx, &host, &first, "Name a shape", "Circle.");
    again(cx, &host, &first, 2, "Square.");
    let root = host.sessions()[0].clone();

    let opened = view(cx, &host, Some(root));
    assert_eq!(
        shown(cx, &opened),
        [
            turn("Name a fruit", "Apple."),
            turn("Name a colour", "Blue."),
            turn("Name a shape", "Square."),
        ]
    );
    assert_eq!(versions(cx, &opened, 1), Some((2, 1)));
    assert_eq!(versions(cx, &opened, 2), Some((2, 1)));

    // Back to the first colour: its own later turns, and no shape.
    pick(cx, &opened, 1, 0);
    assert_eq!(
        shown(cx, &opened),
        [
            turn("Name a fruit", "Apple."),
            turn("Name a colour", "Red.")
        ]
    );
    assert_eq!(versions(cx, &opened, 2), None);

    // Trying the first colour again from there adds a third version.
    again(cx, &host, &opened, 1, "Yellow.");
    assert_eq!(shown(cx, &opened)[1], turn("Name a colour", "Yellow."));
    assert_eq!(versions(cx, &opened, 1), Some((3, 2)));
}

#[ui::test]
fn an_edited_prompt_is_a_new_version_and_cancel_keeps_the_old(cx: &mut TestAppContext) {
    cx.update(ui::init);
    let host = ScriptedHost::new();
    let shared: Rc<dyn ConversationHost> = Rc::new(host.clone());
    let (view, window) = cx.add_window_view(|window, cx| {
        ConversationView::new(shared, "test", ConversationCopy::default(), window, cx)
    });
    let mut window = window.clone();
    cx.run_until_parked();
    say(cx, &host, &view, "Name a fruit", "Apple.");

    // Cancel leaves the prompt and makes no version.
    window.update(|window, cx| view.update(cx, |view, cx| view.begin_edit(0, window, cx)));
    assert_eq!(cx.update(|cx| view.read(cx).editing()), Some(0));
    window.update(|window, cx| view.update(cx, |view, cx| view.cancel_edit(window, cx)));
    assert_eq!(cx.update(|cx| view.read(cx).editing()), None);
    assert_eq!(versions(cx, &view, 0), None);

    // Save sends the new prompt in a fork with reason edit.
    window.update(|window, cx| view.update(cx, |view, cx| view.begin_edit(0, window, cx)));
    window.update(|window, cx| {
        let field = view.read(cx).edit_field().expect("an edit field");
        field.update(cx, |field, cx| {
            field.set_value("Name a red fruit", window, cx)
        });
    });
    host.script(ScriptedReply::text("Cherry."));
    window.update(|window, cx| view.update(cx, |view, cx| view.save_edit(window, cx)));
    cx.run_until_parked();
    host.finish();
    cx.run_until_parked();
    assert_eq!(shown(cx, &view), [turn("Name a red fruit", "Cherry.")]);
    assert_eq!(versions(cx, &view, 0), Some((2, 1)));
    let child = host.sessions()[1].clone();
    let reason = host.events(&child).into_iter().find_map(|e| match e.event {
        conversation_model::schema::Event::Forked(forked) => Some(forked.reason),
        _ => None,
    });
    assert_eq!(reason, Some(conversation_model::schema::ForkReason::Edit));

    // The pager brings back the first prompt with its reply.
    pick(cx, &view, 0, 0);
    assert_eq!(shown(cx, &view), [turn("Name a fruit", "Apple.")]);
}
