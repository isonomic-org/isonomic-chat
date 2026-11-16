# Home screen

The home screen is what a new chat looks like before the first message: a greeting, the composer and the composer footer, centred in the content area. This file covers the content area only. The app sidebar (`AppSidebar`, `AppSidebarRail`) and the session header (`SessionHeader`, kind Home) are named here but documented in `app-shell.md` and `sidebar.md`. The composer and its footer are detailed in `composer.md`.

Pixel values are design pixels at 1x. Coordinates are inside the `content` frame unless they say "screen".

## Variants and version 1

| Variant | Frame | Version 1 |
| --- | --- | --- |
| Home | `HomeView/01_Home` `97:25386` | version 1 |
| Home, sidebar collapsed | `HomeView/09_SidebarCollapsed` `97:25684` | version 1 |
| Incognito home | `HomeView/10_Incognito` `97:25692` | outside version 1 |
| Home with usage banner | `SystemState/03_UsageWarningHome` `97:126672` | V1? (needs a usage limit) |
| Start page: connected apps and suggestion cards | `StartPage/01_ConnectedAndSuggestions` `106:3` | outside version 1 |
| Home with setup-guide checklist | `SetupGuide/08_HomeChecklist` `106:2848` | outside version 1 |
| Home with tips panel | `Tips/10_TipsPanel` `106:3410` | outside version 1 |

Pictures: `../images/navigation-01-home.png`, `../images/navigation-09-sidebar-collapsed.png`, `../images/navigation-10-incognito.png`, `../images/system-states-03-usage-warning-home.png`, `../images/home-onboarding-01-connected-and-suggestions.png`, `../images/home-onboarding-08-home-checklist.png`, `../images/home-onboarding-10-tips-panel.png`.

## Screen structure (1440 x 900)

| Region | Position (screen) | Size |
| --- | --- | --- |
| `app_sidebar` | 0, 0 | 288 x 900 |
| `main_panel` | 288, 0 | 1152 x 900 |
| `session_header` (inside main panel) | 288, 0 | 1152 x 52 |
| `content` (inside main panel) | 288, 52 | 1152 x 848 |

## Home (`97:25390`, content of `97:25386`)

### Layout rule

| Property | Value |
| --- | --- |
| Fill | none of its own (the panel behind is white) |
| Layout | column, gap 20, children centred horizontally, group centred vertically |
| Padding | bottom 140, others 0 |
| Column width | 720 (the composer and footer are fixed at 720; the greeting hugs its text) |

The bottom padding lifts the group above true centre. With a 206 high group in an 848 high area: free height is 848 - 140 = 708, top offset is (708 - 206) / 2 = 251. The group therefore sits 70 px above the geometric centre.

### Children, in order

| # | Element | Instance of | Position | Size |
| --- | --- | --- | --- | --- |
| 1 | Greeting `97:25391` | `HomeGreeting` `97:19528` | 413, 251 | 326 x 42 |
| 2 | Composer `97:25392` | `Composer` `97:11361`, state Empty, context chip hidden | 216, 313 | 720 x 96 |
| 3 | Footer `97:25393` | `ComposerFooter` `97:19517` | 216, 429 | 720 x 28 |

Left margin of the 720 column is (1152 - 720) / 2 = 216.

### `HomeGreeting` (`97:19528`)

Component note in the file: "Time-of-day greeting above the home composer."

| Property | Value |
| --- | --- |
| Layout | row, gap 14, items centred; hugs content |
| Icon | `icon_sparkle` `97:19529`, icon `sparkle`, 32 x 32, dark (gray/900 family) |
| Text | `97:19530`; display serif, 34 / 42, weight 400, `#111827` (gray/900), one line |
| Default text | `Evening thoughts` |
| Other text seen | `What should we get done?` (on `ComposerMenu/15_WorkMode`, 461 wide); `Good morning, Omar` (tips screen, 386 wide). The setup-guide screen uses a 300 wide greeting whose text was not read; see its picture |

So the greeting can carry the time of day and the user's first name. The rule that picks the wording is not shown in the design file beyond the component note.

### Composer and footer on home

- Composer: state Empty, placeholder `How can I help you today?`, 96 high. The context chip is hidden on home (`showContext` off); plus, dictation and voice buttons show. See `composer.md`.
- Footer, left to right: project picker `Project` with a caret, a spacer, model `[model name]` + effort `Extra`, mode `Auto`, `Apps` chip. For version 1 only the model control is in scope; the rest is recorded in `composer.md` as outside version 1.

Nothing else is on the plain home: no suggestion chips, no recent chats, no disclaimer line.

## Sidebar collapsed (`97:25684`)

| Region | Position (screen) | Size |
| --- | --- | --- |
| `app_sidebar_rail` `97:25685` | 0, 0 | 56 x 900 |
| `main_panel` `97:25686` | 56, 0 | 1384 x 900 |
| `session_header` `97:25687` | 56, 0 | 1384 x 52 |
| `content` `97:25688` | 56, 52 | 1384 x 848 |

