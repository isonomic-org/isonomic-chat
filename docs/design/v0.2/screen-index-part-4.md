# Screen index, part 4 of 4

Every screen of version 0.2: node id, frame name, size, the state it shows, and its picture. States on pages outside version 1 come from frame and layer names and were mostly not checked against the pictures. See `README.md` for the page list.

## Session Management (`97:119518`)

8 rows. Pictures are in `images/`.

| Node | Frame | Size | State shown | Image |
| --- | --- | --- | --- | --- |
| `97:119520` | `SessionMenu/01_TitleMenu` | 1440 x 900 | Session with the title menu open under the header title (6 items, separator before the last, 220 x 211) | `session-management-01-title-menu.png` |
| `97:119544` | `SessionMenu/02_AddToProject` | 1440 x 900 | Title menu with the add-to-project submenu open beside it (5 items and a separator, 220 x 179) | `session-management-02-add-to-project.png` |
| `97:119575` | `ShareSession/03_Popover` | 1440 x 900 | Share popover (380 x 215) under the header: "Share this chat", access row "Anyone with the link", action button | `session-management-03-popover.png` |
| `97:119604` | `ShareSession/04_AccessLevel` | 1440 x 900 | Share popover with the access level menu open (header and 3 MenuItemDetail, 348 x 189) | `session-management-04-access-level.png` |
| `97:119638` | `SessionMenu/05_DeleteConfirm` | 1440 x 900 | Delete confirmation dialog (440 x 240) over a scrim: title, subtitle, two footer buttons | `session-management-05-delete-confirm.png` |
| `97:119665` | `SessionMenu/06_RenameInline` | 1440 x 900 | Inline rename: a TextField (300 x 32) replaces the session title in the header | `session-management-06-rename-inline.png` |
| `97:119682` | `SidebarMenu/07_SessionMenu` | 1440 x 900 | Session menu opened from a sidebar row (6 items and a separator, 220 x 211, at x 255 y 607) | `session-management-07-session-menu.png` |
| `97:119706` | `SidebarMenu/08_MoveToSection` | 1440 x 900 | Sidebar session menu with the move-to-section submenu open (header, 4 items, separator, 210 x 173) | `session-management-08-move-to-section.png` |

## Settings (`97:121605`)

13 rows. Pictures are in `images/`.

| Node | Frame | Size | State shown | Image |
| --- | --- | --- | --- | --- |
| `97:121607` | `Settings/01_Appearance` | 1440 x 900 | Settings modal, Appearance: color mode choices Light, Dark, Match system, then 3 SettingRow | `settings-01-appearance.png` |
| `97:121669` | `Settings/02_Account` | 1440 x 900 | Settings, Account: avatar, name and plan, "Full name" and "What should Isonomic call you?" fields, 3 SettingRow | `settings-02-account.png` |
| `97:121714` | `Settings/03_Usage` | 1440 x 900 | Settings, Usage: three meters (current session 34%, weekly all models 76%, weekly large models 12%), 1 SettingRow | `settings-03-usage.png` |
| `97:121773` | `Settings/04_YourIsonomic` | 1440 x 900 | Settings, Your Isonomic: Memory entry, personal preferences text area, 3 SettingRow | `settings-04-your-isonomic.png` |
| `97:121813` | `Settings/05_Capabilities` | 1440 x 900 | Settings, Capabilities: 6 SettingRow | `settings-05-capabilities.png` |
| `97:121852` | `Settings/06_Notifications` | 1440 x 900 | Settings, Notifications: 4 SettingRow | `settings-06-notifications.png` |
| `97:121889` | `Settings/07_Privacy` | 1440 x 900 | Settings, Privacy: 4 SettingRow | `settings-07-privacy.png` |
| `97:121926` | `Settings/08_Wellbeing` | 1440 x 900 | Settings, Wellbeing: intro line and 4 SettingRow | `settings-08-wellbeing.png` |
| `97:121964` | `Settings/09_Billing` | 1440 x 900 | Settings, Billing: plan card with price and renewal, button, 2 SettingRow, Invoices list of 3 with Badge | `settings-09-billing.png` |
| `110:2394` | `Settings/10_Models` | 1440 x 900 | Settings, Models: "Available models" with 4 ModelOption and MetaChip, "If a model is busy" with 3 SettingRow | `settings-10-models.png` |
| `110:3660` | `Settings/11_Permissions` | 1440 x 900 | Settings, Permissions: default autonomy AutonomyDial, "Hard limits" SettingRow, "Per-app actions" ToolPermissionRow | `settings-11-permissions.png` |
| `110:4097` | `Settings/12_UsageDetail` | 1440 x 900 | Settings, Usage detail: UsageMeterCard, weekly bar chart, "By project" breakdown, pace forecast line | `settings-12-usage-detail.png` |
| `110:4487` | `Settings/13_PricingPlans` | 1440 x 900 | Settings, Change plan: Monthly / Yearly switch and 3 plan cards (Pro, Max, Ultra marked Current), compare link | `settings-13-pricing-plans.png` |

