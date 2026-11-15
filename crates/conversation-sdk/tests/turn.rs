//! A whole turn through `SdkHost`, against the SDK testkit's in-memory
//! agents host: no engine, the real host loop.

use conversation_host::{ConversationHost, NewSession, SessionQuery, UpdateStream};
use conversation_model::schema::Block;
use conversation_model::{InputState, Thread, TurnState};
use conversation_sdk::SdkHost;
use futures::StreamExt as _;
use harness_host::ModelChunk;
use harness_log::ModelStop;
use isonomic_app::testing::TestHost;
use isonomic_proto::manifest::AgentsAccess;

struct Probe;

impl isonomic_app::App for Probe {
    fn manifest() -> isonomic_app::Manifest {
        isonomic_app::Manifest {
            id: "com.example.probe".into(),
            name: "Probe".into(),
            version: "0.1.0".into(),
            icon: "sparkle".into(),
            capabilities: isonomic_app::Capabilities {
                agents: Some(AgentsAccess {}),
                ..Default::default()
            },
            schemas: vec![],
        }
    }

    fn build(_: isonomic_app::AppContext, _: &mut ui::Window, app: &mut ui::App) -> ui::AnyView {
        use ui::AppContext as _;
        app.new(|_| ui::Empty).into()
    }
}

fn done(stop: ModelStop) -> ModelChunk {
    ModelChunk::Done {
        stop,
        usage: Default::default(),
    }
}

/// Fold updates until `done` holds.
async fn until(thread: &mut Thread, updates: &mut UpdateStream, done: impl Fn(&Thread) -> bool) {
    while !done(thread) {
        let update = updates.next().await.expect("the stream ended early");
        thread.apply(&update);
    }
}

fn text(text: &str) -> Vec<Block> {
    vec![Block::text(text)]
}

#[test]
fn turns_stream_and_a_reply_cut_short_continues_in_its_turn() {
    let host = TestHost::new();
    let (app, _) = host.context::<Probe>();
    let sdk = SdkHost::new(app.client().agents());
    host.engine().agents().with(|memory| {
        // Turn 1, then the title step that follows it.
        memory
            .model
            .text("Start small, write it down, review on Friday.");
        memory.model.text("Getting started");
        // Turn 2 stops at the output limit; continue finishes it.
        memory.model.push(vec![
            Ok(ModelChunk::Text("Friday reviews keep ".into())),
            Ok(done(ModelStop::MaxTokens)),
        ]);
        memory.model.text("what worked and drop the rest.");
    });

    host.runtime().block_on(async {
        let new = NewSession {
            preset: "chat".into(),
            title: None,
        };
        let session = sdk.create(new).await.expect("create");
        let mut updates = sdk.subscribe(&session, 0).await.expect("subscribe");
        let mut thread = Thread::for_session(session.clone());

        thread.begin_send("Three ideas?");
        sdk.send(&session, text("Three ideas?"))
            .await
            .expect("send");
        until(&mut thread, &mut updates, |t| t.title().is_some()).await;
        let first = &thread.turns()[0];
        assert_eq!(first.prompt, "Three ideas?");
        assert_eq!(
            first.reply(),
            "Start small, write it down, review on Friday."
        );
        assert_eq!(first.state, TurnState::Done);
        assert_eq!(thread.title(), Some("Getting started"));

        sdk.send(&session, text("Why Friday?")).await.expect("send");
        until(&mut thread, &mut updates, |t| {
            t.turns().len() == 2 && t.input_state() == InputState::Continuable
        })
        .await;
        assert_eq!(thread.turns()[1].reply(), "Friday reviews keep ");
        assert_eq!(thread.turns()[1].state, TurnState::Truncated);

        sdk.resume(&session).await.expect("continue");
        until(&mut thread, &mut updates, |t| {
            t.turns()[1].state == TurnState::Done
        })
        .await;
        assert_eq!(thread.turns().len(), 2, "continue stays in its turn");
        assert_eq!(
            thread.turns()[1].reply(),
            "Friday reviews keep what worked and drop the rest."
        );

        // Stop with nothing running is safe; continue then has nothing to do.
        sdk.cancel(&session).await.expect("cancel");
        let refused = sdk.resume(&session).await.expect_err("nothing to continue");
        assert_eq!(refused.code, "continue_refused");
    });
}

#[test]
fn sessions_list_newest_first_and_archive_hides_them() {
    let host = TestHost::new();
    let (app, _) = host.context::<Probe>();
    let sdk = SdkHost::new(app.client().agents());
    host.runtime().block_on(async {
        let mut ids = Vec::new();
        for title in ["First", "Second"] {
            let new = NewSession {
                preset: "chat".into(),
                title: Some(title.into()),
            };
            ids.push(sdk.create(new).await.expect("create"));
            host.engine().agents().advance(1_000);
        }
        let listed = sdk.list(SessionQuery::default()).await.expect("list");
        let titles: Vec<_> = listed.iter().map(|e| e.title.clone()).collect();
        assert_eq!(titles, [Some("Second".into()), Some("First".into())]);

        sdk.archive(&ids[0], true).await.expect("archive");
        let listed = sdk.list(SessionQuery::default()).await.expect("list");
        assert_eq!(listed.len(), 1);
        assert_eq!(listed[0].id, ids[1]);
        let archived = SessionQuery {
            archived: true,
            ..Default::default()
        };
        let listed = sdk.list(archived).await.expect("list");
        assert_eq!(listed[0].id, ids[0]);
        assert!(listed[0].archived);
    });
}

#[test]
fn a_first_reply_cut_short_continues_after_its_title_step() {
    let host = TestHost::new();
    let (app, _) = host.context::<Probe>();
    let sdk = SdkHost::new(app.client().agents());
    host.engine().agents().with(|memory| {
        memory.model.push(vec![
            Ok(ModelChunk::Text("Pack light, ".into())),
            Ok(done(ModelStop::MaxTokens)),
        ]);
        memory.model.text("Packing tips");
        memory.model.text("and leave room for souvenirs.");
    });
    host.runtime().block_on(async {
        let new = NewSession {
            preset: "chat".into(),
            title: None,
        };
        let session = sdk.create(new).await.expect("create");
        let mut updates = sdk.subscribe(&session, 0).await.expect("subscribe");
        let mut thread = Thread::for_session(session.clone());
        sdk.send(&session, text("Packing tips?"))
            .await
            .expect("send");
        until(&mut thread, &mut updates, |t| {
            t.title().is_some() && t.input_state() == InputState::Continuable
        })
        .await;
        assert_eq!(thread.turns()[0].reply(), "Pack light, ");

        sdk.resume(&session)
            .await
            .expect("continue after the title step");
        until(&mut thread, &mut updates, |t| {
            t.turns()[0].state == TurnState::Done
        })
        .await;
        assert_eq!(thread.turns().len(), 1);
        assert_eq!(
            thread.turns()[0].reply(),
            "Pack light, and leave room for souvenirs."
        );
        assert_eq!(thread.title(), Some("Packing tips"));
    });
}
