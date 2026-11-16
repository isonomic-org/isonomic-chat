# Search palette, account menu, shortcut dialogs and shared menu parts

Source: the design file, page "Navigation, Sidebar and Search 0.2" (`97:25384`) and "Components 0.2" (`97:3`). Colour tokens and type styles are listed in `app-shell.md`. Pixels are design pixels at 1x; positions are in the 1440 x 900 window. All text is the interface sans.

## Screens covered

| Frame | Node | Overlay node | Screenshot |
| --- | --- | --- | --- |
| `SearchPalette/02_Empty` | `97:25394` | `search_palette` `97:25403` | `../images/navigation-02-empty.png` |
| `SearchPalette/03_Results` | `97:25445` | `97:25454` | `../images/navigation-03-results.png` |
| `SearchPalette/04_ResultActions` | `97:25490` | `97:25499`, `menu` `97:25529` | `../images/navigation-04-result-actions.png` |
| `SearchPalette/05_ArtifactsType` | `97:25536` | `97:25545` | `../images/navigation-05-artifacts-type.png` |
| `SearchPalette/06_Filters` | `97:25577` | `97:25586` | `../images/navigation-06-filters.png` |
| `SearchPalette/07_FilterByType` | `97:25615` | `97:25624` | `../images/navigation-07-filter-by-type.png` |
| `SearchPalette/08_ComposeMode` | `97:25652` | `97:25661` | `../images/navigation-08-compose-mode.png` |
| `AccountMenu/11_Open` | `97:25705` | `menu` `97:25713` | `../images/navigation-11-open.png` |
| `AccountMenu/12_LearnMore` | `97:25724` | `menu` `97:25732`, `submenu` `97:25743` | `../images/navigation-12-learn-more.png` |
| `KeyboardShortcuts/13_General` | `97:25750` | `dialog` `97:25759` | `../images/navigation-13-general.png` |
| `KeyboardShortcuts/14_Composer` | `97:25774` | `dialog` `97:25783` | `../images/navigation-14-composer.png` |

Every one of these is drawn over the home screen (expanded sidebar, Home header).

## Shared parts

### Scrim

`scrim` (for example `97:25402`): a 1440 x 900 rectangle over the whole window, sidebar included. Fill #111827 (`gray/900`) at 25 percent opacity. Used under the search palette and the dialogs. Menus and the tooltip have no scrim.

### Menu container

Used by `menu` and `submenu` frames. Fill `white`; border 1 px `gray/200` #E5E7EB; radius 10; padding 4 on all sides; vertical stack, no gap; items stretch to the inner width (menu width minus 10). Shadow style named `ring-1/ring-black ring-opacity-5 shadow-lg`: three layers, #000000 at 5 percent with 0 offset, 0 blur, 1 spread; #000000 at 5 percent, offset (0, 4), blur 6, spread -2; #000000 at 10 percent, offset (0, 10), blur 15, spread -3. Widths seen: 220 and 256.

### MenuItem (`97:11392`)

Component note: "Row in any dropdown or context menu: optional icon, shortcut, check and submenu caret." Properties: `state` (Default, Hover), `tone` (Default, Destructive), `label`, `shortcut`, `icon` (swap), `showIcon`, `showShortcut`, `showCheck`, `showSubmenu`.

Height 32; radius 6; row, items centred vertically, gap 10; padding left and right 8. Master width 232.

| Order | Part | Detail |
| --- | --- | --- |
| 1 | `icon` | 16 x 16; `gray/600` #4B5563, or `red/600` #DC2626 for Destructive. Master icon `gear` |
| 2 | Label | Fills the width; 14 / 20 regular; `gray/800` #1F2937, or `red/600` #DC2626 for Destructive |
| 3 | Shortcut | Optional; 12 / 16 regular, `gray/400` #9CA3AF, single line. Master text "⌘K" |
| 4 | `check` | Optional; `check` icon 16 x 16, `gray/900` #111827 |
| 5 | `submenu` | Optional; `caret-right` 14 x 14, `gray/400` #9CA3AF |

| Variant | Node | Fill |
| --- | --- | --- |
| Default / Default | `97:11393` | none |
| Hover / Default | `97:11399` | `gray/100` #F3F4F6 |
| Default / Destructive | `97:11405` | none |
| Hover / Destructive | `97:11411` | `gray/100` #F3F4F6 |

