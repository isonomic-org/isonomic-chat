# Screen index, part 1 of 4

Every screen of version 0.2: node id, frame name, size, the state it shows, and its picture. States on pages outside version 1 come from frame and layer names and were mostly not checked against the pictures. See `README.md` for the page list.

## Start a Basic Chat (`97:22304`)

6 rows. Pictures are in `images/`.

| Node | Frame | Size | State shown | Image |
| --- | --- | --- | --- | --- |
| 97:22306 | Chat-Screenshot | 1440x1024 | Three-column chat: sidebar, growth-analysis reply with docked composer, context sidebar on the right | basic-chat-00-chat-screenshot.png |
| 97:22357 | ChatView/01_NewChat | 1440x1024 | New chat empty state: greeting, centred prompt composer, suggestion chips, context sidebar | basic-chat-01-new-chat.png |
| 97:22367 | ChatView/02_Composing | 1440x1024 | Same empty state while the user is typing a prompt in the composer | basic-chat-02-composing.png |
| 97:22377 | ChatView/03_Thinking | 1440x1024 | Prompt sent; assistant thinking before the first words of the reply arrive | basic-chat-03-thinking.png |
| 97:22389 | ChatView/04_Streaming | 1440x1024 | User message plus a short assistant reply still streaming; composer docked at bottom with disclaimer | basic-chat-04-streaming.png |
| 97:22401 | ChatView/05_Complete | 1440x1024 | User message plus the finished, taller assistant reply; composer docked at bottom with disclaimer | basic-chat-05-complete.png |

## Navigation, Sidebar and Search (`97:25384`)

16 rows. Pictures are in `images/`.

| Node | Frame | Size | State shown | Image |
| --- | --- | --- | --- | --- |
| 97:25386 | HomeView/01_Home | 1440x900 | Home: 288 px sidebar, session header, centred greeting, composer and composer footer | navigation-01-home.png |
| 97:25394 | SearchPalette/02_Empty | 1440x900 | Search palette open over a scrim, empty query: Needs attention, Active, Recents, Actions groups | navigation-02-empty.png |
| 97:25445 | SearchPalette/03_Results | 1440x900 | Palette with query "isonomic ui": results grouped by Chats, Artifacts, Projects | navigation-03-results.png |
| 97:25490 | SearchPalette/04_ResultActions | 1440x900 | Palette with Chats results and a 220 px actions menu open beside a result row | navigation-04-result-actions.png |
| 97:25536 | SearchPalette/05_ArtifactsType | 1440x900 | Palette on the artifacts type tab, empty query, listing five recent artifacts | navigation-05-artifacts-type.png |
| 97:25577 | SearchPalette/06_Filters | 1440x900 | Palette after typing "/": five filter rows and one action row, type tabs hidden | navigation-06-filters.png |
| 97:25615 | SearchPalette/07_FilterByType | 1440x900 | Palette with a "type:" filter token in the input and five type choices listed | navigation-07-filter-by-type.png |
| 97:25652 | SearchPalette/08_ComposeMode | 1440x900 | Palette in compose mode: "Ask anything…" input, attach button, footer chips, send button | navigation-08-compose-mode.png |
| 97:25684 | HomeView/09_SidebarCollapsed | 1440x900 | Home with the sidebar collapsed to a 56 px icon rail; main panel widens to 1384 px | navigation-09-sidebar-collapsed.png |
| 97:25692 | HomeView/10_Incognito | 1440x900 | Incognito home: ghost icon, "Incognito chat" title and note, composer, header tooltip | navigation-10-incognito.png |
| 97:25705 | AccountMenu/11_Open | 1440x900 | Home with the account menu open above the sidebar footer (header, seven items, two separators) | navigation-11-open.png |
| 97:25724 | AccountMenu/12_LearnMore | 1440x900 | Account menu open with its Learn more submenu flown out to the right | navigation-12-learn-more.png |
| 97:25750 | KeyboardShortcuts/13_General | 1440x900 | Keyboard shortcuts dialog over a scrim, general set, eight shortcut rows | navigation-13-general.png |
| 97:25774 | KeyboardShortcuts/14_Composer | 1440x900 | Keyboard shortcuts dialog over a scrim, composer set, eight shortcut rows | navigation-14-composer.png |
| 97:25798 | Sidebar/15_PinnedItemMenu | 1440x900 | Home with a context menu open on a pinned sidebar item (six items, one separator) | navigation-15-pinned-item-menu.png |
| 97:25814 | Sidebar/16_PinnedCollapsed | 1440x900 | Home with the sidebar's Pinned section collapsed | navigation-16-pinned-collapsed.png |

