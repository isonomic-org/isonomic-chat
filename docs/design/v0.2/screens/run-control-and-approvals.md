# Run control and approvals (design 0.2)

Outside version 1. Recorded so the design is not lost.

Page "Run Control and Approvals 0.2" (`101:60680`), section `102:2`, 14 frames of 1440 x 900. One session ("Palette rollout") runs a five-step plan that needs one approval. Values marked "seen" were read from the screenshot, not from the layer data.

## Shared layout

| Region | Position, size | Notes |
| --- | --- | --- |
| `app_sidebar` | 0,0 288 x 900 | Same as every other page |
| `main_panel` | 288,0 1152 x 900 | `session_header` 1152 x 52, `session_body` 1152 x 848 |
| `conversation` | 1152 wide, or 772 / 692 beside a side panel | `transcript` 720 wide centred (x 216); 448 wide beside a panel |
| `transcript_scroll` | height 680 or 664 | Transcript column: padding 32 top and bottom, gap 24 |
| `composer_dock` | height 132, 168 or 184 | Stack from the top: optional strip (run status bar 44 high, queued message, or autonomy dial 28 high), 8 gap, `Composer` 720 x 96, disclaimer 16 high |
| `side_panel` | 380 wide (checkpoints) or 460 wide (activity) | 1 px left border, content inset 17 left, 16 top and right |

User message in frames 01 to 10 and 13: `Send the design team a short note that Teal is our new accent, attach palette-notes.md, then update the palette doc.` In frames 11, 12 and 14 (narrow column): `Send the palette note to design, then update the doc.`

Plan steps everywhere: `Read palette-notes.md` (`0:02`), `Draft the email to the design team` (`0:06`), `Send the email (asks first)` (`—`), `Update the "Isonomic palette note" doc` (`~20s`), `Summarize what changed` (`~5s`). The design uses curly quotes.

## Screens

| # | Frame | Node | Screenshot | What it shows |
| --- | --- | --- | --- | --- |
| 01 | AgentRun/01_PlanReview | `102:3` | `../images/run-control-01-plan-review.png` | Reply `Here's my plan. Sending email needs your OK, so I'll ask at step 3.` Plan card titled `Proposed plan · 5 steps` with buttons `Edit` (outline, pencil icon) and `Run plan` (dark, play icon); all steps Todo. `AutonomyDial` (Ask) centred above the composer |
| 02 | AgentRun/02_PlanRunningQueue | `102:1512` | `../images/run-control-02-plan-running-queue.png` | Plan card `Running plan · step 2 of 5`: step 1 Done, step 2 Running. Dock: `QueuedMessage` `Also mention the dark-mode accent`, then `RunStatusBar` Running. Header pill `2 running · 1 waiting` |
| 03 | AgentRun/03_Steering | `102:2192` | `../images/run-control-03-steering.png` | Plan card `Plan · 5 steps`; streaming reply `Drafting: "Hi team, quick update on the Isonomic palette. Starting today our accent colour is Teal (#0D9488)..."`; `SteerNote` `Steer: keep it under 5 lines`; block cursor; status bar Running |
| 04 | AgentRun/04_ToolCallsStreaming | `102:2853` | `../images/run-control-04-tool-calls-streaming.png` | Thinking state bar `Thinking` `0:06` (640 wide, collapsed, caret down); three `ToolCallCard` rows: `read_file` `path: "palette-notes.md"` `0.2s` Done, `search_files` `query: "palette"` `0.4s` Done, `draft_email` `to: "design@isonomic.dev"` `0:02` Running |
| 05 | AgentRun/05_ToolError | `102:4224` | `../images/run-control-05-tool-error.png` | `ToolCallCard` Done (`read_file` `path: "palette-notes.md"` `0.2s`, 38 high) and `ToolCallCard` Error (`read_file` `path: "~/Design/tokens.css"` `0.1s`, 101 high) with `Error` / `Permission denied: ~/Design is not a linked folder.` |
| 06 | AgentRun/06_Stopped | `102:4873` | `../images/run-control-06-stopped.png` | Plan card `Plan · 5 steps` with steps 1 and 2 Done, the rest Todo, nothing running |
| 07 | Approvals/07_PermissionSafe | `102:5509` | `../images/run-control-07-permission-safe.png` | Plan with step 3 Running; `PermissionRequest` Safe card docked above the composer, overlapping the bottom of the plan |
| 08 | Approvals/08_PermissionDestructive | `102:6897` | `../images/run-control-08-permission-destructive.png` | Plan with step 4 Running; `PermissionRequest` Destructive card |
| 09 | Approvals/09_AllowedWithUndo | `102:8672` | `../images/run-control-09-allowed-with-undo.png` | Plan with step 4 Running; decided permission row; status bar Running; toast `Email sent to design@isonomic.dev` / `Undo` |
| 10 | Approvals/10_AutonomyMenu | `102:9736` | `../images/run-control-10-autonomy-menu.png` | Reply `Ready when you are.`; composer footer row below the composer; autonomy menu open above it |
| 11 | Checkpoints/11_Panel | `102:10377` | `../images/run-control-11-panel.png` | Plan complete (all Done) in a 448 column; checkpoints panel 380 wide |
| 12 | Checkpoints/12_RewoundBranch | `102:11125` | `../images/run-control-12-rewound-branch.png` | Reply `Rewound to "Drafted email" and started a new attempt with a shorter draft. The earlier attempt stays in the panel.`; panel shows abandoned rows and a new branch; status bar Running |
| 13 | AgentRun/13_Done | `102:11813` | `../images/run-control-13-done.png` | Plan card `Plan complete · 5 of 5`, all Done; reply `Done. The design team has the note and the palette doc now lists Teal as the accent.` `10:14 PM` |
| 14 | Activity/14_LedgerUndo | `102:12496` | `../images/run-control-14-ledger-undo.png` | Reply `Here's everything I changed in your apps today.`; activity panel 460 wide; toast `Doc edit undone` / `Redo` |

