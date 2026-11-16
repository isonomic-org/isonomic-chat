# Composer, part 2: menus

Every menu that opens from the composer or its footer on the page "Composer and Session Setup 0.2" (`97:29007`). Part 1 is `composer.md`. The model picker, effort and more-models menus are detailed in `model-picker.md` and only listed here.

All positions are in the 1440 x 900 screen frame. On these screens the composer box spans x 504 to 1224, y 365 to 461, and the footer spans y 481 to 509. Trigger positions that depend on text width are measured from the pictures.

## Shared menu parts

Surface (every `menu` frame): fill `#FFFFFF`; 1 px border `#E5E7EB` (gray/200); radius 10; padding 4; column, no gap; shadow style "ring-1/ring-black ring-opacity-5 shadow-lg" (0 0 0 1 px `#0000000D`; 0 4 6 -2 `#0000000D`; 0 10 15 -3 `#0000001A`). Rows fill the inner width (menu width minus 10).

| Part | Component | Height | Detail |
| --- | --- | --- | --- |
| Row | `MenuItem` `97:11392` | 32 | row, gap 10, padding 0 / 8, radius 6. Icon 16 x 16; label 14 / 20, weight 400, `#1F2937` (gray/800), fills; optional trailing text 12 / 16, weight 400, `#9CA3AF` (gray/400); optional `check` 16 x 16 (icon `check`); optional `submenu` caret 14 x 14 (icon `caret-right`). Hover fill `#F3F4F6` (gray/100) |
| Toggle row | `MenuToggleItem` `97:11493` (enabled True / False) | 32 | same row; trailing switch 35.2 x 19.2. On: dark track (`#111827`) with a white knob at the right. Off: light gray track with the knob at the left |
| Two-line row | `MenuItemDetail` `97:11502` | 51 or 67 | see `model-picker.md` |
| Header | `MenuHeader` `97:19543` | 26 | padding 6 top, 4 bottom, 8 left/right; 12 / 16, weight 500, `#6B7280` (gray/500) |
| Separator | `MenuSeparator` `97:19541` | 9 | 1 px line `#E5E7EB` with 4 above and below |
| Search field | `TextField` `97:11433`, state Focus | 38 | fill white, 2 px border `#111827` (gray/900), radius 8, padding 0 / 12, gap 8; icon `magnifying-glass` 16; placeholder 14 / 20, `#9CA3AF` |

Submenu rule seen on every screen: a submenu opens 6 px to the right of its parent menu, its top 4 px above the top of the row that opened it (so the first row of the submenu lines up with the parent row). The parent row keeps the hover fill while the submenu is open. When the submenu would run off the bottom, it is moved up so its bottom sits 16 px above the window bottom (y 884).

All menus on this page are outside version 1 except the model picker. Whether effort and more models are in version 1 follows the model picker decision.

## Menu list

| Menu | Screen | Menu node | Size | Position (x, y) | Anchor | Version 1 |
| --- | --- | --- | --- | --- | --- | --- |
| Plus menu | `01_PlusMenu` `97:29009` | `97:29017` | 232 x 275 | 512, 467 | plus button (x 521 to 549, y 420 to 448). Top 6 below the composer box; left 8 inside the box's left edge. It covers the footer's project control | outside |
| Skills | `02_Skills` `97:29027` | `97:29045` | 236 x 237 | 750, 500 | submenu of plus menu row `Skills` | outside |
| Connectors | `03_Connectors` `97:29054` | `97:29072` | 248 x 316 | 750, 532 | submenu of plus menu row `Connectors` | outside |
| Tool access | `04_ToolAccess` `97:29084` | `97:29114` | 296 x 189 | 1004, 695 | submenu of connectors row `Tool access` (moved up, bottom at 884) | outside |
| Add connector | `05_AddConnector` `97:29119` | `97:29149` | 236 x 237 | 1004, 533 | submenu of connectors row `Add connector` | outside |
| Design system | `06_DesignSystem` `97:29158` | `97:29176` | 240 x 173 | 750, 564 | submenu of plus menu row `Design system` | outside |
| Plugins | `07_Plugins` `97:29183` | `97:29201` | 248 x 237 | 750, 596 | submenu of plus menu row `Plugins` | outside |
| Model picker | `08_ModelPicker` `97:29210` | `97:29218` | 300 x 262 | 1035, 513 | footer `model` control | version 1 |
| Effort | `09_Effort` `97:29226` | `97:29242` | 280 x 240 | 1341, 644 | submenu of model picker row `Effort` | V1? |
| More models | `10_MoreModels` `97:29248` | `97:29264` | 280 x 240 | 1341, 644 | submenu of model picker row `More models` | V1? |
| Project picker | `11_ProjectPicker` `97:29270` | `97:29278` | 260 x 275 | 532, 513 | footer `project` control (starts at x 512). Top 4 below the footer; left 20 right of the control's left edge | outside |
| Permission mode | `12_PermissionMode` `97:29288` | `97:29296` | 300 x 221 | 908, 513 | footer `mode` control (about x 1070 to 1116). Top 4 below the footer; not edge-aligned | outside |
| Dictation | `13_Dictation` `97:29301` | `97:29309` | 260 x 173 | 924, 467 | dictation button (x 1147 to 1175). Top 6 below the composer box; right edge 9 past the button | outside |
| Model picker, bars | `18_ModelPickerBars` `107:4326` | `107:5260` | 380 x 342 | 686, 517 | footer `model` control, right-aligned to it, top 8 below the footer | V1? (see `model-picker.md`) |
| Connect apps | `20_ConnectAppsMenu` `107:5694` | `107:5917` | 280 x 237 | 936, 516 | footer `apps` chip, right-aligned to it (1216), top 7 below the footer | outside |
| Work in folder | `21_WorkInFolder` `107:5976` | `107:6199` | 280 x 295 | 512, 517 | footer `project` control, left-aligned to it, top 8 below the footer | outside |

