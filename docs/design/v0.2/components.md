# Version 0.2 components

Every component on the design file's "Components 0.2" page (node `97:3`), where version 0.2 uses it, and whether version 1 (plain streamed chat) needs it.

Marks:

- **V1**: needed for version 1 plain chat.
- **V1?**: shown on a version 1 screen, but the feature behind it may be out of scope; the build decides and records it.
- **Out**: shown in the design, outside version 1. Recorded so nothing is lost.
- **Old**: carried over from version 0.1; only the "Start a Basic Chat 0.2" page uses it.

"Repo today" is the nearest entry in `docs/name-map.md`. Layout detail for each component is in the screen file named in the last column. Pictures of every component section are in `images/components-*.png`.

## Shell

| Component | Node | Variants | Mark | Repo today | Detail |
| --- | --- | --- | --- | --- | --- |
| `AppSidebar` | `97:19485` | one (288 x 900) | V1 | `ChatSidebar` layout over the sidebar module | `screens/app-shell.md` |
| `AppSidebarRail` | `97:19556` | one (56 x 900) | V1 | none (new) | `screens/app-shell.md` |
| `SidebarNavItem` | `97:11268` | state: Default, Hover, Selected | V1 | sidebar menu item | `screens/sidebar.md` |
| `SidebarGroupLabel` | `97:19474` | one (collapsible label with caret) | V1 | sidebar group | `screens/sidebar.md` |
| `SidebarSessionRow` | `97:11285` | state: Default, Hover, Selected x status: None, Running, Unread, Attention | V1 | `chat_item` | `screens/sidebar.md` |
| `SidebarAccount` | `97:19477` | one | V1? | none; repo says the desktop shell owns the account | `screens/sidebar.md` |
| `SessionHeader` | `97:11347` | kind: Session, Home | V1 | `ChatHeader` (Untitled, Titled) | `screens/app-shell.md` |
| `PageHeader` | `97:19648` | one | Out | none | projects and scheduled pages |
| `ShortcutRow` | `97:19567` | one | V1? | none | `screens/search-and-menus.md` |

## Home, composer and model picker

