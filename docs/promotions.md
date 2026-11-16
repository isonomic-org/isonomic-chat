# Promotions

Every decision about where a type, helper, binding or screen lives: the kit (`crates/conversation-*`), Chat (`crates/chat`), or neither (a request to the host). The test: could Agents, an in-app assistant or the Activity tab use it? Newest first.

## Inventory of existing Chat code in isonomic-desktop

Read at `main` `c3c745b` (15 Nov 2026), plus the open pull requests #4 (xtask, draft) and #5 (CI).

| Piece | Where | Verdict | Why |
| --- | --- | --- | --- |
| Chat screens, models or bindings | none | nothing to move | The desktop has no Chat code yet: its crates are `ui`, `engine-link` and `shell` (rail, Home tab, menu bar icon, lifecycle). Chat appears only as a milestone in `docs/architecture/plan/spec.md` |
| Agent loop, tools or session storage | none | nothing to drop | None exist in the desktop; the engine's host owns them |
| The rail and its items (`AppRailItem` in Figma) | `crates/shell` | stays in the desktop | The rail is the window's, not Chat's |
| One `ui` crate re-exporting the component library | `crates/ui` | pattern reused | This repo's `crates/ui` follows it, pointed at Isonomic UI |

## Decisions

| What | Where it went | Why |
| --- | --- | --- |
| Re-export of Isonomic UI and its conversation components | `crates/ui` | One crate names the library and GPUI, so a UI pin bump touches one place. Kit and Chat both use it; it moves with the kit |
| Shell layout: sidebar, header, context panel and their sizes | `crates/chat` `shell/` | A screen and its layout: Chat only |
| Static session list in the sidebar | `crates/chat` `shell/`, temporary | Replaced by the kit's `SessionList` binding once it exists; the list mechanism (grouping, pinning, rename, search) goes in the kit because Agents and the Activity tab list sessions too |
| Greeting, starter suggestions and empty state | `crates/chat` `home/` | Copy and defaults: Chat only |
| Composer on home and in the thread | Isonomic UI's `PromptComposer`; home's thin wrapper in `crates/chat` `composer.rs`, the thread's in the kit's `ConversationView` | Home only adds its placeholder, the starters' draft and the model pick for new chats. Attachments, mentions and commands go in the kit's composer-model when they come |
| Context panel content | `crates/chat` `shell/`, static | Rows become the kit's `ContextItem` binding (Isonomic UI request #208); which sections show and when the panel opens stay in Chat |
| Static demo data (session titles, connections, user name, model label) | `crates/chat` `demo` module | Development data for the shell and home; replaced by the host's data, never shipped as defaults |
| Copy: greeting by part of day, starter prompts, titles from the first message | `crates/chat` `copy` module | Chat's words and defaults. The title heuristic is a stand-in: titles come from the preset's post-turn step run by the host (story 4) |
| Layout sizes (sidebar, context panel, content width, header height) | `crates/chat` `metrics` module | Chat's layout; colours and type stay in the theme |
| Icons Chat draws | `crates/chat` `icons` module, with `ALL` for the host | One place to map roles to Isonomic UI icons |
| Rename and delete of the open chat | `crates/chat` `shell` (header and its menu) | Chat owns rename in the header; persistence moves to the host's session registry through the kit |
| Key bindings: new chat, toggle sidebar, toggle context panel | `crates/chat` `init` | App navigation: Chat only |
| Offscreen screenshots | `crates/chat/tests/screenshots.rs` | Chat's states only; `xtask screenshots` will cover the gallery and every binding |
| Session list and archive (`SessionEntry`, `SessionQuery`, `list`, `archive`) | `crates/conversation-host` | Any app with sessions lists them the same way; mirrors `sessions.list` and `sessions.archive` |
| Pins and the `chat` schema (`store.rs`) | `crates/chat` | Only Chat pins chats; storage is Chat's own schema through the SDK |