## Plan card (`agent_plan`, frame, for example `102:6102`)

| Part | Values |
| --- | --- |
| Card | 720 wide (448 beside a panel), white, 1 px border `#E5E7EB`, radius 12, clips content |
| Header | background `#F9FAFB`, 1 px bottom border `#E5E7EB`, padding 12 x 10, gap 8; `icon_list_checks` 16 px; title 14/20 semibold `#111827`. 41 high, or 51 high with buttons (frame 01) |
| Header buttons (frame 01) | `Edit` 66 x 30 outline with pencil icon; `Run plan` 93 x 30 dark (`#111827`, white text) with play icon; 8 apart |
| Rows | one `PlanStep` per step, 44 high |

Title strings: `Proposed plan · 5 steps`, `Running plan · step 2 of 5`, `Plan · 5 steps`, `Plan complete · 5 of 5`.

### PlanStep (`101:4314`)

Description: editable step in an agent plan: drag to reorder, per-step timer, retry on failure. 640 x 44 by default, padding 0 x 10, gap 10, radius 8, 1 px bottom border `#F3F4F6`. Parts: `icon_dots_six_vertical` 14 px drag handle (gray/300), state icon 16 px, label 14/20, timer 12/16 `#9CA3AF`.

| Variant | Node | Row background | State icon | Label colour |
| --- | --- | --- | --- | --- |
| Todo | `101:4310` | none | `icon_circle` | `#374151` |
| Running | `101:4311` | `#F9FAFB` | `icon_circle_notch` (spinner) | `#111827` |
| Done | `101:4312` | none | `icon_check_circle`, green/600 `#059669` | `#6B7280` |

Defaults: label `Write palette-notes.md`, timer `~20s`. The description mentions retry on failure but no failed variant is drawn.

## ToolCallCard (`100:3299`)

Fully specified in `agent-work-and-side-panel.md` (variants Running `100:3295`, Done `100:3296`, Expanded `100:3297`, Error `100:3298`). On this page the cards sit directly in the transcript, 720 wide, 24 apart, not inside a work card.

## PermissionRequest (`100:4472`)

Description: agent asks for one permission: reason, preview, once / 1 hour / always, Allow or Deny. Decided collapses with Revoke. Properties: `risk` (Safe, Destructive), `state` (Waiting, Decided), `title`, `meta`.

