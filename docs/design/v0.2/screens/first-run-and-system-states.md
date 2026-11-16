# First run and system states

Onboarding steps, the page-not-found screen, the usage warning banner, and the `EmptyState` and `Toast` components. Pixel values are design pixels at 1x. Token names in brackets are the variable names the design file attaches to the colour.

| Item | Version 1 |
| --- | --- |
| Onboarding 05 "About you" | V1? (the only first-run screen that needs no connector or agent feature; the build decides) |
| Onboarding 06 "Connect apps", 07 "How Isonomic acts" | outside version 1 |
| Page not found | version 1 (the "Go to Projects" button is outside version 1) |
| `UsageBanner` | V1? (needs a usage limit to exist) |
| `EmptyState` | version 1 |
| `Toast` | version 1 |

Pictures: `../images/home-onboarding-05-about-you.png`, `../images/home-onboarding-06-connect-apps.png`, `../images/home-onboarding-07-autonomy.png`, `../images/system-states-01-page-not-found.png`, `../images/system-states-02-weekly-usage-warning.png`, `../images/system-states-03-usage-warning-home.png`.

## Shared buttons used on these screens

These are instances of the shared `button` component (documented with the other basics); values repeated here because every screen below uses them.

| Button | Fill | Border | Padding | Radius | Text | Shadow |
| --- | --- | --- | --- | --- | --- | --- |
| Primary, 34 high | `#111827` (gray/900) | none | 9 top/bottom, 13 left/right | 6 | 14 / 16, weight 500, `#FFFFFF` | shadow/sm: 0 1 2 `#0000000D` |
| Primary with icon | same | none | 9 top/bottom, 11 left, 13 right, gap 8 | 6 | same; icon 16 at the left | same |
| Secondary, 34 high | `#FFFFFF` | 1 px `#D1D5DB` (gray/300) | 9 top/bottom, 13 left/right | 6 | 14 / 16, weight 500, `#374151` (gray/700) | same |
| Small secondary, 30 high | `#FFFFFF` | 1 px `#D1D5DB` | 7 top/bottom, 11 left/right | 4 | 12 / 16, weight 500, `#374151` | same |

All button text is the interface sans. Hover, pressed, focus and disabled states are not shown on these screens.

## Onboarding (steps 05 to 07)

Frames: `Onboarding/05_AboutYou` `106:2649` (1440 x 396), `Onboarding/06_ConnectApps` `106:2691` (1440 x 510), `Onboarding/07_Autonomy` `106:2782` (1440 x 320). The frames hug their content, so the vertical position inside a real window is not shown in the design file. There is no sidebar and no header.

### Page

| Property | Value |
| --- | --- |
| Fill | `#F9FAFB` (gray/50) |
| Layout | column, gap 20, centred horizontally and vertically |
| Order | wordmark, stepper, card |

Wordmark (`106:2650`): text `Isonomic`, display serif, 20 / 28, weight 600, `#111827` (gray/900); 91 x 28.

### Stepper (`106:2651`, 491 x 22)

Row, gap 8, items centred. Four steps. Each step is a row with gap 6: circle, label, then a connector line (the last step has no line).

| Part | Value |
| --- | --- |
| Circle | 22 x 22, radius 11, content centred |
| Label | 12 / 16, weight 500, interface sans |
| Connector | 24 x 1, `#D1D5DB` (gray/300) |
| Labels, in order | `About you`, `Connect apps`, `How Isonomic acts`, `Done` |

| Step state | Circle | Circle content | Label colour |
| --- | --- | --- | --- |
| Current | fill `#FFFFFF`, 1 px border `#111827` (gray/900) | step number, 12 / 16, weight 600, `#111827` | `#111827` |
| Upcoming | fill `#F3F4F6` (gray/100), no border | step number, 12 / 16, weight 600, `#9CA3AF` (gray/400) | `#9CA3AF` |
| Done | fill `#111827`, no border | icon `check`, 12 x 12, white | `#111827` |

Step 05 shows 1 current; step 06 shows 1 done, 2 current; step 07 shows 1 and 2 done, 3 current. A screen for step 4 `Done` is not in the design file.

### Card (`onboarding_card`: `106:2671`, `106:2714`, `106:2806`)

| Property | Value |
| --- | --- |
| Width | 600 (height hugs: 306, 420, 230) |
| Fill / border / radius | `#FFFFFF`, 1 px `#E5E7EB` (gray/200), 16 |
| Padding / gap | 28 on all sides; column, gap 14, left-aligned |
| Shadow | style "/shadow/md": 0 2 4 -1 `#0000000F`; 0 4 6 -1 `#0000001A` |
| Title | 20 / 28, weight 600, interface sans, `#111827`, width 540 |
| Button row | last child, full width, row, gap 8: optional `Back` (secondary) at the left, a spacer, `Continue` (primary) at the right |

### Step 05, "About you"

