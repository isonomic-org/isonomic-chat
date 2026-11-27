// Copyright 2026 Anud Inc.
// SPDX-License-Identifier: GPL-3.0-or-later

//! An approval through `SdkHost`, against the SDK testkit's agents host: a
//! call that acts outward waits for the person, `decide` answers it through
//! `approvals.decide` with the window's user-action token, and the kit folds
//! what follows. Without a token the engine refuses the answer.

use conversation_host::{ConversationHost, NewSession, UpdateStream};
use conversation_model::schema::{
    ApprovalId, Basis, Block, Concurrency, Decision, Effect, SessionId, ToolKind, ToolOutcome,
    Value,
};
use conversation_model::{Thread, TurnState};
use conversation_sdk::SdkHost;
use futures::{FutureExt as _, StreamExt as _};
use isonomic_app::testing::TestHost;
use isonomic_proto::agents::{ClientToolResult, ToolSpec};
use isonomic_proto::manifest::AgentsAccess;
use std::rc::Rc;

/// Chat's id: a first-party app, which may decide its own approvals.
const APP: &str = "com.isonomic.chat";

struct Probe;

impl isonomic_app::App for Probe {
    fn manifest() -> isonomic_app::Manifest {
        isonomic_app::Manifest {
            id: APP.into(),
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

async fn until(thread: &mut Thread, updates: &mut UpdateStream, done: impl Fn(&Thread) -> bool) {
    while !done(thread) {
        let update = updates.next().await.expect("the stream ended early");
        thread.apply(&update);
    }
}

/// A session whose app offers one tool that acts outward, any name.
async fn start(host: &TestHost, sdk: &SdkHost, tool: &str) -> (SessionId, Thread, UpdateStream) {
    let (app, _) = host.context::<Probe>();
    let new = NewSession {
        preset: "chat".into(),
        ..NewSession::default()
    };
    let session = sdk.create(new).await.expect("create");
    let spec = ToolSpec {
        name: tool.into(),
        description: "Runs an operation through a connection.".into(),
        input_schema: Value::object([("type", Value::from("object"))]),
        kind: ToolKind::Client,
        effects: vec![Effect::ActsOutward],
        concurrency: Concurrency::Serial,
    };
    app.client()
        .agents()
        .register_client_tools(&session, vec![spec])
        .await
        .expect("register");
    let updates = sdk.subscribe(&session, 0).await.expect("subscribe");
    let thread = Thread::for_session(session.clone());
    (session, thread, updates)
}

fn waiting(thread: &Thread) -> Option<ApprovalId> {
    let turn = thread.turns().last()?;
    turn.pending_approval().map(|approval| approval.id.clone())
}

fn script(host: &TestHost, tool: &str, answer: &str) {
    let input =
        r#"{"connection":"Tasks","operation":"create_task","input":{"title":"Book the room"}}"#;
    let calls = [(tool, input)];
    host.engine().agents().with(|memory| {
        memory.model.calls(&calls);
        memory.model.text(answer);
    });
}

/// The host with the testkit standing in for the window's token.
fn sdk(host: &TestHost) -> SdkHost {
    let (app, _) = host.context::<Probe>();
    let engine = host.engine().clone();
    let mint = Rc::new(move |session: SessionId, approval: ApprovalId| {
        let token = engine.agents().user_action(Some(APP), &session, &approval);
        futures::future::ready(Ok(token)).boxed_local()
    });
    SdkHost::new(app.client().agents()).with_user_actions(mint)
}

#[test]
fn an_allowed_call_runs_and_the_reply_follows() {
    let host = TestHost::new();
    let sdk = sdk(&host);
    let (app, _) = host.context::<Probe>();
    script(&host, "connection_call", "Booked it.");
    host.runtime().block_on(async {
        let (session, mut thread, mut updates) = start(&host, &sdk, "connection_call").await;
        sdk.send(&session, vec![Block::text("Add a task")])
            .await
            .expect("send");
        until(&mut thread, &mut updates, |t| waiting(t).is_some()).await;
        let approval = waiting(&thread).expect("an approval waits");
        let call = thread.turns()[0].tools[0].clone();
        assert_eq!(call.tool, "connection_call");
        assert_eq!(call.target().operation.as_deref(), Some("create_task"));

        sdk.decide(&session, &approval, Decision::Allow)
            .await
            .expect("decide");
        // The call is the app's to run once allowed.
        let result = ClientToolResult {
            session: session.clone(),
            call: call.call.clone(),
            content: vec![Block::text("Created.")],
            error: None,
        };
        app.client()
            .agents()
            .tool_result(result)
            .await
            .expect("result");
        until(&mut thread, &mut updates, |t| !t.turns()[0].state.is_open()).await;

        let turn = &thread.turns()[0];
        assert_eq!(turn.state, TurnState::Done);
        assert_eq!(turn.reply(), "Booked it.");
        assert_eq!(turn.tools[0].outcome, Some(ToolOutcome::Ok));
        assert_eq!(
            turn.approvals[0].decision,
            Some((Decision::Allow, Basis::User))
        );
    });
}

#[test]
fn a_denied_call_is_not_run() {
    let host = TestHost::new();
    let sdk = sdk(&host);
    script(&host, "send_message", "I didn't send it.");
    host.runtime().block_on(async {
        let (session, mut thread, mut updates) = start(&host, &sdk, "send_message").await;
        sdk.send(&session, vec![Block::text("Send it")])
            .await
            .expect("send");
        until(&mut thread, &mut updates, |t| waiting(t).is_some()).await;
        let approval = waiting(&thread).expect("waits");

        sdk.decide(&session, &approval, Decision::Deny)
            .await
            .expect("decide");
        until(&mut thread, &mut updates, |t| !t.turns()[0].state.is_open()).await;

        let turn = &thread.turns()[0];
        assert_eq!(turn.tools[0].outcome, Some(ToolOutcome::Denied));
        assert!(!turn.approvals[0].allowed());
        assert_eq!(turn.reply(), "I didn't send it.");
    });
}

#[test]
fn stop_while_waiting_denies_the_call() {
    let host = TestHost::new();
    let sdk = sdk(&host);
    script(&host, "connection_call", "Never written.");
    host.runtime().block_on(async {
        let (session, mut thread, mut updates) = start(&host, &sdk, "connection_call").await;
        sdk.send(&session, vec![Block::text("Add a task")])
            .await
            .expect("send");
        until(&mut thread, &mut updates, |t| waiting(t).is_some()).await;

        sdk.cancel(&session).await.expect("stop");
        until(&mut thread, &mut updates, |t| !t.turns()[0].state.is_open()).await;

        let turn = &thread.turns()[0];
        assert_eq!(turn.state, TurnState::Stopped);
        assert_eq!(turn.tools[0].outcome, Some(ToolOutcome::Cancelled));
        assert_eq!(
            turn.approvals[0].decision,
            Some((Decision::Deny, Basis::Cancelled))
        );
    });
}

#[test]
fn without_the_windows_token_the_engine_refuses_the_answer() {
    let host = TestHost::new();
    let (app, _) = host.context::<Probe>();
    let sdk = SdkHost::new(app.client().agents());
    script(&host, "connection_call", "Never written.");
    host.runtime().block_on(async {
        let (session, mut thread, mut updates) = start(&host, &sdk, "connection_call").await;
        sdk.send(&session, vec![Block::text("Add a task")])
            .await
            .expect("send");
        until(&mut thread, &mut updates, |t| waiting(t).is_some()).await;
        let approval = waiting(&thread).expect("waits");
        let refused = sdk.decide(&session, &approval, Decision::Allow).await;
        assert_eq!(
            refused.map_err(|e| e.code),
            Err("missing_user_action".into())
        );
    });
}
