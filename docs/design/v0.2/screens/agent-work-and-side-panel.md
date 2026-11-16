# Agentic work and the side panel (design 0.2)

Outside version 1. Recorded so the design is not lost.

Page "Agentic Work and the Side Panel 0.2" (`97:44917`), section `97:44918`, 14 frames of 1440 x 900. One session ("Agentic UI design reference") is followed from the first tool call to the finished task. Values marked "seen" were read from the screenshot, not from the layer data.

## Shared layout

| Region | Node (frame 03) | Position, size | Notes |
| --- | --- | --- | --- |
| `app_sidebar` | `97:44975` | 0,0 288 x 900 | Same as every other page |
| `main_panel` | `97:44976` | 288,0 1152 x 900 | |
| `session_header` | `97:44977` | 1152 x 52 | Title, caret, device icon, panel toggle |
| `step_counter` | `97:45012` | text at x 1280, y 18 (frame coordinates), 20 high | Sits in the header, left of the panel toggle. 14/20, dark grey (seen) |
| `session_body` | `97:44978` | 0,52 1152 x 848 | Holds the split |
| `conversation` | `97:44979` | 832 x 848 | `transcript_scroll` 832 x 716, then `composer_dock` 832 x 132 |
| `transcript` | `97:44981` | x 56, 720 wide | Column: padding 32 top and bottom, gap 24 |
| `side_panel` | `97:44997` | x 832, 320 x 848 | 1 px left border (sections start at x 17), content 287 wide, top padding 16 |
| `Composer` | `97:44995` | x 56, 720 x 96 | Disclaimer below at y 104, 12/16 |

Split widths:

| State | conversation | right pane | transcript column |
| --- | --- | --- | --- |
| Side panel open (frames 01 to 04, 07 to 10, 14) | 832 | `side_panel` 320 | 720, inset 56 |
| File preview open (frames 11 to 13) | 592 | `file_preview_pane` 560 | 528, inset 32 |
| Question card (frames 05, 06) | full 1152, no side panel | none | 720, centred (seen) |

Step counter values across the flow: none (01, 05, 06, 11 to 13), `Step 1 of 4` (02, 04), `Step 2 of 4` (07), `Step 4 of 4` (08, 10), `Done` (03), `4 of 4 done` (09, 14).

Common strings: composer placeholder `Reply to Isonomic...`, context ring label `42%`, disclaimer `Isonomic is AI and can make mistakes. Please double-check responses.` The composer shows a stop button while a run is active and a voice button when idle.

User message used in frames 01 to 06: `I'm documenting the agentic UI. Please, in order: 1) ask me one multiple-choice question (which accent colour I prefer), 2) set up a 4-step task list and tick it off as you go, 3) write a 5-line markdown file called palette-notes.md, 4) make a one-page Docs artifact titled "Isonomic palette note", 5) finish with one sentence.` (the design uses curly quotes and a curly apostrophe).

## Screens

| # | Frame | Node | Screenshot | What it shows |
| --- | --- | --- | --- | --- |
| 01 | AgentWork/01_PanelEmpty | `97:44919` | `../images/agent-work-01-panel-empty.png` | Run just started, empty side panel |
| 02 | AgentWork/02_ToolsAndSearch | `97:44937` | `../images/agent-work-02-tools-and-search.png` | Work card open with four tool calls; panel shows Progress and Context |
| 03 | AgentWork/03_WorkSummaryDone | `97:44974` | `../images/agent-work-03-work-summary-done.png` | Work card collapsed to a summary, reply below, all sections filled |
| 04 | AgentWork/04_ToolCallDetail | `97:45013` | `../images/agent-work-04-tool-call-detail.png` | One tool call opened to request and response |
| 05 | AgentWork/05_QuestionCard | `97:45055` | `../images/agent-work-05-question-card.png` | Question card above the composer |
| 06 | AgentWork/06_QuestionHoverUsage | `97:45081` | `../images/agent-work-06-question-hover-usage.png` | Option 2 hovered, usage banner between card and composer |
| 07 | AgentWork/07_AnsweredThinking | `97:45108` | `../images/agent-work-07-answered-thinking.png` | Answered question as a compact card, thinking row |
| 08 | AgentWork/08_PanelProgressOutputsContext | `97:45139` | `../images/agent-work-08-panel-progress-outputs-context.png` | File card, skill work card, artifact card; all three panel sections |
| 09 | AgentWork/09_TaskCompleteSummary | `97:45173` | `../images/agent-work-09-task-complete-summary.png` | Final reply, counter `4 of 4 done` |
| 10 | AgentWork/10_WorkLogExpanded | `97:45205` | `../images/agent-work-10-work-log-expanded.png` | Work card listing seven task-list tool calls |
| 11 | FilePreview/11_Open | `97:45245` | `../images/agent-work-11-open.png` | File preview pane, Preview tab |
| 12 | FilePreview/12_SaveMenu | `97:45287` | `../images/agent-work-12-save-menu.png` | Save menu open |
| 13 | FilePreview/13_CodeView | `97:45336` | `../images/agent-work-13-code-view.png` | File preview pane, Code tab |
| 14 | AgentWork/14_AllStepsChecked | `97:45380` | `../images/agent-work-14-all-steps-checked.png` | Task list card with every step checked |