Waiting card (Destructive `100:4469`; the Safe waiting variant was read from frame 07):

| Part | Values |
| --- | --- |
| Card | 640 wide by default (720 in place), white, 1 px border `#E5E7EB`, radius 12, padding 16 x 14, column gap 12, shadow md |
| Header row | gap 10: agent orb 20 x 20; title 14/20 semibold `#111827`; meta 12/16 `#6B7280` 1 px below; `risk_tag` at the right |
| `risk_tag` | radius 6, padding 7 x 2, 12/16 medium. Destructive: background `#FEF2F2` (red/50), text `#B91C1C` (red/700). Safe: pale green background, green text (seen) |
| Reason | 14/20 `#4B5563` |
| `preview` | background `#F9FAFB`, radius 8, padding 12 x 10, gap 6; label `What will happen` 12/16 medium `#6B7280`; key and value rows 14/20, key `#4B5563` left, value `#111827` right |
| `duration` | segmented control: track `#F3F4F6`, radius 8, padding 2, gap 4; options padding 10 x 4, radius 6, 12/16 medium; selected is white with a soft shadow and `#111827` text, others `#6B7280` |
| Buttons | right-aligned, gap 8, padding 13 x 9, radius 6, 14/16 medium. `Deny`: white, 1 px border `#D1D5DB`, text `#374151`. Allow: filled, white text |

| | Safe (frame 07) | Destructive (frame 08, `100:4469`) |
| --- | --- | --- |
| Title | `Isonomic wants to send an email` | `Isonomic wants to replace the whole palette doc` |
| Meta | `[model name] · Mail` | `[model name] · Isonomic Docs` |
| Risk tag | `Safe` | `Destructive` |
| Reason | `You asked me to tell the design team that Teal is the new accent.` | same text |
| Preview | `What will happen`; `To` / `design@isonomic.dev`; `Subject` / `Palette update`; `Attachments` / `palette-notes.md` | same rows |
| Duration options | `Once` (selected), `For 1 hour`, `Always` | `Once` (selected), `For 1 hour` (no `Always`) |
| Buttons | `Deny` (outline), `Allow once` (dark fill `#111827`) | `Deny` (outline), `Allow once` (red fill `#DC2626`) |

Which button is the default: in both variants the allow button is the filled, emphasised button and `Deny` is the outline one. The duration control starts on `Once` in both; no variant starts on `Always`, and the destructive variant does not offer it. Nothing in the file marks a keyboard default. Note for the build: the product rule is deny-safe (no binding defaults to allow), so the visual emphasis on allow must not become the Enter or default action.

Decided row (`100:4471`, risk Safe): 640 wide (720 in place), background `#F9FAFB`, 1 px border `#E5E7EB`, radius 10, padding 12 x 10, gap 8. `icon_check_circle` 16 px green/600 `#059669`; text 14/20 `#374151`; meta 12/16 `#9CA3AF`; small button `Revoke` (white, 1 px border `#D1D5DB`, radius 4, padding 11 x 7, 12/16 medium `#374151`). Frame 09 text: `Allowed once · sent email to design@isonomic.dev`, meta `just now`, `Revoke`.

## AutonomyDial (`100:4516`)

Description: how much the agent may do without asking. Segmented control, 257 x 28 in place: track `#F3F4F6`, radius 8, padding 2, gap 2. Each `opt`: padding 10 x 4, gap 4, radius 6, 14 px icon and 12/16 medium label. Selected option: white with a soft shadow, text `#111827`; others `#6B7280`.

| Value | Node | Icon | Label | Notes |
| --- | --- | --- | --- | --- |
| Ask | `100:4513` | `icon_hand` | `Ask first` | Shown selected in frame 01 |
| Edits | `100:4514` | `icon_pencil_simple` | `Edits` | |
| Full | `100:4515` | `icon_lightning` | `Full access` | When selected the label is red/600 `#DC2626` |

Autonomy menu (frame 10, `102:10344`): 320 x 237 popover at x 797, y 591, padding 5; a header 310 x 26 and three detail items 310 x 67.

