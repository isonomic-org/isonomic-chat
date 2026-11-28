# Status

Newest first. Every deviation from the brief is recorded here.

## 2026-11-28 · Display widgets in the kit; approvals ask for their token inside the click

- **Display widgets (#51).** A reply's display calls (`show_table`, `show_steps`, `show_metrics`, `show_chart`) draw Isonomic UI's sortable table, step trail, stat grid and bar, line or area chart at full column width under the reply's words, in call order. `WidgetRegistry::standard()` maps each tool name to a `WidgetBinding` (input in, props out, one component); the view uses it by default, and `ConversationView::set_widgets` swaps it. Display calls never show as tool rows.
- **Never broken:** a call being retried draws nothing; the host's fallback, anything but a clear `shown`, bad data the schema let through, or a name the registry does not know draw the input's `fallback_text` as Markdown; with no usable text, the generic "Content this version can't show (show_map)" block. `WidgetRegistry::resolve` says which without drawing.
- **Sorting** a table is the view's own state (`sort_widget`, `widget_sort`): the same column flips, another sorts ascending. Nothing goes back to the host.
- The model: `ToolUse` keeps the call's `kind` and its `display` outcome; `Turn::widgets()` lists what to draw. The scripted host's results carry `display: None`.
- **Deviations.** Widgets sit under the whole reply, not under the words of their own step (the turn keeps one text). The sortable table still shows its selection column, doing nothing, until isonomic-ui #413 gives it a read-only mode. A step's detail shows as its first sub-step, because the step trail's status line needs a live timer (isonomic-ui #428). Chart titles are a medium 14 label above the chart (no chart card yet).
- **Approvals live (O20 done, isonomic-sdk #65).** `SdkHost::decide` now asks for the user-action token at once, inside the person's click or key press, and awaits it after, as the window requires. Chat wires the window's token (`AppContext::user_action`); with no input in hand the window refuses, the card asks again, and the error reads "Allow again from this window." It is live once the window installs its token source and the desktop moves `SDK_REF` (its own work, after the launch).
- `SDK_REF` moves to `24b945a` (`UI_REF` `6e0c4a5`, unchanged); the harness to `ba26c80` (schema revision 1: tool kind `display`, `tool.result.display`). `Cargo.lock` follows, and `serde` (already in it) is a direct dependency of `conversation-ui`, for the bindings' inputs.
- The xtask boundary check keeps `WidgetBinding` and `DisplayOutcome` out of the chat crate.
- Tests: `conversation-ui/tests/widgets.rs` (the display fixture: four widgets, a retry, a fallback, an unknown; bad data, blank text, an empty registry, the view drawing widgets and no rows for them, sorting). The gallery plays the new `display` fixture in light and dark.

## 2026-11-27 · The kit's gallery

- **The gallery** (`crates/conversation-ui/tests/gallery.rs`, opt-in like Chat's screenshots): every recorded harness fixture (chat-basic, tool-heavy, approval, autonomous-run, long-thread, client-gone), each session in it, through the kit's conversation view, light and dark: opened whole, and replayed word by word with a frame per update, stopped half way. 36 images under `target/gallery/<theme>/`. Run: `cargo test -p conversation-ui --features test-support --test gallery` (`GALLERY_FIXTURE=<name>` for one).
- Replay prints the slowest frame per fixture. On the Linux software renderer it was at most 9 ms (tool-heavy) and 8 ms for the 10,000-event thread at 200 updates a frame; the Intel Mac timing is still the gate's number.
- **The fixture player:** `FixtureHost` (`conversation-host`) serves a recorded log through the host trait: lists its sessions, replays each whole, or in replay mode releases updates on `advance`, with each reply's words as live deltas before its durable event. Writes are refused (`read_only`); the stream stays open as a live one does.
- **Headless:** `conversation-ui/tests/fixtures.rs` opens every fixture whole (no turn left open), replays every fixture under 2,000 events word by word to the same thread as opened whole, finds every tool row of tool-heavy and approval by its id, and checks a recorded conversation keeps its log on a send.
- **The boundary and product-term checks** are built (`xtask`; `cargo xtask check`, and as tests in `cargo test --workspace`): no kit crate depends on Chat; Chat's sources never match session events, fold them, render a tool call or decide an approval; no kit public name carries "chat", "agent" or "project".
- Seen in the gallery, for later work: `ask_user` shows as a plain tool row (the ask-widgets spec covers it), and the approval fixture's denied call reads "Failed" with "Denied" beside it.

## 2026-11-27 · Tool calls and approvals in a thread

- A reply's tool calls show as Isonomic UI's tool call rows, one per call, above the reply's words: the operation (or the tool's name), the connection when the call's input names one, and running, done or failed with the time it took. Arguments and result stay folded until opened, a failed call too. Generic over every tool and name: the connection and operation are read from the input's `connection` / `operation` / `input` fields when present.
- A call the host asks approval for shows Isonomic UI's approval card under its row: "Allow create_task?", the app, the action and each argument, Deny (Esc) and Allow, no "always allow". The status bar says "Waiting for your approval" with Stop; Stop denies the call as the engine does. Only a clear Allow allows.
- The kit: `Turn::tools` carry input, effects, output and duration; `Turn::approvals` and `pending_approval()` (`conversation-model`); `ConversationHost::decide` over `approvals.decide`; `SdkHost::with_user_actions` supplies the window's user-action token; `ConversationView::decide`. The scripted host makes tool calls (`ScriptedTool`, `asking`, `failing`).
- **Not live yet in the window:** `approvals.decide` needs a user-action token only the shell can mint (`approvals.user_action`), and no SDK call lets an app ask the window for one. Chat's `SdkHost` has none, so an answer is refused (`missing_user_action`) until the SDK adds that path (request to the SDK). Tested with the testkit standing in for the window.
- Tests: `conversation-ui/tests/tools.rs` (a read, any tool name, allow, deny, stop while waiting), `conversation-sdk/tests/approvals.rs` (the same on the testkit, and the refusal without a token), `conversation-model/tests/tools.rs`. Screenshot states `thread-tools` and `thread-approval`.
## 2026-11-27 · SDK at 91c8d2a, harness at 077909e

- `SDK_REF` moves to `91c8d2a` (`UI_REF` `6e0c4a5`, unchanged). The SDK adds tool server connections and the model setup calls, both for the window; nothing in Chat calls them.
- The harness moves to `077909e` with it, as the SDK asks of every app that pins it; no change to the event schema or the host API. `Cargo.lock` follows (harness crates only).
- Builds, and every test passes, as before.
## 2026-11-25 · The thought row and the stop glyph

- Once a reply's words come, a reply with reasoning shows a light bulb row, "Thought for N seconds" (whole seconds, at least one; whole minutes from a minute), that opens to the reasoning. Closed by default; the open state is kept per reply. Words are the kit's copy (`ConversationCopy::thought_for`).
- `Turn::thought_ms` comes from the durable log (the turn's start to the first reply with reasoning), so a reloaded chat shows the same time. It includes the time spent writing, since the log has no separate end of reasoning.
- The status bar's Stopped icon is a filled stop glyph from Isonomic UI; no Chat change.
- `SDK_REF` moves to `123d0b4` (`UI_REF` `6e0c4a5`); harness unchanged. `Cargo.lock` gains the SDK test engine's new dependencies (`isonomic-words`, from the SDK's recovery kit). Tested in `conversation-ui/tests/thought.rs`.
## 2026-11-25 · Engine errors part way through a reply

- A reply the engine gives up on part way keeps the words it had written on show, above the error, with Try again (before, they vanished and only the error remained). The words are display only, as the session schema says of an errored attempt's content; they are not sent to the model. Fixed in `conversation-model` (the errored attempt's content is committed to the turn).
- A passing error the engine tries again clears the first try's words and streams the answer from the start, once, with the "Trying again" note meanwhile; nothing is doubled.
- The scripted host can now fail part way (`ScriptedReply::fail_after`, `retry_after`), writing what the engine writes: an errored attempt with what streamed, then a fresh try or a failed step and turn (`scripted/fail.rs`).
- Tested in `conversation-ui/tests/failures.rs`; the keep-the-words test fails without the fix.
## 2026-11-25 · Pins survive a store that cannot be read

- When Chat's store cannot be read during a list load, the chats already shown as pinned stay pinned, in their order, instead of every pin being dropped. The store's next good answer wins. Tested in `crates/chat/tests/reload.rs` (it fails without the change); test hook `MemoryStore::refuse_reads` / `accept_reads`.
## 2026-11-25 · Delete asks first

- Deleting a chat from its row's menu or the title menu opens Isonomic UI's alert dialog (design 0.2 `SessionMenu/05_DeleteConfirm`, decided in issue #49): 440 wide, centred, "Delete chat?", the chat's title in the body, Cancel and a danger-tone Delete. Enter deletes; Escape, Cancel and the close button keep the chat. `ChatShell::confirm_delete` asks; `delete_session` still deletes at once for callers that already asked.
- Tested in `crates/chat/tests/confirm.rs`; screenshot state `delete-confirm`. C6 in `docs/design-diffs.md` is Fixed.
- "Delete all chats" asks the same way once Settings exists; there is no Settings screen yet.
## 2026-11-16 · The window's header in Chat's sidebar

- `SDK_REF` moves to `c849f9d` (`UI_REF` `fa70c9b`); harness unchanged. `Cargo.lock` needs no change.
- Chat hosts the window's shell header (`hosts_shell_header`), so the window draws no rail beside it. Expanded, the toggle and then the header (the product wordmark) top the sidebar; collapsed, the header (the app's icon) and then the toggle. Chat tells the window on every fold and open (`set_sidebar`) and leaves the window's inset above the row. Chat draws its own wordmark only with no window around it (tests, the demo, the screenshot harness), the same one the window places. Tested with the SDK's test host.
- Lists: bullets 4 in with their text 21 in; numbers 4 in with their text 28 in (`bullet_list_indent`, `ordered_list_indent`), measured in `thread-markdown` and `thread-mac`. The reply pager is quiet: arrows and numbers at 12 in the muted colour.
## 2026-11-16 · Writes that fail (non-visual hardening)

- A rename, delete or pin that the host or the store refuses says why (`save_error`), and the chat list loads again, so the sidebar shows what they hold rather than the change that did not land. A chat whose delete was refused comes back; one archived but not unpinned stays deleted (an archived chat is not listed). A later write that goes through clears the error.
- Tested in `crates/chat/tests/reload.rs`: a refused delete, rename and pin, and a failed list that loads on Try again. The delete and pin tests fail without the reload.
- A fork the host refuses, on try again or on an edited prompt, leaves the thread and its versions as they were and reports the error; the next attempt goes through (`conversation-ui/tests/refusals.rs`).
- Test hooks: `ScriptedHost::refuse_next_write` (rename, archive, fork) and `MemoryStore::refuse_writes` / `accept_writes`.
