//! The scripted host's session list.

use conversation_host::{ConversationHost, NewSession, ScriptedHost, ScriptedReply, SessionQuery};
use futures::executor::block_on;

#[test]
fn the_list_is_newest_first_titled_and_honours_archive() {
    let host = ScriptedHost::new();
    host.script(ScriptedReply::text("Hi.").with_title("Greeting"));
    let new = || NewSession {
        preset: "p".into(),
        title: None,
    };
    let first = block_on(host.create(new())).unwrap();
    let second = block_on(host.create(new())).unwrap();
    block_on(host.send(
        &first,
        vec![conversation_model::schema::Block::text("Hello")],
    ))
    .unwrap();
    host.finish();

    let listed = block_on(host.list(SessionQuery::default())).unwrap();
    assert_eq!(
        listed[0].id, first,
        "the session that just changed comes first"
    );
    assert_eq!(listed[0].title.as_deref(), Some("Greeting"));
    assert_eq!(listed[1].id, second);
    assert_eq!(listed[1].title, None);

    block_on(host.archive(&first, true)).unwrap();
    let listed = block_on(host.list(SessionQuery::default())).unwrap();
    assert_eq!(listed.len(), 1);
    let query = SessionQuery {
        text: Some("Greet".into()),
        archived: true,
        ..Default::default()
    };
    assert_eq!(block_on(host.list(query)).unwrap()[0].id, first);
}
