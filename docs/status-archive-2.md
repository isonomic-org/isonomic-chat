# Status, the earliest entries

Moved from `status-archive.md` to keep files under the size limit. Newest first.

## 2026-11-16 · SDK at abe8fe4: the run status bar and no attaching

- `SDK_REF` moves to `abe8fe4` (its `UI_REF` is isonomic-ui `b4c9670`); harness unchanged at `ae4f1a7`. `Cargo.lock` needs no change.
- The kit's status bar is now Isonomic UI's `RunStatusBar`: stopped (with Continue, passed as its action label), cut short (Continue), failed (Try again), reconnecting (also for an app away) and closed. Its button is the child `"action"` of the bar's id (`conversation_ui::action_button`).
- Both composers turn attaching off (`set_attach(false)`): no plus button, no dropped files, no pasted images.
- The `gpui` alias stays: `#[ui::test]` still resolves through it.
- Chat's icon list gains the bar's icons.

## 2026-11-16 · The chat list by newest version

- A chat is as new as its newest version: the list orders chats by the latest change in any of their sessions (`conversation_model::newest_first`), pins still on top in pin order. A fork whose first session is not listed (an archived chat) never shows as a chat.
- Tested in the model and against the SDK test engine: an older chat with a version made after a newer chat is listed first, and is not without the change.

## 2026-11-16 · Editing a prompt

- Edit (under a prompt, on hover) opens Isonomic UI's textarea in place of the bubble, with Cancel and Save; Mod-Enter saves. Save forks before that turn with reason edit and sends the new prompt there; it is a new version of the turn, in the same pager as try again. An empty or unchanged prompt does nothing.
- Tested on the scripted host (cancel keeps the prompt; save makes a version with reason edit; the pager brings the first prompt back) and on the SDK test engine (the fork carries reason edit and runs the new prompt). Screenshot state `thread-editing`.
- Chat's icon list now has the icons try again, the pager and edit draw.

## 2026-11-16 · Try again and versions

- Try again on any finished reply forks the session before that turn (`ConversationHost::fork`, over `sessions.fork` with reason regenerate) and sends the prompt again in the fork. The fork shares every turn before it; the turns after it belong to the old version.
- The session and its forks at one point are that turn's versions. The pager under the reply switches between them, and later turns follow the version shown. Trying a version again adds a sibling, never a fork of a fork at the same turn.
- A conversation is a tree of sessions; what shows is the path to the session most recently changed (`conversation_model::plan`). Opening a chat folds each session on the path up to where the next forked, then follows the last live.
- Chat's list leaves forks out: a version is inside its chat. Rename, pin and delete act on the chat's first session.
- Tested on the scripted host (last reply, an earlier reply, reopening with nested versions) and on the SDK's real host loop (a fork's own log holds only `session.start` and `forked`; Chat lists one chat and opens the newer version).
- Not built: editing a prompt (the same fork with reason edit, once there is an edit box), and ordering the chat list by a chat's newest version.

## 2026-11-16 · Thread to design 0.2

- The thread scrolls across the whole panel, with its turns in a centred 720 column: 32 above the first and below the last, 24 between them. The composer and the status bar sit in the same column, with the disclaimer under them.
- A prompt is a muted bubble on the right, with Copy under it on hover. A reply is unboxed Markdown, with Copy under it once it ends. Each code fence has its own copy button.
- The status bar above the composer is the one place for reconnecting, a closed conversation, and how the last reply ended: stopped or cut short (with Continue), failed (in the danger colours, with Try again). Earlier turns keep a quiet note. It is a placeholder until Isonomic UI has a run status bar.
- Code block decision: the text view's own fences, because a fence renderer that keeps the streaming re-parse tail-only is not proven (request O9).
- Differences: `docs/design-diffs.md`, T1 to T10. Requests to Isonomic UI: O9 to O12.

Older entries: `docs/status-archive.md`.

## 2026-11-15 · The conversation kit and the thread screen

**Built**

- `conversation-model`: `Thread` folds one session's durable events and live deltas into turns (prompt, reasoning, the reply as it streams, tools used, how it ended). Continue keeps the reply in its turn; deltas are dropped once the durable reply arrives; replayed events are ignored; optimistic sends are adopted by the host's turn. `InputState` says whether the composer offers send, stop or continue. Tested against every harness fixture.
- `conversation-host`: the `ConversationHost` trait, shaped like the SDK's `sessions.create`, `send`, `continue`, `cancel`, `rename` and `subscribe`, and `ScriptedHost`, which plays scripted replies word by word as real schema-v1 events, with stop sealing the partial reply and continue finishing it.
- `conversation-ui`: `ConversationView`, with the turns in Isonomic UI's virtualized message scroller, replies as Markdown text views, the thinking indicator before the first words, Isonomic UI's prompt composer (send turns into stop) and continue under a reply that stopped early. The app supplies the words (`ConversationCopy`) and the preset.
- Chat: a chat sent from home opens a `ConversationView` on the host Chat is given and sends the prompt; the model's title replaces the one taken from the prompt. The window build has no host until the SDK adapter lands, so there a new chat opens empty, as before. `examples/dev` runs on the scripted host.

**Deviations**

- Until the SDK's agents interface re-exports it, the kit takes harness-core and its fixtures over ssh at the SDK's URL and rev (`7a73ca6`), so cargo builds one copy. CI passes `harness: true`.
- Approvals, questions, artifacts and memory are not folded yet; the turns ignore them, and unknown events and blocks show as a generic notice.

