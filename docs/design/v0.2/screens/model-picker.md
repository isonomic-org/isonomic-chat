# Model picker

The model picker opens from the model control in `ComposerFooter` (`97:19517`, see `composer.md`). The design file shows two designs for it. Version 1 needs a model picker; which of the two designs to build is an open choice (see "Which design is later").

Pictures: `../images/composer-08-model-picker.png`, `../images/composer-09-effort.png`, `../images/composer-10-more-models.png`, `../images/composer-18-model-picker-bars.png`.

All coordinates below are in the 1440 x 900 screen frame unless they say otherwise. The content area starts at x 288, y 52.

## Which design is later

| Design | Screen frame | Menu frame | Row component |
| --- | --- | --- | --- |
| A, "two-line rows" | `ComposerMenu/08_ModelPicker` `97:29210` | `97:29218` | `MenuItemDetail` `97:11502` |
| B, "bars" | `ComposerMenu/18_ModelPickerBars` `107:4326` | `107:5260` | `ModelOption` `101:5309` |

Design B looks like the later one. This is an inference, not something the file states: its screen and menu have higher node ids (`107:*`, added after the `97:*` frames), and its row component `ModelOption` (`101:*`) was added after `MenuItemDetail` (`97:*`). Design A is still the only one with the effort and more-models submenus (screens 09 and 10). Design B has no submenu rows; it shows a thinking toggle and a thinking-budget bar instead. The file does not say that B replaces A.

## Trigger (in the composer footer)

Node `97:19523`, layer name `model`, inside `ComposerFooter`.

| Property | Value |
| --- | --- |
| Layout | row, gap 6, items centred, padding 4 top/bottom, 8 left/right, radius 6, no fill, no border |
| Height | 28 (20 line + 8 padding) |
| Text 1 (model) | `[model name]`, 14 / 20, weight 500, interface sans, `#374151` (gray/700) |
| Text 2 (effort) | `Extra`, 14 / 20, weight 400, interface sans, `#9CA3AF` (gray/400) |
| Icon | none (no caret) |
| Position on home | right group of the footer; about x 936 to 1066, y 481 to 509 (measured from the picture, text widths are not in the reference code) |
| States | only the resting state is drawn. Hover, pressed and open states are not shown in the design file |

## Shared menu surface

Both designs and both submenus use the same surface.

| Property | Value |
| --- | --- |
| Fill | `#FFFFFF` (white) |
| Border | 1 px `#E5E7EB` (gray/200) |
| Radius | 10 |
| Padding | 4 on every side (rows start 5 px in, counting the border) |
| Layout | column, no gap, rows stretch to full inner width |
| Shadow | style name "ring-1/ring-black ring-opacity-5 shadow-lg": 0 0 0 1 px `#0000000D`; 0 4 6 -2 `#0000000D`; 0 10 15 -3 `#0000001A` |

`MenuHeader` (`97:19543`): padding 6 top, 4 bottom, 8 left/right; height 26; text 12 / 16, weight 500, interface sans, `#6B7280` (gray/500).

`MenuSeparator` (`97:19541`): 9 high (4 padding above and below a 1 px line), line `#E5E7EB` (gray/200), full inner width.

## Design A: menu `97:29218`

| Property | Value |
| --- | --- |
| Size | 300 x 262 |
| Position | x 1035, y 513 |
| Relative to trigger | top is 4 px below the footer bottom (509). Left edge is about 99 px right of the trigger's left edge; the menu runs 111 px past the footer's right edge (1224). It is not aligned to either trigger edge |

Rows in order (inner width 290):

| # | Y in menu | Height | Element | Content |
| --- | --- | --- | --- | --- |
| 1 | 5 | 26 | `MenuHeader` | `Model` |
| 2 | 31 | 51 | `MenuItemDetail` selected=True, Default | `[model name]` / `Fast, capable everyday model` |
| 3 | 82 | 51 | `MenuItemDetail` selected=False, Default | `[model name]` / `Most capable for complex work` |
| 4 | 133 | 51 | `MenuItemDetail` selected=False, Default | `[model name]` / `Strong reasoning and code` |
| 5 | 184 | 9 | `MenuSeparator` | |
| 6 | 193 | 32 | `MenuItem` `97:29224` | icon `gauge` 16, `Effort`, trailing value `Extra`, submenu caret |
| 7 | 225 | 32 | `MenuItem` `97:29225` | icon `stack` 16, `More models`, submenu caret |

