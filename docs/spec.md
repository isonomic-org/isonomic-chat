# Spec: Milestone 0

Condensed from the kickoff brief ("Kickoff: Isonomic Chat (app and conversation kit), Milestone 0", approved 14 Nov 2026). Where this file and the brief differ, the brief wins.

## What this repository builds

1. **The conversation kit** (`crates/conversation-*`): headless models that fold session events into something renderable, and bindings that connect them to Isonomic UI. Used by Chat now; later by the assistant panels in Docs, Tables and Canvas, the Agents app and the desktop's Activity tab. It moves to its own repository (isonomic-conversation) at Milestone 4, so the move must be a directory copy.
2. **Chat** (`crates/chat`): thin. Its screens, navigation, home, settings and preset.

## The kit

1. **Headless first.** `conversation-model` folds events into turns, steps, tool calls, approvals, questions, artifacts, branches, streaming state and optimistic sends. `composer-model` holds the draft, attachments and upload state, mentions, commands and the selected model. Both are tested against all six fixtures.
2. **One way in.** The kit reaches the platform only through a `ConversationHost` trait, implemented by a fixture player and an SDK adapter. It never opens sockets. Until the SDK's agents interface ships, event types come from harness-core.
3. **Slots and registries, not flags:** tool renderers by tool name, message actions, composer buttons and commands, mention providers, empty states, header accessories.
4. **Forward compatible.** Unknown events render as a generic collapsible block; unknown tools with the generic card.
5. **Message bodies** use Isonomic UI's Markdown text view; only the tail re-renders on a token.
6. **Long threads** are virtualized; earlier messages never re-render on a token; scroll stays anchored unless at the bottom.
7. **Tokens only, light and dark, fully keyboard accessible.**
8. **Clean to move:** a lint rejects product terms in kit public types; no kit crate depends on `crates/chat`.
9. **Deny-safe:** approval bindings never default to allow; an ambiguous answer is a denial.

## Chat

| Screen part | Built from | Chat owns |
| --- | --- | --- |
| Sidebar | Kit `SessionList`; Isonomic UI navigation items | Layout, new-chat entry, grouping and pinning defaults |
| Header | Kit `ModelSelector` and `UsageMeter` | Title with rename, the menu, Share (disabled until sharing lands) |
| Home | Kit `PromptComposer`; a chip row | Greeting, starter suggestions, empty state |
| Thread | Kit `ConversationView` and default renderers | Wiring and registered message actions |
| Context panel | Kit `ContextItem` | Panel layout and when it opens |
| Settings | Isonomic UI form components | Default model, reasoning display, styles, opt-in location, delete all chats |
| Preset | Harness preset schema | Chat's production preset, registered at install |

## Milestone 0 stories

1. Message tree (siblings, "2/3", only the selected path sent). 2. Stop and continue. 3. Thinking panel. 4. Titles, rename, delete. 5. Text attachments. 6. Rendering with tokens only. 7. Request context (date, language, opt-in location). 8. Refusals and errors. 12. Delete all chats.

## Order of work

Step 0 and the name map; Chat's shell and home with static data; `conversation-model` and `composer-model` against the fixtures; the host trait and fixture player; the gallery; bindings as Isonomic UI components land; the AssistantPanel; the SDK adapter; wiring Chat to it; the report.

## The gate

All bindings render all six fixtures light and dark; replay streams without jank; headless tests cover branches, edit and regenerate, stop, optimistic send, approvals and questions, unknown events and client-gone; the AssistantPanel runs with a fake client tool and no kit change; the lint and boundary check pass; stories 1–8 and 12 pass against the SDK testkit's in-memory host; one real turn through the engine's host; Chat runs as a desktop tab; docs complete; screenshots compared with Figma; the keyboard-only path works; and on the Intel Mac a 10,000-event thread opens in under 300 ms and scrolls at 60 fps, and switching between two 1,000-message chats takes under 150 ms.