### Text per screen

| # | Transcript | Side panel |
| --- | --- | --- |
| 01 | User message. Thinking row `Getting started` `12s` | Empty state: `icon_list_checks` (seen, about 20 px, light grey) and `Progress, outputs and context show up here while Isonomic works.` centred near the top, 14/20 light grey |
| 02 | Work card `Researching accent colours` `4 steps`; rows `web_search` `accessible accent colours UI` `0.2s`, `web_search` `teal vs indigo contrast ratio` `0.5s`, `web_fetch` `WCAG contrast guidelines` `0.8s`, `read_file` `Isonomic brand notes` `1.1s`. Thinking row `Comparing contrast ratios` `12s` | Progress: current `Ask accent colour question`; pending `Set up task list`, `Write palette-notes.md`, `Create Docs artifact`. Context `3`: `Web search` / `Tool`, `Isonomic Docs` / `Connector`, `docs` / `Skill` |
| 03 | Work card `Used 6 tools` `48s`. Reply: `Done. I asked about the accent colour, tracked the four steps, wrote palette-notes.md and created the "Isonomic palette note" doc.` then `Teal is now the accent across both outputs.` Reply actions, `10:02 PM` | Progress: four done steps. Outputs `2`: `palette-notes.md` / `MD`, `Isonomic palette note` / `Docs`. Context `2`: `Isonomic Docs` / `Connector`, `docs` / `Skill` |
| 04 | Same card as 02; first row (`1.4s`) is followed by `Request` and `Response` blocks (text below), then `web_fetch` `WCAG contrast guidelines` `0.2s`. Thinking row as 02 | As 02 |
| 05 | User message, thinking row `Working on it` `12s`, question card (below). Composer placeholder `Or reply directly...` | No panel |
| 06 | As 05 plus banner `You've used 76% of your weekly limit` with button `Get more usage` and a close icon | No panel |
| 07 | Answered card: `Which accent colour do you prefer for the Isonomic palette?` then check icon and `Teal · Fresh, balanced, neutral-leaning`. Thinking row `Setting up the task list` `12s` | Progress: done `Ask accent colour question`, current `Set up task list`, two pending. Context `2` |
| 08 | File card `palette-notes.md` / `Document · MD` / `Open`. Work card `Using Isonomic Docs`. Artifact card `Isonomic palette note` / `Docs · Only you` | Progress: `1 earlier step`, `Write palette-notes.md`, `Create Docs artifact` done; current `Verifying outputs`. Outputs `2`, Context `2` as 03 |
| 09 | File card, artifact card, reply `All four steps are done: palette-notes.md and the "Isonomic palette note" doc both use Teal as the accent.` `10:04 PM` | As 03 |
| 10 | Work card `Working through the task list` `7 steps`; rows `created_task_list` `4 steps` `0.5s`, `completed` `Ask accent colour question` `0.8s`, `started` `Set up task list` `1.1s`, `completed` `Set up task list` `1.4s`, `wrote_file` `palette-notes.md` `0.2s`, `completed` `Write palette-notes.md` `0.5s`, `started` `Create Docs artifact` `0.8s`. Thinking row `Creating the Docs artifact` `12s` | Progress: three done, current `Create Docs artifact`. Outputs `1`: `palette-notes.md` / `MD`. Context `2` |
| 11 to 13 | File card, artifact card, reply `Here's the markdown file and the doc.` `10:04 PM` | File preview pane (below) |
| 14 | Task list card (below), file card, artifact card, reply `Everything's finished: the palette is set to Teal and documented in both places.` `10:05 PM` | As 03 |

