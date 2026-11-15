# Status

Newest first. Every deviation from the brief is recorded here.

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