Same layout rule and same vertical positions (251, 313, 429). Only the horizontal centring changes with the wider area:

| Element | Position | Size |
| --- | --- | --- |
| Greeting `97:25689` | 529, 251 | 326 x 42 |
| Composer `97:25690` | 332, 313 | 720 x 96 |
| Footer `97:25691` | 332, 429 | 720 x 28 |

The column stays 720 wide; it does not grow with the content area. Behaviour when the content area is narrower than 720 is not shown in the design file.

## Incognito home (`97:25692`, outside version 1)

Content `97:25696` uses the same rule (column, gap 20, centred, bottom padding 140). The greeting is replaced by an incognito block.

| # | Element | Position | Size |
| --- | --- | --- | --- |
| 1 | `incognito_greeting` `97:25697` | 369.5, 214 | 413 x 116 |
| 2 | Composer `97:25702` | 216, 350 | 720 x 96 |
| 3 | Footer `97:25703` | 216, 466 | 720 x 28 |

`incognito_greeting`: column, gap 8, centred.

| # | Part | Detail |
| --- | --- | --- |
| 1 | `ghost` `97:25698` | 44 x 44 circle (radius 22), fill `#F3F4F6` (gray/100); icon `ghost` 22 x 22 centred |
| 2 | Title `97:25700` | `Incognito chat`; display serif, 28 / 36, weight 400, `#111827` (gray/900) |
| 3 | Body `97:25701` | `Incognito chats aren’t saved to history or used to update memory.`; interface sans, 14 / 20, weight 400, `#6B7280` (gray/500), one line |

Also on this screen: a `Tooltip` `97:25704` at screen 1300, 52 (97 x 26) with the text `Incognito is on`, hanging below the ghost button at the right end of the session header. Tooltip style: fill `#111827`, radius 6, padding 5 top/bottom, 8 left/right, text 12 / 16, weight 500, `#FFFFFF`.

The composer on this screen shows the context chip (`42%`), unlike the plain home.

## Home with usage banner (`97:126672`)

Content `97:126676`, same rule (gap 20, bottom padding 140).

| # | Element | Position | Size |
| --- | --- | --- | --- |
| 1 | Greeting `97:126677` | 413, 217 | 326 x 42 |
| 2 | `UsageBanner` `97:126678` | 216, 279 | 720 x 48 |
| 3 | Composer `97:126679` | 216, 347 | 720 x 96 |
| 4 | Footer `97:126680` | 216, 463 | 720 x 28 |

The banner goes between the greeting and the composer and pushes the group up by 34 px. Banner detail is in `first-run-and-system-states.md`.

## Start page with connected apps and suggestions (`106:3`, outside version 1)

Here the content sits in `home_body` `106:1017` > `content` `106:1018` (1152 x 848). The rule changes slightly: column, gap 18, centred, bottom padding 100.

| # | Element | Position | Size |
| --- | --- | --- | --- |
| 1 | Greeting `106:1019` | 413, 200 | 326 x 42 |
| 2 | Composer `106:1024` (context chip hidden) | 216, 260 | 720 x 96 |
| 3 | Footer `106:1051` | 216, 374 | 720 x 28 |
| 4 | `connected_apps` `106:1081` | 344.5, 420 | 463 x 20 |
| 5 | `suggestions` `106:1105` | 216, 458 | 720 x 90 |

### Connected apps row

Row, gap 8, items centred, hugs content.

| # | Part | Detail |
| --- | --- | --- |
| 1 | Label | `Connected`; 12 / 16, weight 500, `#6B7280` (gray/500) |
| 2 to 5 | `MetaChip` (`97:19603`) x 4 | fill `#F3F4F6` (gray/100), radius 6, padding 2 top/bottom, 6 left/right, gap 4; icon 12 x 12; text 12 / 16, weight 500, `#374151` (gray/700); 20 high |
| 6 | Link | `+ Connect more`; 12 / 16, weight 500, `#111827` (gray/900); plain text, no underline |

Chips: `Mail` (icon `envelope-simple`), `Calendar` (icon `calendar`), `Files` (icon `folder`), `Design files` (icon `frame-corners`).

### Suggestion cards

Row, gap 12, three cards of equal width (232 each), 90 high.

| Property | Value |
| --- | --- |
| Card | fill `#FFFFFF`, 1 px border `#E5E7EB` (gray/200), radius 12, padding 12 top/bottom, 14 left/right, column gap 6, left-aligned, no shadow |
| Icon | 16 x 16 at the top |
| Title | 14 / 20, weight 600, interface sans, `#111827` |
| Description | 12 / 16, weight 400, `#6B7280`, width 200 |

| Card | Icon | Title | Description |
| --- | --- | --- | --- |
| `106:1106` | `calendar` | `Plan my week` | `From your calendar and open tasks` |
| `106:1111` | `envelope-simple` | `Catch up on mail` | `Summarize unread and draft replies` |
| `106:1116` | `file-text` | `Review the palette doc` | `Suggest edits inline` |

Hover and pressed states of the cards are not shown in the design file.

## Setup-guide checklist (`106:2848`, outside version 1)