Request block text in frame 04 (mono face, 13/20):

```
{
  "query": "teal vs indigo contrast ratio",
  "max_results": 5
}
```

Response block text:

```
[
  { "title": "Contrast checker", "url": "…/contrast" },
  { "title": "Choosing accessible accents", "url": "…/accents" }
]
```

## Work card

Frame `work_card` (`97:45023` in frame 04, `97:44984` in frame 03). Not a library component.

| Part | Values |
| --- | --- |
| Card | 720 wide, white, 1 px border `#E5E7EB` (gray/200), radius 12, clips content, column gap 6 |
| `work_header` | 718 x 40, background `#F9FAFB` (gray/50), padding 12 x 10, gap 10 |
| Leading icon | 16 px: `icon_globe` (research), `icon_list_checks` (task list, seen), `icon_file_text` (skill in use), `icon_check_circle` (finished summary) |
| Title | 14/20 medium, `#1F2937` (gray/800), fills the row |
| Meta | 12/16 regular, `#9CA3AF` (gray/400): step count (`4 steps`) while open, duration (`48s`) when collapsed; absent on `Using Isonomic Docs` |
| Trailing icon | 14 px: `icon_caret_down` when open, `icon_caret_right` when collapsed |
| Body | One `ToolCallCard` per call, full width, stacked with the 6 gap |
| Collapsed | Header only; card height 42 |

`tool_call_detail` (`97:45030`, shown under an opened call): padding 10 top, 12 sides, 12 bottom, gap 8. Labels `Request` and `Response` are 12/16 medium `#6B7280`. Each block (`request_block` `97:45032`, `response_block` `97:45035`) has background `#F9FAFB`, 1 px border `#E5E7EB`, radius 8, padding 12 x 10, mono face 13/20 `#1F2937`.

## Components

### ToolCallCard (`100:3299`)

Description in the design file: one-line tool call with timer and status, expandable to arguments and results. 640 wide by default (fills the work card). White, 1 px border, radius 10, clips content.

| Part | Values |
| --- | --- |
| `header` | padding 12 x 8, gap 8, about 38 high |
| `icon` | 14 px `terminal-window`, gray/500 |
| Tool name | mono face 13/20, `#111827` |
| Arguments | mono face 13/20, `#9CA3AF`, fills the row |
| Timer | interface sans 12/16, `#9CA3AF` |
| Status icon | 14 px, see variants |
| Caret | 12 px |
| `body` (Expanded, Error) | background `#F9FAFB`, 1 px top border `#F3F4F6`, padding 8 top, 12 sides, 12 bottom, gap 6; label 12/16 medium; text mono 13/20 |

| Variant | Node | Border | Status icon | Caret | Body |
| --- | --- | --- | --- | --- | --- |
| Running | `100:3295` | `#E5E7EB` | `icon_circle_notch` (spinner) | `icon_caret_down` | none |
| Done | `100:3296` | `#E5E7EB` | `icon_check_circle`, green/600 `#059669` | `icon_caret_down` | none |
| Expanded | `100:3297` | `#E5E7EB` | `icon_check_circle` | `icon_caret_up` | label `Result` `#6B7280`; text `3 files · palette-notes.md, palette.json, tokens.css` `#374151` |
| Error | `100:3298` | `#FEE2E2` (red/100) | `icon_warning_circle`, red | `icon_caret_down` | label `Error` `#DC2626`; text `Permission denied: ~/Design is not a linked folder.` `#B91C1C` |

Defaults: tool `search_files`, args `query: "palette"`, timer `0:02`.

### ToolCallRow (`97:19552`)

