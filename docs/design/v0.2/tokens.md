# Version 0.2 tokens

Colours, type styles, radii, spacing and shadows used by the version 0.2 pages of the design file.

How this was read: the design file's variables were listed for five nodes (thread screen `97:41023`, model picker screen `97:29210`, component sections `100:2449` and `101:4261`, and the older-shell frame `97:22357`). Radii and spacing are not variables in the file; they are raw numbers on the layers and were read per component (see `screens/`).

Rule 6 of this repository still holds: the build uses Isonomic UI theme tokens, not these raw values. This table says what the design draws so each difference can be recorded in `docs/design-diffs.md`.

## Light and dark

Version 0.2 is light only. No dark variables, no dark frames and no mode switch exist in the file. Dark values must come from the Isonomic UI theme.

## Colour variables

Names are the design file's variable names.

### Neutrals

| Variable | Hex | Where 0.2 uses it |
| --- | --- | --- |
| `white` | `#FFFFFF` | main panel, composer, menus, dialogs, cards |
| `gray/50` | `#F9FAFB` | sidebar fill, code block fill, quiet panels |
| `gray/100` | `#F3F4F6` | hover fill on rows and menu items, user message bubble, chips |
| `gray/200` | `#E5E7EB` | borders (sidebar edge, composer, cards, menus), selected row fill, separators |
| `gray/300` | `#D1D5DB` | input and outline-button borders |
| `gray/400` | `#9CA3AF` | placeholder text, disclaimer, timestamps, Running status dot |
| `gray/500` | `#6B7280` | secondary text, group labels, icons at rest in quiet places |
| `gray/600` | `#4B5563` | icon buttons, tertiary labels |
| `gray/700` | `#374151` | sidebar row icons, body text in menus |
| `gray/800` | `#1F2937` | row labels, code text |
| `gray/900` | `#111827` | primary text, headings, the filled send button, Unread dot, focus border, scrim base (25% opacity) |

### Status and accent

| Variable | Hex | Where 0.2 uses it |
| --- | --- | --- |
| `orange/500` | `#F97316` | Attention status dot, warning accents |
| `orange/600`, `orange/50` | `#EA580C`, `#FFF7ED` | warning text and fill (usage and context warnings) |
| `green/500` | `#10B981` | "connected" dot in the session header, success marks |
| `green/600`, `green/700`, `green/50`, `green/100` | `#059669`, `#047857`, `#ECFDF5`, `#D1FAE5` | done states, allowed states, inserted text |
| `red/500`, `red/600`, `red/700` | `#EF4444`, `#DC2626`, `#B91C1C` | error text, destructive menu items, failed states |
| `red/50`, `red/100` | `#FEF2F2`, `#FEE2E2` | error fills, deleted text |
| `yellow/400`, `yellow/50`, `yellow/800` | `#FBBF24`, `#FFFBEB`, `#92400E` | caution fills in approval cards |
| `purple/500`, `teal/500` | `#8B5CF6`, `#14B8A6` | segments in the context window card only |

There is no blue in the version 0.2 shell. `blue/600` (`#0161FE`) and `blue/500` (`#3280FF`) appear only in the frames of the "Start a Basic Chat 0.2" page, which still use the version 0.1 components. Two component notes in the file mention a blue unread dot and a blue focus ring, but both are drawn in `gray/900`.

One more raw value appears only on that older page: an avatar fill `#DC8E76`.

## Type

Three faces. The repository bundles the interface sans and the mono face; the display serif is not bundled (see D1 in `docs/design-diffs.md`).

| Face | Used for |
| --- | --- |
| Interface sans | everything not listed below, including message bodies |
| Display serif | the sidebar wordmark, the home greeting, large page titles ("Incognito chat", "Page not found"), the math block |
| Mono face | code blocks, tool names and arguments |

### Text style variables (interface sans)

Format: size / line height, weight.

