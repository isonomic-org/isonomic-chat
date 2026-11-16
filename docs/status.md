# Status

Newest first. Every deviation from the brief is recorded here.

## 2026-11-16 · Final layouts of the version 1 screens

Built from the components Chat already uses, with Isonomic UI's spacing, type and theme tokens; nothing restyled, no new widgets, no new agent components.

- **Home:** Isonomic UI's prompt composer, the same one the thread uses, with the model menu from the catalog; the picked model runs every new chat (it was only applied at the view level before, and is now tested end to end). With no chat model installed, one button asks the window to install the recommended one.
- **Thread:** the divider of a compaction now sits after the last turn the host confirmed, never after a send in flight (it had landed under the new prompt).
- **Narrow and wide:** below 1,100 px the context panel gives way, below 860 px the sidebar too; the header's toggle still opens it, and both come back when the window widens. The header title truncates and Share keeps only its icon.
- **Live versus examples:** on a backend, the greeting names no one and the context panel shows its empty state instead of example connections.
- **Chat list states:** "Chats you start appear here." and a failed list with its reason and Try again (`ChatShell::reload`); a failed write says so.
- **Rail icon:** `chat` from Isonomic UI's icon catalog (was `message-circle`, which is not in it), with a test that it resolves.
- **Screenshots:** `tests/screenshots.rs` now renders 13 states in both themes, including live ones on the scripted host (thread, narrow thread, streaming, errors, live home, models, failed list, install a model). All were looked at.
- **Keyboard:** every control is an Isonomic UI button or field, so Tab reaches it; the sidebar, context panel and new chat keep their shortcuts.
## 2026-11-16 · Fixes found while laying out the screens

- Rail icon: the manifest names `chat`, which Isonomic UI's icon catalog has (`message-circle` is not in it, so the window's rail showed a "C"). A test checks that `icons/<name>.svg` resolves.
- A model picked in one chat now runs every new chat. The shell kept the pick but never handed it to a new conversation, and a new conversation's catalog arrives after its first send could read it. The view now keeps a pick until its catalog has loaded. Tested end to end.
- A compaction's divider sits after the last turn the host confirmed, never after a send in flight.
- The final layouts wait for the version 0.2 layout reference (`docs/design/v0.2/`).

## 2026-11-16 · SDK 439a069: the QA'd components

- `SDK_REF` moves to `439a069` (its `UI_REF` is isonomic-ui `2e3116b`, with all 86 version 1 components through QA); harness unchanged at `ae4f1a7`. `Cargo.lock` needs no change.
- Behaviour from the components, checked against Chat: Tab skips disabled text fields (the composer disabled on a closed session); icon-only buttons are named by their tooltip (the sidebar and context panel toggles); a focused toggle draws a focus ring; the prompt composer wraps its controls below about 430 px. Every test passes unchanged.

## 2026-11-16 · SDK 99630cc: preset version 3, compaction, a conversation too large

- `SDK_REF` moves to `99630cc`; every harness crate is at `ae4f1a7`, the SDK's string character for character.
- Chat preset version 3 caps replies at 1,024 tokens. A reply cut at the limit is `Truncated` and offers Continue, which finishes it in the same turn. Tested with the scripted host's `cut_after`.
- `compacted` with a summary is a quiet divider after the last turn (`Turn::divider_after`, or `Thread::divider` before any turn), showing the summary's note. It is never a message. Tested on the scripted host and on the `autonomous-run` fixture.
- `turn.end` `failed` with `context.too_large` and no steps shows its `reason` as written; the composer stays usable, and Try again is not offered, since the same prompt would not fit either (`Turn::error_code`, `TOO_LARGE`).
- `scripted.rs` stays under 20 KB: `ScriptedReply` moves to `scripted/reply.rs`.

## 2026-11-16 · SDK 12ddf2f: the picked model, stream ends, installing a model

- `SDK_REF` moves to `12ddf2f`; every harness crate is at `9a3809f`, the SDK's URL and rev character for character. `Cargo.lock` regenerated.
- A new chat passes the picked model to `sessions.create`; the session's model shows in `SessionEntry.model` (from the summary, or from `session.start` in the scripted host). An app without the `models` capability that names a model gets `capability_denied`, which is tested.
- The catalog's recommendation comes from `ModelInfo.recommended`.
- Stream ends: `SessionUpdate::Gap` ends the kit's stream, so the view follows again from its last `seq` and reads what was skipped. `SessionUpdate::Ended` (a purge) becomes `Update::Ended`, which closes the thread, so the view stops following.
- Installing a model: `Backend::install_model` is the window's install step, wired in `Chat::build` to `AppContext::request(WindowRequest::InstallModel { model })`. Chat calls it on `ModelUnavailable`, and from home: when no chat model can answer, home shows one button that asks to install the recommended model, in place of a composer that cannot reply.
- Chat preset version 2: no test here asserts the preset version or its text, so nothing changed.

## 2026-11-16 · Stop and continue, checked after the first run on the Mac

- From the Intel Mac run (a reply stopped at its first character, then continue left it `Stopped` with no new text): the kit's path is correct. With the scripted host, a reply stopped at its first delta continues in the same turn; on the real harness, a late or doubled cancel does not stop the continue that follows. Both are tested now.
- Fixed: a continue the host refused was dropped silently, so the turn stayed `Stopped` with nothing said, which matches what was seen. The reason now shows on the turn (`Turn::refusal`) and the view reports `HostError`. The refusal clears when the host opens the turn again.

## 2026-11-15 · Retry and error states

**Built**

