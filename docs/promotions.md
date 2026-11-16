# Promotions

Every decision about where a type, helper, binding or screen lives: the kit (`crates/conversation-*`), Chat (`crates/chat`), or neither (a request to the host). The test: could Agents, an in-app assistant or the Activity tab use it? Newest first.

## 2026-11-16 · Models watch and the status bar's buttons

| Piece | Where | Verdict | Why |
| --- | --- | --- | --- |
| `ConversationHost::availability` | `crates/conversation-host`, `crates/conversation-sdk` | Kit | Any app that waits on a model asks the same; mirrors `models.info` |
| Watching an asked model until it is ready | `crates/chat` `shell/sessions.rs` | Chat | Only Chat asks the window for installs |
| The status bar's buttons, Dismiss, and a failed load with Try again | `crates/conversation-ui` | Kit | Every conversation surface |
| Paging the session list | `crates/conversation-sdk` | Kit | The adapter's job: the kit asks for every session |

## 2026-11-16 · Editing a prompt

| Piece | Where | Verdict | Why |
| --- | --- | --- | --- |
| The edit field on a sent prompt, and `ConversationView::edit` | `crates/conversation-ui` `view/edit.rs`, `turn.rs` | Kit | Any conversation surface edits a prompt the same way; the words come from `ConversationCopy` |

## 2026-11-16 · Try again and versions

| Piece | Where | Verdict | Why |
| --- | --- | --- | --- |
| `ConversationHost::fork`, `ForkAt`, `SessionEntry::forked_from` and `created_at` | `crates/conversation-host`, `crates/conversation-sdk` | Kit | Mirrors `sessions.fork`; any app with sessions can branch them |
| Carried turns (`Thread::with_turns`, `turns_through`, `Turn::start_seq`) and the version planner (`plan`, `Branch`, `Slot`, `Path`) | `crates/conversation-model` | Kit | Pure folding over sessions, no product terms |
| Try again and the pager | `crates/conversation-ui` `view/versions.rs` | Kit | Every conversation surface regenerates the same way |
| Leaving forks out of the chat list | `crates/chat` `shell/sessions.rs` | Chat | Which sessions are "chats" is Chat's list |

## 2026-11-16 · Thread to design 0.2

| Piece | Where | Verdict | Why |
| --- | --- | --- | --- |
| The status bar above the composer (reconnecting, closed, how the last reply ended, continue and try again) | `crates/conversation-ui` `status` | Kit | Any conversation surface shows the same states with the same actions |
| Copy under a prompt and a reply, copy in code fences | `crates/conversation-ui` `turn` | Kit | Generic; the label comes from `ConversationCopy::copy` |
| The centred 720 column and its dock | `crates/conversation-ui` `view`, `metrics::column_width` | Kit | Every conversation surface reads the same way; the app keeps only the disclaimer under it |

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