### `MenuItemDetail` (`97:11502`)

Variants: selected True / False x state Default / Hover (`97:11503` selected default, `97:11508` unselected default, `97:11513` unselected hover, `97:11518` selected hover).

| Property | Value |
| --- | --- |
| Layout | row, gap 10, items centred, padding 7 top/bottom, 10 left/right, radius 6 |
| Height | 51 with a one-line description (20 + 1 + 16 + 14); 67 with a two-line description |
| Text block | column, gap 1, fills the row |
| Label | 14 / 20, weight 500, interface sans, `#111827` (gray/900), one line |
| Description | 12 / 16, weight 400, interface sans, `#6B7280` (gray/500), fixed width 220 in the component, wraps |
| Check | layer `check`, 16 x 16, icon `check`, at the right. The slot is present in all four variants so text does not shift; it is drawn only when selected=True |
| Default fill | none |
| Hover fill | `#F3F4F6` (gray/100), both selected and unselected |
| Selected | no fill change; only the check shows |

### Extra rows (`MenuItem` `97:11392`)

Row: height 32, row layout, gap 10, padding 0 / 8, radius 6. Icon 16 x 16 at the left. Label 14 / 20, weight 400, `#1F2937` (gray/800), fills. Trailing value text 12 / 16, weight 400, `#9CA3AF` (gray/400). Submenu caret: layer `submenu`, 14 x 14, icon `caret-right`. Full `MenuItem` detail is in `search-and-menus.md`.

## Effort submenu (`ComposerMenu/09_Effort` `97:29226`, menu `97:29242`)

Opens from row 6 of design A. The parent menu stays open at the same place and the `Effort` row keeps the hover fill `#F3F4F6` while the submenu is open.

