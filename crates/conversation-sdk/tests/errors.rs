//! Errors and updates from the SDK, as the kit sees them.

use conversation_host::{ConversationHost, NewSession};
use conversation_sdk::{SdkHost, host_error};
use isonomic_app::testing::TestHost;
use isonomic_client::ClientError;
use isonomic_proto::{ErrorCode, WireError};

#[test]
fn engine_errors_keep_their_wire_code_and_words() {
    let error = host_error(ClientError::Engine(WireError::new(
        ErrorCode::NotFound,
        "No such session.",
    )));
    assert_eq!(error.code, "not_found");
    assert_eq!(error.message, "No such session.");
    assert_eq!(host_error(ClientError::Disconnected).code, "disconnected");
}

#[test]
fn a_call_goes_out_under_the_apps_token() {
    let host = TestHost::new();
    let (app, _) = host.context::<Probe>();
    let sdk = SdkHost::new(app.client().agents());
    let created = host.runtime().block_on(sdk.create(NewSession {
        preset: "probe".into(),
        title: None,
    }));
    let error = created.expect_err("the testkit has no agents host yet");
    assert!(!error.message.is_empty());
    let calls = host.engine().calls();
    assert!(
        calls
            .iter()
            .any(|c| c.method == "sessions.create" && c.app.as_deref() == Some("com.example.probe"))
    );
}

/// The smallest app with the agents capability.
struct Probe;

impl isonomic_app::App for Probe {
    fn manifest() -> isonomic_app::Manifest {
        isonomic_app::Manifest {
            id: "com.example.probe".into(),
            name: "Probe".into(),
            version: "0.1.0".into(),
            icon: "sparkle".into(),
            capabilities: isonomic_app::Capabilities {
                agents: Some(isonomic_proto::manifest::AgentsAccess {}),
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