| Component | Node | Variants | Mark | Repo today | Detail |
| --- | --- | --- | --- | --- | --- |
| `HomeGreeting` | `97:19528` | one | V1 | `ChatGreeting` | `screens/home.md` |
| `Composer` | `97:11361` | state: Empty, Filled, Working | V1 | `PromptComposer` (request #206) | `screens/composer.md` |
| `ComposerFooter` | `97:19517` | one (project, model, effort, apps) | V1 (model part); rest Out | none (new) | `screens/composer.md` |
| `MenuItemDetail` | `97:11502` | selected: True, False x state: Default, Hover | V1 | model picker rows | `screens/model-picker.md` |
| `ModelOption` | `101:5309` | selected: True, False | V1? (second picker design) | model picker rows | `screens/model-picker.md` |
| `SettingRow` | `97:11770` | control: Toggle, Select, Button | V1? (inside the second picker design); else Out | none | `screens/model-picker.md` |
| `ContextRing` | `100:4529` | level: Normal, Warn | V1? (shown in the docked composer) | usage meter | `screens/composer.md` |
| `UsageBanner` | `97:19531` | one | V1? (limit warning) | usage meter | `screens/first-run-and-system-states.md` |
| `ModeSwitch` | `97:11536` | value: Chat, Work | Out | none | `screens/composer.md` |
| `MenuToggleItem` | `97:11493` | enabled: True, False | Out | none | `screens/composer.md` |
| `AttachmentThumb` | `97:11523` | state: Default, Hover | Out | attachment chip | `screens/attachments.md` |
| `DropzoneFile` | `101:5724` | state: Uploading, Done, Error | Out | file dropzone | `screens/attachments.md` |
| `AutonomyDial` | `100:4516` | value: Ask, Edits, Full | Out | none | `screens/run-control-and-approvals.md` |
| `SetupTask` | `101:5327` | state: Todo, Done | Out | none | `screens/home.md` |
| `PromptTemplateCard` | `101:5227` | one | Out | none | prompt library screen |

## Thread and messages

| Component | Node | Variants | Mark | Repo today | Detail |
| --- | --- | --- | --- | --- | --- |
| `UserMessage` | `97:11440` | state: Default, Hover x size: Short, Long | V1 | `ChatMessage` (user) | `screens/thread.md` |
| `ReplyActions` | `97:19545` | one (copy, read aloud, up, down, retry, time) | V1 | `MessageActions`, message action bar | `screens/thread.md` |
| `ThinkingRow` | `97:11474` | see the screen file | V1 (the wait before the first words) | `ThinkingIndicator` (request #207) | `screens/thread-states.md` |
| `ThinkingState` | `100:3242` | see the screen file | V1? (reasoning shown apart from the reply) | reasoning steps panel | `screens/thread-states.md` |
| `RunStatusBar` | `100:4574` | status: Running, Stopped, Error, Done | V1? (Stopped and Error are the only stop and error bars in the file) | none | `screens/thread-states.md` |
| `Toast` | `101:5177` | tone: Success, Info, Error | V1 | none named | `screens/first-run-and-system-states.md` |
| `EmptyState` | `101:5213` | one | V1 | none named | `screens/first-run-and-system-states.md` |
| `QueuedMessage` | `100:4584` | one | Out | none | `screens/thread-states.md` |
| `SteerNote` | `100:4603` | one | Out | none | `screens/thread-states.md` |
| `ToolCallRow` | `97:19552` | one | Out | tool call card | `screens/agent-work-and-side-panel.md` |

## Menus and small parts

| Component | Node | Variants | Mark | Repo today | Detail |
| --- | --- | --- | --- | --- | --- |
| `MenuItem` | `97:11392` | state: Default, Hover x tone: Default, Destructive | V1 | context menu items | `screens/search-and-menus.md` |
| `MenuSeparator` | `97:19541` | one | V1 | same | `screens/search-and-menus.md` |
| `MenuHeader` | `97:19543` | one | V1 | same | `screens/search-and-menus.md` |
| `TextField` | `97:11433` | state: Default, Focus | V1 | input | `screens/search-and-menus.md` |
| `Kbd` | `97:19537` | one | V1 | none named | `screens/search-and-menus.md` |
| `Tooltip` | `97:19539` | one | V1 | tooltip | `screens/search-and-menus.md` |
| `SearchPaletteRow` | `97:11419` | state: Default, Highlighted | V1? (search is in the sidebar, not in the version 1 list) | none | `screens/search-and-menus.md` |
| `TabPill` | `97:11428` | selected: False, True | V1? (search palette only) | none | `screens/search-and-menus.md` |

## Advanced agent components

All outside version 1 unless marked. Detail in `screens/agent-work-and-side-panel.md`, `screens/run-control-and-approvals.md`, `screens/context-memory-and-research.md`.

| Component | Node | Variants | Mark | Repo today |
| --- | --- | --- | --- | --- |
| `ToolCallCard` | `100:3299` | state: Running, Done, Expanded, Error | Out | tool call card and group |
| `PermissionRequest` | `100:4472` | see the screen file | Out | approval card, permission prompt |
| `PlanStep` | `101:4314` | state: Todo, Running, Done, Failed | Out | none |
| `ProgressStep` | `97:11548` | status: Done, Current, Pending | Out | none |
| `PanelSectionHeader` | `97:19572` | one | Out | none |
| `PanelRow` | `97:11562` | state: Default, Hover | Out | `ContextItem` (request #208) |
| `QuestionOption` | `97:11571` | state: Default, Hover | Out | question card |
| `FileCard` | `97:19577` | one | Out | file card |
| `ArtifactCard` | `97:19584` | one | Out | none |
| `CheckpointRow` | `101:4357` | state: Head, Past, Failed, Faded | Out | none |
| `LedgerRow` | `101:5249` | state: Default, Undone | Out | none |
| `ContextWindowCard` | `101:5112` | level: Normal, Warn, Compacting | Out | context usage grid |
| `MemoryItem` | `101:5158` | state: Default, Pending, Paused | Out | none |
| `UsageMeterCard` | `101:5199` | one | Out | usage meter |
| `RunningNowPill` | `101:5682` | one | Out | none |
| `InlineDiffToken` | `101:5690` | kind: Insert, Delete | Out | none |
| `CitationChip` | `97:19665` | one | Out | citation marker |
| `LinkPreviewCard` | `97:19667` | one | Out | none |
| `SourceRow` | `97:19673` | one | Out | sources panel |

## Other feature areas (all outside version 1)

| Area | Components (node) |
| --- | --- |
| Artifacts | `ArtifactTile` (`97:11587`: Default, Hover), `TypeTile` (`97:19590`), `ArtifactListRow` (`97:11613`: Default, Hover), `ArtifactHeader` (`97:19595`), `MetaChip` (`97:19603`), `DocToolbar` (`97:19606`) |
| Inline widgets | `WidgetHeader` (`97:19615`), `KpiTile` (`97:19620`), `QuizOption` (`97:11636`: Default, Selected, Correct, Wrong), `PlaceRow` (`97:19626`), `ItineraryStop` (`97:19632`), `StepItem` (`97:11649`: Todo, Current, Done), `IngredientRow` (`97:19640`) |
| Projects | `ProjectCard` (`97:11670`: Default, Hover), `ProjectRailRow` (`97:19643`), `SessionListRow` (`97:11688`: Default, Hover), `ContextFileRow` (`97:11702`: Default, Hover, Selected) |
| Settings and customize | `ToolPermissionRow` (`97:11719`: Allow, Ask, Block), `ConnectorRow` (`97:11753`: Default, Hover), `DirectoryCard` (`97:19657`) |
| Scheduled | `ScheduledTaskCard` (`97:11790`: Active, Paused, Failed) |

## Carried over from version 0.1

These sit at the top of the Components 0.2 page, outside any section. Only the "Start a Basic Chat 0.2" page instantiates them; every other 0.2 page uses the sections above. See `screens/basic-chat-page.md`.

| Old component | Node in 0.2 | Variants | Replaced in the rest of 0.2 by |
| --- | --- | --- | --- |
| `ChatSidebar` | `97:6` | one (272 wide, with a 40 px rail) | `AppSidebar` (288), `AppSidebarRail` (56) |
| `ContextSidebar` | `97:11` | one (320 wide) | no always-open panel; side panels open per task |
| `ChatHeader` | `97:27` | state: Untitled, Titled (848 x 54) | `SessionHeader` (52 high) |
| `PromptComposer` | `97:73` | state: Empty, Filled, Streaming (800 x 144) | `Composer` (720 x 96) and `ComposerFooter` |
| `ChatMessage` | `97:41` | role x state | `UserMessage` and unboxed reply text |
| `MessageActions` | `97:17` | role: Assistant, User | `ReplyActions`, hover actions on `UserMessage` |
| `ThinkingIndicator` | `97:19462` | one | `ThinkingRow`, `ThinkingState` |
| `ThinkingOrb`, `AgentOrb` | `97:100`, `97:4006` | several | not used on any 0.2 screen outside this page |
| `ChatGreeting` | `97:19465` | one (340 x 36) | `HomeGreeting` (326 x 42) |
| `SuggestionChips` | `97:19468` | one (505 x 34) | suggestion cards on the start page; none on the plain home |
| `IconButton` | `97:12` | state: Default, Hover | icon buttons inside each component |
