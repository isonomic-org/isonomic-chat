# Design differences

Every visible difference between the version 0.2 design (`docs/design/v0.2/`) and what this repository draws, from the audit of 16 Nov 2026. Each row has one verdict:

- **Fixed**: layout or structure matched with existing components (fixed in the audit pull request).
- **Needs UI**: Isonomic UI's component cannot do it; the ask is in the row.
- **Accepted**: Isonomic UI's tokens, type and icons, or outside version 1.

Screenshots: `SCREENSHOT_DIR=… cargo test -p isonomic-chat --features test-support --test screenshots` (19 states, light and dark). The keyboard path and accessible names are in `docs/accessibility.md`. The superseded 0.1 tables are in `docs/design-diffs-v01.md`.

## Shell (`app-shell.md`)

| # | Design 0.2 | Built | Verdict |
| --- | --- | --- | --- |
| A1 | Rail 56 wide | Isonomic UI's icon rail, 48 | Accepted (component width) |
| A2 | Rail padding 12 top, 8 sides; expanded header pt 10, px 12; content px 8 | The sidebar's own 8 / 12 | Needs UI: `Sidebar` header, content and footer padding settable |
| A3 | Rail items centred; toggle, 16, then 28 × 28 nav buttons 4 apart | Same | Fixed |
| A4 | Rail shows no selected item | Same | Fixed |
| A5 | Toggle is a 28 × 28 icon button | Same | Fixed |
| A6 | Wordmark serif semibold 20 / 28 | Theme sans semibold 20 | Accepted (font) |
| A7 | Account row and rail avatar; Projects, Artifacts, Scheduled, Customize | Left out | Accepted (version 1; no account ID yet) |
| A8 | Scrim `gray/900` at 25 % | Theme `overlay` | Accepted (token) |
| A9 | Narrow windows not shown | Below 1,040 the sidebar collapses to its rail | Accepted (collapse rule) |

## Sidebar (`sidebar.md`)

| # | Design 0.2 | Built | Verdict |
| --- | --- | --- | --- |
| B1 | Rows 8 from the edge, 271 wide | 12 from the edge, 263 wide | Needs UI: as A2 (content px 8) |
| B2 | Nav and chat rows 32 high, 1 apart | 28 high, 8 apart | Needs UI: `SidebarMenuItem` height after its defaults; `SidebarMenu` gap settable |
| B3 | Row padding 10 left (chats 6 right), icon to label 10 | Same | Fixed |
| B4 | A selected chat row keeps regular weight | Same | Fixed |
| B5 | Group label 30 high, pt 10, pb 4, with a caret that collapses Pinned | 32 high, px 8, no caret, not collapsible | Needs UI: `SidebarGroup` padding and a collapsible group |
| B6 | Groups 1 apart | 12 apart | Needs UI: `Sidebar` content gap settable |
| B7 | A row's More shows on hover only | Always shown | Accepted (keeps it reachable; rows cannot take focus) |
| B8 | Status dot in a 20 × 20 box, More 10 after it | Same | Fixed |
| B9 | Dots: running, unread, attention | Running only | Accepted (version 1) |
| B10 | On home, New chat stays selected under the palette | Same; Search is never shown selected | Fixed |
| B11 | Empty and failed list not drawn | Footer note "Start a chat and Isonomic will keep track of it here." (the `EmptyState` wording); failed list with its reason and a Try again that hugs its label | Fixed (wording, button) / Accepted (placement: a 360 empty state does not fit 288) |
| B12 | Selected `gray/200`, hover `gray/100`, `gray/50` background | Sidebar tokens | Accepted (tokens) |

## Header (`app-shell.md` session header)

| # | Design 0.2 | Built | Verdict |
| --- | --- | --- | --- |
| C1 | Title button 28 high, 14 / 20 medium | Same | Fixed |
| C2 | Long titles: no rule | The label clips and pushes the caret out | Needs UI: `Button` label truncation that keeps the dropdown caret |
| C3 | Inline rename field 300 × 32 | Same, named "Chat title"; Escape cancels | Fixed |
| C4 | Title menu (page undecided) | Pin or Unpin, Rename, separator, Delete, with icons, 220 wide; the same order as a row's menu | Fixed (order, icons, width) |
| C5 | Device, running pill, side panel, Share; incognito on home | Left out | Accepted (version 1) |
| C6 | Delete asks first (dialog 440 × 240) | Delete is immediate | Accepted until the session management page is decided |

