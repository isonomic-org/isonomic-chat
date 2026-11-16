# Requests

What this repository needs from others, and what others asked of it. Newest first. Each entry says exactly what to add.

## Out

### O12 · Isonomic UI: a run status bar

**To:** isonomic-ui (as a `request` issue; this session cannot file there, so the orchestrator files it). **Status:** open. Design 0.2 `RunStatusBar` (`thread-states-part-2.md`): a 720 x 44 bar above the composer with an icon, a message and buttons; statuses Running, Stopped, Error and Done. The kit draws a plain placeholder (`conversation-ui/src/status.rs`) on theme tokens until it lands.

### O11 · Isonomic UI: hide the prompt composer's attach button

**To:** isonomic-ui. **Status:** open. `PromptComposer` always shows its plus (attach) button; Chat has no attachments in version 1. Add a switch, like `set_dictation`, to leave it out.

### O10 · Isonomic UI: streamed answer with a Markdown body and no question

**To:** isonomic-ui. **Status:** open, for after version 1. `StreamedAnswer` renders a subset of Markdown and draws the question above the answer. Chat needs the full Markdown text view as its body and a mode without the question, since the prompt is already shown as a bubble. Until then replies use the text view.

### O9 · Isonomic UI: host-rendered code fences that stay tail-only

**To:** isonomic-ui. **Status:** open. To draw fences with `extra::code_block::CodeBlock`, the text view needs a fence renderer that keeps the streaming re-parse tail-only, with a test. Until then replies keep the text view's own fences, with a copy button in each fence's corner (`code_block_actions`).

### O8 · SDK: re-export the conversation components

**To:** isonomic-sdk. **Status:** done in isonomic-sdk #28 (`isonomic_app::extra`, feature `extra`); the direct path dependency is gone. `isonomic_app::ui` re-exports `isonomic-ui` but not `isonomic-ui-extra`, which holds the prompt composer, thinking indicator, approval card, tool call card and the rest of the conversation components. Add `pub use isonomic_ui_extra as extra;` next to `pub use isonomic_ui as ui;`, so apps take them from the SDK too. Until then `crates/ui` depends on `isonomic-ui-extra` directly, at the SDK's `UI_REF`.

### O7 · Harness: continue skips the title step

**To:** isonomic-harness. **Status:** done in isonomic-harness #37 (`f061211`); tested here. `harness-loop` `machine.rs` (around line 417) takes the turn's last `assistant` to decide whether a reply can be continued. On a session's first turn the title step's `assistant` (purpose `title`, stop `end`) comes after the reply's, so continuing a first reply cut off at the output limit or stopped is refused with "The reply is complete." Skip assistants from steps whose purpose is not `reply`.

### O6 · SDK: tokio's multi-thread runtime for `test-support`

**To:** isonomic-sdk. **Status:** done in isonomic-sdk #22; the workaround is gone. `isonomic-app`'s `testing::TestHost` calls `tokio::runtime::Builder::new_multi_thread()`, but `crates/app/Cargo.toml` enables `rt-multi-thread` only in its dev-dependencies. An app that turns on `isonomic-app/test-support` outside the SDK workspace fails to compile. Add: `test-support = ["dep:isonomic-testkit", "isonomic-ui/test-support", "tokio/rt-multi-thread"]`. Until then `isonomic-chat`'s `test-support` asks tokio for it.

### O5 · Isonomic UI: sidebar labels

isonomic-ui #210. Long `SidebarMenuItem` labels clip without an ellipsis.

### O4 · Desktop: icons and linking

**To:** isonomic-desktop. **Status:** open.

1. Chat draws icons from Isonomic UI's full catalog (`isonomic_ui::assets::IconName`), not only the default set. The window registers `isonomic_ui::assets::AllAssets`, or an asset source that includes the icons in `chat::icons::ALL`.
2. Chat depends on Isonomic UI through `../isonomic-ui`, pinned by `UI_REF`, like the SDK. Linking `chat` at a pinned revision works when the window and Chat resolve the same `isonomic-ui` checkout; otherwise GPUI is built twice and Chat's views cannot be placed in the window's tabs.

### O3 · Isonomic UI: stacked list item

isonomic-ui #208. For the kit's `ContextItem`.

### O2 · Isonomic UI: Thinking indicator

isonomic-ui #207, from proposal #14. For the kit's `ThinkingIndicator` (story 3).

### O1 · Isonomic UI: Prompt composer

isonomic-ui #206. For the kit's `PromptComposer` (home and thread).

### O0 · `ci.yml`

**Status:** done in #2. The master session's branch `ci/shared-workflow` (issue #1) was merged into the Step 0 branch, and two inputs were added to its call of the shared workflow through the web editor: `ui: true` (Isonomic UI as a sibling checkout at `UI_REF`, which needs the `UI_DEPLOY_KEY` secret) and `features: --all-features --locked`. When `xtask` gains the product-term lint and the boundary check, `xtask: lint boundary` follows the same way.

## In

### I1 · Desktop: a Chat app crate the window can mount

**From:** isonomic-desktop (issue #6). **Status:** done. Package `isonomic-chat` at `crates/chat`, type `isonomic_chat::Chat` implementing `isonomic_app::App` with id `com.isonomic.chat`. It pins isonomic-sdk (`SDK_REF`) and isonomic-ui (`UI_REF`) as sibling checkouts by path, the same pins as the window, and is tested with `isonomic_app::testing::TestHost`. Mount it with `isonomic_app::register::<isonomic_chat::Chat>()`. The window must call `ui::init` (Isonomic UI's init) before mounting it, as it does for every app. `open_link` takes `isonomic://chat/...`; `search` returns nothing yet. The manifest asks for no capabilities while Chat runs on static data; it gains `agents` once the SDK's agents interface ships.