## Composer and Session Setup (`97:29007`)

21 rows. Pictures are in `images/`.

| Node | Frame | Size | State shown | Image |
| --- | --- | --- | --- | --- |
| 97:29009 | ComposerMenu/01_PlusMenu | 1440x900 | Home with the composer plus menu open (eight items, one separator) | composer-01-plus-menu.png |
| 97:29027 | ComposerMenu/02_Skills | 1440x900 | Plus menu with the Skills submenu open: header, five skills, separator, one action | composer-02-skills.png |
| 97:29054 | ComposerMenu/03_Connectors | 1440x900 | Plus menu with the Connectors submenu: two items, six toggle rows, one action | composer-03-connectors.png |
| 97:29084 | ComposerMenu/04_ToolAccess | 1440x900 | Connectors submenu with a third-level tool access menu of three detailed options | composer-04-tool-access.png |
| 97:29119 | ComposerMenu/05_AddConnector | 1440x900 | Connectors submenu with a third-level add-connector menu (header, six items) | composer-05-add-connector.png |
| 97:29158 | ComposerMenu/06_DesignSystem | 1440x900 | Plus menu with the design system submenu: header, three choices, separator, one action | composer-06-design-system.png |
| 97:29183 | ComposerMenu/07_Plugins | 1440x900 | Plus menu with the Plugins submenu: header, five toggle rows, separator, one action | composer-07-plugins.png |
| 97:29210 | ComposerMenu/08_ModelPicker | 1440x900 | Model picker menu open from the composer footer: three detailed options plus two items | composer-08-model-picker.png |
| 97:29226 | ComposerMenu/09_Effort | 1440x900 | Model picker with the effort submenu open to the right, four detailed levels | composer-09-effort.png |
| 97:29248 | ComposerMenu/10_MoreModels | 1440x900 | Model picker with the more-models submenu open to the right, four detailed options | composer-10-more-models.png |
| 97:29270 | ComposerMenu/11_ProjectPicker | 1440x900 | Project picker menu: header, search field, five projects, separator, one action | composer-11-project-picker.png |
| 97:29288 | ComposerMenu/12_PermissionMode | 1440x900 | Permission mode menu from the composer footer with three detailed options | composer-12-permission-mode.png |
| 97:29301 | ComposerMenu/13_Dictation | 1440x900 | Dictation menu by the microphone button: header, two items, separator, two items | composer-13-dictation.png |
| 97:29316 | ComposerMenu/14_LongDraftModeSwitch | 1440x900 | Taller composer holding a long draft, mode switch shown with a tooltip below it | composer-14-long-draft-mode-switch.png |
| 97:29326 | ComposerMenu/15_WorkMode | 1440x900 | Work mode selected: wider greeting, tall composer, hint line about multi-step tasks | composer-15-work-mode.png |
| 97:29338 | ComposerAttachment/16_ImageAttached | 1440x900 | Composer with an attachment tray holding one image thumbnail above the input | composer-16-image-attached.png |
| 97:29349 | ComposerAttachment/17_HoverRemove | 1440x900 | Same attached image hovered, with a remove tooltip over the thumbnail | composer-17-hover-remove.png |
| 107:4326 | ComposerMenu/18_ModelPickerBars | 1440x900 | Wide model picker: three model options, a setting row, and a thinking budget bar at 16k | composer-18-model-picker-bars.png |
| 107:5416 | ComposerAttachment/19_DropzoneUploading | 1440x900 | Dropzone above the composer with drop hint text and three file rows uploading | composer-19-dropzone-uploading.png |
| 107:5694 | ComposerMenu/20_ConnectAppsMenu | 1440x900 | Connect apps menu from the composer footer: header, four toggle rows, two actions | composer-20-connect-apps-menu.png |
| 107:5976 | ComposerMenu/21_WorkInFolder | 1440x900 | Work-in-folder menu: header, four folders, one action, second header, two items | composer-21-work-in-folder.png |

## Session Lifecycle (`97:40979`)

