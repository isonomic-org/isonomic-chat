# Status

Newest first. Every deviation from the brief is recorded here.

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
## 2026-11-16 · The thread against the 0.2 frame

- The chat header has no rule under it: design 0.2 has none (`app-shell.md`); the rule came from an older frame. The title stays 14 / 20 medium, the bundled Medium face.
- Reply text is 16 / 24 (it inherited about 26). The prompt bubble takes the theme's `accent` neutral, the closest to the design's `gray/100`, at radius 18; `muted` nearly vanished on white.
- The reply's action row is mid grey until pointed at, then the text colour (the bar's quiet mode), with 16 px icons in 28 px buttons.
- Lists, from `message-content.md`: a bullet 4 in and its text 21 in, as designed. Numbers share the right-aligned marker column, so their text is 24 in where the design has 28.
## 2026-11-16 · SDK at a6276aa (UI_REF 2b8a583): a quiet composer

- `SDK_REF` moves to `a6276aa`; harness unchanged. `Cargo.lock` needs no change.
- Both composers (home and the thread) take Isonomic UI's quiet focus: no blue border, as design 0.2 draws them. Empty, each is 96 high (the comfortable size at `2b8a583`). The palette's chip reads "Esc" on macOS too.
- The greeting and empty-state icon is `Beacon`, a four-point star: the closest drawing in the set to the design's sparkle with a plus (`Sparkles` draws a wand). Omar may choose another.
- Thread spacing checked against `screens/thread.md`: 58 from a prompt bubble to its reply's first line (6, the prompt's 28 actions row that keeps its space when hidden, 24), 12 from the reply to its action row, 24 to the next prompt. All match.
## 2026-11-16 · Earlier replies stay whole; reloads keep deletes; the Mac comparison

- **Thread:** the conversation view owns the text view state behind each reply, kept in step after every update, instead of the one the text view finds by element id each frame. A reply that has ended is drawn from one parse of its whole text. On the Mac, a finished reply was drawn as its first streamed token ("Here") once a later turn started; that was not reproduced here (scripted host and SDK test engine, frames drawn while parses were pending), so this removes every path through which a finished reply could show a partial parse. Tested in `conversation-ui/tests/stream.rs`; screenshot state `thread-earlier-reply`.
- An opening emphasis or code marker at the very end of a reply (a reply stopped at `**`) is not drawn, and a reply with nothing to draw shows no Copy or Try again.
- **Chat list:** a reload answered after a delete no longer brings the chat back; `is_reloading()` and `loads_finished()` tell the window when a reload has finished (requests I2, I3).
- **Mac comparison:** New chat and Search sit 8 from the edge, the same width as the chat rows; a chat's header has its 1 px rule. The composer's blue focus border, its height (about 107, not 96), the four-point sparkle and the "Esc" chip on macOS need Isonomic UI (O15). The palette is centred on the window, as the reference draws it.
- The screenshot harness's scenes moved to `tests/screenshots/scenes.rs` (file size).
## 2026-11-16 · SDK at 8f21cb6: the window's QA ids

- `SDK_REF` moves to `8f21cb6` (`UI_REF` `527d955`, stable ids); harness unchanged. `Cargo.lock` needs no change.
- Each chat row is `chat-row-<session id>`; its menu items are `chat-row-menu-<id>-open`, `-pin`, `-rename`, `-delete`. Palette rows are Isonomic UI's `command-<id>` (time at `command-<id>-meta`); the model picker's list and rows come from `model_picker::list_id` and `row_id`. All are tested in `tests/ids.rs` and listed in `docs/accessibility.md`.
## 2026-11-16 · Model names, and the Mac window checked

- A model's name is shown without its quantisation tag: "Qwen3.5 2B (Q4_K_M)" reads "Qwen3.5 2B" (`conversation_model::display_name`, used by the SDK adapter; ids unchanged).
- A test checks that every icon Chat draws resolves in Isonomic UI's catalog (`AllAssets`). Chat registers no asset source; the window does.
- Two screenshot states at the Mac window's size (1100 wide, its 2200 px capture at 2x): `home-mac`, `thread-mac`. At that size the 720 column fills most of the 812 content area, as the design's rule gives.
## 2026-11-16 · The wordmark, by Omar's brand rule

- `SDK_REF` moves to `a3ac3bf` (`UI_REF` `a1b30e1`); harness unchanged.
- The sidebar's wordmark is Isonomic UI's `Wordmark::product("Chat")` at 20: "Isonomic" in IBM Plex Sans Regular, "Chat" in Light. The home greeting is back in the theme's sans, 34 / 42. No serif anywhere in Chat: the serif is for long-form reading. A6 and H1 in `docs/design-diffs.md` follow this rule, not the reference.
- A chat row's More button has the id `chat-row-menu-<session id>` for the window's QA. The other QA ids wait on Isonomic UI (`docs/accessibility.md`).
## 2026-11-16 · SDK at 1f28327: the serif, and 28 px buttons

- `SDK_REF` moves to `1f28327` (`UI_REF` `0a8f39d`); harness unchanged.
- The wordmark (20 / 28, medium: the heaviest bundled weight) and the home greeting (34 / 42, regular) are in the bundled serif, as design 0.2 draws them.
- Copy and Edit under a prompt and the reply action row are 28 px buttons with 16 px icons.
- What still differs from the design is decided for version 1 or is Isonomic UI's tokens: see `docs/design-diffs.md`.
## 2026-11-16 · SDK at 4947e43: chat icons in search, list indent

- `SDK_REF` moves to `4947e43` (`UI_REF` `ecd21d1`, isonomic-ui #385); harness unchanged.
- Search rows show the chat icon (any catalog icon now), and New chat its plus. List text sits 28 in, as designed.
- Still open: Copy under a prompt and the reply action bar take no icon size (only `Button` does), so those stay 24 px buttons with 16 px icons; Edit matches Copy. The serif wordmark and greeting stay the theme's sans until Omar chooses a typeface.
## 2026-11-16 · How long ago, in search

- Each chat keeps when it was last active: the newest change in any of its versions, from the host's list, or now for a chat started here. Search shows it on the right of the row in the design's short form ("now", "20m", "2h", "3d", "2w", "4mo", "1y").
- The rest of `docs/design-diffs.md` is either fixed, decided, or with Isonomic UI (#385: a chat icon on palette rows, list indent, icon size apart from button size). The serif wordmark and greeting wait on whether the theme has a serif family.
## 2026-11-16 · SDK at 72aca92: Isonomic UI's new options in use

- `SDK_REF` moves to `72aca92` (`UI_REF` `4f1207b`, isonomic-ui #381); harness unchanged. `Cargo.lock` needs no change.
- Sidebar: the design's padding and 1 px gaps, 32 px rows that take focus (Tab, Enter, Space, Up, Down), Pinned folds. The header title truncates and keeps its caret; Delete is destructive in both menus.
- Search: 640 wide, "Search or ask…", Esc in the field, 36 px rows, groups kept while filtering, the design's hints, nothing selected until the person types or moves, "No chats match.", focus held inside.
- Composer: the comfortable size, round 32 px send and stop. Model picker: "Model" header, name-only trigger, trailing check, chosen-row fill, the design's row padding, no footer.
- Thread: list indent and marker gap; the reply bar's pager first and "Try again". Buttons under a prompt and a reply stay 24 px so their icons stay 16 px.
- Still open: palette chat icon and meta, hints as key chips, list text at 28, icon size apart from button size (`docs/design-diffs.md`).

Older entries are in `status-archive-3.md` and `status-archive-2.md`.