Hover changes only the fill. An item whose submenu is open is drawn with the Hover variant. Disabled, focus and pressed states are not shown in the design file.

### MenuSeparator (`97:19541`) and MenuHeader (`97:19543`)

| Part | Detail |
| --- | --- |
| `MenuSeparator` | "Divider between menu groups." Full item width, 9 high: 4 padding, a 1 px `line` (`97:19542`) in `gray/200` #E5E7EB, 4 padding |
| `MenuHeader` | "Muted label or meta line at the top of a menu group." Padding top 6, bottom 4, left and right 8; height 26. Text 12 / 16 medium, `gray/500` #6B7280. Master text "Header" |

### Kbd (`97:19537`)

"Keyboard shortcut chip." Fill `gray/100` #F3F4F6; border 1 px `gray/200` #E5E7EB; radius 4; padding 1 top and bottom, 6 left and right; height 20; hugs its text. Text 12 / 16 medium, `gray/600` #4B5563. Master text "⌘K". One state only.

### Tooltip (`97:19539`)

"Dark hover tooltip." Fill `gray/900` #111827; radius 6; padding 5 top and bottom, 8 left and right; height 26; hugs its text; no arrow, border or shadow. Text 12 / 16 medium, `white`. Only use shown: "Incognito is on" on `HomeView/10_Incognito` (`97:25704`), see `app-shell.md`. Delay and placement rules are not shown in the design file.

### TabPill (`97:11428`)

"Pill tab for type filters and segmented choices." Padding 5 top and bottom, 10 left and right; radius 8; height 30; hugs its label. Label 14 / 20 medium.

| Variant | Node | Fill | Label colour |
| --- | --- | --- | --- |
| selected=false | `97:11429` | none | `gray/500` #6B7280 |
| selected=true | `97:11431` | `gray/100` #F3F4F6 | `gray/900` #111827 |

Hover state is not shown.

### SearchPaletteRow (`97:11419`)

"Result or action row in the Cmd+K palette." Height 36; radius 8; row, items centred vertically, gap 12; padding left and right 10; 622 wide in the palette.

| Order | Part | Detail |
| --- | --- | --- |
| 1 | `icon` | 16 x 16, `gray/500` #6B7280 |
| 2 | Label | Fills the width; 14 / 20 regular, `gray/800` #1F2937; single line with ellipsis |
| 3 | Meta | Right side; 12 / 16 regular, `gray/400` #9CA3AF; single line. Holds a time, a status, a type, a filter key or a shortcut. May be empty |

| Variant | Node | Fill |
| --- | --- | --- |
| Default | `97:11420` | none |
| Selected (the keyboard cursor row) | `97:11424` | `gray/100` #F3F4F6 |

### ShortcutRow (`97:19567`)

"One row in the keyboard-shortcut sheet. Override key_1/key_2 Kbd text." Height 36; bottom border 1 px `gray/100` #F3F4F6; row, items centred vertically, gap 8; padding left and right 4. Label fills the width, 14 / 20 regular, `gray/800` #1F2937. Right side `keys`: one or two `Kbd` chips, gap 4 (`showKey2` toggles the second). Master: "New chat" with "⇧⌘O" and "⌘N"; master width 420.

### TextField (`97:11433`)