The plus menu row `Devices` has a submenu caret, but its submenu is not on this page. The dictation rows `Language` and `Input` have carets with no submenu drawn.

Pictures follow the naming rule, for example `../images/composer-01-plus-menu.png`, `../images/composer-03-connectors.png`, `../images/composer-12-permission-mode.png`, `../images/composer-21-work-in-folder.png`.

## Items, verbatim

Icons are icon names from the design file's icon set, 16 x 16 unless noted. "caret" means the row has the submenu caret.

### Plus menu (`97:29017`)

| # | Icon | Label | Trailing |
| --- | --- | --- | --- |
| 1 | `paperclip` | `Upload or add files` | `⌘U` |
| 2 | `sparkle` | `Skills` | caret |
| 3 | `plugs-connected` | `Connectors` | `6`, caret |
| 4 | `palette` | `Design system` | caret |
| 5 | `puzzle-piece` | `Plugins` | caret |
| 6 | `devices` | `Devices` | caret |
| | separator | | |
| 7 | `globe` | `Research` | |
| 8 | `brain` | `Memory` | check (on) |

### Skills (`97:29045`)

| # | Icon | Label |
| --- | --- | --- |
| | header | `Skills` |
| 1 | `sparkle` | `Brand voice` |
| 2 | `sparkle` | `[third-party service] UI QA` |
| 3 | `magnifying-glass` | `Deep research` |
| 4 | `file-text` | `Docs` |
| 5 | `grid-four` | `Swiss grid design` |
| | separator | |
| 6 | `gear` | `Manage skills` |

### Connectors (`97:29072`)

| # | Icon | Label | Trailing |
| --- | --- | --- | --- |
| 1 | `plus` | `Add connector` | caret |
| 2 | `briefcase` | `Manage connectors` | |
| | separator | | |
| 3 | `folder` | `Files` | switch on |
| 4 | `envelope-simple` | `Mail` | switch on |
| 5 | `calendar` | `Calendar` | switch on |
| 6 | `frame-corners` | `Design files` | switch on |
| 7 | `git-branch` | `Code repos` | switch on |
| 8 | `notepad` | `Notes` | switch off |
| | separator | | |
| 9 | `wrench` | `Tool access` | caret |

### Tool access (`97:29114`)

`MenuItemDetail` rows, each 51 high.

| # | Label | Description | Selected |
| --- | --- | --- | --- |
| | header | `Tool access` | |
| 1 | `Ask every time` | `Approve each tool call before it runs` | no |
| 2 | `Allow reads` | `Read tools run freely; writes ask first` | yes |
| 3 | `Allow all` | `Every enabled tool runs without asking` | no |

### Add connector (`97:29149`)

| # | Icon | Label |
| --- | --- | --- |
| | header | `Suggested` |
| 1 | `table` | `Spreadsheets` |
| 2 | `chats` | `Chat workspace` |
| 3 | `list-checks` | `Task manager` |
| 4 | `bookmark-simple` | `Bookmarks` |
| | separator | |
| 5 | `compass` | `Browse all connectors` |
| 6 | `plus` | `Add custom connector` |

### Design system (`97:29176`)

| # | Icon | Label | Trailing |
| --- | --- | --- | --- |
| | header | `Design system` | |
| 1 | `swatches` | `[design system name]` | check |
| 2 | `palette` | `Isonomic Brand` | |
| 3 | `x` | `None` | |
| | separator | | |
| 4 | `gear` | `Manage design systems` | |