| # | Element | Detail |
| --- | --- | --- |
| 1 | Title | `Welcome. What should Isonomic call you?` |
| 2 | Field label | `Name`; 14 / 20, weight 500, `#374151` (gray/700) |
| 3 | `TextField` state Default (`106:2674`) | 542 x 38; fill `#FFFFFF`; 1 px border `#D1D5DB` (gray/300); radius 8; padding 0 / 12; text `Omar`, 14 / 20, weight 400, `#9CA3AF` (gray/400, placeholder colour); no icon |
| 4 | Field label | `What do you work on?` (same style as 2) |
| 5 | `TextField` state Default (`106:2680`) | same; text `Founder · product and design` |
| 6 | Button row | spacer, then `Continue` (84 x 34). No `Back` on this step |

Both field texts are drawn in the placeholder colour, so they read as examples, not typed values. Focused, filled and error states of the fields are not shown here. Validation and whether `Continue` is disabled while empty are not shown in the design file.

### Step 06, "Connect apps" (outside version 1)

| # | Element | Detail |
| --- | --- | --- |
| 1 | Title | `Connect the apps you work in` |
| 2 | Body | `Isonomic only reads what a chat needs, and asks before changing anything.`; 14 / 20, weight 400, `#6B7280` (gray/500), width 540 |
| 3 | Card grid (`106:2717`) | wraps, gap 10, two columns of 266; four `DirectoryCard` (`97:19657`), each 266 x 114 |
| 4 | Button row | `Back` (58 x 34), spacer, `Continue` (84 x 34) |

`DirectoryCard`: fill `#FFFFFF`, 1 px border `#E5E7EB`, radius 12, padding 14, column gap 8. Top row: logo tile 32 x 32, fill `#F3F4F6`, radius 8, icon 16; spacer; small secondary button `Add` with icon `plus` 16 (padding 7 top/bottom, 9 left, 11 right, gap 8). Then name 14 / 20, weight 600, `#111827`; description 12 / 16, weight 400, `#6B7280`, width 188, one line with ellipsis.

| Card | Icon | Name | Description |
| --- | --- | --- | --- |
| `106:2718` | `envelope-simple` | `Mail` | `Draft and triage` |
| `106:2735` | `calendar` | `Calendar` | `Plan and schedule` |
| `106:2748` | `folder` | `Files` | `Read your docs` |
| `106:2761` | `git-branch` | `Code repos` | `Review and ship` |

### Step 07, "How Isonomic acts" (outside version 1)

| # | Element | Detail |
| --- | --- | --- |
| 1 | Title | `How much should Isonomic do on its own?` |
| 2 | `AutonomyDial` value=Edits (`106:2808`, set `100:4516`) | 257 x 28 |
| 3 | Body | `Edits: Isonomic can write and edit files and docs, and asks before sending, paying or deleting anything. You can change this per chat.`; 14 / 20, weight 400, `#4B5563` (gray/600), width 540, two lines |
| 4 | Button row | `Back`, spacer, `Continue` |

`AutonomyDial`: track fill `#F3F4F6`, radius 8, padding 2, row gap 2. Each option: row gap 4, padding 4 top/bottom, 10 left/right, radius 6, icon 14, label 12 / 16, weight 500. Selected option: fill `#FFFFFF`, shadow/sm, label `#111827`. Other options: no fill, label `#6B7280`. Options: icon `hand` + `Ask first`; icon `pencil-simple` + `Edits`; icon `lightning` + `Full access`.

## Page not found (`SystemState/01_PageNotFound` `97:126644`)

The app sidebar stays (with no row selected). The main panel has no session header; `content` (`97:126647`) fills all 1152 x 900.

| Property | Value |
| --- | --- |
| Layout | column, gap 12, centred horizontally and vertically, bottom padding 120 (so the block sits 60 px above true centre) |
| Block position | top at y 299, bottom at y 481 in the panel |

| # | Element | Detail |
| --- | --- | --- |
| 1 | Icon tile (`97:126648`) | 56 x 56, fill `#F3F4F6` (gray/100), radius 14, icon `compass` 28 x 28 centred |
| 2 | Title (`97:126650`) | `Page not found`; display serif, 28 / 36, weight 400, `#111827` |
| 3 | Body (`97:126651`) | `This chat or artifact doesn’t exist, or you don’t have access to it.`; interface sans, 14 / 20, weight 400, `#6B7280`, one line |
| 4 | Buttons (`97:126652`) | row, gap 8: primary with icon `plus` 16, `Start a new chat` (153 x 34); secondary `Go to Projects` (116 x 34) |

This is the only full-page error screen in the file. Errors inside a thread (failed reply, stopped reply) are in `thread-states.md`.

## Usage warning banner (`UsageBanner` `97:19531`)

Component note in the file: "Weekly-limit warning that docks above the composer."

