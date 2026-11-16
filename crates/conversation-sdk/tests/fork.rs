//! Forks through `SdkHost`, against the SDK testkit's in-memory agents
//! host: what a fork's own log holds, and how it lists.

use conversation_host::{ConversationHost, ForkAt, NewSession, SessionQuery, UpdateStream};
use conversation_model::schema::{Block, Event, ForkReason};
use conversation_model::{Thread, TurnState, Update};
use conversation_sdk::SdkHost;
use futures::StreamExt as _;
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
fn a_fork_logs_only_its_start_and_lists_its_parent() {
    let host = TestHost::new();
    let (app, _) = host.context::<Probe>();
    let sdk = SdkHost::new(app.client().agents());
    host.engine().agents().with(|memory| {
        memory.model.text("Start small.");
        memory.model.text("Getting started");
        memory.model.text("Because the week is done.");
        memory.model.text("Because it is quiet.");
    });

    host.runtime().block_on(async {
        let new = NewSession {
            preset: "chat".into(),
            title: None,
            model: None,
        };
        let parent = sdk.create(new).await.expect("create");
        let mut updates = sdk.subscribe(&parent, 0).await.expect("subscribe");
        let mut thread = Thread::for_session(parent.clone());
        sdk.send(&parent, text("Three ideas?")).await.expect("send");
        until(&mut thread, &mut updates, |t| t.title().is_some()).await;
        sdk.send(&parent, text("Why Friday?")).await.expect("send");
        until(&mut thread, &mut updates, |t| {
            t.turns().len() == 2 && t.turns()[1].state == TurnState::Done
        })
        .await;

        let second = thread.turns()[1].ids[0].clone();
        let child = sdk
            .fork(&parent, ForkAt::Before(second), ForkReason::Regenerate)
            .await
            .expect("fork");
        let mut own = sdk.subscribe(&child, 0).await.expect("subscribe");
        let mut kinds = Vec::new();
        let mut at = None;
        while at.is_none() {
            let Some(Update::Event(event)) = own.next().await else {
                continue;
            };
            assert_eq!(event.session, child, "only the fork's own events");
            if let Event::Forked(forked) = &event.event {
                assert_eq!(forked.from, parent);
                at = Some(forked.at_seq);
            }
            kinds.push(event.event.type_name().to_owned());
        }
        assert_eq!(kinds, ["session.start", "forked"]);

        let listed = sdk.list(SessionQuery::default()).await.expect("list");
        let entry = listed.iter().find(|e| e.id == child).expect("listed");
        assert_eq!(entry.forked_from, Some((parent.clone(), at.unwrap())));
        assert_eq!(entry.title.as_deref(), Some("Getting started"));

        let mut branch = Thread::for_session(child.clone());
        sdk.send(&child, text("Why Friday?")).await.expect("send");
        until(&mut branch, &mut own, |t| {
            t.turns()
                .first()
                .is_some_and(|turn| turn.state == TurnState::Done)
        })
        .await;
        assert_eq!(branch.turns().len(), 1);
        assert_eq!(branch.turns()[0].reply(), "Because it is quiet.");
    });
}

#[test]
fn an_edited_prompt_forks_with_reason_edit_and_runs_the_new_prompt() {
    let host = TestHost::new();
    let (app, _) = host.context::<Probe>();
    let sdk = SdkHost::new(app.client().agents());
    host.engine().agents().with(|memory| {
        memory.model.text("Apple.");
        memory.model.text("Fruit");
        memory.model.text("Cherry.");
    });

    host.runtime().block_on(async {
        let new = NewSession {
            preset: "chat".into(),
            title: None,
            model: None,
        };
        let parent = sdk.create(new).await.expect("create");
        let mut updates = sdk.subscribe(&parent, 0).await.expect("subscribe");
        let mut thread = Thread::for_session(parent.clone());
        sdk.send(&parent, text("Name a fruit")).await.expect("send");
        until(&mut thread, &mut updates, |t| t.title().is_some()).await;

        let first = thread.turns()[0].ids[0].clone();
        let child = sdk
            .fork(&parent, ForkAt::Before(first), ForkReason::Edit)
            .await
            .expect("fork");
        let mut own = sdk.subscribe(&child, 0).await.expect("subscribe");
        let mut branch = Thread::for_session(child.clone());
        sdk.send(&child, text("Name a red fruit"))
            .await
            .expect("send");
        let mut reason = None;
        while !branch
            .turns()
            .first()
            .is_some_and(|turn| turn.state == TurnState::Done)
        {
            let update = own.next().await.expect("an update");
            if let Update::Event(event) = &update
                && let Event::Forked(forked) = &event.event
            {
                reason = Some(forked.reason.clone());
            }
            branch.apply(&update);
        }
        assert_eq!(reason, Some(ForkReason::Edit));
        assert_eq!(branch.turns()[0].prompt, "Name a red fruit");
        assert_eq!(branch.turns()[0].reply(), "Cherry.");
    });
}
