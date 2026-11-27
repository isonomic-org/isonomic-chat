// Copyright 2026 Anud Inc.
// SPDX-License-Identifier: GPL-3.0-or-later

//! The live screenshot states: what each one sends, streams and stops.

use conversation_host::{ScriptedHost, ScriptedReply, ScriptedTool};
use conversation_model::schema::Value;
use conversation_model::{Availability, ModelCatalog, ModelOption};
use isonomic_chat::Route;

use crate::screenshots::Scene;

const ANSWER: &str = "Public-key cryptography uses **two keys**: one you share and one you keep.\n\n1. Anyone can lock a message with your public key.\n2. Only your private key unlocks it.\n\nThat is how a browser and a server agree on a secret without ever sending it.";

/// A call's input through a connection, as a connected-app tool sends it.
fn through(connection: &str, operation: &str, field: &str, value: &str) -> Value {
    Value::object([
        ("connection", Value::from(connection)),
        ("operation", Value::from(operation)),
        ("input", Value::object([(field, Value::from(value))])),
    ])
}

/// A reply that read through a connection first: the call's row, folded.
pub(crate) fn tools(scene: &mut Scene) {
    let read = ScriptedTool::new(
        "connection_call",
        through("Design files", "list_files", "folder", "Q4 launch"),
        "plan.md, budget.xlsx, notes.md",
    );
    scene.host().script(
        ScriptedReply::text(
            "The Q4 launch folder has three files: the plan, the budget and your notes.",
        )
        .with_tool(read)
        .with_title("Q4 launch files"),
    );
    scene.send("What is in my Q4 launch folder?");
    scene.host().finish();
    scene.settle();
}

/// A call that changes something, waiting for the person: its row and the
/// approval card, with the status bar's Stop.
pub(crate) fn approval(scene: &mut Scene) {
    let act = ScriptedTool::new(
        "connection_call",
        through(
            "Tasks",
            "create_task",
            "title",
            "Book the room for Thursday",
        ),
        "Created.",
    )
    .asking("connection_call create_task");
    scene.host().script(
        ScriptedReply::text("Added the task.")
            .with_tool(act)
            .with_title("Book the room"),
    );
    scene.send("Add a task to book the room for Thursday.");
    scene.host().finish();
    scene.settle();
}

/// A finished exchange, a second reply stopped part way (continue shows),
/// and the quiet divider of a compaction between them.
pub(crate) fn thread(scene: &mut Scene) {
    scene
        .host()
        .script(ScriptedReply::text(ANSWER).with_title("Public-key cryptography"));
    scene.host().script(ScriptedReply::text("A digital signature is the reverse: you lock with your private key, and anyone can check it with your public key."));
    scene.send("Explain how public-key cryptography works, in plain terms.");
    scene.host().finish();
    scene.settle();
    let session = scene.host().sessions()[0].clone();
    scene
        .host()
        .compact(&session, "Earlier messages are not shown to the model.");
    scene.reply("And signatures?");
    scene.host().pump(9);
    scene.settle();
    scene.shell(|shell, _, cx| {
        if let Route::Session(id) = shell.route().clone()
            && let Some(thread) = shell.thread(&id).cloned()
        {
            thread.update(cx, |thread, cx| thread.stop(cx));
        }
    });
    scene.settle();
}

/// A reply part way through streaming, with reasoning done.
pub(crate) fn streaming(scene: &mut Scene) {
    scene
        .host()
        .script(ScriptedReply::text(ANSWER).with_reasoning("Plain terms, one analogy, short."));
    scene.send("Explain how public-key cryptography works, in plain terms.");
    scene.host().pump(20);
    scene.settle();
}

/// A reply written twice: the second shows, with the pager.
pub(crate) fn versions(scene: &mut Scene) {
    scene.host().script(
        ScriptedReply::text("Try a short walk after lunch.").with_title("Afternoon energy"),
    );
    scene.send("One tip for afternoon energy?");
    scene.host().finish();
    scene.settle();
    scene.host().script(ScriptedReply::text(
        "Drink a glass of water before you reach for coffee.",
    ));
    scene.again(0);
    scene.host().finish();
    scene.settle();
}

