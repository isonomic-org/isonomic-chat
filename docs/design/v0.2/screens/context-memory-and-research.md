# Context, memory and research (design 0.2)

Outside version 1. Recorded so the design is not lost.

Two pages: "Context and Memory 0.2" (`101:60681`, section `104:4880`, 8 frames) and "Research and Web Search 0.2" (`97:117938`, section `97:117939`, 6 frames). All frames are 1440 x 900 with the usual 288 sidebar and 1152 main panel (52 header, 848 body). Values marked "seen" were read from the screenshot, not from the layer data.

## Context and memory screens

| # | Frame | Node | Screenshot | What it shows |
| --- | --- | --- | --- | --- |
| 01 | ContextWindow/01_Panel | `104:4881` | `../images/context-memory-01-panel.png` | Session "Inference cost model"; conversation 740 wide (transcript 448 at x 146); side panel 412 wide with `ContextWindowCard` Normal (380 x 294 at 17,16) and a note under it |
| 02 | ContextWindow/02_NearlyFull | `104:5711` | `../images/context-memory-02-nearly-full.png` | Full-width conversation; `context_banner` 720 x 44 above the composer (dock 184 high) |
| 03 | ContextWindow/03_Compacting | `104:6312` | `../images/context-memory-03-compacting.png` | As 01 with the card in Compacting; a status chip in the transcript; composer shows the stop button |
| 04 | MemoryPanel/04_List | `104:7759` | `../images/context-memory-04-list.png` | Settings modal, Memory page, five saved items |
| 05 | MemoryPanel/05_PendingApproval | `104:8142` | `../images/context-memory-05-pending-approval.png` | Two pending items above three saved ones |
| 06 | MemoryPanel/06_Paused | `104:8545` | `../images/context-memory-06-paused.png` | Memory switched off; notice row; five items in Paused state |
| 07 | MemoryPanel/07_ForgetUndo | `104:8991` | `../images/context-memory-07-forget-undo.png` | Four items left; toast with Undo |
| 08 | MemoryInChat/08_SavedNote | `104:9373` | `../images/context-memory-08-saved-note.png` | Inline "saved to memory" note under a reply |

### Text and regions per screen

Frames 01 to 03 share: user message `Add the egress line items and recompute the monthly total.`; reply `Added egress at $45/TB across 12 TB, so $540 a month. The new monthly total is $12,380, up 4.6%.`; reply actions and `3:12 PM`. The header also carries a `Share` button.

| # | Details |
| --- | --- |
| 01 | Note under the card (`104:5710`, 370 wide, 12/16 grey): `Isonomic compacts older turns automatically when the window is nearly full.` |
| 02 | `context_banner` (`104:6295`): pale orange fill with a soft orange border, radius 10 (seen); `ContextRing` Warn at 13,10 (66 x 24); text `Context is 84% full. Compact older turns to keep going.` 14/20 at x 87; buttons `Details` (outline, 61 x 30) and `Compact` (dark, 72 x 30), 8 apart. The composer's own ring is also in the Warn colour |
| 03 | Transcript chip (seen): about 354 x 38, 1 px border, radius 10, agent orb and `Summarizing 38 earlier turns into a short brief...` 14/20. Card tray reads `Compacting conversation...` |
| 04 to 07 | Modal `settings_modal` 1080 x 760 at 180,70 over a `scrim` covering the window. `settings_nav` 240 wide (title `Settings`, search field, items `Account`, `Billing`, `Usage`, divider, `Appearance`, `Your Isonomic` (selected), `Capabilities`, `Notifications`, `Privacy`, `Wellbeing`, divider, `Customize`, `Devices`, `Code`). `settings_content` 840 wide, 32 side padding: back link `Your Isonomic` with `icon_arrow_left` and a close button (y 20); heading `Memory` (28 line height) with `What Isonomic remembers about you across chats. Edit or forget anything.` (14/20 grey) and, at the right, a state label (`On` or `Paused`) with a 44 x 24 switch; search field 776 x 38 with placeholder `Search memory`; then the list, items 776 wide, 16 apart |
| 04 | Items: `Founder building Isonomic, a cloud for autonomous agents` / `From a chat · Nov 2`; `Prefers tables for comparisons` / `From a chat · Nov 12`; `Design work uses [third-party service] naming: PascalCase components, snake_case layers` / `From "Isonomic UI buildout" · Nov 14` (two lines, 80 high); `Keeps a faceless public presence` / `You added this · Oct 30`; `Teal is the Isonomic accent colour` / `From "Agentic UI design reference" · today` |
| 05 | Group label `Waiting for your OK · 2` (14/20 grey), then Pending items `Wants dark sections behind light screens in [third-party service]` and `Uses black, not blue, for primary actions`, both with source `Suggested from "Isonomic UI buildout" · today` and buttons `Dismiss`, `Save`. Group label `Saved`, then the first three items of frame 04 |
| 06 | State label `Paused`, switch off. Notice row 776 x 38 (pale yellow fill, yellow border, radius about 8, brown text, seen): `icon_hand` 14 px and `Memory is paused. Isonomic won't save or use memories until you turn it back on.` Items use the Paused state |
| 07 | Toast (Info) centred under the modal at x 720, y 828: `Forgot "Keeps a faceless public presence"` / `Undo` |
| 08 | User message 458 wide (text not read). Reply, then `memory_saved_note` (`104:9956`, 409 x 26): pill with `icon_brain` 14 px, `Saved to memory: "Use black, not blue, for primary actions"` 12/16 and a link-style `Manage` 12/16; padding about 9 x 5; sits 12 below the paragraph and 12 above the reply actions |