16 rows. Pictures are in `images/`.

| Node | Frame | Size | State shown | Image |
| --- | --- | --- | --- | --- |
| 97:40981 | SessionView/01_Streaming | 1440x900 | User message and first paragraph of the reply streaming, with a block cursor | lifecycle-01-streaming.png |
| 97:40997 | SessionView/02_StreamingStructured | 1440x900 | Reply streaming with headings and a bullet list, cursor at the end | lifecycle-02-streaming-structured.png |
| 97:41023 | SessionView/03_Complete | 1440x900 | Finished reply with headings, bullets, paragraphs and the reply action row | lifecycle-03-complete.png |
| 97:41050 | SessionView/04_Thinking | 1440x900 | User message sent; assistant shows the thinking row before any reply text | lifecycle-04-thinking.png |
| 97:41065 | SessionView/05_ThinkingExpanded | 1440x900 | Thinking row expanded to show a three-line thinking summary | lifecycle-05-thinking-expanded.png |
| 97:41084 | SessionView/06_ThinkingLogOpen | 1440x900 | Thinking log open with five checked steps, above the start of the reply | lifecycle-06-thinking-log-open.png |
| 97:41134 | SessionView/07_Stopped | 1440x900 | Partial reply with a "You stopped this response" note and reply actions | lifecycle-07-stopped.png |
| 97:41158 | SessionView/08_UserMessageHover | 1440x900 | Finished reply with the user message in its hover state | lifecycle-08-user-message-hover.png |
| 97:41184 | SessionView/09_EditUserMessage | 1440x900 | User message replaced by an inline edit box with two buttons | lifecycle-09-edit-user-message.png |
| 97:41214 | Feedback/10_NegativeDialog | 1440x900 | Negative feedback dialog over a scrim: issue type chosen, details filled in | lifecycle-10-negative-dialog.png |
| 97:41258 | Feedback/11_IssueTypes | 1440x900 | Feedback dialog with the issue type select open, listing seven options | lifecycle-11-issue-types.png |
| 97:41310 | SessionView/12_RetriedReply | 1440x900 | Retried reply with a version pager showing "2 / 2" beside the reply actions | lifecycle-12-retried-reply.png |
| 97:41333 | SessionView/13_ImageSentRetitled | 1440x900 | User turn with an image thumbnail and message; reply with a numbered list | lifecycle-13-image-sent-retitled.png |
| 97:41361 | AttachmentViewer/14_Open | 1440x900 | Full-window attachment viewer: top bar with file name, size, download, close; roadmap image | lifecycle-14-open.png |
| 107:9889 | SessionView/15_ThinkingStateSources | 1440x900 | Thinking block expanded: searched-source chips, two checked steps, and a drafting step in progress | lifecycle-15-thinking-state-sources.png |
| 107:11915 | SessionView/16_ThinkingStateDone | 1440x900 | Finished thinking state block above a short reply with reply actions | lifecycle-16-thinking-state-done.png |

## System States (`97:126642`)

3 rows. Pictures are in `images/`.

| Node | Frame | Size | State shown | Image |
| --- | --- | --- | --- | --- |
| 97:126644 | SystemState/01_PageNotFound | 1440x900 | "Page not found" with compass icon, one-line explanation and two buttons; sidebar kept | system-states-01-page-not-found.png |
| 97:126655 | SystemState/02_WeeklyUsageWarning | 1440x900 | Session with a usage banner docked above the composer | system-states-02-weekly-usage-warning.png |
| 97:126672 | SystemState/03_UsageWarningHome | 1440x900 | Home with a usage banner between the greeting and the composer | system-states-03-usage-warning-home.png |

## Home, Onboarding and Library (`101:60682`)

10 rows. Pictures are in `images/`.