/// A sent prompt open for editing, its reply below.
pub(crate) fn editing(scene: &mut Scene) {
    scene.host().script(
        ScriptedReply::text("Try a short walk after lunch.").with_title("Afternoon energy"),
    );
    scene.send("One tip for afternoon energy?");
    scene.host().finish();
    scene.settle();
    scene.begin_edit(0);
}

/// A failed send with try again, and a conversation too large.
pub(crate) fn errors(scene: &mut Scene) {
    scene.host().script(ScriptedReply::text("Sure."));
    scene.host().script(ScriptedReply::too_large(
        "This conversation is too long for the model. Start a new chat to keep going.",
    ));
    scene.send("Hello");
    scene.host().finish();
    scene.settle();
    scene.reply("One more question");
    scene.settle();
}

/// Thinking before any words of the reply.
pub(crate) fn thinking(scene: &mut Scene) {
    scene.host().script(
        ScriptedReply::text("Here is a plan.").with_reasoning("Picking an analogy that holds up"),
    );
    scene.send("Explain public-key cryptography with one analogy.");
    scene.host().pump(5);
    scene.settle();
}

/// A reply with a heading, a list and a code block.
pub(crate) fn markdown(scene: &mut Scene) {
    scene.host().script(
        ScriptedReply::text(
            "Signing takes three steps.\n\n## The two jobs it does\n\n- Encryption: anyone can lock a message with your public key.\n- Signatures: you sign with your private key.\n\n```rust\nfn sign(message: &[u8], key: &PrivateKey) -> Signature {\n    key.sign(message)\n}\n```",
        )
        .with_title("Signing"),
    );
    scene.send("How do I sign a message?");
    scene.host().finish();
    scene.settle();
}

/// A send the host refused: the bar says so and offers Try again.
pub(crate) fn failed(scene: &mut Scene) {
    scene
        .host()
        .script(ScriptedReply::text("Sure.").with_title("Hello"));
    scene.send("Hello");
    scene.host().finish();
    scene.settle();
    // Nothing is scripted, so the host refuses this one.
    scene.reply("One more question");
    scene.settle();
}

/// Two installed models and one to download; the recommended one first.
pub(crate) fn models(host: &ScriptedHost) {
    let option = |id: &str, name: &str, availability| ModelOption {
        id: id.into(),
        name: name.into(),
        remote: false,
        availability,
    };
    host.set_models(ModelCatalog::new(
        vec![
            option("small", "Small", Availability::Installed),
            option("medium", "Medium", Availability::Installed),
            option("large", "Large", Availability::NotInstalled),
        ],
        Some("medium".into()),
    ));
}

/// No chat model installed: home asks to install the recommended one.
pub(crate) fn install(host: &ScriptedHost) {
    let option = |id: &str, name: &str| ModelOption {
        id: id.into(),
        name: name.into(),
        remote: false,
        availability: Availability::NotInstalled,
    };
    host.set_models(ModelCatalog::new(
        vec![option("small", "Small"), option("medium", "Medium")],
        Some("medium".into()),
    ));
}

/// A long reply streamed a word at a time and finished, then a second
/// prompt stopped at its first piece, an opening `**`: the earlier reply
/// stays whole and no raw markup shows.
pub(crate) fn earlier_reply(scene: &mut Scene) {
    scene
        .host()
        .script(ScriptedReply::text(ITINERARY).with_title("A day in Lisbon"));
    scene
        .host()
        .script(ScriptedReply::text("** Morning in Belem."));
    scene.send("Plan one day in Lisbon");
    while scene.host().pump(1) {
        scene.settle();
        scene.frame();
    }
    scene.settle();
    scene.reply("Plan one day in Lisbon");
    scene.host().pump(1);
    scene.settle();
    scene.shell(|shell, _, cx| {
        if let Route::Session(id) = shell.route().clone()
            && let Some(thread) = shell.thread(&id).cloned()
        {
            thread.update(cx, |thread, cx| thread.stop(cx));
        }
    });
    scene.settle();
}

const ITINERARY: &str = "Here is a one-day itinerary for Lisbon:\n\n\
    1. **Morning:** coffee and a pastry in Belem, then the monastery.\n\
    2. **Afternoon:** the tram up to Alfama and the castle walls.\n\
    3. **Evening:** dinner in Bairro Alto and the view from a miradouro.";