| Row | Title | Description |
| --- | --- | --- |
| Header | `How much can Isonomic do on its own?` | |
| 1 (checked) | `Ask first` | `Approve every action that changes something` |
| 2 | `Edits` | `Edit files and docs freely; ask before sending or deleting` |
| 3 | `Full access` (red title) | `Act without asking. Hard limits still apply.` |

Frame 10 also shows a `ComposerFooter` row (720 x 28) under the composer: `Project` with a folder icon and caret, `[model name]` `Extra`, `Auto`, and an `Apps` pill with three small icons. The menu is anchored to `Auto`.

## RunStatusBar (`100:4574`)

Description: run controls above the composer: Stop / Steer while running; Resume, Retry, Dismiss after. 720 x 44 (448 beside a panel), 1 px border, radius 10, padding 12 left, 6 right, 6 top and bottom, gap 8. Icon 14 px, message 14/20 medium filling the row, small buttons (radius 4, padding 11 x 7, 12/16 medium; outline is white with border `#D1D5DB` and text `#374151`; primary is `#111827` with white text).

| Status | Node | Background, border | Icon | Message colour | Buttons, left to right |
| --- | --- | --- | --- | --- | --- |
| Running | `100:4570` | `#F9FAFB`, `#E5E7EB` | `icon_circle_notch` | `#1F2937` | `Steer` (outline), `Stop` (primary) |
| Stopped | `100:4571` | `#F9FAFB`, `#E5E7EB` | `icon_stop` | `#1F2937` | `Dismiss`, `Retry` (outline), `Resume` (primary) |
| Error | `100:4572` | `#FEF2F2`, `#FEE2E2` | `icon_warning_circle` (red) | `#B91C1C` | `Dismiss` (outline), `Retry` (primary) |
| Done | `100:4573` | `#F9FAFB`, `#E5E7EB` | `icon_check_circle` (green/600) | `#1F2937` | `Retry` (outline) |

Default message: `Working · 0:14`. Messages for the other statuses are not set in the component.

## QueuedMessage (`100:4584`)

Description: message waiting to send after the current run. 720 wide, white, 1 px border `#E5E7EB`, radius 8, padding 10 x 6, gap 8. `icon_clock` 14 px (gray/400), text 14/20 `#374151` filling the row, `Queued` 12/16 `#9CA3AF`, `icon_button_x` 28 px ghost button to remove it. Default text `Also add a dark-mode note`. Sits directly above the status bar.

## SteerNote (`100:4603`)

Description: note sent to a running agent, applied at its next step. Inline chip: background `#FFFBEB` (yellow/50), 1 px border `#FBBF24` (yellow/400), radius 6, padding 8 x 4, gap 6; `icon_arrow_elbow_down_left` 12 px; text 12/16 medium `#92400E` (yellow/800). Default text `Steer: keep it under 5 lines`. Placed under the streaming paragraph.

## RunningNowPill (`101:5682`)

Not read from the layer data. Seen in the header of frames 02 to 04 and 09, left of the panel toggle: a rounded pill, light grey fill, about 160 x 26, small agent orb, text `2 running · 1 waiting` in 12 px medium grey.

## Checkpoints panel (frames 11 and 12)

Side panel 380 wide. Header row 347 x 36: title 14/20 semibold (seen) and `icon_button_x` 28 px. Then `CheckpointRow` items 347 x 40, 4 apart. Titles: `Checkpoints · $0.04` (frame 11), `Checkpoints · $0.05` (frame 12).

### CheckpointRow (`101:4357`)

Description: checkpoint in an agent run; rewind from any past point. Faded = abandoned attempt. 560 x 40 by default, padding 0 x 10, gap 10. Parts: `rail` 12 x 10 (timeline dot), `icon` 14 px (swappable; default `file-text`), title 14/20 `#111827` filling the row, then state-specific parts, meta 12/16 `#9CA3AF`.

