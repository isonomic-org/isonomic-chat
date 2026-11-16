# App shell: window frame, sidebar frame, collapsed rail, session header

Source: the design file, page "Navigation, Sidebar and Search 0.2" (`97:25384`) and "Components 0.2" (`97:3`). All values are design pixels at 1x, light theme only (no dark frames exist on these pages). Sidebar rows, groups and the account row are in `sidebar.md`; the search palette, menus and dialogs are in `search-and-menus.md`.

Colour tokens used below (the design file's variable names):

| Token | Hex | Token | Hex |
| --- | --- | --- | --- |
| `white` | #FFFFFF | `gray/500` | #6B7280 |
| `gray/50` | #F9FAFB | `gray/600` | #4B5563 |
| `gray/100` | #F3F4F6 | `gray/700` | #374151 |
| `gray/200` | #E5E7EB | `gray/800` | #1F2937 |
| `gray/300` | #D1D5DB | `gray/900` | #111827 |
| `gray/400` | #9CA3AF | `green/500` | #10B981 |
| `orange/500` | #F97316 | `red/600` | #DC2626 |

Type styles (all the interface sans unless noted):

| Style name | Size / line height | Weight |
| --- | --- | --- |
| `text-xs/leading-4/font-normal` | 12 / 16 | 400 |
| `text-xs/leading-4/font-medium` | 12 / 16 | 500 |
| `text-sm/leading-5/font-normal` | 14 / 20 | 400 |
| `text-sm/leading-5/font-medium` | 14 / 20 | 500 |
| `text-sm/leading-4/font-medium` | 14 / 16 | 500 |
| `text-base/leading-6/font-normal` | 16 / 24 | 400 |
| `text-lg/leading-7/font-semibold` | 18 / 28 | 600 |

## Screens covered

| Frame | Node | Screenshot |
| --- | --- | --- |
| `HomeView/01_Home` | `97:25386` | `../images/navigation-01-home.png` |
| `HomeView/09_SidebarCollapsed` | `97:25684` | `../images/navigation-09-sidebar-collapsed.png` |
| `HomeView/10_Incognito` (header only) | `97:25692` | `../images/navigation-10-incognito.png` |
| `SessionView/03_Complete` (header only, `session_header` `97:41026`) | `97:41023` | `../images/lifecycle-03-complete.png` |

## Window frame

Every screen is a 1440 x 900 frame with two children side by side, no gap, no outer padding. No title bar, traffic lights or window chrome is drawn.

| Region | Layer | x | Width | Height | Notes |
| --- | --- | --- | --- | --- | --- |
| Sidebar, expanded | `app_sidebar` (instance of `AppSidebar` `97:19485`) | 0 | 288 | 900 | The 288 includes a 1 px right border |
| Sidebar, collapsed | `app_sidebar_rail` (instance of `AppSidebarRail` `97:19556`) | 0 | 56 | 900 | The 56 includes a 1 px right border |
| Main panel | `main_panel` frame | 288 or 56 | 1152 or 1384 | 900 | Fills the rest of the window |

Main panel children, top to bottom:

| Region | Layer | y | Height | Notes |
| --- | --- | --- | --- | --- |
| Header | `session_header` (instance of `SessionHeader` `97:11347`) | 0 | 52 | Pinned; full panel width; fill `white` #FFFFFF; no bottom border and no shadow |
| Content | `content` frame | 52 | 848 | Fill appears white in the screenshots. What scrolls inside it is described in the thread and home docs |

Collapsing the sidebar changes only widths: sidebar 288 to 56, main panel 1152 to 1384 (x 288 to 56), header 1152 to 1384. The home column stays 720 wide and centred in the content area (composer at x 216 of 1152 expanded, x 332 of 1384 collapsed; greeting at x 413 expanded, x 529 collapsed; same y values). Behaviour at other window sizes, a minimum window width, a drag handle to resize the sidebar, and any animation between the two widths are not shown in the design file.

Overlays (search palette, dialogs, menus, tooltip) are drawn as later children of the 1440 x 900 frame, above both the sidebar and the main panel. See `search-and-menus.md`.

## Expanded sidebar frame (`AppSidebar` `97:19485`)

288 x 900. Fill `gray/50` #F9FAFB. Right border 1 px `gray/200` #E5E7EB. Vertical stack, children stretch to full width, content clipped to the frame.

| Order | Layer | Node | Height | Padding | Pinned or scrolls |
| --- | --- | --- | --- | --- | --- |
| 1 | `sidebar_top` | `97:19486` | 44 | top 10, bottom 6, left and right 12 | Pinned top |
| 2 | `nav` | `97:19489` | 205 | top and bottom 4, left and right 8 | Pinned top |
| 3 | `session_list` | `97:19496` | fills the rest (606 at 900 tall) | top and bottom 4, left and right 8 | Clipped. Its content is taller than the space (the 12th recent row is cut off by the account row in `01_Home`). No scrollbar is drawn; scrolling is implied, not shown in the design file |
| 4 | `SidebarAccount` | `97:19477` | 45 | top and bottom 8, left 10, right 8 | Pinned bottom; 1 px top border `gray/200` |

`sidebar_top` (row, items centred vertically, gap 8):

| Element | Detail |
| --- | --- |
| `toggle_sidebar` `97:19487` | Instance of `IconButton` `97:12` ("Ghost icon button, 28px"): 28 x 28, radius 6, no fill, icon centred. Icon `sidebar-simple`, 16 x 16, `gray/500` #6B7280. Hover and pressed states are not shown on these pages |
| Wordmark `97:19488` | Text "Isonomic". The display serif, semibold, 20 / 28, `gray/900` #111827, single line. It is live text, not a logo image |

Rows inside `nav` and `session_list` are 271 wide in the sidebar (288 minus the 1 px border minus 8 + 8 padding) and sit 1 px apart (gap 1). Component masters are drawn 256 wide; rows always stretch to the container width.

## Collapsed rail (`AppSidebarRail` `97:19556`)

Component note in the design file: "Collapsed sidebar: icon-only rail with account avatar."

56 x 900. Fill `gray/50` #F9FAFB. Right border 1 px `gray/200` #E5E7EB. Vertical stack, items centred horizontally, gap 4, padding top and bottom 12, left and right 8.

| Order | Layer | Node | Size | Icon (16 x 16, `gray/500` #6B7280) |
| --- | --- | --- | --- | --- |
| 1 | `toggle_sidebar` | `97:19557` | 28 x 28 | `sidebar-simple` |
| 2 | `gap` | `97:19558` | 8 x 8 empty spacer | none (so 16 px between the toggle and the first nav icon, counting the stack gaps) |
| 3 | `nav_new` | `97:19559` | 28 x 28 | `plus` |
| 4 | `nav_search` | `97:19560` | 28 x 28 | `magnifying-glass` |
| 5 | `nav_projects` | `97:19561` | 28 x 28 | `folder-simple` |
| 6 | `nav_artifacts` | `97:19562` | 28 x 28 | `squares-four` |
| 7 | `nav_scheduled` | `97:19563` | 28 x 28 | `clock` |
| 8 | `nav_customize` | `97:19564` | 28 x 28 | `puzzle-piece` |
| 9 | `spacer` | `97:19565` | flexible | pushes the avatar to the bottom |
| 10 | Avatar | instance of the avatar component `3:67` (type Circular, size 8) | 32 x 32, radius 16 | A photo fill over `avatar/background/clay` #DC8E76 |

All eight buttons are instances of `IconButton` `97:12` (28 x 28, radius 6, no fill). No selected, hover or tooltip state is shown for rail buttons.

### What the rail keeps and drops

| Expanded sidebar | Rail |
| --- | --- |
| Toggle button | Kept, same icon and size; top padding 12 instead of 10 |
| "Isonomic" wordmark | Dropped |
| Six nav rows with labels | Kept as six icon-only 28 px buttons, 4 px apart (rows were 32 high, 1 px apart) |
| Nav icon colour `gray/700` #374151 | Icons become `gray/500` #6B7280 |
| Selected nav row (`gray/200` fill) | No selected state shown, even on the home screen |
| Dot on "Scheduled" | Dropped |
| "Pinned" and "Recents" groups and every session row | Dropped; nothing replaces them |
| Account row: 24 px initial avatar, name, plan, caret, top border | Replaced by a 32 px photo avatar at the bottom; name, plan, caret and top border dropped |
| `devices` button | Dropped |

How the account menu or a chat list is reached from the rail is not shown in the design file.

## Session header (`SessionHeader` `97:11347`)

Component note in the design file: "Top bar of the main panel. Session: title menu, linked-computer indicator, side-panel toggle, Share. Home: incognito toggle."

Both kinds: height 52, fill `white` #FFFFFF, row with items centred vertically, gap 6, padding left and right 16, width = main panel width. No border, no shadow.

Component properties: `kind` (Session or Home), `title` (text), `showDevice`, `showPanel`, `showShare` (default on), `showRunning` (default off).

### kind=Home (`97:11358`)

Used on the home screen and on every frame of the Navigation page.

| Order | Layer | Node | Detail |
| --- | --- | --- | --- |
| 1 | `spacer` | `97:11359` | Flexible; pushes the button to the right edge |
| 2 | `incognito` | `97:11360` | `IconButton` 28 x 28, radius 6, no fill; icon `ghost` 16 x 16, `gray/500` #6B7280. In a 1440 window with the sidebar expanded it sits at x 1396 to 1424, y 12 to 40 |

There is no title and no left-side content in the Home kind.

`HomeView/10_Incognito` uses the same Home header with no visual change to the button (same icon, same colour, no fill). The only difference in the header area is a `Tooltip` instance (`97:25704`, component `97:19539`) reading "Incognito is on", 97 x 26, placed at x 1300, y 52 in the window (directly under the header, right edge 27 px left of the button's right edge). Tooltip styling is in `search-and-menus.md`. Whether the button gets an active style when incognito is on is not shown in the design file.

### kind=Session (`97:11348`)

Used on thread screens. In `SessionView/03_Complete` the instance is `97:41026` with `title` = "Public-key cryptography explained", device, panel and Share shown, running pill hidden.

| Order | Layer | Node | Detail |
| --- | --- | --- | --- |
| 1 | `title_button` | `97:11349` | Hugs its content. Padding 4 top and bottom, 8 left and right; radius 8; no fill; gap 6. Text: the session title, 14 / 20 medium (`text-sm/leading-5/font-medium`), `gray/800` #1F2937, single line. Then `icon_caret_down` `97:11351`: `caret-down` 14 x 14, `gray/500` #6B7280. Total height 28. With 16 px header padding the title text starts at x 24 of the main panel |
| 2 | `device` | `97:11352` | 28 x 28. A `laptop` icon in `gray/600` #4B5563 with an `online_dot`: a 6 px circle (radius 3) in `green/500` #10B981 centred at (21, 8) inside the 28 box. Shown when `showDevice` is on. Outside version 1 |
| 3 | `spacer` | `97:11355` | Flexible |
| 4 | `running_now` | `101:6025` | Hidden by default (`showRunning`). Pill: fill `gray/100` #F3F4F6, radius 999, padding 3 top and bottom, 10 left and right, gap 6. Holds a 20 x 20 `AgentOrb` (an animated mark made of 26 frames) and the text "2 running · 1 waiting", 12 / 16 medium, `gray/700` #374151. Outside version 1 |
| 5 | `panel` | `97:11356` | `IconButton` 28 x 28, radius 6, no fill; icon `list` 16 x 16, `gray/500` #6B7280. Toggles the side panel. Outside version 1 |
| 6 | `share` | `97:11357` | Button: fill `white`, border 1 px `gray/300` #D1D5DB, radius 6, shadow `shadow/sm` (0, 1, blur 2, #000000 at 5 percent). Padding 9 top and bottom, 11 left, 13 right; gap 8. Icon `export` 16 x 16, `gray/500` #6B7280. Label "Share", 14 / 16 medium (`text-sm/leading-4/font-medium`), `gray/700` #374151. Height 36. Outside version 1 |

Behaviour not shown in the design file: how a long title truncates (the title is single line with no ellipsis rule and no max width), hover and pressed styles of `title_button`, the menu the title opens (see the session management docs), and the header at narrow widths.

### Header elements and version 1

| Element | Shown in | Version 1 |
| --- | --- | --- |
| Session title with caret | Session | In scope (title display; rename lives in the title menu) |
| `device` linked-computer indicator | Session | Outside version 1 |
| `running_now` pill | Session (hidden by default) | Outside version 1 |
| `panel` side-panel toggle | Session | Outside version 1 |
| `share` button | Session | Outside version 1 |
| `incognito` button and tooltip | Home | Not in the version 1 scope list; treat as outside version 1 unless decided otherwise |

## Design oddities seen in these frames

| Item | Detail |
| --- | --- |
| Selected chat versus title | `SessionView/03_Complete` shows the title "Public-key cryptography explained" while the sidebar row marked Selected is "Isonomic UI buildout" |
| Projects icon | The expanded nav row uses `folders`; the rail button uses `folder-simple` |
| Avatar | Expanded: 24 px grey circle with the initial "O". Rail: 32 px photo avatar from a different component |
| Icon colour | Nav icons are `gray/700` in the expanded sidebar and `gray/500` in the rail |
| Tooltip position | The "Incognito is on" tooltip is not centred under the incognito button; it sits to its lower left |