"Single-line input. Focus = 2px blue ring." 320 x 38 master; fill `white`; radius 8; row, gap 8, padding left and right 12. Optional `icon` 16 x 16 (`magnifying-glass`, `gray/400` #9CA3AF), then text 14 / 20 regular, `gray/400` #9CA3AF ("Placeholder").

| State | Node | Border |
| --- | --- | --- |
| Default | `97:11434` | 1 px `gray/300` #D1D5DB |
| Focus | `97:11437` | 2 px `gray/900` #111827 (the note says "blue"; the drawn value is `gray/900`) |

Entered-text colour, error and disabled states are not shown. No screen in this document uses `TextField`; the palette input is its own frame.

## Search palette

### Frame

`search_palette`: 640 wide at x 400, y 96 (centred on the window, not on the main panel). Height hugs the content. Fill `white`; border 1 px `gray/200` #E5E7EB; radius 14; content clipped; shadow `shadow/2xl` (#000000 at 25 percent, offset (0, 25), blur 50, spread -12). Scrim underneath. Vertical stack, no gap, 638 inner width:

| Order | Layer | Height | Detail |
| --- | --- | --- | --- |
| 1 | `palette_input` | 53 (55 with a filter token) | Bottom border 1 px `gray/200`. Row, gap 10, padding 14 top and bottom, 16 left and right. Leading icon 18 x 18, `gray/500` #6B7280 (`magnifying-glass`; `chat-circle` in compose mode). Text fills the width, 16 / 24 regular: placeholder `gray/400` #9CA3AF, typed text `gray/900` #111827. Trailing `Kbd` "Esc" |
| 2 | `type_tabs` | 38 | Only in states 02 to 05. Row, gap 4, padding top 8, left and right 12. Five `TabPill`: "All", "Chats", "Projects", "Artifacts", "Scheduled" |
| 3 | `palette_results` | varies | Vertical stack, no gap; padding top 6, bottom 8, left and right 8. Holds `group_label` frames and `SearchPaletteRow` instances |
| 4 | `palette_footer` | 37 | Fill `gray/50` #F9FAFB; top border 1 px `gray/200`. Row, gap 16, padding 8 top and bottom, 14 left and right. Holds `hint` groups: a `Kbd` then a label 12 / 16 regular `gray/500` #6B7280, gap 6 |

`group_label`: padding top 8, bottom 4, left and right 10; height 28; text 12 / 16 medium, `gray/500` #6B7280.

Footer hints in states 02 to 07: "↑↓" Navigate, "↵" Open, "Tab" Compose, "/" Filters.

A maximum height, scrolling of long result lists, a no-results state, a loading state and match highlighting are not shown in the design file.

### The seven states

| State | Height | Input | Tabs | Results |
| --- | --- | --- | --- | --- |
| 02 Empty | 580 | Placeholder "Search or ask…" | "All" selected | No row selected. Four groups, below |
| 03 Results | 408 | Typed "isonomic ui" | "All" selected | First row selected |
| 04 ResultActions | 280 | Typed "isonomic ui" | "All" selected | Only the "Chats" group; first row selected; actions menu open beside it |
| 05 ArtifactsType | 352 | Placeholder "Search or ask…" | "Artifacts" selected | Group "Recent artifacts"; first row selected |
| 06 Filters | 378 | Typed "/" | No tab row | Groups "Filters" and "Actions"; first row selected |
| 07 FilterByType | 316 | Token "type:" then an empty text field | No tab row | Group "Type"; first row selected |
| 08 ComposeMode | 170 | Placeholder "Ask anything…", `chat-circle` icon | No tab row | Help text and a toolbar instead of rows |

Rows by state (label / meta / icon):

| State | Group | Rows |
| --- | --- | --- |
| 02 | "Needs attention" | "AIOS second setup run" / "Waiting for you" / `chat-circle` |
| 02 | "Active" | "Agentic UI design reference" / "Running" / `chat-circle`; "Isonomic Desktop prototype" / "Running" / `chat-circle` |
| 02 | "Recents" | "Visual answer formats demo" / "14m" / `chat-circle`; "Isonomic palette note" / "22m" / `file-text`; "Isonomic" / "1h" / `folder-simple`; "Daily autobrief" / "11h" / `clock` |
| 02 | "Actions" | "New chat" / "⇧⌘O" / `plus`; "Open settings" / "⇧⌘," / `gear` |
| 03, 04 | "Chats" | "Isonomic UI buildout" / "2h" (selected); "Isonomic UI agentic components build" / "3h"; "Isonomic UI QA session" / "Yesterday"; all `chat-circle` |
| 03 | "Artifacts" | "Isonomic UI QA" / "Data · 34m" / `squares-four` |
| 03 | "Projects" | "Isonomic" / "9 files" / `folder-simple` |
| 05 | "Recent artifacts" | "Isonomic palette note" / "Docs" / `file-text` (selected); "Isonomic Devices V0 Look Gate" / "Page" / `browser`; "Dashboard" / "Data" / `chart-bar`; "Isonomic Cells: per-agent machines" / "Docs" / `file-text`; "Design System" / "Design system" / `swatches` |
| 06 | "Filters" | "Owner" / "owner:" / `user` (selected); "Date" / "date:" / `calendar`; "Project" / "project:" / `folder-simple`; "Type" / "type:" / `squares-four`; "Archived" / "archived:" / `archive` |
| 06 | "Actions" | "Clear filters" / empty / `x` |
| 07 | "Type" | "Chats" / `chat-circle` (selected); "Projects" / `folder-simple`; "Artifacts" / `squares-four`; "Scheduled tasks" / `clock`; "Files" / `file`; all with empty meta |

State 07 input detail: `filter_token` `97:25627` sits between the icon and the text field. Fill `gray/50` #F9FAFB; border 1 px `gray/200` #E5E7EB; radius 6; padding 2 top and bottom, 8 left and right; 51 x 26. Text "type:" 14 / 20 medium, `gray/900` #111827.

State 04 actions menu: `menu` `97:25529`, 220 x 179 at x 1052, y 196 (12 px to the right of the palette; its top is 26 px above the selected row's top at y 222). Menu container style. Items, each with a number as its shortcut text:

| Node | Label | Icon | Shortcut | Variant |
| --- | --- | --- | --- | --- |
| `97:25530` | "Open" | `arrow-up-right` | "1" | Hover |
| `97:25531` | "Open in new tab" | `browser` | "2" | Default |
| `97:25532` | "Unpin" | `push-pin` | "3" | Default |
| `97:25533` | "Rename" | `pencil-simple` | "4" | Default |
| `97:25534` | `MenuSeparator` | | | |
| `97:25535` | "Delete" | `trash` | "5" | Destructive |

How this menu is opened (key or pointer) is not shown in the design file.

State 08 compose mode, inside `palette_results` `97:25666`:

| Order | Part | Detail |
| --- | --- | --- |
| 1 | Help text `97:25667` | "Start a new chat from anywhere. Attach files or pick a project before you send." 14 / 20 regular, `gray/500` #6B7280, 560 wide, flush with the 8 px padding (not indented like rows) |
| 2 | `toolbar` `97:25668` | Row, gap 6, padding top 12, 622 wide. `icon_button_paperclip` (`IconButton` 28, `paperclip` 16, `gray/500`); a `ComposerFooter` instance (`97:19517`, mode hidden; shows "Project", `[model name]`, "Extra", "Apps"; see the composer docs); flexible spacer; `send` `97:25672`: 32 x 32 circle, fill `gray/200` #E5E7EB, `arrow-up` 16 x 16 `gray/500` (the empty, not-ready look) |

Footer hints in state 08: "Tab" Back to search, "↵" Send, "⇧↵" New line.

### Palette and version 1

| Part | Version 1 |
| --- | --- |
| Input, "Chats" results, "Recents" chats, "New chat" action, Esc, arrow keys, Enter | In scope if chat search ships |
| Tabs "Projects", "Artifacts", "Scheduled"; groups "Needs attention", "Active", "Artifacts", "Projects", "Recent artifacts" | Outside version 1 |
| Filters "Owner", "Project", "Type", "Archived" and type rows other than "Chats" | Outside version 1 |
| Compose mode: paperclip and project picker | Outside version 1 (plain text and model pick only) |
| Actions menu: "Open in new tab", "Unpin" | Outside version 1 |
| "Open settings" action | Depends on the settings scope |

## Account menu

`AccountMenu/11_Open`: `menu` `97:25713`, 256 x 278 at x 16, y 562, opened from the sidebar account row. It floats over the session list with its bottom edge (y 840) 15 px above the account row's top border; left and right edges are 16 px inside the sidebar. No scrim. Menu container style; items 246 wide.

| Node (11 / 12) | Part | Label | Icon | Right side |
| --- | --- | --- | --- | --- |
| `97:25714` / `97:25733` | `MenuHeader` | "Omar · Ultra plan" | | |
| `97:25715` / `97:25734` | `MenuItem` | "Settings" | `gear` | shortcut "⇧⌘," |
| `97:25716` / `97:25735` | `MenuItem` | "Language" | `globe` | submenu caret |
| `97:25717` / `97:25736` | `MenuItem` | "Get help" | `question` | |
| `97:25718` / `97:25737` | `MenuSeparator` | | | |
| `97:25719` / `97:25738` | `MenuItem` | "Upgrade plan" | `crown` | |
| `97:25720` / `97:25739` | `MenuItem` | "Get apps and extensions" | `download-simple` | |
| `97:25721` / `97:25740` | `MenuItem` | "Learn more" | `info` | submenu caret |
| `97:25722` / `97:25741` | `MenuSeparator` | | | |
| `97:25723` / `97:25742` | `MenuItem` | "Log out" | `sign-out` | |

In `11_Open` no item is hovered. "Log out" uses the Default tone, not Destructive. The "Language" submenu is not shown.

`AccountMenu/12_LearnMore`: same menu with "Learn more" in the Hover variant, plus `submenu` `97:25743`, 220 x 179 at x 276, y 705: 4 px to the right of the menu, crossing the sidebar edge into the main panel, bottom edge at y 884 (the submenu's last item lines up below the parent item rather than its first item lining up with it).

| Node | Label | Icon | Right side |
| --- | --- | --- | --- |
| `97:25744` | "About Isonomic" | `info` | |
| `97:25745` | "Tutorials" | `books` | |
| `97:25746` | "Usage policy" | `file-text` | |
| `97:25747` | "Privacy policy" | `shield-check` | |
| `97:25748` | `MenuSeparator` | | |
| `97:25749` | "Keyboard shortcuts" | `keyboard` | shortcut "⌘/" |

Version 1: "Settings", "Log out" and "Keyboard shortcuts" are the likely in-scope items; "Language", "Upgrade plan", "Get apps and extensions", the plan text and the policy links are outside the version 1 scope list and need a decision.

## Keyboard shortcut dialogs

`dialog`: 520 x 514 at x 460, y 193 (centred in the window), over the scrim. Fill `white`; border 1 px `gray/200` #E5E7EB; radius 12; content clipped; shadow `shadow/2xl`.

| Order | Layer | Detail |
| --- | --- | --- |
| 1 | `dialog_header` (84 high) | Row, items top-aligned, gap 8; padding top 20, bottom 12, left 24, right 20. `dialog_title` fills the width: title "Keyboard shortcuts" 18 / 28 semibold, `gray/900` #111827; 4 px gap; subtitle 14 / 20 regular, `gray/500` #6B7280. Then `icon_button_x`: `IconButton` 28 x 28, `x` icon 16, `gray/500` |
| 2 | `dialog_body` (428 high) | Vertical stack, gap 16; padding top 4, bottom 24, left and right 24. Eight `ShortcutRow`, each 470 wide, one `Kbd` each |

The two frames are the same dialog with different content; no tabs, page control or scroll is drawn, so how the user moves between the two lists is not shown in the design file.

| `13_General`, subtitle "Navigation" | Key | `14_Composer`, subtitle "Composer and session" | Key |
| --- | --- | --- | --- |
| "Search and command palette" | "⌘K" | "Send message" | "↵" |
| "New chat" | "⇧⌘O" | "New line" | "⇧↵" |
| "New incognito chat" | "⇧⌘I" | "Stop response" | "Esc" |
| "Toggle sidebar" | "⌘." | "Attach files" | "⌘U" |
| "Previous chat" | "⌥↑" | "Edit last message" | "↑" |
| "Next chat" | "⌥↓" | "Copy last reply" | "⇧⌘C" |
| "Open settings" | "⇧⌘," | "Toggle side panel" | "⌘\" |
| "Show shortcuts" | "⌘/" | "Share chat" | "⇧⌘S" |

Outside version 1: "New incognito chat", "Attach files", "Toggle side panel", "Share chat".

## Design oddities seen in these frames

| Item | Detail |
| --- | --- |
| Compose toolbar overflow | In state 08 the `ComposerFooter` instance is 720 wide inside a 622-wide toolbar, so the spacer and the `send` button sit at x 760 to 799, past the 638 px inner width of the palette, which clips its content |
| Input height | `palette_input` is 53 high in six states and 55 in state 07 because the filter token is 26 high |
| Focus colour | `TextField` note says a blue ring; the drawn border is `gray/900` |
| Submenu alignment | The "Learn more" submenu extends down past the parent menu instead of aligning its top with the parent item |
| Actions menu top | The state 04 menu starts 26 px above the selected row, not level with it |
