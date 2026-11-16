# Thread states, part 2: feedback, run control, outside version 1

Continues `thread-states.md`. Sources: page "Session Lifecycle 0.2" (`97:40979`), page "Run Control and Approvals 0.2" (`101:60680`) and the Components page (`97:3`). Type is size / line height / weight / face.

## 10 Feedback dialog (`Feedback/10_NegativeDialog`, `97:41214`)

Opened from the thumbs-down button of `ReplyActions`. The thread behind is frame 03 unchanged.

| Layer | Spec |
| --- | --- |
| `scrim` (`97:41240`) | 1440 x 900, covers the whole window including the sidebar. Fill not read |
| `dialog` (`97:41241`) | 520 x 390 at (460, 255): centred in the window. Fill white, border 1 `#E5E7EB` (`gray/200`), radius 12, shadow 0 25 50 -12 at 25% black (`shadow/2xl`), clips content |

`dialog_header` (`97:41242`): row, top aligned, gap 8, padding 20 top, 24 left, 20 right, 12 bottom.

| Part | Spec |
| --- | --- |
| `title` | "Give feedback"; 18 / 28 / semibold / interface sans, `#111827` (`gray/900`) |
| `subtitle` | 4 below the title: "Help us improve Isonomic. Your conversation will be included with this report."; 14 / 20 / regular, `#6B7280` (`gray/500`); 424 wide, two lines |
| `icon_button_x` | ghost icon button 28 x 28, `x` icon 16, top right |

`dialog_body` (`97:41247`): stacked, gap 16, padding 4 top, 24 left and right, 24 bottom.

| Part | Spec |
| --- | --- |
| Label 1 | "What type of issue do you want to report? (optional)"; 14 / 20 / medium, `#374151` (`gray/700`) |
| `select_issue_type` | 470 x 38, fill white, border 1 `#D1D5DB` (`gray/300`), radius 8, padding 12 left and right. Value "Not factually correct" 14 / 20 / regular, `#111827`; `icon_caret_up_down` 14 x 14 at the right |
| Label 2 | "Please provide details (optional)"; same style as label 1 |
| `details_textarea` | 470 x 96, fill white, border 1 `#D1D5DB`, radius 8, padding 12 left and right, 10 top and bottom. Entered text "The prime-factoring example is fine, but it should mention that most systems today use elliptic curves." 14 / 20 / regular, `#111827` |

`dialog_footer` (`97:41255`): row, right aligned, gap 8, 34 high. Buttons as in the edit box: "Cancel" (70 x 34, white, border 1 `#D1D5DB`, radius 6, label `#374151`) and "Submit" (72 x 34, fill `#111827`, white label), both 14 / 16 / medium with `shadow/sm`.

Oddity: the footer has no padding. Its buttons end exactly at the dialog's inner right edge (x = 518 of 520) and bottom edge (y = 389 of 390), while the body above has 24 padding.

## 11 Issue types (`Feedback/11_IssueTypes`, `97:41258`)

Same dialog in its untouched state, with the select open.

| Part | Spec |
| --- | --- |
| Select placeholder | "Select…" |
| Textarea placeholder | "What was unsatisfying about this response?" |
| Placeholder colour | not read |
| `menu` (`97:41302`) | 470 x 234 at (485, 442) in the window: same left edge and width as the select, 4 below it. It overlaps the textarea and runs past the dialog's bottom edge |
| Menu box | fill white, border 1 `#E5E7EB` (`gray/200`), radius 10, padding 4, large soft shadow (0 10 15 -3 at 10% black, 0 4 6 -2 at 5% black, plus a 1 px ring at 5% black) |
| Rows | `MenuItem` component (`97:11392`): 460 x 32, radius 6, padding 8 left and right, row with gap 10: icon 16 x 16, then label 14 / 20 / regular, `#1F2937` (`gray/800`) |
| Highlighted row | fill `#F3F4F6` (`gray/100`); the others have no fill |

Rows in order:

| # | Icon | Label | Highlighted |
| --- | --- | --- | --- |
| 1 | `warning` | "UI bug" | |
| 2 | `list-checks` | "Didn’t follow instructions" | |
| 3 | `warning-circle` | "Not factually correct" | yes |
| 4 | `minus-circle` | "Incomplete response" | |
| 5 | `globe` | "Should have searched the web" | |
| 6 | `flag` | "Harmful or offensive" | |
| 7 | `dots-three` | "Other" | |

Row 5 refers to web search, which is outside version 1. A thumbs-up flow, a confirmation after Submit and validation are not shown in the design file.

## RunStatusBar (component set `100:4574`)

Note: "Run controls above the composer: Stop / Steer while running; Resume, Retry, Dismiss after." Property `status`, property `message` (default "Working · 0:14").

Shared layout: 720 x 44, row, centred vertically, gap 8, padding 12 left, 6 right, 6 top and bottom, border 1, radius 10. Children: status icon 14 x 14, message text (fills the width) 14 / 20 / medium / interface sans, then the buttons.

Buttons are a small size: 30 high, radius 4, padding 11 left and right, 7 top and bottom (including the border), label 12 / 16 / medium, `shadow/sm`. Secondary: fill white, border 1 `#D1D5DB` (`gray/300`), label `#374151` (`gray/700`). Primary: fill `#111827` (`gray/900`), white label.