## Search palette (`search-and-menus.md`)

| # | Design 0.2 | Built | Verdict |
| --- | --- | --- | --- |
| D1 | 640 wide, centred | The palette caps itself at 520; now centred | Fixed (centred) / Needs UI: `CommandPalette` width setter |
| D2 | Placeholder "Search or ask…" | "Type a command" | Needs UI: `CommandPalette::placeholder` |
| D3 | Esc key chip in the input; input 53 high, 16 / 24 | No chip; about 48 high, 14 px | Needs UI: input suffix slot and size |
| D4 | Rows 36 high, 16 px icon, meta such as "2h" on the right | 32 high, no icon, keys only | Needs UI: `PaletteCommand::icon`, `::meta`, row height 36 |
| D5 | Chats under "Recents", then "Actions" | Same | Fixed |
| D6 | New chat shows its shortcut | Same | Fixed |
| D7 | Typed query: results grouped "Chats" | One "Best matches" heading | Needs UI: keep groups while filtering, or set the heading |
| D8 | Footer hints as key chips plus label | Plain text hints | Needs UI: settable footer hints |
| D9 | Nothing selected until the person types or presses an arrow | First row always selected | Needs UI: no cursor on open |
| D10 | No-results state not drawn | "No commands match." | Needs UI: `CommandPalette::empty_text` |
| D11 | Tabs, filters, compose mode, groups by status | Left out | Accepted (version 1 searches titles) |
| D12 | Radius 14, `shadow/2xl` | Theme radius and shadow | Accepted (tokens) |

## Menus (`sidebar.md` pinned item menu, `search-and-menus.md` menu)

| # | Design 0.2 | Built | Verdict |
| --- | --- | --- | --- |
| E1 | 220 wide | Same | Fixed |
| E2 | Open, Pin or Unpin, Rename, Delete, each with a 16 icon | Same | Fixed |
| E3 | Delete in the destructive tone | Default tone | Needs UI: `PopupMenuItem::destructive()` |
| E4 | Open in new tab, Add to project | Left out | Accepted (version 1) |

## Home (`home.md`)

| # | Design 0.2 | Built | Verdict |
| --- | --- | --- | --- |
| H1 | Greeting: brand mark, display serif | Sparkles icon at 32, theme sans 3xl | Accepted (no mark or serif in Isonomic UI) |
| H2 | Greeting names the person | No name on a live backend | Accepted (the window does not hand over the name) |
| H3 | Column: greeting, 20, composer, 20, a 28 footer; bottom padding 140 | Same; the footer keeps its 28 when empty | Fixed |
| H4 | 16 at each side on narrow windows | Same | Fixed |
| H5 | Footer padding 8 at the sides | Same | Fixed |
| H6 | Footer always names a model | No picker while the catalog is empty; "Models couldn't load." with Try again when listing failed | Fixed |
| H7 | Project, effort, mode and Apps in the footer; usage banner | Left out | Accepted (version 1) |

## Composer (`composer.md`)

| # | Design 0.2 | Built | Verdict |
| --- | --- | --- | --- |
| K1 | 720 × 96; padding 14 / 12 / 10 / 16; 14 above a 32 toolbar | 720 × about 85 | Needs UI: `PromptComposer` comfortable size |
| K2 | Send and stop: round 32, filled `gray/900` | Small rounded squares; stop is not filled | Needs UI: round 32 trailing button, stop filled like send |
| K3 | Empty shows voice; plus; context ring | No voice, no plus (attaching off), no ring | Accepted (version 1) |
| K4 | Focus look not drawn | Ring colour border | Accepted (token) |
| K5 | Placeholders and disclaimer | Match exactly | — |

## Model picker (`model-picker.md`, design B)