| Variable | Value | Typical use |
| --- | --- | --- |
| `text-xs/leading-none/font-normal` | 12 / 12, 400 | tiny counters |
| `text-xs/leading-4/font-normal` | 12 / 16, 400 | disclaimer, timestamps, captions |
| `text-xs/leading-4/font-medium` | 12 / 16, 500 | group labels, menu headers, chips |
| `text-xs/leading-4/font-semibold` | 12 / 16, 600 | small badges (agent components) |
| `text-sm/leading-4/font-medium` | 14 / 16, 500 | button labels |
| `text-sm/leading-5/font-normal` | 14 / 20, 400 | sidebar rows, menu items, secondary copy |
| `text-sm/leading-5/font-medium` | 14 / 20, 500 | selected nav item, header title, card titles |
| `text-sm/leading-5/font-semibold` | 14 / 20, 600 | emphasised labels (agent components) |
| `text-base/leading-6/font-normal` | 16 / 24, 400 | message bodies, composer text and placeholder |
| `text-base/leading-6/font-semibold` | 16 / 24, 600 | card headings |
| `text-lg/leading-7/font-semibold` | 18 / 28, 600 | headings inside a reply, dialog titles |

### Styles that are not variables

| Style | Value | Use |
| --- | --- | --- |
| Display serif 34 / 42, 400 | raw | home greeting |
| Display serif 28 / 36, 400 | raw | large page titles |
| Display serif 20 / 28, 600 | raw | "Isonomic" wordmark in the sidebar and onboarding; math block |
| Mono 13 / 20, 400 | raw | code |

Letter spacing is 0 everywhere.

## Radii

Raw numbers, most used first.

| Radius | Use |
| --- | --- |
| 6 | menu items, small buttons, inputs, icon buttons, chips |
| 8 | sidebar rows, standard buttons, small cards, tooltips |
| 10 | menus and popovers, code blocks |
| 12 | cards, dialogs, larger tiles |
| 14 | the search palette |
| 16 | the composer |
| 18 | the user message bubble |
| 999 (full) | round send, stop and voice buttons, status dots, pills |
| 2 to 4 | progress bars, checkboxes, the stream cursor |

## Spacing

No spacing variables. The values that repeat:

| Value | Use |
| --- | --- |
| 1 | gap between sidebar rows; all borders are 1 px |
| 4 | menu padding; icon to label in tight chips |
| 6 | submenu offset from its parent |
| 8 | gap between the docked composer and its disclaimer; gaps between buttons |
| 12 | gap between blocks inside a reply; card inner padding |
| 16 | panel and card padding |
| 20 | gap between home greeting, composer and footer |
| 24 | gap between turns in a thread; dialog side padding |
| 32 | top padding of the transcript; row height in sidebar and menus |
| 52 | header height |
| 288 / 56 | sidebar expanded / collapsed |
| 720 | reading column (transcript, composer, home column) |

## Shadows

| Variable | Value | Use |
| --- | --- | --- |
| `shadow/sm` | 0 1 2 0 `#0000000D` | composer, outline buttons, inputs |
| `/shadow/md` | 0 2 4 -1 `#0000000F`, plus 0 4 6 -1 `#0000001A` | toasts, floating cards |
| `/shadow/lg` | 0 4 6 -2 `#0000000D`, plus 0 10 15 -3 `#0000001A` | dialogs, search palette |
| `ring-1/ring-black ring-opacity-5 shadow-lg` | 0 0 0 1 `#0000000D`, plus the two `/shadow/lg` layers | menus and popovers |

Scrim behind dialogs and the search palette: `gray/900` at 25% opacity, full window. Menus have no scrim.

## Icons

All icons come from the design file's one icon set, regular weight, drawn as vectors. Layer names follow `icon_<name>` (for example `icon_magnifying_glass`, `icon_caret_right`, `icon_check_circle`). Sizes in use: 12, 14, 16 (most common), 18, 20, 22 and 28. Colours were read from the exported vectors, not from variables, so treat icon colours as approximate and take them from the row or button the icon sits in. The repository's nearest-match table is in `docs/design-diffs.md` ("Icon matches"); version 0.2 adds many icon names that table does not cover (listed per screen in `screens/`).