| State | Node | Rail dot | Title weight | Extra parts |
| --- | --- | --- | --- | --- |
| Head | `101:4353` | dark, filled | medium | tag `Current`: background `#111827`, white text, radius 6, padding 6 x 2, 12/16 medium; then meta |
| Past | `101:4354` | grey | regular | small outline button `Rewind` with a 16 px `arrow-clockwise` icon (padding 9 left, 11 right, 7 vertical); then meta |
| Failed | `101:4355` | red (red/500) | regular | tag `Failed`: background `#FEF2F2`, text `#B91C1C`; then meta |
| Faded | `101:4356` | grey | regular | meta only |

Defaults: title `Wrote palette-notes.md`, meta `2 files · 10:02`.

| Frame | Rows (title, state, meta) |
| --- | --- |
| 11 | `Read palette-notes.md` Past `10:00`; `Drafted email` Past `10:05`; `Sent email` Past `10:00`; `Updated palette doc` Past `10:00`; `Summarized changes` Head `10:06` |
| 12 | `Read palette-notes.md` Past `10:12`; `Drafted email` Past `10:12`; `Sent email` Faded `abandoned`; `Updated palette doc` Faded `abandoned`; `Redrafted email (shorter)` Head `10:12`, indented about 24 px to show the new branch |

Row icons seen in frame 11: file, pencil, envelope, file, check.

## Activity ledger (frame 14)

Side panel 460 wide. Header row 427 x 36: `Activity · today` and `icon_button_x`. Then `LedgerRow` items 427 x 48, 4 apart.

### LedgerRow (`101:5249`)

Description: one change the agent made in a connected app, with Undo. 680 x 48 by default, padding 0 x 12, gap 10, 1 px bottom border `#F3F4F6`. `icon` 16 px (default `envelope-simple`), title 14/20 `#111827`, meta 12/16 `#6B7280` 1 px below.

| State | Node | Trailing part |
| --- | --- | --- |
| Default | `101:5247` | small outline button `Undo` with a 16 px `arrow-clockwise` icon |
| Undone | `101:5248` | tag `Undone`: background `#F3F4F6`, text `#4B5563`, radius 6, padding 6 x 2, 12/16 medium |

Default meta: `Mail · 10:04 PM · from Agentic UI design reference`.

| Row | Title | Meta | State |
| --- | --- | --- | --- |
| 1 | `Sent "Palette update" to design@isonomic.dev` | `Mail · 10:04 PM` | Default |
| 2 | `Edited "Isonomic palette note"` | `Isonomic Docs · 10:06 PM` | Undone |
| 3 | `Created event "Palette review"` | `Calendar · 10:08 PM` | Default |
| 4 | `Renamed palette.json to tokens.json` | `Files · 10:09 PM` | Default |

## Toast (`101:5177`)

Description: toast with one action (Undo / Retry). 360 x 40, background `#111827`, radius 10, padding 12 x 10, gap 10, shadow lg. Icon 16 px, message 14/20 white filling the row, action 14/20 semibold `#D1D5DB`. Placed at x 1056, y 828 (24 from the right and bottom edges).

| Tone | Node | Icon |
| --- | --- | --- |
| Success | `101:5174` | `icon_check_circle`, green/500 `#10B981` |
| Info | `101:5175` | `icon_info` |
| Error | `101:5176` | `icon_warning_circle`, red/500 `#EF4444` |

Texts used: `Email sent to design@isonomic.dev` / `Undo` (Success, frame 09); `Doc edit undone` / `Redo` (Info, frame 14).

## Oddities

- Frames 05 (tool error), 06 (stopped) and 13 (done) all show the status bar in its Running state (`Working · 0:14`, `Steer`, `Stop`); the Error, Stopped and Done variants exist only in the component set, with no message text.
- Frame 08: the destructive request is titled as a doc replacement but keeps the email reason and the email preview rows from the safe request.
- Frame 09 toast offers `Undo` for a sent email, and the decided row offers `Revoke` after a one-time allow that already ran.
- Frame 11 checkpoint times are out of order (`10:00`, `10:05`, `10:00`, `10:00`, `10:06`); frame 12 shows `10:12` on rows that were `10:00` and `10:05`.
- Frame 14: the user bubble is clipped at the left edge of the narrow column and the disclaimer wraps to two lines.
- The header pill reads `2 running · 1 waiting` in frames with a single visible run.
- Sample data names a model in the permission meta and composer footer; written here as `[model name]`.