| # | Design 0.2 | Built | Verdict |
| --- | --- | --- | --- |
| M1 | Menu 380 wide | Same | Fixed |
| M2 | Menu header "Model" | None (it draws only with meter columns) | Needs UI: `ModelPicker::header` without columns |
| M3 | Trigger is the model's name only | Letter tile, name, chevron | Needs UI: `tile(false)`, `caret(false)` on trigger and rows |
| M4 | Trailing check slot; chosen row filled | Check after the name; the fill follows the active row | Needs UI: trailing check slot, chosen-row fill apart from the active row |
| M5 | Rows 8 / 10 padding, 12 gap | 44 high, 8 padding, 10 gap | Needs UI: row padding and gap |
| M6 | No footer | Key-hint footer always shown | Needs UI: `footer(false)` |
| M7 | Rows are named for assistive technology | Rows are plain, unnamed | Needs UI: list box roles and names on rows |
| M8 | Taglines, ratings, settings | Notes only: Recommended, Not installed, Downloading, Unavailable | Accepted (version 1: honest data only) |
| M9 | Choosing a model that is not installed installs it | Only Not installed asks the window; Downloading and Unavailable do nothing | Fixed |

## Empty and error states (`first-run-and-system-states.md`)

| # | Design 0.2 | Built | Verdict |
| --- | --- | --- | --- |
| N1 | `EmptyState` 360 wide, padding 28 / 24, gap 8; tile 44, icon 22; text wraps at 300; primary button with an icon | Same, with the download icon | Fixed |
| N2 | Wording "Nothing here yet" | "Install a model to start" / "Install Medium" | Accepted (content) |
| N3 | After the button: not drawn | The button says "Downloading" and stands still | Fixed (the catalog does not refresh until the kit's host can watch models) |
| N4 | Errors as a toast with Retry | In place: the sidebar foot, home's footer and the status bar, each with Try again; a failed save is a quiet line | Accepted (version 1 keeps errors where they happen) |
| N5 | Page not found | Not built; a deleted chat returns home | Accepted (no route reaches it) |

## Thread (`thread.md`, `thread-states.md`, `thread-states-part-2.md`, `message-content.md`)

| # | Design 0.2 | Built | Verdict |
| --- | --- | --- | --- |
| T1 | Column min(720, width − 64); 32 above and below; 24 between turns | Same (the scroller's own insets removed) | Fixed |
| T2 | Bubble 16 / 24, padding 16 / 10, wraps at 560 | Same | Fixed |
| T3 | Hover row: 28 buttons, gap 2, hidden but keeping its space | Same; it also shows while one of its buttons has focus | Fixed |
| T4 | Hover row holds time, retry, edit, copy | Copy and Edit | Accepted (try again sits on the reply; turns carry no time) |
| T5 | One heading level, 18 / 28 | Every level 18 | Fixed |
| T6 | Lists: text 28 in, marker 4 in, `gray/500` markers | About 22 and 1 | Needs UI: `TextViewStyle` list indent, marker gap and marker colour |
| T7 | Code block: no header, padding 14 / 12, radius 10 | The text view's fence with a copy button | Accepted (fences stay the text view's; isonomic-ui #375) |
| T8 | Reply actions: copy, read aloud, votes, retry, time; 28 buttons | Copy and Try again; 20 buttons | Accepted (version 1) / Needs UI: `MessageActionBar` button size |
| T9 | Pager before the actions, 8 apart | After Try again | Needs UI: `MessageActionBar` pager position |
| T10 | Retry is called as the app says ("Try again") | The bar says "Regenerate", "Copy", "Previous version", "Next version" | Needs UI: `MessageActionBar` label overrides |
| T11 | Thinking row: orb, label, seconds, expands | Isonomic UI's thinking indicator; no seconds; not expandable | Accepted (no expandable row in Isonomic UI) |
| T12 | Stream cursor block | The text view's fade | Accepted |
| T13 | Stopped note in the reply | The last turn says it in the status bar with Continue; earlier turns keep the note | Accepted (the bar is the one stop and error bar) |
| T14 | Run status bar 44 high; Running, Stopped, Error, Done; several buttons | Isonomic UI's run status bar at `b4c9670`: one action | Needs UI: alignment already with the UI worker |
| T15 | Edit box: input 16 / 24 at the padding edge; 34 high buttons; small shadow | Same (buttons 32: token) | Fixed |
| T16 | Composer dock 720, disclaimer 8 below, 12 bottom padding | Same | — |