### Plugins (`97:29201`)

| # | Icon | Label | Trailing |
| --- | --- | --- | --- |
| | header | `Plugins` | |
| 1 | `palette` | `Design` | switch on |
| 2 | `code` | `Engineering` | switch on |
| 3 | `cube` | `Product` | switch on |
| 4 | `briefcase` | `Sales` | switch off |
| 5 | `receipt` | `Finance` | switch off |
| | separator | | |
| 6 | `compass` | `Browse plugins` | |

### Model picker, effort, more models

See `model-picker.md`. Model picker rows: header `Model`; three model rows; separator; `Effort` (icon `gauge`, trailing `Extra`, caret); `More models` (icon `stack`, caret). Effort rows: `Low`, `Medium`, `High`, `Extra` (selected). More models: four model rows.

### Project picker (`97:29278`)

| # | Icon | Label | Trailing / state |
| --- | --- | --- | --- |
| | header | `Add to project` | |
| | search field (Focus) | placeholder `Search projects` | |
| 1 | `x` | `No project` | check |
| 2 | `folder-simple` | `Isonomic` | hover fill |
| 3 | `folder-simple` | `Content` | |
| 4 | `folder-simple` | `AIOS` | |
| 5 | `folder-simple` | `Isonomic website rebuild` | |
| | separator | | |
| 6 | `plus` | `New project` | |

### Permission mode (`97:29296`)

`MenuItemDetail` rows. Rows 1 and 3 are 67 high (the description wraps to two lines at 220 wide); row 2 is 51.

| # | Label | Description | State |
| --- | --- | --- | --- |
| | header | `Permission mode` | |
| 1 | `Ask before acting` | `Approve every action that changes something` | |
| 2 | `Auto` | `Acts on its own; asks before risky steps` | selected |
| 3 | `Plan first` | `Proposes a plan and waits for your go-ahead` | hover |

The selected label is what the footer `mode` control shows.

### Dictation (`97:29309`)

| # | Icon | Label | Trailing / state |
| --- | --- | --- | --- |
| | header | `Dictation` | |
| 1 | `microphone` | `Dictate` | `⌘D`, hover fill |
| 2 | `waveform` | `Voice mode` | |
| | separator | | |
| 3 | `translate` | `Language · English` | caret |
| 4 | `microphone` | `Input · [third-party service] microphone` | caret |

### Connect apps (`107:5917`)

| # | Icon | Label | Trailing |
| --- | --- | --- | --- |
| | header | `Connected apps` | |
| 1 | `envelope-simple` | `Mail` | switch on |
| 2 | `calendar` | `Calendar` | switch on |
| 3 | `folder` | `Files` | switch on |
| 4 | `frame-corners` | `Design files` | switch off |
| | separator | | |
| 5 | `plus` | `Connect another app` | |
| 6 | `gear` | `Manage connectors` | |

### Work in folder (`107:6199`)

| # | Icon | Label | Trailing |
| --- | --- | --- | --- |
| | header | `Work in a folder` | |
| 1 | `x` | `No folder (chat only)` | |
| 2 | `folder` | `~/Projects/Isonomic` | check |
| 3 | `folder` | `~/Design` | |
| 4 | `folder` | `~/Documents/Grants` | |
| | separator | | |
| 5 | `folder-simple` | `Choose another folder…` | |
| | header | `Projects` | |
| 6 | `folder-simple` | `Isonomic` | |
| 7 | `folder-simple` | `Content` | |

## Not shown in the design file

- How any menu closes, keyboard movement between rows, and focus rings.
- Disabled rows.
- Scrolling inside a long menu and a maximum menu height.
- Menus opened from the docked composer in a thread (all screens use the home composer).
- Menu placement when there is no room below the trigger.

## Oddities

- Vertical offset from the trigger varies: 6 below the composer box (plus, dictation), 4 below the footer (`97:*` menus), 7 or 8 below the footer (`107:*` menus).
- Horizontal alignment varies: the later `107:*` menus align to an edge of their trigger; the earlier project, permission and model menus do not.
- `Manage connectors` uses icon `briefcase` in the connectors menu and icon `gear` in the connect-apps menu.
- Two different menus hang off the footer `project` control: `Add to project` (screen 11) and `Work in a folder` (screen 21, later node ids). The file does not say which one wins.
- Connector toggles differ between the two connector menus (`Design files` is on in screen 03 and off in screen 20).
- The effort and more-models submenus are drawn past the right screen edge (x 1341 to 1621).
- The `TextField` component note describes a blue focus ring; the drawn focus state is a 2 px `#111827` border.