## Context and memory components

### ContextWindowCard (`101:5112`)

Description: context usage: 216-cell field coloured by part, status tray (amber at 80%), parts list with Compact. 380 wide, white, 1 px border `#E5E7EB`, radius 12, padding 16 x 14, column gap 12.

| Part | Values |
| --- | --- |
| Header row | gap 8: `Context window` 14/20 semibold `#111827`; tag `[model name] · 128k` (background `#F3F4F6`, radius 6, padding 6 x 1, 12/16 medium `#4B5563`); spacer; percentage 14/20 semibold |
| `cells` | 348 x 54 field: 6 rows of 36 cells, each 8 x 8 with radius 2, pitch 9.66. Filled in reading order by part, then empty cells `#F3F4F6` |
| Cell colours | system prompt `#9CA3AF` (gray/400), tools `#8B5CF6` (purple/500), files `#14B8A6` (teal/500), conversation `#111827` (gray/900) |
| `tray` | full width, radius 8, padding 10 x 6, 12/16 medium text, optional small button |
| `part_row` (x4) | gap 8: 8 px dot in the part colour, label 14/20 `#1F2937`, size 12/16 `#6B7280`, optional trailing part |

Part rows: `System prompt` `5k` with `icon_lock_simple` 12 px; `Tools` `12k`; `Files · 2` `15k` with `icon_caret_down` 12 px; `Conversation` with a small outline `Compact` button.

| Level | Node | Size | Percentage | Tray | Conversation size |
| --- | --- | --- | --- | --- | --- |
| Normal | `101:5109` | 380 x 294 | `42%`, dark | grey fill, `74k tokens left` (seen) | `13k` |
| Warn | `101:5110` | 380 x 308 | `84%`, `#EA580C` (orange/600) | background `#FFF7ED` (orange/50), text `#EA580C`: `Getting full · 20k tokens left`, plus a `Compact` button | `58k` |
| Compacting | `101:5111` | 380 x 294 | `42%` (seen) | grey fill, `Compacting conversation...`; conversation cells shown faded (seen) | `13k` |

In the Warn variant the cells split 10 system, 24 tools, 30 files, 117 conversation, 35 empty.

### ContextRing (`100:4529`)

Description: context usage ring for the composer and header; orange from 80%. Pill with 1 px border `#E5E7EB`, fully rounded, padding 8 x 3, gap 6; `ring` 16 px progress ring; value 12/16 medium.

| Level | Node | Ring | Text |
| --- | --- | --- | --- |
| Normal | `100:4527` | dark arc | `#4B5563` |
| Warn | `100:4528` | orange/500 `#F97316` arc | `#EA580C` |

Default value `42%`.

### MemoryItem (`101:5158`)

Description: one remembered fact with source; pending ones need approval; edit or forget. 640 wide by default (776 in place), 1 px border, radius 10, padding 12 x 10, gap 10. `icon` 16 px; text 14/20 `#111827` (wraps); source 12/16 `#6B7280` 2 px below.

| State | Node | Fill, border | Icon | Trailing controls |
| --- | --- | --- | --- | --- |
| Default | `101:5155` | none, `#E5E7EB` | `brain` | ghost buttons `icon_button_pencil_simple` and `icon_button_trash`, 28 px each |
| Pending | `101:5156` | `#FFFBEB` (yellow/50), `#FBBF24` (yellow/400) | `sparkle` | small buttons `Dismiss` (outline) and `Save` (dark `#111827`, white text) |
| Paused | `101:5157` | as Default | `brain` | as Default (frame 06 shows no visible difference from Default) |

