# Status, the earliest entries

Moved from `status-archive.md` to keep files under the size limit. Newest first.

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
