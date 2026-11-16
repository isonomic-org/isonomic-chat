# Sidebar: nav items, groups, session rows, account row

Source: the design file, component `AppSidebar` `97:19485` on "Components 0.2" and its instances on "Navigation, Sidebar and Search 0.2". The sidebar frame itself (288 wide, regions, what is pinned) and the colour and type tables are in `app-shell.md`. Menus are in `search-and-menus.md`. Pixels are design pixels at 1x.

Component note in the design file: "Left app sidebar: toggle + wordmark, primary nav (nav_*), Pinned and Recents (pinned_0n, recent_nn rows), account. Set an item state=Selected per screen."

## Screens covered

| Frame | Node | Screenshot |
| --- | --- | --- |
| `HomeView/01_Home` | `97:25386` | `../images/navigation-01-home.png` |
| `Sidebar/15_PinnedItemMenu` | `97:25798` | `../images/navigation-15-pinned-item-menu.png` |
| `Sidebar/16_PinnedCollapsed` | `97:25814` | `../images/navigation-16-pinned-collapsed.png` |
| `SessionView/03_Complete` (selected chat row) | `97:41023` | `../images/lifecycle-03-complete.png` |

## Vertical order in the expanded sidebar

1. `sidebar_top`: toggle button and "Isonomic" wordmark (see `app-shell.md`).
2. `nav` `97:19489`: six `SidebarNavItem` rows, 1 px apart.
3. `session_list` `97:19496`: `SidebarGroupLabel` "Pinned", five rows, `SidebarGroupLabel` "Recents", twelve rows; everything 1 px apart.
4. `SidebarAccount` `97:19477`, pinned to the bottom.

Rows in `nav` and `session_list` start 8 px from the sidebar's left edge and are 271 wide.

## SidebarNavItem (`97:11268`)

Component note: "Primary navigation row in AppSidebar. Selected = current page." Properties: `state` (Default, Hover, Selected), `label`, `icon` (swap), `showDot`.

Layout, all states: height 32, radius 8, row with items centred vertically, gap 10, padding left and right 10. Children in order: `icon` 16 x 16, label (hugs its text, single line), flexible `spacer`, optional `dot`.

| State | Node | Fill | Label |
| --- | --- | --- | --- |
| Default | `97:11269` | none | 14 / 20 regular, `gray/800` #1F2937 |
| Hover | `97:11274` | `gray/100` #F3F4F6 | same as Default |
| Selected | `97:11279` | `gray/200` #E5E7EB | 14 / 20 medium, `gray/800` #1F2937 |

Icon colour is `gray/700` #374151 in every state. `dot`: a 6 x 6 circle, `gray/900` #111827, at the right end (its right edge is 10 px inside the row). Focus, pressed and disabled states are not shown in the design file.

Rows in the sidebar, top to bottom:

| Layer | Node | Label | Icon | Dot | Version 1 |
| --- | --- | --- | --- | --- | --- |
| `nav_new` | `97:19490` | "New chat" | `plus` | no | In scope |
| `nav_search` | `97:19491` | "Search" | `magnifying-glass` | no | In scope if chat search ships; opens the palette |
| `nav_projects` | `97:19492` | "Projects" | `folders` | no | Outside version 1 |
| `nav_artifacts` | `97:19493` | "Artifacts" | `squares-four` | no | Outside version 1 |
| `nav_scheduled` | `97:19494` | "Scheduled" | `clock` | yes | Outside version 1 |
| `nav_customize` | `97:19495` | "Customize" | `puzzle-piece` | no | Outside version 1 |

On every frame of the Navigation page (home screens) `nav_new` is in the Selected state. On `SessionView/03_Complete` no nav row is selected; a session row is selected instead. What the dot on "Scheduled" means is not stated in the file.

## SidebarGroupLabel (`97:19474`)

Component note: "Collapsible group heading in AppSidebar (Pinned, Recents)." Property: `label`.

| Property | Value |
| --- | --- |
| Layout | Row, items centred vertically, gap 4; padding top 10, bottom 4, left and right 10; height 30 |
| Label | 12 / 16 medium (`text-xs/leading-4/font-medium`), `gray/500` #6B7280; "Pinned" or "Recents" |
| Caret | `icon_caret_down` `97:19476`: `caret-down` 12 x 12, `gray/400` #9CA3AF, directly after the label |
| States | One master only; no hover style shown |

### Pinned group collapsed (`Sidebar/16_PinnedCollapsed` `97:25814`)

