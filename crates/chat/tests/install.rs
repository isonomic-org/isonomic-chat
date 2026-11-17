// Copyright 2026 Anud Inc.
// SPDX-License-Identifier: GPL-3.0-or-later

//! First run against the SDK's headless host: no chat model, Install asks
//! the window, the download runs, and home leaves Downloading the moment the
//! model is ready, with no restart.

extern crate ui as gpui;

use isonomic_app::testing::{TestHost, wait_until};
use isonomic_chat::{Chat, ChatShell};
use isonomic_testkit::{DownloadStep, Gate};
use ui::{Entity, TestAppContext};

fn mount(
    cx: &mut TestAppContext,
    host: &TestHost,
) -> (isonomic_app::testing::OpenApp, Entity<ChatShell>) {
    cx.update(ui::init);
    let open = host.open::<Chat>(cx);
    let chat = open.root::<Chat>(cx);
    let shell = chat.read_with(cx, |chat, _| chat.shell().clone());
    wait_until(cx, &shell, |shell| shell.is_loaded());
    (open, shell)
}

/// Let the watch ask the host once more.
fn poll(cx: &mut TestAppContext) {
    cx.executor()
        .advance_clock(std::time::Duration::from_secs(1));
    cx.run_until_parked();
}

#[ui::test]
fn home_leaves_downloading_the_moment_the_model_is_ready(cx: &mut TestAppContext) {
    let host = TestHost::new();
    let models = host.engine().models();
    models.uninstall("chat-small");
    let gate = Gate::new();
    models.script_download(
        "chat-small",
        vec![
            DownloadStep::Progress(1 << 20),
            DownloadStep::Hold(gate.clone()),
        ],
    );
    let (_open, shell) = mount(cx, &host);
    wait_until(cx, &shell, |shell| shell.missing_model().is_some());

    // Install asks the window; the window starts the download.
    shell.update(cx, |shell, cx| {
        assert!(shell.install_model("chat-small", cx))
    });
    // The window (the shell's own client) runs the download.
    let window = host.engine().client_on(host.runtime());
    let download = host.runtime().spawn(async move {
        let mut stream = window
            .models()
            .download("chat-small")
            .await
            .expect("download");
        while stream.next().await.is_some() {}
    });

    // While it runs, home stays on the install screen and says Downloading.
    poll(cx);
    wait_until(cx, &shell, |shell| {
        shell
            .catalog()
            .and_then(|c| c.get("chat-small"))
            .is_some_and(|o| o.availability == conversation_model::Availability::Downloading)
    });
    shell.read_with(cx, |shell, _| {
        assert!(shell.missing_model().is_some());
        assert_eq!(
            shell.install_asked().map(|s| s.to_string()),
            Some("chat-small".into())
        );
    });

    // The download ends: at the next poll the model can answer, and home
    // shows the composer.
    gate.open();
    host.runtime()
        .block_on(download)
        .expect("the download ends");
    poll(cx);
    wait_until(cx, &shell, |shell| shell.missing_model().is_none());
    shell.read_with(cx, |shell, _| {
        assert_eq!(shell.install_asked(), None);
        let catalog = shell.catalog().expect("a catalog");
        assert!(catalog.choose(None).is_some(), "a model can answer");
    });

    // Nothing is pending, so the watch has stopped asking.
    let asked = host.engine().calls().len();
    poll(cx);
    poll(cx);
    assert_eq!(host.engine().calls().len(), asked);
}