## Devices and Phone (`97:124264`)

9 rows. Pictures are in `images/`.

| Node | Frame | Size | State shown | Image |
| --- | --- | --- | --- | --- |
| `97:124266` | `AppsAndExtensions/01_Top` | 1440 x 900 | Apps and extensions page at scroll top: title, desktop hero card, "Office and desktop" and "Code and mobile" panels | `devices-01-top.png` |
| `97:124320` | `AppsAndExtensions/02_OfficeAndDesktop` | 1440 x 900 | Same page scrolled 220 px: "Office and desktop" panel with 2 app cards in view | `devices-02-office-and-desktop.png` |
| `97:124374` | `AppsAndExtensions/03_CodeAndMobile` | 1440 x 900 | Same page scrolled 360 px: "Code and mobile" panel with 2 app cards in view | `devices-03-code-and-mobile.png` |
| `97:124428` | `AppsAndExtensions/04_Mobile` | 1440 x 900 | Same page scrolled 420 px to the mobile app card and the phone pairing note | `devices-04-mobile.png` |
| `97:124482` | `AppsAndExtensions/05_MobileQrModal` | 1440 x 900 | Mobile download dialog (420 x 370) over a scrim: title, subtitle, 200 x 200 QR code, 2 MetaChip | `devices-05-mobile-qr-modal.png` |
| `97:124666` | `DeviceLink/06_SessionDevicePopover` | 1440 x 900 | Session with device popover (300 x 284): "This chat is linked to", 3 devices with status dots, folders, button | `devices-06-session-device-popover.png` |
| `97:124707` | `SettingsDevices/07_Devices` | 1440 x 900 | Settings modal, Devices: 3 ConnectorRow devices, "On this computer" with 3 SettingRow | `devices-07-devices.png` |
| `97:124748` | `SettingsDevices/08_BrowserOnComputer` | 1440 x 900 | Settings, Devices sub-page "Browser on your computer": back link, status line, 4 SettingRow | `devices-08-browser-on-computer.png` |
| `97:124793` | `ComposerMenu/09_PlusDevices` | 1440 x 900 | Home composer plus menu (8 items, 232 x 275) with the devices submenu open (header, 5 items, 250 x 205) | `devices-09-plus-devices.png` |

## Components (sheets) (`97:3`)

27 rows. Pictures are in `images/`.

