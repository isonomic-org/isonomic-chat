# Name map

Every component in the Isonomic Chat Figma file (`A0H5z7vRJfA8J7HYFy1aKW`, pages "Start a Basic Chat 0.1" and "Components 0.1"), the Isonomic UI component that draws it, and the kit binding or Chat layout that owns it.

Verified 15 Nov 2026 against the Figma file and isonomic-ui `main` at `4cf5571` (the commit in `UI_REF`). "UI" means a module of `isonomic-ui` (`component::…`) or `isonomic-ui-extra` (`extra::…`). Requests are issues in isonomic-ui labelled `request`.

## Conversation components

| Figma component | Isonomic UI component | Kit binding or owner | Status |
| --- | --- | --- | --- |
| `ChatMessage` (role × state: Thinking, Streaming, Complete) | `component::text::TextView` (Markdown) for bodies; `extra::streamed_answer::StreamedAnswer` for a reply with sources; `component::avatar::Avatar` | `Message` (user, assistant, system, error, refusal) | on main |
| `PromptComposer` (state: Empty, Filled, Streaming) | Prompt composer, queued next in #1 | `PromptComposer` over composer-model | **missing**: request #206; Chat shows a placeholder (`Textarea` and `Button`) |
| `MessageActions` (role: Assistant, User) | `extra::message_action_bar::MessageActionBar` (copy, regenerate, version switch, rating) | `MessageActions` registry, `BranchSwitcher` | on main |
| `ThinkingIndicator` | Proposal #14 (orbs and shimmer label), not queued | `ThinkingIndicator` | **missing**: request #207; placeholder is a plain muted label |
| `ThinkingOrb`, `AgentOrb` | Proposal #14 | used inside `ThinkingIndicator` | **missing**: request #207 |
| Thinking panel (reasoning shown apart from the answer) | `extra::reasoning_steps_panel::ReasoningStepsPanel` | `ThinkingPanel` | on main |
| Tool call | `extra::tool_call_card::{ToolCallCard, ToolCallGroup}` | `ToolCallCard` and its renderer registry | on main |
| Approval | `extra::approval_card::ApprovalCard`, `extra::permission_prompt::PermissionPrompt` | `ApprovalCard`, `ApprovalSheet` | on main; the Sev 1 keyboard issue #168 is closed |
| Question | `extra::question_card::QuestionCard` | `QuestionCard` | on main |
| Citations | `extra::sourced_answer::SourcedAnswer` | `CitationMarker`, `SourcesPanel` | on main (web citations are out of scope for M0) |
| Attachments | `extra::file_dropzone::FileDropzone`; `component::attachment` | `AttachmentChip`, `FileCard` | on main |
| Code in messages | `extra::code_block::CodeBlock` | used inside `Message` | on main |
| `ModelSelector` | `extra::model_picker::ModelPicker` | `ModelSelector` | on main |
| Usage | `extra::usage_meter::UsageMeter`, `extra::context_usage_grid::ContextUsageGrid` | `UsageMeter` | on main |
| `ContextItem` | none yet; `component::list::ListItem` composed as a placeholder | `ContextItem` (files, connections, tools in scope) | **missing**: request #208 |
| Long threads | `component::message_scroller`, `component::VirtualList` | `ConversationView` | on main |

## Chat layouts and generic parts

| Figma component or layer | Isonomic UI component | Owner | Status |
| --- | --- | --- | --- |
| `ChatSidebar` | `component::sidebar::{Sidebar, SidebarGroup, SidebarMenu, SidebarMenuItem, SidebarToggleButton}` | Chat `shell::sidebar` layout; the list inside becomes the kit's `SessionList` | on main |
| `chat_item` (row, pin, selected) | `SidebarMenuItem` (`active`, `suffix`, `context_menu`) | kit `SessionList` | on main |
| `ChatHeader` (state: Untitled, Titled) | `component::button::Button` (ghost, dropdown caret; outline for Share) | Chat `shell::header` | on main |
| `ChatGreeting` | text with theme tokens; the Isonomic mark from the icon set | Chat `home` | on main |
| `SuggestionChips` | `Button` (outline, small, with icon) in a row | Chat `home` (content); the row is plain layout | on main |
| `ContextSidebar` | `component::tab::{TabBar, Tab}` (underline) | Chat `shell::context_panel` | on main |
| `IconButton` (state: Default, Hover) | `Button` with `icon` and `ghost` | none | on main |
| `Avatar` (kit instance) | `component::avatar::Avatar` | `Message` | on main |
| `disclaimer` (under the docked composer) | text with theme tokens | Chat copy | on main |
| `AppRailItem`, `app_rail` | none | the desktop shell | not Chat's |

## Figma pieces Chat does not build

| Figma layer | Why |
| --- | --- |
| `saved_prompts`, `models_group` in the sidebar | Not in Milestone 0; recorded in `design-diffs.md` |
| `projects_group` in the sidebar | Projects are out of scope for Milestone 0 |
| `Chat / Content / Growth Analysis`, `Chat/Conversation Heading/Two Actions` (hidden, Components page) | Reference instances from the kit, not Chat components |

## Requests filed

| Issue | For |
| --- | --- |
| isonomic-ui #206 | Prompt composer |
| isonomic-ui #207 | Thinking indicator (from proposal #14) |
| isonomic-ui #208 | Stacked list item, for `ContextItem` |
