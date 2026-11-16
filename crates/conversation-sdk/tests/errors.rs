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
        model: None,
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

#[test]
fn the_catalog_lists_chat_models_with_their_state() {
    let host = TestHost::new();
    let (app, _) = host.context::<Modeler>();
    let sdk = SdkHost::new(app.client().agents()).with_models(app.client().models());
    let catalog = host.runtime().block_on(sdk.models()).expect("models");
    let ids: Vec<&str> = catalog.options().iter().map(|o| o.id.as_str()).collect();
    // The embeddings model is not offered for chat.
    assert_eq!(ids, ["chat-small"]);
    assert_ne!(
        catalog.options()[0].availability,
        conversation_model::Availability::Unknown,
        "its state comes from models.info"
    );
    // The engine's recommendation comes from `ModelInfo.recommended`.
    for profile in ["low", "standard", "high"] {
        host.engine().models().recommend(profile, None);
    }
    let none = host.runtime().block_on(sdk.models()).expect("models");
    assert!(
        none.recommended().is_none(),
        "no recommendation, none shown"
    );
    for profile in ["low", "standard", "high"] {
        host.engine()
            .models()
            .recommend(profile, Some("chat-small"));
    }
    let catalog = host.runtime().block_on(sdk.models()).expect("models");
    assert_eq!(
        catalog.recommended().map(|m| m.id.as_str()),
        Some("chat-small")
    );
    // A picked model goes to sessions.create.
    let new = NewSession {
        preset: "chat".into(),
        title: None,
        model: Some("chat-small".into()),
    };
    host.runtime()
        .block_on(sdk.create(new))
        .expect("a session on the picked model");
    // Without the models interface there is no catalog, not an error.
    let bare = SdkHost::new(app.client().agents());
    assert!(
        host.runtime()
            .block_on(bare.models())
            .unwrap()
            .options()
            .is_empty()
    );
}

/// An app that may use any model.
struct Modeler;

impl isonomic_app::App for Modeler {
    fn manifest() -> isonomic_app::Manifest {
        isonomic_app::Manifest {
            id: "com.example.modeler".into(),
            name: "Modeler".into(),
            version: "0.1.0".into(),
            icon: "sparkle".into(),
            capabilities: isonomic_app::Capabilities {
                agents: Some(isonomic_proto::manifest::AgentsAccess {}),
                models: Some(isonomic_app::ModelAccess {
                    any: true,
                    ..Default::default()
                }),
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