Description: one tool call inside a work card: verb + object + chevron. 640 x 38, 1 px bottom border `#E5E7EB`, padding 0 x 12, gap 8. Verb 14/20 `#6B7280` (default `Searched the web`), object 14/20 `#1F2937` filling the row (default names a company chief executive; write `[third-party service] CEO`), `icon_caret_right` 14 px gray/400. This component exists in the library but none of the 14 frames uses it; the frames use `ToolCallCard`.

### ThinkingRow (`97:11474`)

Row with gap 8: animated agent orb 20 x 20, label 14/20 `#4B5563`, seconds 14/20 `#9CA3AF`, `icon_caret_right` 14 px. Description: collapsible thinking summary above a reply; working animates the orb and counts seconds. The related set `ThinkingState` (`100:3242`, 688 x 448) was too large to read as a set and was not broken down; frame 04 of the run control page shows one state (a 640 wide bar reading `Thinking` `0:06`).

### Question card (`97:45067`, frame, not a component)

| Part | Values |
| --- | --- |
| Card | 720 wide, white, 1 px border `#E5E7EB`, radius 14, padding 12, column gap 2, shadow md (0 2 4 -1 at 6% black and 0 4 6 -1 at 10% black). Sits directly above the composer |
| `question_header` | padding 2 top, 6 sides, 8 bottom, gap 8; question 14/20 medium `#111827`; `icon_button_x` 28 px ghost button with a 16 px `x` icon |
| Options | four `QuestionOption` rows |
| `something_else` (`97:45075`) | padding 10 x 8, gap 12; `icon_pencil_simple` 16 px; `Something else` 14/20 `#9CA3AF` (free-text entry); small button `Skip` |
| `Skip` button | white, 1 px border `#D1D5DB`, radius 4, padding 11 x 7, shadow sm, 12/16 medium `#374151` |

Options: `1` `Indigo` / `Cool, calm, technical`; `2` `Teal` / `Fresh, balanced, neutral-leaning`; `3` `Amber` / `Warm, energetic, high-contrast`; `4` `Graphite` / `Minimal, near-monochrome`.

Answered state (frame 07, seen): a 720 x about 66 card, white, 1 px border, radius 12, padding about 14; line 1 the question in 14/20 grey; line 2 a green check icon and the chosen title and description in 14/20 medium, joined by ` · `.

### QuestionOption (`97:11571`)

640 wide (fills the card), padding 10 x 8, gap 12, radius 8, 1 px bottom border `#F3F4F6`. `number` chip 24 x 24, radius 6, digit 12/16 medium `#4B5563`. Title 14/20 medium `#111827`, description 12/16 `#6B7280`, 1 px apart.

| Variant | Node | Row background | Number chip |
| --- | --- | --- | --- |
| Default | `97:11572` | none | `#F3F4F6` |
| Hover | `97:11578` | `#F3F4F6` | white with 1 px border `#E5E7EB` |

### Task list card (`97:45388`, frame, not a component)

720 x 166, white, 1 px border, radius 12 (seen), padding about 14 x 12. Header row: `icon_list_checks` 16 px and `Task list · 4 of 4 done` 14/20 medium. Then one 24-high row per step, 6 apart: `icon_check_square` 16 px (green, seen) and the step label 14/20 grey with strike-through (seen). Labels: `Ask accent colour question`, `Set up task list`, `Write palette-notes.md`, `Create Docs artifact`. Only the all-checked state is drawn.

### Side panel sections

Each section is a column: `PanelSectionHeader`, then rows 4 apart; sections are 20 apart. Order: Progress, Outputs, Context. A section is left out when it has nothing to show (Outputs is absent in frames 02, 04, 07).

**PanelSectionHeader** (`97:19572`): 280 wide by default (287 in place), padding 0 x 4, gap 6. Label 14/20 medium `#374151`; optional count 14/20 `#9CA3AF` (`showCount`, hidden for Progress); spacer; `chevron` 14 px `caret-down`. Labels used: `Progress`, `Outputs`, `Context`.

**ProgressStep** (`97:11548`): 280 x 28, padding 0 x 4, gap 10, `marker` 16 x 16, label 14/20 truncating with an ellipsis.

