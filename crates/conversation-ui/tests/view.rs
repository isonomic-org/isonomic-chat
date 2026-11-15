//! Headless tests of the conversation view against the scripted host: send,
//! stream, stop, continue, refusal and a refused send.

extern crate ui as gpui;

use conversation_host::{ConversationHost, ScriptedHost, ScriptedReply};
use conversation_model::{InputState, TurnState};
use conversation_ui::{ConversationCopy, ConversationEvent, ConversationView};
use std::cell::RefCell;
use std::rc::Rc;
use ui::test::TestWindowExt as _;
use ui::{
    AppContext as _, Bounds, Entity, TestAppContext, Window, WindowBounds, WindowHandle,
    WindowOptions, base::Root, px, size,
};

struct Harness {
    window: WindowHandle<Root>,
    view: Entity<ConversationView>,
    host: ScriptedHost,
    events: Rc<RefCell<Vec<ConversationEvent>>>,
}

fn open(cx: &mut TestAppContext) -> Harness {
    cx.update(ui::init);
    let host = ScriptedHost::new();
    let events = Rc::new(RefCell::new(Vec::new()));
    let (window, view) = cx.update(|cx| {
        let shared: Rc<dyn ConversationHost> = Rc::new(host.clone());
        let log = events.clone();
        let (window, view) = ui::open_window(
            WindowOptions {
                window_bounds: Some(WindowBounds::Windowed(Bounds {
                    origin: Default::default(),
                    size: size(px(900.), px(800.)),
                })),
                ..Default::default()
            },
            cx,
            move |window, cx| {
                let view = cx.new(|cx| {
                    ConversationView::new(shared, "test", ConversationCopy::default(), window, cx)
                });
                cx.subscribe(&view, move |_, event: &ConversationEvent, _| {
                    log.borrow_mut().push(event.clone())
                })
                .detach();
                view
            },
        )
        .expect("open the test window");
        (window.downcast::<Root>().expect("a root view"), view)
    });
    Harness {
        window,
        view,
        host,
        events,
    }
}

impl Harness {
    fn frame<R>(&self, cx: &mut TestAppContext, f: impl FnOnce(&mut Window, &mut ui::App) -> R) -> R {
        cx.run_until_parked();
        cx.update_window(self.window.into(), |_, window, cx| {
            // The list measures new rows on one frame and paints them on the next.
            window.render_frame(cx);
            window.render_frame(cx);
            f(window, cx)
        })
        .expect("the window is open")
    }

    fn send(&self, cx: &mut TestAppContext, text: &str) {
        let text = text.to_owned();
        cx.update(|cx| self.view.update(cx, |view, cx| view.send(text, cx)));
        cx.run_until_parked();
    }

    fn pump(&self, cx: &mut TestAppContext, words: usize) {
        self.host.pump(words);
        cx.run_until_parked();
    }

    fn state(&self, cx: &mut TestAppContext) -> InputState {
        cx.update(|cx| self.view.read(cx).input_state())
    }

    fn last(&self, cx: &mut TestAppContext) -> (String, TurnState) {
        cx.update(|cx| {
            let view = self.view.read(cx);
            let thread = view.thread();
            let turn = thread.turns().last().expect("a turn");
            (turn.reply(), turn.state.clone())
        })
    }
}

#[ui::test]
fn a_reply_streams_in_and_the_composer_comes_back(cx: &mut TestAppContext) {
    let h = open(cx);
    h.host
        .script(ScriptedReply::text("Three good names follow.").with_title("Book club names"));
    h.send(cx, "Name my book club");

    assert_eq!(h.state(cx), InputState::Busy);
    assert!(matches!(
        h.events.borrow().first(),
        Some(ConversationEvent::SessionStarted(_))
    ));

    h.pump(cx, 2);
    assert_eq!(h.last(cx), ("Three good ".into(), TurnState::Running));
    h.frame(cx, |window, _| assert!(window.try_find("stop").is_some()));

    h.host.finish();
    cx.run_until_parked();
    assert_eq!(h.last(cx), ("Three good names follow.".into(), TurnState::Done));
    assert_eq!(h.state(cx), InputState::Ready);
    assert!(
        h.events
            .borrow()
            .contains(&ConversationEvent::TitleChanged("Book club names".into()))
    );
    h.frame(cx, |window, _| assert!(window.try_find("send").is_some()));
}