| What changes | Detail |
| --- | --- |
| Caret | The caret next to "Pinned" points right (`caret-right`) instead of down; same size and colour as seen in the screenshot |
| Rows | All five pinned rows are removed from the layout (no placeholder, no count badge) |
| Recents | The "Recents" label moves up to sit directly under the "Pinned" label (1 px gap), with its caret still pointing down and all twelve rows visible (the list now fits without clipping) |
| Not shown | Collapsing "Recents", any animation, and whether the collapsed state persists |

## SidebarSessionRow (`97:11285`)

Component note: "Session, project or artifact row in AppSidebar. status: Running (grey dot), Unread (blue), Attention (orange)." Properties: `state` (Default, Hover, Selected), `status` (None, Running, Unread, Attention), `label`, `icon` (swap).

Layout, all variants: height 32, radius 8, row with items centred vertically, gap 10, padding left 10, right 6. Children in order:

| Order | Layer | Detail |
| --- | --- | --- |
| 1 | `icon` | 16 x 16, `gray/600` #4B5563. Default icon `chat-circle` |
| 2 | Label | Fills the remaining width; 14 / 20 regular, `gray/800` #1F2937; single line, truncated with an ellipsis. The weight stays regular in the Selected state (unlike `SidebarNavItem`) |
| 3 | `status` | Only when status is not None. A 20 x 20 box holding a 6 px dot (radius 3) at its centre |
| 4 | `more` | Only in the Hover state. A 20 x 20 ghost icon button (the 28 px `IconButton` scaled down; radius about 4.3) with a `dots-three` icon about 11.4 px, `gray/500` #6B7280. It comes after the status dot, so both show on hover |

Fill by state: Default none; Hover `gray/100` #F3F4F6; Selected `gray/200` #E5E7EB.

Status dot colour: Running `gray/400` #9CA3AF; Unread `gray/900` #111827; Attention `orange/500` #F97316. The component note calls Unread "blue" but the dot drawn is `gray/900`.

All twelve variants:

| State | None | Running | Unread | Attention |
| --- | --- | --- | --- | --- |
| Default | `97:11286` | `97:11289` | `97:11294` | `97:11299` |
| Hover | `97:11304` | `97:11308` | `97:11314` | `97:11320` |
| Selected | `97:11326` | `97:11329` | `97:11334` | `97:11339` |