| Property | Value |
| --- | --- |
| Size | 720 x 48 (same width as the composer) |
| Layout | row, gap 10, items centred, padding 8 top/bottom, 14 left, 10 right |
| Fill / border / radius | `#FFFFFF`, 1 px `#E5E7EB` (gray/200), 12 |
| Shadow | none |

| # | Part | Detail |
| --- | --- | --- |
| 1 | `icon_gauge` | icon `gauge`, 16 x 16 |
| 2 | Message | 14 / 20, weight 400, interface sans, `#4B5563` (gray/600), one line. Default: `You’ve used 78% of your weekly limit` |
| 3 | Spacer | fills |
| 4 | `action` | small secondary button `Get more usage` (30 high) |
| 5 | `dismiss` | icon button 28 x 28, radius 6, no fill, icon `x` 16 |

The component has one variant. The two screens override its colours and text:

| Screen | Where | Fill | Border | Message |
| --- | --- | --- | --- | --- |
| `02_WeeklyUsageWarning` `97:126655` (in a thread) | first child of `composer_dock` `97:126668` | `#FFF7ED` (orange/50) | 1 px `#FB923C` (orange/400) | `You’ve used 94% of your weekly limit. Resets Monday 9:00 AM.` |
| `03_UsageWarningHome` `97:126672` (on home) | `97:126678`, between greeting and composer | `#FFFFFF` (component default) | 1 px `#E5E7EB` (default) | `You’ve used 76% of your weekly limit` |

The file gives no rule for when the neutral (white) and the warning (orange) look apply; 76 and 78 percent are neutral and the 94 percent screen is orange.

In a thread: `composer_dock` is 1152 x 188 at the bottom of the 848 high body (y 660). Column, gap 8, centred, bottom padding 12. Order: banner (48), composer (96), disclaimer line (16). So the banner adds 56 px to the normal 132 high dock. The disclaimer is `Isonomic is AI and can make mistakes. Please double-check responses.`, 12 / 16, weight 400, `#9CA3AF` (gray/400).

On home: order is greeting (y 217), banner (y 279), composer (y 347), footer (y 463) inside the 848 high content area, each 20 apart, centred as described in `home.md`.

What `Get more usage` opens, whether dismissal is remembered, and a "limit reached" (100 percent) state are not shown in the design file.

## `EmptyState` (`101:5213`)

Component note in the file: "Placeholder for empty lists or panels with one first-step button."

| Property | Value |
| --- | --- |
| Size | 360 wide, hugs height (222 with the action) |
| Layout | column, gap 8, centred, padding 28 top/bottom, 24 left/right |
| Fill / border | none |

| # | Part | Detail |
| --- | --- | --- |
| 1 | `icon_tile` (`101:5201`) | 44 x 44, fill `#F3F4F6` (gray/100), radius 12; icon 22 x 22 centred. Default icon `sparkle`; the icon is a swappable slot |
| 2 | Title (`101:5204`) | default `Nothing here yet`; 16 / 24, weight 600, interface sans, `#111827` |
| 3 | Description (`101:5205`) | default `Start a chat and Isonomic will keep track of it here.`; 14 / 20, weight 400, `#6B7280`, centred, width 300, wraps |
| 4 | `action` (`101:5206`), optional | primary button with icon `plus` 16, default label `Start a chat`. A boolean property hides it |

One variant only. No screen in the pages covered here places it (for example an empty chat list in the sidebar); where it sits inside a list or panel is not shown in the design file.

## `Toast` (`101:5177`)

Component note in the file: "Toast with one action (Undo / Retry)."

| Property | Value |
| --- | --- |
| Size | 360 x 40 |
| Layout | row, gap 10, items centred, padding 10 top/bottom, 12 left/right |
| Fill / radius | `#111827` (gray/900), 10; no border |
| Shadow | style "/shadow/lg": 0 4 6 -2 `#0000000D`; 0 10 15 -3 `#0000001A` |
| Message | 14 / 20, weight 400, interface sans, `#FFFFFF`, fills the row. Default `Email sent to design@isonomic.dev` |
| Action | text only, 14 / 20, weight 600, `#D1D5DB` (gray/300). Default `Undo` |

| Variant | Node | Icon (16 x 16) | Icon colour |
| --- | --- | --- | --- |
| tone=Success | `101:5174` | `check-circle` | `#10B981` (green/500) |
| tone=Info | `101:5175` | `info` | light, close to white (vector; exact value not in the reference code) |
| tone=Error | `101:5176` | `warning-circle` | `#EF4444` (red/500) |

The three tones differ only in the icon. Where toasts appear on screen, how they stack, how long they stay and a close button are not shown in the component; see the session pages for placed examples.

## Oddities

- The `TextField` component note says focus is a "2px blue ring", but the focused instance in the project picker menu draws a 2 px `#111827` border (see `composer.md`).
- `UsageBanner` has one variant, yet the thread screen recolours it by hand to orange. A build needs two tones.
- Onboarding frames are numbered 05 to 07; no 01 to 04 onboarding frames exist on this page, and step 4 `Done` has no screen.