Defaults: text `Prefers tables for comparisons`, source `From a chat · Nov 12`.

### UsageMeterCard (`101:5199`)

Description: plan usage by category with near-limit colour. 360 wide, white, 1 px border `#E5E7EB`, radius 12, padding 16 x 14, column gap 10. Not placed on these two pages; the related banner `You've used 76% of your weekly limit` appears in the agent work flow.

| Part | Values |
| --- | --- |
| Header | `Weekly usage` 14/20 semibold `#111827`; percentage `76%` 14/20 semibold `#EA580C` |
| `bar` | 8 high, radius 4, track `#F3F4F6`; segments 150 wide `#111827`, 70 wide `#6B7280`, 30 wide `#D1D5DB` |
| Legend rows | 8 px dot, label 12/16 `#4B5563`, value 12/16 `#1F2937`: `Chat` `52%`, `Work tasks` `18%`, `Scheduled` `6%` |
| Footer | `Resets Monday 9:00 AM` 12/16 `#9CA3AF` |

### Toast (`101:5177`)

Specified in `run-control-and-approvals.md`. Used here in the Info tone with the action `Undo`.

## Research screens

One session ("Confidential GPU options"). Conversation is full width; transcript 720 wide at x 216. User message in frames 01 to 05: `What are the main options for confidential computing on cloud GPUs right now?`

| # | Frame | Node | Screenshot | What it shows |
| --- | --- | --- | --- | --- |
| 01 | Research/01_SearchResultsExpanded | `97:117940` | `../images/research-01-search-results-expanded.png` | Search card open with five results; thinking row `Reading the most relevant results` `12s` |
| 02 | Research/02_LinkPreviewCards | `97:117965` | `../images/research-02-link-preview-cards.png` | Reply `Here are the best places to start reading:` and three `LinkPreviewCard` |
| 03 | Research/03_SourcesAndLiveData | `97:117984` | `../images/research-03-sources-and-live-data.png` | Live data widget, reply paragraph, sources list |
| 04 | Research/04_CitationsQueuedFollowUp | `97:118022` | `../images/research-04-citations-queued-follow-up.png` | Paragraph with inline citation chips; a follow-up sent while the run was active; second reply |
| 05 | Research/05_CitationHoverCard | `97:118068` | `../images/research-05-citation-hover-card.png` | Same paragraph with the hover card open under the first chip |
| 06 | Research/06_CitedPassage | `109:10212` | `../images/research-06-cited-passage.png` | Sentence highlighted, citation chip, quoted passage card |

### Text and regions per screen