| Variant | What is visible |
| --- | --- |
| Default / None | Icon and label only; no fill |
| Default / Running, Unread, Attention | Adds the status dot at the right (dot centre 16 px from the row's right edge) |
| Hover / None | `gray/100` fill; adds the `more` button at the right |
| Hover / Running, Unread, Attention | `gray/100` fill; status dot, then the `more` button 10 px to its right |
| Selected / None | `gray/200` fill; no `more` button |
| Selected / Running, Unread, Attention | `gray/200` fill; status dot; no `more` button |

Not shown in the design file: focus ring, pressed state, a Selected row while hovered, inline rename, drag to reorder, and an empty list.

### Rows in the sample sidebar

Text is written exactly as in the file, with third-party names replaced by placeholders. All rows are Default state unless a screen says otherwise.

| Layer | Node | Label | Icon | Status |
| --- | --- | --- | --- | --- |
| `pinned_01` | `97:19498` | "Isonomic" | `folder-simple` | None |
| `pinned_02` | `97:19499` | "Content" | `folder-simple` | Unread |
| `pinned_03` | `97:19500` | "AIOS" | `folder-simple` | Unread |
| `pinned_04` | `97:19501` | "Write grant document assets" | `chat-circle` | None |
| `pinned_05` | `97:19502` | "Isonomic website rebuild" | `chat-circle` | None |
| `recent_01` | `97:19504` | "Visual answer formats demo" | `chat-circle` | None |
| `recent_02` | `97:19505` | "Agentic UI design reference" | `chat-circle` | Running |
| `recent_03` | `97:19506` | "Isonomic UI buildout" | `chat-circle` | Unread |
| `recent_04` | `97:19507` | "Isonomic Desktop prototype" | `chat-circle` | Running |
| `recent_05` | `97:19508` | "Daily autobrief" | `clock` | Unread |
| `recent_06` | `97:19509` | "Email security strategy" | `chat-circle` | None |
| `recent_07` | `97:19510` | "[model name] open-source integration" | `chat-circle` | None |
| `recent_08` | `97:19511` | "Isonomic agent VM architecture" | `file-text` | None |
| `recent_09` | `97:19512` | "AIOS second setup run" | `chat-circle` | Attention |
| `recent_10` | `97:19513` | "Isonomic SDK kickoff brief" | `chat-circle` | None |
| `recent_11` | `97:19514` | "Current CEO of [third-party service]" | `chat-circle` | None |
| `recent_12` | `97:19515` | "Weekday morning brief setup" | `clock` | None |

Icon meaning by use: `chat-circle` = a chat; `folder-simple` = a project; `clock` = a scheduled task run; `file-text` = an artifact. In version 1 only chat rows exist, so every row uses `chat-circle`. Pinning is not in the version 1 scope list; record the "Pinned" group as outside version 1 unless decided otherwise, leaving one "Recents" list.

Selected example: on `SessionView/03_Complete` the row `recent_03` is Selected with status Unread (`gray/200` fill, dot kept).

## SidebarAccount (`97:19477`)

Component note: "Account row at the foot of AppSidebar: opens the account menu; devices button opens apps and devices." Properties: `name`, `plan`.

Full sidebar width, height 45 (1 px top border `gray/200` #E5E7EB, 8 padding, 28 content, 8 padding). Row, items centred vertically, gap 8, padding left 10, right 8. No fill of its own (the sidebar's `gray/50` shows through).

| Order | Layer | Node | Detail |
| --- | --- | --- | --- |
| 1 | `avatar` | `97:19478` | 24 x 24 circle (radius 12), fill `gray/500` #6B7280, with the initial "O" centred: 12 / 12 regular, `white` |
| 2 | `identity` | `97:19479` | Row, gap 4: name "Omar" 14 / 20 medium, `gray/900` #111827; plan "· Ultra" 12 / 16 regular, `gray/500` #6B7280; `icon_caret_down` `97:19482`, `caret-down` 12 x 12, `gray/400` #9CA3AF |
| 3 | `spacer` | `97:19483` | Flexible |
| 4 | `devices` | `97:19484` | `IconButton` 28 x 28, radius 6, no fill; icon `devices` 16 x 16, `gray/500` #6B7280. Outside version 1 |

Clicking the avatar and identity area opens the account menu (`AccountMenu/11_Open`, see `search-and-menus.md`). Hover, pressed and open styles of the row are not shown in the design file; in `11_Open` the row looks the same as when the menu is closed.

## Pinned item menu (`Sidebar/15_PinnedItemMenu` `97:25798`)

The menu for a session row, opened from the row's `more` button.

| Property | Value |
| --- | --- |
| Trigger row | `pinned_01` "Isonomic", drawn in the Hover state (`gray/100` fill) while the menu is open |
| Menu frame | `menu` `97:25806`, 220 x 211, at x 248, y 300 in the window. Its left edge is 31 px left of the row's right edge (279) and its top is 16 px below the row's top (284), so it hangs from the `more` button and overlaps the main panel by 180 px |
| Scrim | None; the rest of the window is not dimmed |
| Container style | Shared menu container: see "Menu container" in `search-and-menus.md` |

Items, top to bottom (each an instance of `MenuItem` `97:11392`, 210 wide, 32 high):

| Node | Label | Icon | Right side | Tone |
| --- | --- | --- | --- | --- |
| `97:25807` | "Open" | `arrow-up-right` | none | Default |
| `97:25808` | "Open in new tab" | `browser` | none | Default |
| `97:25809` | "Unpin" | `push-pin` | none | Default |
| `97:25810` | "Rename" | `pencil-simple` | none | Default |
| `97:25811` | "Add to project" | `folder-simple` | submenu caret (`caret-right`) | Default |
| `97:25812` | `MenuSeparator` | | | |
| `97:25813` | "Delete" | `trash` | none | Destructive |

No item is drawn hovered. The "Add to project" submenu is not shown here. For version 1: "Rename" and "Delete" are in scope; "Open in new tab", "Unpin" and "Add to project" are outside version 1.

## Sidebar entries outside version 1

| Entry | Where | What the design shows |
| --- | --- | --- |
| Projects | Nav row; rail button; `folder-simple` rows in Pinned | Row and icon only on these pages |
| Artifacts | Nav row; rail button; `file-text` rows | Row and icon only |
| Scheduled | Nav row with a dot; rail button; `clock` rows | Row, dot and icon only |
| Customize | Nav row; rail button | Row and icon only |
| Devices button | Account row, right end | 28 px icon button, `devices` icon |
| Pinned group | `session_list` | Group label, five rows, collapse; "Unpin" in the row menu |
| Status dots Running and Attention | Session rows | Grey and orange dots (agent runs and approvals) |
| "Open in new tab", "Add to project" | Row menu | Menu items |
| Plan label "· Ultra" | Account row | Text after the name |

The Unread dot is not in the version 1 scope list either; keep the row able to show one status dot so it can be added later.