| Node | Frame | Size | State shown | Image |
| --- | --- | --- | --- | --- |
| 106:3 | StartPage/01_ConnectedAndSuggestions | 1440x900 | Home with a Connected apps chip row and three suggestion cards under the composer | home-onboarding-01-connected-and-suggestions.png |
| 106:1121 | Overview/02_Workspace | 1440x900 | "Your month in Isonomic": four KPI tiles, line chart, recent chats, three files created | home-onboarding-02-workspace.png |
| 106:1425 | AwaySummary/03_WhileYouWereAway | 1440x900 | Session with a while-you-were-away card of four items, each with an action button | home-onboarding-03-while-you-were-away.png |
| 106:2021 | Recap/04_WeeklyRecap | 1440x900 | Weekly recap card: three KPI tiles, daily line chart, "Needs a look" list of two | home-onboarding-04-weekly-recap.png |
| 106:2649 | Onboarding/05_AboutYou | 1440x396 | Onboarding step 1 of 4: stepper and card asking for name and what you work on | home-onboarding-05-about-you.png |
| 106:2691 | Onboarding/06_ConnectApps | 1440x510 | Onboarding step 2: card with four app directory cards, back and continue buttons | home-onboarding-06-connect-apps.png |
| 106:2782 | Onboarding/07_Autonomy | 1440x320 | Onboarding step 3: autonomy dial set to Edits with explanation, back and continue | home-onboarding-07-autonomy.png |
| 106:2848 | SetupGuide/08_HomeChecklist | 1440x900 | Home with a 380 px setup guide on the right: "Get set up · 2 of 5", progress bar, five tasks | home-onboarding-08-home-checklist.png |
| 106:3129 | PromptLibrary/09_Library | 1440x900 | Prompt library grid of six template cards, with a fill-in-variables dialog over a scrim | home-onboarding-09-library.png |
| 106:3410 | Tips/10_TipsPanel | 1440x900 | Home with a tips panel of three tip cards under the composer, first one marked done | home-onboarding-10-tips-panel.png |

## Run Control and Approvals (`101:60680`)

14 rows. Pictures are in `images/`.

| Node | Frame | Size | State shown | Image |
| --- | --- | --- | --- | --- |
| 102:3 | AgentRun/01_PlanReview | 1440x900 | Proposed five-step plan card with two header buttons; autonomy dial above the composer | run-control-01-plan-review.png |
| 102:1512 | AgentRun/02_PlanRunningQueue | 1440x900 | "Running plan · step 2 of 5" card, a queued follow-up row, and a Working bar with Steer and Stop | run-control-02-plan-running-queue.png |
| 102:2192 | AgentRun/03_Steering | 1440x900 | Plan card, streaming draft text with an inline steer chip, Working bar with Steer and Stop | run-control-03-steering.png |
| 102:2853 | AgentRun/04_ToolCallsStreaming | 1440x900 | Collapsed Thinking row and three tool call cards (two done, one running); Working bar | run-control-04-tool-calls-streaming.png |
| 102:4224 | AgentRun/05_ToolError | 1440x900 | Two tool call cards, the second expanded with an error; run status bar above composer | run-control-05-tool-error.png |
| 102:4873 | AgentRun/06_Stopped | 1440x900 | Five-step plan card for a stopped run; run status bar above the composer | run-control-06-stopped.png |
| 102:5509 | Approvals/07_PermissionSafe | 1440x900 | Approval card over the plan: send an email, "Safe" badge, Once / For 1 hour / Always, Deny, Allow once | run-control-07-permission-safe.png |
| 102:6897 | Approvals/08_PermissionDestructive | 1440x900 | Approval card: replace a whole doc, "Destructive" badge, Once / For 1 hour only, red Allow once | run-control-08-permission-destructive.png |
| 102:8672 | Approvals/09_AllowedWithUndo | 1440x900 | "Allowed once" row with Revoke under the plan, plus a bottom-right toast with Undo | run-control-09-allowed-with-undo.png |
| 102:9736 | Approvals/10_AutonomyMenu | 1440x900 | Autonomy menu open from the composer footer with three detailed options | run-control-10-autonomy-menu.png |
| 102:10377 | Checkpoints/11_Panel | 1440x900 | Narrow conversation with plan card; 380 px Checkpoints side panel with five rows | run-control-11-panel.png |
| 102:11125 | Checkpoints/12_RewoundBranch | 1440x900 | After rewinding to a checkpoint: short branch transcript, run status bar, Checkpoints panel | run-control-12-rewound-branch.png |
| 102:11813 | AgentRun/13_Done | 1440x900 | "Plan complete · 5 of 5" card, closing reply with actions, run status bar | run-control-13-done.png |
| 102:12496 | Activity/14_LedgerUndo | 1440x900 | 460 px Activity side panel with four ledger rows and a toast at bottom right | run-control-14-ledger-undo.png |