- Model: a turn shows `retrying` while the host retries a transient error, cleared when the reply or the turn's end arrives. `Thread::awaiting_client` follows `session.status`. `Thread::take_retry` gives the prompt of a failed or rejected last turn; a rejected send the host never opened is dropped first, so trying again replaces it.
- View: "Try again" under a failed or rejected last turn. A note while the host retries. A line above the composer while reconnecting, while a tool's app is away, or once the session is closed, with the composer disabled when closed.
- Reconnect: when a session's stream ends, the view follows it again from the last `seq` it saw, waiting 250 ms and doubling to at most 8 s. Nothing is missed or shown twice. A refusal other than `disconnected` stops it, and so does a closed session.
- `ScriptedHost::disconnect` ends every open stream, for tests.
- CLAUDE.md: the budget rule (one paid check run per push).

**Not yet**

- The first-run hand-off from the window: its shape is not specified here (asked in Worker reports).

## 2026-11-15 · Model choice

**Built**

- Kit: `ModelCatalog` (chat models only; the recommended one first, then those that can answer, then downloading, not installed and unavailable) with `choose`, which names a listed model that can answer, or nothing. A model is never assumed to exist. `ConversationHost::models` loads it: `SdkHost` reads `models.list` plus each model's `models.info` state, and a model whose state cannot be read is `Unknown`, never installed.
- `ConversationView` offers the catalog in the composer's model menu. A pick applies to the session the view creates (`NewSession.model`) and is reported as `ModelSelected`. `capability_denied`, `unknown_model` and `model_unavailable` read in plain words (`ConversationCopy`); `model_unavailable` also reports `ModelUnavailable` so the app can offer the download.
- Chat applies the last picked model to every new chat. The manifest asks for `models` with `any: true`, local only.

**Waiting on the SDK**

- `ModelInfo.recommended` and `sessions.create { model }` are in review (needs-master). Until they merge, the catalog has no recommendation and the preset's model runs. Switching model inside a chat is not in version 1.

## 2026-11-15 · Everything through the SDK

- `SDK_REF` moves to `c0d2bf7`; every harness crate is pinned at `f061211`, the SDK's URL and rev character for character.
- The conversation components come through the SDK (`isonomic_app::extra`); no crate depends on isonomic-ui directly. The `../isonomic-ui` checkout stays for the macros patch.
- Continuing a first reply after its title step works (harness #37) and is tested.

## 2026-11-15 · One UI pin

- `UI_REF` is gone. The shared workflow reads the pin from the SDK checkout (isonomic-org/.github #8), so the SDK's `UI_REF` is the one pin.

## 2026-11-15 · Chats that persist

**Built**

- Kit: `ConversationHost` gains `list` (newest first, archived or not, by title text, with a limit) and `archive`, as `SessionEntry` and `SessionQuery`. `SdkHost` maps them onto `sessions.list` and `sessions.archive`; `ScriptedHost` keeps them in memory.
- Chat: with a `Backend` (a host and a store), the sidebar lists the host's chats instead of the examples. Opening one replays its history; rename goes to `sessions.rename`; delete archives it and drops its pin. A new chat takes the host's session id as soon as the host creates it. A list that arrives after a chat was started keeps that chat.
- Chat's own state is only the pins, in its schema `chat` (`store.rs`): a `pins` table of `session text` and `pinned_at timestamptz`, one statement per call. The manifest now asks for `sql` and declares the schema. `MemoryStore` serves tests and the dev window. Writes in flight are counted (`is_saving`), and the last failed write is kept (`save_error`).
- Tests: the mounted app lists the host's chats with their pins, and writes a pin in Chat's schema; a stored chat reopens with its history, and rename and delete reach the host.

**Notes**

- The SDK testkit's database is scripted and runs no SQL, so the tests check the statements Chat sends and the rows it reads back, not a real database.
- History lives in the host's session log, so `blobs` is not used yet; attachments will use it.

## 2026-11-15 · Isonomic UI through the SDK

- `SDK_REF` moves to `c68bf0d`. `crates/ui` re-exports `isonomic_app::ui` instead of depending on isonomic-ui, so the SDK's `UI_REF` is the pin. The conversation components still come from `isonomic-ui-extra` directly until the SDK re-exports them (requests O8). `UI_REF` stays until the shared workflow reads the pin from the SDK checkout.
- The tokio workaround for the SDK's `TestHost` is gone (isonomic-sdk #22).

## 2026-11-15 · The SDK adapter

**Built**

- `conversation-sdk`: `SdkHost` implements `ConversationHost` on the SDK's agents interface (`sessions.create`, `send`, `continue`, `cancel`, `rename`, `subscribe`) under the app's own token. Session updates pass through unchanged: the SDK re-exports the same harness-core the kit folds. Engine errors keep their wire code and words.
- Chat's manifest asks for `agents` and nothing else, and `Chat::build` runs every new chat on `SdkHost`.
- `SDK_REF` moves to `21ed50d` (isonomic-sdk main with the agents interface and the testkit's in-memory agents host).
- Real turns in tests, no engine: through `SdkHost` (stream, title, a reply cut off at the output limit, continue in the same turn, stop with nothing running, continue refused), and inside the mounted Chat app (a chat sent from home gets the host's reply under `com.isonomic.chat`).
- Fix: the fold ignores steps that are not the reply. The title step runs after `turn.end` in the same turn, and its text had been added to the reply.

**Not yet**

- The host refuses to continue a turn whose title step ran after it, because the last reply it sees is the title's (requests O7).
- Approvals, questions and client tool results come after V1's plain text turns.

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