`home_body` `106:3023` (1152 x 848) splits into two columns:

| Column | Position | Size |
| --- | --- | --- |
| `content` `106:3024` | 0, 0 | 772 x 848 |
| `setup_guide` `106:3072` | 772, 0 | 380 x 848 |

`content` keeps the start-page rule (gap 18, bottom padding 100) in its narrower area: greeting `106:3025` at 236, 273 (300 x 42); composer `106:3029` at 26, 333; footer `106:3047` at 26, 447.

### `setup_guide` panel

| Property | Value |
| --- | --- |
| Fill / border | `#FFFFFF`; 1 px left border `#E5E7EB` (gray/200) |
| Layout | column, gap 10, padding 20 top/bottom, 16 left/right |
| Title `106:3073` | `Get set up · 2 of 5`; 14 / 20, weight 600, `#111827` |
| Progress track `106:3074` | 347 x 6, radius 3, fill `#F3F4F6` (gray/100) |
| Progress fill `106:3075` | 139 x 6 (2 of 5), radius 3, fill `#111827` (gray/900) |
| Tasks | five `SetupTask` (`101:5327`), each 347 x 59 |

`SetupTask`: row, gap 10, items centred, padding 10 top/bottom, 12 left/right, radius 10, 1 px border `#E5E7EB`.

| State | Fill | Icon (18 x 18) | Title | Description | Button |
| --- | --- | --- | --- | --- | --- |
| Todo `101:5325` | `#FFFFFF` | `circle`, light gray | 14 / 20, weight 500, `#111827` | 12 / 16, weight 400, `#6B7280` | `Start`: fill white, 1 px `#D1D5DB`, radius 4, padding 7 / 11, text 12 / 16, weight 500, `#374151` |
| Done `101:5326` | `#F9FAFB` (gray/50) | `check-circle`, `#059669` (green/600) | same size, `#6B7280` | same size, `#6B7280` | none |

| # | State | Title | Description |
| --- | --- | --- | --- |
| 1 | Done | `Tell Isonomic about you` | `Name and role` |
| 2 | Done | `Connect your mail` | `Draft replies for you` |
| 3 | Todo | `Connect your calendar` | `Plan your week` |
| 4 | Todo | `Create a project` | `Keep files and chats together` |
| 5 | Todo | `Schedule a morning brief` | `Delivered weekdays at 7:30` |

How the panel is closed is not shown in the design file.

## Tips panel (`106:3410`, outside version 1)

Content `106:3586` (1152 x 848), start-page rule (gap 18, bottom padding 100): greeting `106:3587` at 383, 211 (386 x 42); composer `106:3591` at 216, 271; footer `106:3609` at 216, 385; `tips_panel` `106:3634` at 216, 431 (720 x 106).

`tips_panel`: row, gap 12, cards top-aligned, three cards of equal width (232).

| Property | Value |
| --- | --- |
| Card | 1 px border `#E5E7EB`, radius 12, padding 12 top/bottom, 14 left/right, column gap 6 |
| Head row | row, gap 6: icon 14 x 14 + title 14 / 20, weight 600, `#111827` |
| Description | 12 / 16, weight 400, `#6B7280`, width 200, wraps |
| Action line | 12 / 16, weight 500 |

| Card | Fill | Icon | Title | Description | Action line | Height |
| --- | --- | --- | --- | --- | --- | --- |
| `106:3635` (done) | `#F9FAFB` (gray/50) | `check-circle`, `#059669` (green/600) | `Try Cmd+K` | `Search chats or start one from anywhere` | `Done`, `#9CA3AF` (gray/400) | 106 |
| `106:3642` | `#FFFFFF` | `lightbulb` | `Pin a project` | `Keep its chats one click away` | `Pin →`, `#111827` | 90 |
| `106:3649` | `#FFFFFF` | `lightbulb` | `Ask for a chart` | `“Chart my usage this month”` | `Try it →`, `#111827` | 90 |

## Other frames on the same page (outside version 1)

- `Overview/02_Workspace` `106:1121`: a monthly overview page with four number tiles, a line chart, recent chats and created files.
- `AwaySummary/03_WhileYouWereAway` `106:1425`: a thread whose first item is a card listing what happened while the user was away.
- `Recap/04_WeeklyRecap` `106:2021`: a thread with a weekly recap card (number tiles, a chart, a "needs a look" list).
- `PromptLibrary/09_Library` `106:3129`: a grid of prompt template cards with a fill-in dialog open.

## Not shown in the design file

- Home in a window narrower than the 720 column plus margins, or shorter than the group.
- A loading state for home, or a home with no model available.
- Any animation between home and the first message of a thread.
- Dark theme.

## Oddities

- Two centring rules: gap 20 with bottom padding 140 on the Navigation and System States pages, gap 18 with bottom padding 100 on the Home, Onboarding and Library page (the later page, node ids `106:*`).
- The incognito home shows a `42%` context chip in an empty composer; the plain home hides the chip.
- The home `SessionHeader` differs between pages; see `app-shell.md`.