| Node | Section or frame | Size | Components and variants inside | Image |
| --- | --- | --- | --- | --- |
| `97:4` | Chat Sidebar (frame) | 272 x 917 | Instance `ChatSidebar` `97:6` (a second, hidden instance `97:5`) | `components-chat-sidebar.png` |
| `97:7` | Chat Content (frame) | 776 x 917 | Instance `Chat / Content / Growth Analysis` `97:8`; hidden instance `Chat/Conversation Heading/Two Actions` `97:9` | `components-chat-content.png` |
| `97:10` | Chat Context Sidebar (frame) | 320 x 917 | Instance `ContextSidebar` `97:11` | `components-chat-context-sidebar.png` |
| `97:12` | IconButton (frame) | 130 x 78 | `IconButton`: `state=Default` `97:13`, `state=Hover` `97:15` (28 x 28) | `components-icon-button.png` |
| `97:17` | MessageActions (frame) | 168 x 130 | `MessageActions`: `role=Assistant` `97:18` (118 x 28), `role=User` `97:23` (58 x 28) | `components-message-actions.png` |
| `97:19462` | ThinkingIndicator (symbol) | 94 x 20 | `ThinkingIndicator`, no variants | `components-thinking-indicator.png` |
| `97:27` | ChatHeader (frame) | 898 x 182 | `ChatHeader`: `state=Untitled` `97:28`, `state=Titled` `97:33` (848 x 54) | `components-chat-header.png` |
| `97:41` | ChatMessage (frame) | 850 x 402 | `ChatMessage`; no child variants listed in the metadata. Image shows 4 rows: user message with edit and copy, assistant "Thinking...", assistant streaming dot, assistant done with 4 actions | `components-chat-message.png` |
| `97:73` | PromptComposer (frame) | 850 x 530 | `PromptComposer`: `state=Empty` `97:74`, `state=Filled` `97:82`, `state=Streaming` `97:90` (800 x 144) | `components-prompt-composer.png` |
| `97:19465` | ChatGreeting (symbol) | 340 x 36 | `ChatGreeting`, no variants | `components-chat-greeting.png` |
| `97:19468` | SuggestionChips (symbol) | 505 x 34 | `SuggestionChips`, no variants | `components-suggestion-chips.png` |
| `97:100` | ThinkingOrb (frame) | 896 x 180 | `ThinkingOrb`; no child variants listed in the metadata. Image shows a row of about 9 orb states, dark on a dark canvas and barely visible | `components-thinking-orb.png` |
| `97:4006` | AgentOrb (frame) | 396 x 500 | `AgentOrb`; no child variants listed in the metadata. Image shows a column of about 9 rows, dark on a dark canvas and barely visible | `components-agent-orb.png` |
| `97:11267` | Shell (section) | 1768 x 1060 | `SidebarNavItem` `97:11268`: `state=Default`, `state=Hover`, `state=Selected` (256 x 32). `SidebarGroupLabel` `97:19474` (256 x 30). `SidebarSessionRow` `97:11285`: `state` = Default, Hover, Selected x `status` = None, Running, Unread, Attention (12 variants, 256 x 32). `SidebarAccount` `97:19477` (272 x 45). `AppSidebar` `97:19485` (288 x 900) | `components-section-shell.png` |
| `97:11346` | Session (section) | 2376 x 1128 | `SessionHeader` `97:11347`: `kind=Session`, `kind=Home` (1152 x 52). `Composer` `97:11361`: `state=Empty`, `state=Filled`, `state=Working` (720 x 96). `ComposerFooter` `97:19517` (720 x 28). `HomeGreeting` `97:19528` (326 x 42). `UsageBanner` `97:19531` (720 x 48). `Kbd` `97:19537` (32 x 20). `Tooltip` `97:19539` (54 x 26) | `components-section-session.png` |
| `97:11391` | Menus & Messages (section) | 1424 x 1771 | `MenuItem` `97:11392`: `state=Default, tone=Default`; `state=Hover, tone=Default`; `state=Default, tone=Destructive`; `state=Hover, tone=Destructive` (232 x 32). `MenuSeparator` `97:19541` (232 x 9). `MenuHeader` `97:19543` (232 x 26). `SearchPaletteRow` `97:11419`: `state=Default`, `state=Highlighted` (616 x 36). `TabPill` `97:11428`: `selected=False`, `selected=True` (44 x 30). `TextField` `97:11433`: `state=Default`, `state=Focus` (320 x 38). `UserMessage` `97:11440`: `state=Default, size=Short`; `state=Hover, size=Short` (454 x 78); `state=Default, size=Long`; `state=Hover, size=Long` (560 x 78). `ReplyActions` `97:19545` (196 x 28). `ThinkingRow` `97:11474` (frame 1044 x 70, no child variants listed). `ToolCallRow` `97:19552` (640 x 38) | `components-section-menus-and-messages.png` |
| `97:11489` | Shell II (section) | 660 x 1060 | `AppSidebarRail` `97:19556` (56 x 900). `ShortcutRow` `97:19567` (420 x 36) | `components-section-shell-ii.png` |
| `97:11492` | Composer Menus (section) | 752 x 1196 | `MenuToggleItem` `97:11493`: `enabled=True`, `enabled=False` (248 x 32). `MenuItemDetail` `97:11502`: `selected=True, state=Default`; `selected=False, state=Default`; `selected=False, state=Hover`; `selected=True, state=Hover` (280 x 51). `AttachmentThumb` `97:11523`: `state=Default`, `state=Hover` (120 x 96). `ModeSwitch` `97:11536`: `value=Chat`, `value=Work` (101 x 26) | `components-section-composer-menus.png` |
| `97:11547` | Agentic Work (section) | 1056 x 1054 | `ProgressStep` `97:11548`: `status=Done`, `status=Current`, `status=Pending` (280 x 28). `PanelSectionHeader` `97:19572` (280 x 20). `PanelRow` `97:11562`: `state=Default`, `state=Hover` (280 x 30). `QuestionOption` `97:11571`: `state=Default`, `state=Hover` (640 x 54). `FileCard` `97:19577` (720 x 64). `ArtifactCard` `97:19584` (360 x 64) | `components-section-agentic-work.png` |
| `97:11586` | Artifacts (section) | 1272 x 1364 | `ArtifactTile` `97:11587`: `state=Default`, `state=Hover` (280 x 216). `TypeTile` `97:19590` (112 x 104). `ArtifactListRow` `97:11613`: `state=Default`, `state=Hover` (880 x 52). `ArtifactHeader` `97:19595` (1152 x 52). `MetaChip` `97:19603` (102 x 20). `DocToolbar` `97:19606` (355 x 44) | `components-section-artifacts.png` |
| `97:11633` | Inline Widgets (section) | 840 x 1390 | `WidgetHeader` `97:19615` (720 x 44). `KpiTile` `97:19620` (160 x 94). `QuizOption` `97:11636`: `state=Default`, `state=Selected`, `state=Correct`, `state=Wrong` (640 x 44). `PlaceRow` `97:19626` (360 x 59). `ItineraryStop` `97:19632` (640 x 70). `StepItem` `97:11649`: `state=Todo`, `state=Current`, `state=Done` (640 x 58). `IngredientRow` `97:19640` (320 x 33) | `components-section-inline-widgets.png` |
| `97:11669` | Projects (section) | 1272 x 1242 | `ProjectCard` `97:11670`: `state=Default`, `state=Hover` (336 x 148). `ProjectRailRow` `97:19643` (300 x 40). `SessionListRow` `97:11688`: `state=Default`, `state=Hover` (720 x 44). `PageHeader` `97:19648` (1152 x 52). `ContextFileRow` `97:11702`: `state=Default`, `state=Hover`, `state=Selected` (640 x 40) | `components-section-projects.png` |
| `97:11718` | Settings & Customize (section) | 848 x 1214 | `ToolPermissionRow` `97:11719`: `value=Allow`, `value=Ask`, `value=Block` (680 x 59). `ConnectorRow` `97:11753`: `state=Default`, `state=Hover` (680 x 56). `SettingRow` `97:11770`: `control=Toggle`, `control=Select`, `control=Button` (680 x 67). `DirectoryCard` `97:19657` (216 x 114) | `components-section-settings-and-customize.png` |
| `97:11789` | Scheduled (section) | 688 x 564 | `ScheduledTaskCard` `97:11790`: `state=Active`, `state=Paused`, `state=Failed` (520 x 102) | `components-section-scheduled.png` |
| `97:11818` | Research (section) | 760 x 436 | `CitationChip` `97:19665` (80 x 18). `LinkPreviewCard` `97:19667` (232 x 90). `SourceRow` `97:19673` (640 x 36) | `components-section-research.png` |
| `100:2449` | Run & Approvals (0.2) (section) | 1472 x 3153 | `ThinkingState` `100:3242` (frame 688 x 448, no child variants listed). `ToolCallCard` `100:3299`: `state=Running`, `state=Done` (640 x 38), `state=Expanded`, `state=Error` (640 x 101). `PermissionRequest` `100:4472` (frame 1352 x 1107, no child variants listed). `AutonomyDial` `100:4516`: `value=Ask`, `value=Edits`, `value=Full` (257 x 28). `ContextRing` `100:4529`: `level=Normal`, `level=Warn` (66 x 24). `RunStatusBar` `100:4574`: `status=Running`, `status=Stopped`, `status=Error`, `status=Done` (720 x 44). `QueuedMessage` `100:4584` (720 x 42). `SteerNote` `100:4603` (184 x 26) | `components-section-run-and-approvals.png` |
| `101:4261` | Workspace & Context (0.2) (section) | 1748 x 2666 | `PlanStep` `101:4314`: `state=Todo`, `state=Running`, `state=Done`, `state=Failed` (640 x 44). `CheckpointRow` `101:4357`: `state=Head`, `state=Past`, `state=Failed`, `state=Faded` (560 x 40). `ContextWindowCard` `101:5112`: `level=Normal` (380 x 294), `level=Warn` (380 x 308), `level=Compacting` (380 x 294). `MemoryItem` `101:5158`: `state=Default`, `state=Pending`, `state=Paused` (640 x 60). `Toast` `101:5177`: `tone=Success`, `tone=Info`, `tone=Error` (360 x 40). `UsageMeterCard` `101:5199` (360 x 172). `EmptyState` `101:5213` (360 x 222). `PromptTemplateCard` `101:5227` (300 x 144). `LedgerRow` `101:5249`: `state=Default`, `state=Undone` (680 x 48). `ModelOption` `101:5309`: `selected=True`, `selected=False` (360 x 70). `SetupTask` `101:5327`: `state=Todo`, `state=Done` (520 x 59). `RunningNowPill` `101:5682` (159 x 26). `InlineDiffToken` `101:5690`: `kind=Insert`, `kind=Delete` (152 x 26). `DropzoneFile` `101:5724`: `state=Uploading` (420 x 50), `state=Done`, `state=Error` (420 x 58) | `components-section-workspace-and-context.png` |