## 2026-11-15 · The app crate the window mounts

**Built**

- `crates/chat` is now the package `isonomic-chat`. `isonomic_chat::Chat` implements `isonomic_app::App`: id `com.isonomic.chat`, no capabilities yet, and a root view that holds the shell. Key bindings register once, however often the window builds Chat. `open_link` takes `isonomic://chat/...` links.
- Pins: isonomic-ui moves to `4c912f6` (`UI_REF`, the V1 pin for every repo); isonomic-sdk is pinned at `7c488c2` (`SDK_REF`), both sibling checkouts by path, the same as the window. CI checks out both (`ui: true`, `sdk: true`).
- Tests against the SDK's `TestHost`: the manifest, mounting on home with no calls of Chat's own, link routing, and building twice.

**Deviations**

- `rust-toolchain.toml` pins Rust 1.98: rustfmt on 1.100 reformats macro arguments in Isonomic UI, and `cargo fmt --all --check` covers path dependencies.
- `isonomic-chat`'s `test-support` feature asks tokio for its multi-thread runtime, which the SDK's `TestHost` needs (requests O6).

## 2026-11-15 · Shell and home with static data

**Built**

- `ChatShell`: the sidebar (collapse, new chat, the chats with pinned first), the header (Untitled and Titled, a title menu with Rename and Delete, Share disabled, the context-panel toggle) and the context panel (Context and Tools tabs; Files and Connections). Hidden panels give their width to the conversation.
- Home: a greeting by part of the day, the composer stand-in, and five starter chips that each open three example prompts and fill the composer.
- Sending from home starts a chat titled from the first line of the text, below the pinned ones, and opens it. The opened chat shows where the conversation view will go, above the docked composer and the disclaimer.
- Keys: new chat (`cmd-shift-o`, `ctrl-shift-o` off macOS), toggle the sidebar (`cmd-\`), toggle the context panel (`cmd-alt-\`).
- `examples/dev` opens Chat in a window; `crates/chat/tests/screenshots.rs` renders home, composing, titled and panels-hidden in light and dark.
- Tests: 4 unit tests (greeting, day parts, titles, starters) and 6 headless window tests (routing, send waits for text, sending starts a chat, keyboard new chat, keyboard toggles, rename keeps a new title and ignores an empty one, delete).

**Looked at**

All eight screenshots, against `ChatView/01`, `02` and `04`. Every visible difference is in `design-diffs.md`.

**Deviations**

- The composer, the chat list and the context rows are stand-ins until Isonomic UI ships the Prompt composer (#206) and the stacked list item (#208) and the kit's bindings exist.
- Long chat titles clip without an ellipsis (isonomic-ui #210).

## 2026-11-15 · Step 0

**Prerequisites** (checked 15 Nov 2026, 00:30 UTC)

| Prerequisite | State | Where |
| --- | --- | --- |
| Harness event schema v1 and its JSON Schema | **missing** from `main`: in review | isonomic-harness #2 (schema), stacked under #3 |
| The six recorded fixtures | **missing** from `main`: in review | isonomic-harness #3 (`chat-basic`, `tool-heavy`, `approval`, `autonomous-run`, `client-gone`; `long-thread` is generated) |
| 13 conversation components on the Isonomic UI foundation | **present** on `main` (merged 15 Nov, 00:19 UTC) | Permission prompt #28, Approval card #27, Tool call card #21, Question card #20, Message action bar #24, Reasoning steps panel #17, Sourced answer #18, Streamed answer #166, File dropzone #19, Code block #5, Model picker #29, Usage meter #30, Context usage grid #153. Sev 1 #168 (Enter on Deny) is closed |
| SDK app crate | **missing** from `main`: in review | isonomic-sdk #6 (App trait, AppContext, TestHost), blocked on the `UI_DEPLOY_KEY` secret |
| SDK agents interface and the testkit's in-memory host | **missing** | Not started in isonomic-sdk; `Capabilities.agents` is reserved |
| The desktop's move to Isonomic UI | **missing** | isonomic-desktop `crates/ui` still re-exports the old kit at 0.9.1 |

None blocks the shell and home, which use static data. The kit's models wait for the schema and fixtures; the SDK adapter waits for the agents interface; linking into the desktop waits for its move to Isonomic UI.

**Built**

- The workspace: `crates/ui` (the single re-export of Isonomic UI and its conversation components), stubs for `crates/chat`, `examples/dev` and `xtask`.
- `UI_REF` pins Isonomic UI at `4cf5571`, the first commit with all 13 conversation components. Isonomic UI is a sibling checkout at `../isonomic-ui`, as in isonomic-sdk.
- `Cargo.lock` is committed; CI runs with `--locked`.
- `docs/promotions.md` (the desktop inventory), `docs/name-map.md` and three requests in isonomic-ui: #206 (Prompt composer), #207 (Thinking indicator), #208 (stacked list item).

**Deviations**

- `ci.yml` came from the master session's `ci/shared-workflow` branch (issue #1); `ui: true` and `--locked` were added through the web editor, since the connector cannot write workflow files.
- Pushes that the connector cannot carry (`Cargo.lock`) go through git on Omar's Mac with the @QadmiriOmar login.
- The first CI run did not start: GitHub reported the organization's Actions payment or spending limit. Escalated to Omar.