| Variant | Node | Marker | Label colour |
| --- | --- | --- | --- |
| Done | `97:11549` | `icon_check` 14 px, grey | `#9CA3AF` (gray/400) |
| Current | `97:11553` | filled dot, orange/500 `#F97316` | `#111827` (gray/900) |
| Pending | `97:11557` | filled dot, gray/300 `#D1D5DB` | `#6B7280` (gray/500) |

Earlier steps can fold into one done row reading `1 earlier step` (frame 08).

**PanelRow** (`97:11562`): 280 x 30, padding 0 x 6, gap 10, radius 6. `icon` 16 px (default `file-text`, gray/500; swappable: `globe` for a tool, `sparkle`-style icon for a skill, seen), label 14/20 `#1F2937` filling the row, kind tag 12/16 `#9CA3AF`. Variants: Default (`97:11563`), Hover (`97:11567`, background `#F3F4F6`). Kind tags used: `MD`, `Docs`, `Tool`, `Connector`, `Skill`.

### File preview pane (`97:45261`, frame)

560 x 848 with a 1 px left border.

| Part | Node | Values |
| --- | --- | --- |
| `preview_bar` | `97:45262` | 559 x 48, 1 px bottom border (seen). `icon_file_text` 16 px at x 16; file name `palette-notes.md` 14/20 medium at x 40; `view_toggle` at x 305; `save_button` 79 x 34 at x 432; `icon_button_x` 28 x 28 at x 519 |
| `view_toggle` | `97:45265` | 119 x 26 segmented control, 2 px inner padding, grey track; options `Preview` (64 x 22) and `Code` (49 x 22), 12/16 medium, padding 10 x 3; selected option is white with a soft shadow (same pattern as `AutonomyDial`) |
| `save_button` | `97:45270` | Outline button with a download icon and `Save` |
| `preview_body`, Preview | `97:45272` | 559 x 800. Heading `Isonomic palette` at 32,28, 32 line height, bold (seen, about 24 px). Bullet list at 32,72: rows 24 high, 8 apart, bullet `•` then text at x 17, 16/24 |
| `preview_body`, Code | `97:45364` | `code` block 495 x 142 at 32,28, grey background, border, radius (seen); padding 15 x 13; rows 20 high, 4 apart; line number 8 wide then text at x 22; mono face 13/20 |

Preview text: `Isonomic palette`; `Accent: Teal (#0D9488) for primary actions`; `Neutrals: gray 50–900 for surfaces and text`; `Status: green ok, orange attention, red errors`; `Contrast: every text pair meets WCAG AA`. Code view shows the same file as five numbered lines: `# Isonomic palette`, then each bullet prefixed with `- `.

Save menu (`97:45329`, frame 12): 240 x 179 at x 1152, y 98.5 (right-aligned under the Save button), padding 5, items 230 x 32, one separator 9 high before the last item.

| Item | Icon (seen) | Shortcut |
| --- | --- | --- |
| `Download` (highlighted) | download | `⌘S` |
| `Save to Files` | folder | |
| `Save to Design files` | design-file icon | |
| `Copy contents` | copy | |
| separator | | |
| `Open as Docs artifact` | file | |

### Cards in the transcript

`FileCard` (`97:19577`) and `ArtifactCard` (`97:19584`) are specified in `attachments.md`. Here the file card spans the transcript width (720, or 528 beside the preview) and the artifact card stays 360 wide.

## Oddities

- Frame 04: the opened call's header says `accessible accent colours UI` but its request block shows the query of the second call (`teal vs indigo contrast ratio`); the second and fourth rows of frame 02 are missing and the timers differ from frame 02 (`1.4s` and `0.2s` against `0.2s` and `0.8s`).
- Frame 03 summarises `Used 6 tools` while frame 02 lists four calls and frame 10 seven.
- Frame 08 shows a fifth progress label (`Verifying outputs`, with `1 earlier step` folded) although the counter still reads `Step 4 of 4`.
- Step counter wording changes between `Done` (frame 03) and `4 of 4 done` (frames 09, 14) for the same finished state.
- `ToolCallRow` (verb + object) is defined but unused; the work card uses `ToolCallCard` (tool name + arguments).
- The sidebar in every frame contains recent-chat titles that name outside products; treat them as `[third-party service]` sample data.