#[ui::test]
fn stop_keeps_the_partial_reply_and_continue_finishes_it(cx: &mut TestAppContext) {
    let h = open(cx);
    h.host
        .script(ScriptedReply::text("Welcome, everyone. Tonight we pick our first book."));
    h.send(cx, "Write a welcome");
    h.pump(cx, 2);

    h.frame(cx, |window, cx| window.click("stop", cx));
    cx.run_until_parked();
    assert_eq!(h.last(cx), ("Welcome, everyone. ".into(), TurnState::Stopped));
    assert_eq!(h.state(cx), InputState::Continuable);

    h.frame(cx, |window, cx| window.click("conversation-continue", cx));
    cx.run_until_parked();
    assert_eq!(h.state(cx), InputState::Busy);

    h.host.finish();
    cx.run_until_parked();
    let (reply, state) = h.last(cx);
    assert_eq!(reply, "Welcome, everyone. Tonight we pick our first book.");
    assert_eq!(state, TurnState::Done);
    let turns = cx.update(|cx| h.view.read(cx).thread().turns().len());
    assert_eq!(turns, 1, "the continued reply stays in its turn");
}

#[ui::test]
fn reasoning_shows_before_the_answer(cx: &mut TestAppContext) {
    let h = open(cx);
    h.host
        .script(ScriptedReply::text("Yes.").with_reasoning("Check the date first."));
    h.send(cx, "Is it Sunday?");
    h.pump(cx, 2);
    let (reply, _) = h.last(cx);
    assert!(reply.is_empty());
    let thinking = cx.update(|cx| h.view.read(cx).thread().turns()[0].thinking());
    assert_eq!(thinking, "Check the ");
    h.frame(cx, |window, _| {
        assert!(window.try_find("stop").is_some());
        assert!(window.try_find("conversation-continue").is_none());
    });
    h.host.finish();
    cx.run_until_parked();
    assert_eq!(h.last(cx), ("Yes.".into(), TurnState::Done));
}

#[ui::test]
fn a_refusal_ends_the_turn_with_its_message(cx: &mut TestAppContext) {
    let h = open(cx);
    h.host.script(ScriptedReply::refusal("I can't help with that."));
    h.send(cx, "Something not allowed");
    h.host.finish();
    cx.run_until_parked();
    assert_eq!(
        h.last(cx).1,
        TurnState::Refused("I can't help with that.".into())
    );
    assert_eq!(h.state(cx), InputState::Ready);
}

#[ui::test]
fn a_send_the_host_refuses_shows_why(cx: &mut TestAppContext) {
    let h = open(cx);
    // Nothing scripted: the host refuses the send.
    h.send(cx, "Hello");
    assert_eq!(
        h.last(cx).1,
        TurnState::Rejected("Nothing is scripted to reply.".into())
    );
    assert_eq!(h.state(cx), InputState::Ready);
}

#[ui::test]
fn a_second_prompt_follows_the_first(cx: &mut TestAppContext) {
    let h = open(cx);
    h.host.script(ScriptedReply::text("One."));
    h.host.script(ScriptedReply::text("Two."));
    h.send(cx, "First");
    h.host.finish();
    cx.run_until_parked();
    h.send(cx, "Second");
    h.host.finish();
    cx.run_until_parked();
    let replies: Vec<String> = cx.update(|cx| {
        h.view
            .read(cx)
            .thread()
            .turns()
            .iter()
            .map(|turn| turn.reply())
            .collect()
    });
    assert_eq!(replies, vec!["One.".to_owned(), "Two.".to_owned()]);
    assert_eq!(h.host.sessions().len(), 1);
}