| Property | Value |
| --- | --- |
| Size | 280 x 240 |
| Position | x 1341, y 644 (6 px right of the parent's right edge at 1335) |
| Vertical relation | the `Effort` row sits at y 706 to 738; the submenu spans 644 to 884, so its bottom is 16 px above the window bottom rather than top-aligned with the row |

| # | Height | Element | Label / description |
| --- | --- | --- | --- |
| 1 | 26 | `MenuHeader` | `Effort` |
| 2 | 51 | `MenuItemDetail` unselected | `Low` / `Quick answers, minimal thinking` |
| 3 | 51 | `MenuItemDetail` unselected | `Medium` / `Balanced speed and depth` |
| 4 | 51 | `MenuItemDetail` unselected | `High` / `Thinks longer on hard problems` |
| 5 | 51 | `MenuItemDetail` selected | `Extra` / `Maximum thinking for the hardest work` |

The selected value is the one echoed in the footer trigger and in the `Effort` row.

## More-models submenu (`ComposerMenu/10_MoreModels` `97:29248`, menu `97:29264`)

Opens from row 7 of design A. Same size and position as the effort submenu: 280 x 240 at x 1341, y 644 (the `More models` row is at y 738 to 770).

| # | Height | Element | Label / description |
| --- | --- | --- | --- |
| 1 | 26 | `MenuHeader` | `More models` |
| 2 | 51 | `MenuItemDetail` unselected | `[model name]` / `Open multimodal model` |
| 3 | 51 | `MenuItemDetail` unselected | `[model name]` / `Multilingual, long context` |
| 4 | 51 | `MenuItemDetail` unselected | `[model name]` / `Lightweight and efficient` |
| 5 | 51 | `MenuItemDetail` unselected, Hover | `[model name]` / `Tuned for code and agents` |

Row 5 is the only place the hover state is drawn on a model row.

## Design B: menu `107:5260`

| Property | Value |
| --- | --- |
| Size | 380 x 342 |
| Position | x 686, y 517 |
| Relative to trigger | top is 8 px below the footer bottom (509). Right edge (1066) lines up with the trigger's right edge, so the menu hangs to the left of the trigger |

Rows in order (inner width 370):

| # | Y in menu | Height | Element | Content |
| --- | --- | --- | --- | --- |
| 1 | 5 | 26 | `MenuHeader` | `Model` |
| 2 | 31 | 70 | `ModelOption` selected=True `107:5263` | `[model name]` / `Everyday · fast` |
| 3 | 101 | 70 | `ModelOption` selected=False `107:5292` | `[model name]` / `Most capable · slower` |
| 4 | 171 | 70 | `ModelOption` selected=False `107:5348` | `[model name]` / `Reasoning and code` |
| 5 | 241 | 1 | `divider` `107:5403` | 1 px line `#E5E7EB` (gray/200), full inner width, no padding around it |
| 6 | 242 | 67 | `SettingRow` control=Toggle `107:5404` | `Extended thinking` / `Think longer before answering` |
| 7 | 309 | 28 | thinking-budget row `107:5411` | `Thinking budget`, bar, `16k` |

### `ModelOption` (`101:5309`)

Variants: selected True (`101:5307`) / False (`101:5308`). No hover variant exists.

| Property | Value |
| --- | --- |
| Layout | row, gap 12, items centred, padding 8 top/bottom, 10 left/right, radius 8 |
| Height | 70 (three 16 px bar rows + two 3 px gaps + 16 padding) |
| Selected fill | `#F3F4F6` (gray/100) |
| Unselected fill | none |
| Text block | column, gap 2, fills the row |
| Name | 14 / 20, weight 500, interface sans, `#111827` (gray/900) |
| Description | 12 / 16, weight 400, interface sans, `#6B7280` (gray/500), one line |
| Bars block | layer `bars`, column, gap 3, fixed to content width (114) |
| Bar row | row, gap 6, items centred: a label 40 wide, then five pips |
| Bar labels | `Speed`, `Quality`, `Cost`; 12 / 16, weight 400, `#9CA3AF` (gray/400) |
| Pip | 10 x 4, radius 2; filled `#1F2937` (gray/800), empty `#E5E7EB` (gray/200) |
| Check | layer `check`, 16 x 16, icon `check`, at the right; slot kept in both variants, drawn only when selected |

Pip counts: Speed 4 of 5, Quality 3 of 5, Cost 1 of 5. All three rows on the screen show these same counts (the component default was not overridden), so the bars are placeholders; per-model values are not shown in the design file. How the count is passed in is a build decision.

### `SettingRow` in the menu (`97:11770`, control=Toggle)

| Property | Value |
| --- | --- |
| Layout | row, gap 24, items centred, padding 14 top/bottom, 10 left/right |
| Border | 1 px bottom, `#F3F4F6` (gray/100) |
| Label | `Extended thinking`, 14 / 20, weight 500, `#111827` (gray/900) |
| Description | `Think longer before answering`, 12 / 16, weight 400, `#6B7280` (gray/500) |
| Control | layer `control`, switch 44 x 24 at the right, drawn on: dark track (`#111827` in the picture) with a white knob at the right. The off look is not drawn in this menu |

### Thinking-budget row (`107:5411`)

| Property | Value |
| --- | --- |
| Layout | row, gap 8, items centred, padding 4 top, 8 bottom, 10 left/right; height 28 |
| Label | `Thinking budget`, 12 / 16, weight 400, `#6B7280` (gray/500) |
| Track | `107:5413`, fills the remaining width (226 here), 4 high, radius 2, `#E5E7EB` (gray/200), clips its content |
| Fill | `107:5414`, 120 x 4, radius 2, `#111827` (gray/900), from the left (about 53 percent) |
| Value | `16k`, 12 / 16, weight 500, `#374151` (gray/700) |
| Handle | none drawn. Whether the bar is draggable is not shown in the design file |

## Not shown in the design file

- Open, hover and pressed states of the footer trigger.
- A hover state for `ModelOption` (design B).
- Keyboard focus rings on any row.
- What the picker does in a narrow window, or when there is no room below the footer.
- How the thinking-budget row behaves when `Extended thinking` is off.
- A loading, empty or error state for the model list.
- The model picker opened from the docked composer in a thread: the docked composer has no footer (see `composer.md`), so no trigger is drawn there.

## Oddities

- Design A and B place the menu differently (4 px versus 8 px below the footer; A not edge-aligned, B right-aligned to the trigger).
- Both submenus are drawn at x 1341 to 1621, past the 1440 px screen edge, so they are cut off in the pictures.
- The `SettingRow` description keeps the component's fixed 480 px text width inside a 370 px menu; the text fits on one line so nothing visibly overflows.
- Descriptions differ between designs for the same three models (for example `Fast, capable everyday model` in A, `Everyday · fast` in B).