| `status` | Node | Fill / border | Icon | Message colour | Buttons, left to right |
| --- | --- | --- | --- | --- | --- |
| Running | `100:4570` | `#F9FAFB` (`gray/50`) / `#E5E7EB` (`gray/200`) | `circle-notch` (spinner) | `#1F2937` (`gray/800`) | "Steer" (secondary), "Stop" (primary) |
| Stopped | `100:4571` | same | `stop` | `#1F2937` | "Dismiss", "Retry" (secondary), "Resume" (primary) |
| Error | `100:4572` | `#FEF2F2` (`red/50`) / `#FEE2E2` (`red/100`) | `warning-circle` | `#B91C1C` (`red/700`) | "Dismiss" (secondary), "Retry" (primary) |
| Done | `100:4573` | `#F9FAFB` / `#E5E7EB` | `check-circle` | `#1F2937` | "Retry" (secondary) |

The style list also carries `red/600` (`#DC2626`) and `green/600` (`#059669`), most likely the Error and Done icon colours; the reference does not tie them to a layer. Only the Running message text ("Working · 0:14") was read; the default messages of Stopped, Error and Done were not.

### Where it sits

In `AgentRun/05_ToolError` (`102:4224`) and `AgentRun/06_Stopped` (`102:4873`) the bar is the first child of `composer_dock`:

| Layer | y in dock | Size |
| --- | --- | --- |
| `RunStatusBar` | 0 | 720 x 44, x = 216 |
| `Composer` | 52 (8 gap) | 720 x 96 |
| `disclaimer` | 156 (8 gap) | 387 x 16 |
| dock | | 1152 x 184 (132 + 44 + 8); bottom padding 12 |

`transcript_scroll` shrinks to 664 high (716 - 52) so the total stays 848.

Oddity: in both frames, including the one named Stopped, the bar instance is the Running variant ("Working · 0:14", "Steer", "Stop"), and in the Stopped frame the composer is in its Empty state (voice button). No frame read here shows the Stopped, Error or Done variant in place. Screenshots `../images/run-control-02-plan-running-queue.png` and `../images/run-control-03-steering.png` show the Running bar with the composer in Working state (stop button).

Version 1 note: a plain chat reply on the lifecycle page has no bar at all; stop is the composer's stop button. The bar belongs to agent runs. Its Stopped and Error variants are the only stop and error bars in the file.

## QueuedMessage (`100:4584`), outside version 1

Note: "Message waiting to send after the current run."

| Part | Spec |
| --- | --- |
| Box | 720 wide, 42 high; fill white, border 1 `#E5E7EB` (`gray/200`), radius 8, padding 10 left and right, 6 top and bottom; row, gap 8, centred vertically |
| Children | `icon_clock` 14 x 14; message text (fills) 14 / 20 / regular, `#374151` (`gray/700`); "Queued" 12 / 16 / regular, `#9CA3AF` (`gray/400`); `icon_button_x` ghost 28 x 28 with `x` 16 (remove) |
| Text | component default "Also add a dark-mode note"; in the run-control frame "Also mention the dark-mode accent" |
| Position | in the dock, above the `RunStatusBar`, same 720 column (from the screenshot; the dock height in that frame was not measured) |

## SteerNote (`100:4603`), outside version 1

Note: "Note sent to a running agent, applied at its next step."

| Part | Spec |
| --- | --- |
| Box | hugs content, 24 high plus border; fill `#FFFBEB` (`yellow/50`), border 1 `#FBBF24` (`yellow/400`), radius 6, padding 8 left and right, 4 top and bottom; row, gap 6 |
| Children | `icon_arrow_elbow_down_left` 12 x 12; text 12 / 16 / medium, `#92400E` (`yellow/800`) |
| Text | "Steer: keep it under 5 lines" |
| Position | inside the reply, as a block after the streaming paragraph and before the `stream_cursor` (from the screenshot) |

## ToolCallRow (`97:19552`), outside version 1

One-line row for a tool call inside a reply; documented with the agent work screens.

## 13 Image sent, retitled (`97:41333`), outside version 1

Screenshot `../images/lifecycle-13-image-sent-retitled.png`.

- `user_turn` is 182 high: an `AttachmentThumb` instance (`97:41342`) 120 x 96 at the right edge, then 8 below it the `UserMessage` (284 wide) "What’s missing from this roadmap?". The thumb shows a grey preview area with the file name "sdk-roadmap.png" under it, inside a rounded border.
- Reply: "Three gaps stand out in the roadmap image:", a numbered list of three items (see `message-content.md`), "Want me to add these as tasks with owners?", `ReplyActions` with time "9:51 PM".
- "Retitled": the header and the selected sidebar row read "Roadmap gap review". Title generation itself is in version 1; the attachment is not.

## 14 Attachment viewer (`AttachmentViewer/14_Open`, `97:41361`), outside version 1

Full-window overlay above the thread.

| Layer | Spec |
| --- | --- |
| `scrim` (`97:41372`) | 1440 x 900 |
| `viewer_bar` (`97:41373`) | 1440 x 56 at the top: file name "sdk-roadmap.png" at x = 20 (20 line height); meta text "1440 × 900 · 214 KB" (16 line height) right of centre; `viewer_download` 32 x 32 with `download-simple` 18; `viewer_x` 32 x 32 with `x` 18, 20 from the right edge; 8 between the two buttons |
| `image` (`97:41380`) | 1000 x 625 at (220, 160), centred horizontally; shows a roadmap titled "Isonomic SDK · 6-week roadmap" with four lanes ("Core API", "Auth & keys", "Examples", "Beta launch") |

Colours and type of the viewer were not read.

## Read notes

- Styled reference output was read for: frames 01, 03 to 07, 09, 10, 11, 12 (their transcript, dock, dialog and menu parts), the `UserMessage`, `ReplyActions`, `ThinkingRow`, `RunStatusBar`, `QueuedMessage` and `SteerNote` components, and the `ThinkingState` Done instance.
- Read from layer geometry or screenshots only (the design file's read limit was reached): frames 02, 08 (beyond the `UserMessage`), 13, 14, 15, the `ThinkingState` Working variant, and the run-control frames other than their bar.