| # | Details |
| --- | --- |
| 01 | Search card (seen; same build as the work card in `agent-work-and-side-panel.md`): 720 wide, 1 px border, radius 12; header with `globe` icon, `Searched the web` 14/20 medium, `5 results` 12/16 grey, caret down; then five `SourceRow`: `Confidential computing on modern GPUs: an overview` / `research.example.org`; `Attested VMs for AI workloads` / `cloudsec.example.com`; `Benchmarking encrypted inference` / `arxiv.example.org`; `Choosing a TEE for model serving` / `engblog.example.net`; `Confidential containers project update` / `opensource.example.org` |
| 02 | `link_previews` (`97:117977`): row 720 x 90, three cards 232 wide, 12 apart. Cards: `research.example.org` / `Confidential computing on mo...` / `Covers the main hardware modes an...`; `cloudsec.example.com` / `Attested VMs for AI workloads` / `How cloud providers expose atteste...`; `arxiv.example.org` / `Benchmarking encrypted infer...` / `Measures the throughput cost of run...` (titles and snippets are cut with an ellipsis) |
| 03 | `widget_card` (`97:117994`, 720 x 178): header 44 high with a chart icon, `GPU spot price index`, `Live · updated 2 min ago` and a more button; body with two KPI tiles 160 x 94 (`H-class GPU, per hour` / `$2.41` / `3% today`; `Confidential mode premium` / `+11%` / `vs. standard`, both with an up arrow in green) and a 320 x 100 line chart with gridlines at `0`, `25`, `50`, `75`, `100`. The widget itself belongs to the inline visuals page. Reply: `There are three practical routes today: GPUs with a built-in confidential mode, attested confidential VMs that pass a GPU through, and confidential containers on top of either. Expect roughly a 5–15% throughput cost.` `sources_list` (`97:118013`, 720 x 178): 1 px border, radius 10 (seen), padding 5; label `Sources` 12/16 medium at 5,9; four `SourceRow` 710 x 36 (the first four results of frame 01) |
| 04 | `paragraph_with_citations` (`97:118033`): 16/24 text laid out word by word with 4 px gaps and 28 px line pitch: `Most providers now offer attested confidential VMs with GPU passthrough` [chip `cloudsec.example.com`] `and the throughput cost is usually in the single digits` [chip `arxiv.example.org`] `.` Chips are 18 high, centred on the 24 line. Follow-up turn (`97:118057`): caption row above the bubble, right-aligned: `icon_clock` 12 px and `Sent while Isonomic was working` 12/16 grey, 4 above the bubble `Which of these would work on Isonomic Cloud?` Second reply: `Attested confidential VMs are the closest fit: a Cell already is a microVM, so running it inside an attested VM adds hardware-backed proof that the Capsule never leaves it.` `11:02 AM` |
| 05 | `citation_hover_card` (`97:118106`): 320 x 144 at x 1002, y 239 in frame coordinates (its top is about 6 below the first chip; it overhangs the transcript's right edge). Padding 15 x 13. Row 1: `icon_globe` 14 px and `cloudsec.example.com` 12/16 grey. Title `Attested VMs for AI workloads` 14/20 medium at y 35. Excerpt (290 wide, 12/16, three lines) at y 61: `"…confidential VMs can now pass a GPU through with its own attestation report, so a verifier can check both…"`. Link `Open source ↗` 12/16 at y 115. White, 1 px border, radius about 10, shadow (seen) |
| 06 | User message 312 wide (text not read). `citing_sentence` (`109:11478`, 720 x 52): the sentence `Attested confidential VMs fit best: a Cell is already a microVM, so it can run inside one and prove the Capsule never leaves it.` with a highlight fill behind it (padding 4 x 2). `CitationChip` 12 below (138 x 18). `quoted_passage` (`109:11482`, 720 x 100, 12 below the chip): bordered card, padding 15 x 13; row 1 `icon_globe` 14 px and `Attested VMs for AI workloads · cloudsec.example.com` 12/16; row 2 a `quote_bar` 3 x 40 and the quote, 640 wide, starting 13 right of the bar: `"Confidential VMs can now pass a GPU through with its own attestation report, so a verifier can check both the VM and the device."` |

## Research components

### CitationChip (`97:19665`)

Description: inline source citation in a reply; hover shows a source card. Background `#F3F4F6` (gray/100), radius 6, padding 6 x 1, label 12/16 medium `#4B5563`; 18 high. Default label `example.org`. The label is the source domain, not a number.

### LinkPreviewCard (`97:19667`)

Description: external link preview: domain, title, snippet. 232 wide (90 high in place), white, 1 px border `#E5E7EB`, radius 10, padding 12, column gap 6. `domain_row`: `favicon` 14 px (a `globe` icon, gray/400) and the domain 12/16 `#6B7280`, 6 apart. Title 14/20 medium `#111827`, one line with ellipsis, 208 wide. Snippet 12/16 `#6B7280`, one line with ellipsis. Defaults: `example.org`, `Page title goes here`, `One or two sentences explaining why this link is relevant.`

### SourceRow (`97:19673`)

Description: numbered source in a Sources list or search results. 640 wide by default, padding 10 x 8, gap 10, radius 8, 36 high. Number 12/16 medium `#9CA3AF` in a 16 wide slot; `favicon` 14 px `globe`; title 14/20 `#111827` filling the row; domain 12/16 `#9CA3AF` at the right. Defaults: `1`, `Source title`, `example.org`. No hover variant is drawn.

## Oddities

- Context frame 02: the banner ring and the composer ring both read `42%` while the banner text says 84% full; only the colour changed.
- Context frame 03: the card still shows `42%` and `13k` for the conversation while compacting.
- The Warn card's cell split (117 conversation cells of 216, about 69k of 128k) does not match its `58k` label.
- Memory frame 05 shows three saved items where frame 04 shows five; the two pending texts differ from the note in frame 08 (`Uses black...` against `Use black...`).
- A pending memory text and several sidebar titles name outside products; written here as `[third-party service]`. The context card tag names a model; written as `[model name]`.
- Research frame 05: the hover card sits at the far right, partly outside the 720 column, not directly under the chip's left edge.
- Research frame 04 and frame 06 give two different wordings of the same answer.
- Citations use domain chips inline, while the sources list uses numbers; nothing links a chip to a number.
