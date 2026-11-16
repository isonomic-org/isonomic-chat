# Screen index, part 2 of 4

Every screen of version 0.2: node id, frame name, size, the state it shows, and its picture. States on pages outside version 1 come from frame and layer names and were mostly not checked against the pictures. See `README.md` for the page list.

## Agentic Work and the Side Panel (`97:44917`)

14 rows. Pictures are in `images/`.

| Node | Frame | Size | State shown | Image |
| --- | --- | --- | --- | --- |
| 97:44919 | AgentWork/01_PanelEmpty | 1440x900 | Session with the right side panel open and still empty, before any work has started | agent-work-01-panel-empty.png |
| 97:44937 | AgentWork/02_ToolsAndSearch | 1440x900 | Run in progress: expanded work card with four finished tool rows (search, fetch, read), status line; panel shows Progress and Context | agent-work-02-tools-and-search.png |
| 97:44974 | AgentWork/03_WorkSummaryDone | 1440x900 | Work finished: collapsed work card "Used 6 tools · 48s", reply below, panel shows Progress, Outputs, Context | agent-work-03-work-summary-done.png |
| 97:45013 | AgentWork/04_ToolCallDetail | 1440x900 | Work card expanded to show the detail of one tool call (input and result) | agent-work-04-tool-call-detail.png |
| 97:45055 | AgentWork/05_QuestionCard | 1440x900 | Agent pauses with a question card of numbered answer options above the composer | agent-work-05-question-card.png |
| 97:45081 | AgentWork/06_QuestionHoverUsage | 1440x900 | Question card with four numbered options (second hovered), "Something else" and Skip; weekly usage limit banner under it | agent-work-06-question-hover-usage.png |
| 97:45108 | AgentWork/07_AnsweredThinking | 1440x900 | Question answered; agent is thinking again and the run continues | agent-work-07-answered-thinking.png |
| 97:45139 | AgentWork/08_PanelProgressOutputsContext | 1440x900 | Mid-run: file card, "Using Isonomic Docs" work card, artifact card; panel lists Progress steps, Outputs, Context | agent-work-08-panel-progress-outputs-context.png |
| 97:45173 | AgentWork/09_TaskCompleteSummary | 1440x900 | Task complete: file card, artifact card and a short summary reply; all panel sections filled | agent-work-09-task-complete-summary.png |
| 97:45205 | AgentWork/10_WorkLogExpanded | 1440x900 | Work card expanded into the full step-by-step work log | agent-work-10-work-log-expanded.png |
| 97:45245 | FilePreview/11_Open | 1440x900 | File preview pane (560 wide) open beside a narrowed chat; rendered view of palette-notes.md, Preview tab active | agent-work-11-open.png |
| 97:45287 | FilePreview/12_SaveMenu | 1440x900 | File preview with the Save button menu open (four items, separator, one more item) | agent-work-12-save-menu.png |
| 97:45336 | FilePreview/13_CodeView | 1440x900 | File preview switched to the Code tab: raw source with line numbers in the mono face | agent-work-13-code-view.png |
| 97:45380 | AgentWork/14_AllStepsChecked | 1440x900 | Task list card "4 of 4 done" with every step checked, then file card, artifact card and reply | agent-work-14-all-steps-checked.png |

## Context and Memory (`101:60681`)

8 rows. Pictures are in `images/`.

| Node | Frame | Size | State shown | Image |
| --- | --- | --- | --- | --- |
| `104:4881` | `ContextWindow/01_Panel` | 1440 x 900 | Session with side panel (412 wide) showing ContextWindowCard and the automatic compaction note | `context-memory-01-panel.png` |
| `104:5711` | `ContextWindow/02_NearlyFull` | 1440 x 900 | Banner above the composer: ContextRing, "Context is 84% full. Compact older turns to keep going.", 2 buttons | `context-memory-02-nearly-full.png` |
| `104:6312` | `ContextWindow/03_Compacting` | 1440 x 900 | Compacting: side panel card shows "Compacting conversation...", transcript shows a summarizing chip, composer shows stop | `context-memory-03-compacting.png` |
| `104:7759` | `MemoryPanel/04_List` | 1440 x 900 | Settings modal, Memory sub-page of Your Isonomic: toggle On, search field, 5 MemoryItem | `context-memory-04-list.png` |
| `104:8142` | `MemoryPanel/05_PendingApproval` | 1440 x 900 | Memory page with "Waiting for your OK · 2" (2 MemoryItem) above "Saved" (3 MemoryItem) | `context-memory-05-pending-approval.png` |
| `104:8545` | `MemoryPanel/06_Paused` | 1440 x 900 | Memory page with toggle Paused and a notice row: memory is paused, nothing saved or used; 5 MemoryItem | `context-memory-06-paused.png` |
| `104:8991` | `MemoryPanel/07_ForgetUndo` | 1440 x 900 | Memory page after forgetting an item: 4 MemoryItem left and a Toast (360 x 40) at the bottom | `context-memory-07-forget-undo.png` |
| `104:9373` | `MemoryInChat/08_SavedNote` | 1440 x 900 | Assistant reply with an inline "Saved to memory: ..." note and a Manage link above the reply actions | `context-memory-08-saved-note.png` |

## Research and Web Search (`97:117938`)

6 rows. Pictures are in `images/`.

| Node | Frame | Size | State shown | Image |
| --- | --- | --- | --- | --- |
| 97:117940 | Research/01_SearchResultsExpanded | 1440x900 | "Searched the web" card expanded to five numbered results with domains; reading-results status row | research-01-search-results-expanded.png |
| 97:117965 | Research/02_LinkPreviewCards | 1440x900 | One-line reply followed by a row of three link preview cards | research-02-link-preview-cards.png |
| 97:117984 | Research/03_SourcesAndLiveData | 1440x900 | Live data card (two KPI tiles, price chart), reply paragraph, Sources list of four rows | research-03-sources-and-live-data.png |
| 97:118022 | Research/04_CitationsQueuedFollowUp | 1440x900 | Reply with inline citation chips, then a follow-up marked "Sent while Isonomic was working" | research-04-citations-queued-follow-up.png |
| 97:118068 | Research/05_CitationHoverCard | 1440x900 | Reply with citation chips and a 320 px hover card: site, title, quote, "Open source" link | research-05-citation-hover-card.png |
| 109:10212 | Research/06_CitedPassage | 1440x900 | Highlighted citing sentence, citation chip, and the quoted source passage card below | research-06-cited-passage.png |

## Inline Visuals and Widgets (`97:76658`)

44 rows. Pictures are in `images/`.

| Node | Frame | Size | State shown | Image |
| --- | --- | --- | --- | --- |
| 97:76660 | InlineVisual/01_DiagramStreaming | 1440x900 | Diagram widget still streaming: three nodes drawn (Client, Engine, Cell), the rest shown as a skeleton | inline-visuals-01-diagram-streaming.png |
| 97:76692 | InlineVisual/02_LineChart | 1440x900 | Line chart widget in a reply: two series, Jun to Nov axis, legend, Chart / Table toggle on Chart | inline-visuals-02-line-chart.png |
| 97:76741 | InlineVisual/03_ChartTableView | 1440x900 | Same widget toggled to Table: Month, Tokens (B), Cost ($k) with six rows | inline-visuals-03-chart-table-view.png |
| 97:76791 | InlineVisual/04_ChartTooltip | 1440x900 | Line chart with a hover guide and point; tooltip reads October, Tokens 61B, Cost $33k | inline-visuals-04-chart-tooltip.png |
| 97:76844 | InlineVisual/05_DonutKpis | 1440x900 | Donut chart with $12.4k in the centre and four KPI tiles: GPU nodes, Storage, Egress, Control plane | inline-visuals-05-donut-kpis.png |
| 97:76883 | InlineVisual/06_WidgetMenu | 1440x900 | Chart widget with its header overflow menu open | inline-visuals-06-widget-menu.png |
| 97:76938 | InlineVisual/07_Flowchart | 1440x900 | Flowchart widget: Tool call, "Risky?" decision, Ask user, Run, with No and Denied branches | inline-visuals-07-flowchart.png |
| 97:76979 | InlineVisual/08_IllustrativeDiagram | 1440x900 | Illustrative diagram: Capsule box, three broker nodes, and a Cell (microVM) box holding the agent process | inline-visuals-08-illustrative-diagram.png |
| 97:77009 | InteractiveWidget/09_Default | 1440x900 | Interactive calculator widget at defaults: sliders GPU nodes 8 and Hours per day 12, KPI result tiles | inline-visuals-09-default.png |
| 97:77045 | InteractiveWidget/10_AfterInput | 1440x900 | Same widget after dragging: GPU nodes 12, Hours per day 20, result tiles updated | inline-visuals-10-after-input.png |
| 97:77081 | InlineVisual/11_UiMockup | 1440x900 | UI mockup widget: a phone frame showing a permission prompt to send an email | inline-visuals-11-ui-mockup.png |
| 97:77107 | InputForm/12_Empty | 1440x900 | Input form widget, empty: Agent name, "What should it do?", three permission checkboxes | inline-visuals-12-empty.png |
| 97:77141 | InputForm/13_Filled | 1440x900 | Same form filled in and ready to submit | inline-visuals-13-filled.png |
| 97:77175 | ImageSearch/14_Results | 1440x900 | Image search results: a row of three image tiles under a reply paragraph | inline-visuals-14-results.png |
| 97:77200 | ImageSearch/15_Lightbox | 1440x900 | Image lightbox over a scrim: large image, caption with source, "2 / 3", left and right arrows, close | inline-visuals-15-lightbox.png |
| 97:77232 | ShoppingCard/16_Comparison | 1440x900 | Product comparison: three product columns with price and rows for Memory, GPUs, Power, Quiet | inline-visuals-16-comparison.png |
| 97:77300 | ShoppingCard/17_FeaturedPick | 1440x900 | Featured pick card: image, badge, product name, price and a short reason | inline-visuals-17-featured-pick.png |
| 97:77323 | ShoppingCard/18_ProductCarousel | 1440x900 | Product carousel on its first page: image, name and price, blurb, page dots, previous and next arrows | inline-visuals-18-product-carousel.png |
| 97:77355 | ShoppingCard/19_CarouselLastPage | 1440x900 | Product carousel on its last page (third product) | inline-visuals-19-carousel-last-page.png |
| 97:77387 | Places/20_Map | 1440x900 | Places widget: stylised map with four numbered pins above a list of place rows | inline-visuals-20-map.png |
| 97:77422 | Places/21_PlaceDetail | 1440x900 | Map with one place opened in a detail card: image, name, hours and walk time, description | inline-visuals-21-place-detail.png |
| 97:77461 | Places/22_ListWithPhotos | 1440x900 | Places as a list with photos: three rows with thumbnail, name, type and walk time, one-line note | inline-visuals-22-list-with-photos.png |
| 97:77513 | Itinerary/23_Day1 | 1440x900 | Itinerary widget with day tab pills, Day 1 selected, listing its stops | inline-visuals-23-day1.png |
| 97:77536 | Itinerary/24_Day2 | 1440x900 | Itinerary widget with Day 2 selected | inline-visuals-24-day2.png |
| 97:77558 | Places/25_OneByOne | 1440x900 | Places shown one at a time: large image, details, chips, pager dots with previous and next arrows | inline-visuals-25-one-by-one.png |
| 97:77595 | StepCard/26_Step1 | 1440x900 | Step card showing step 1 of a how-to | inline-visuals-26-step1.png |
| 97:77618 | StepCard/27_Step2 | 1440x900 | Step card advanced to step 2 | inline-visuals-27-step2.png |
| 97:77642 | StepCard/28_AllSteps | 1440x900 | Step card expanded to list all steps at once | inline-visuals-28-all-steps.png |
| 97:77665 | OptionsCard/29_Option1 | 1440x900 | Options card with an info banner ("General information, not medical advice."), first option tab and its checklist | inline-visuals-29-option1.png |
| 97:77699 | OptionsCard/30_Option2 | 1440x900 | Options card with the second option tab selected ("Strengthen") | inline-visuals-30-option2.png |
| 97:77733 | Quiz/31_HintShown | 1440x900 | Quiz question with answer options and a hint row revealed (lightbulb icon) | inline-visuals-31-hint-shown.png |
| 97:77762 | Quiz/32_AnsweredWrong | 1440x900 | Quiz after a wrong answer: options marked, explanation starting "Not quite." | inline-visuals-32-answered-wrong.png |
| 97:77791 | Flashcard/33_Front | 1440x900 | Flashcard front: "Question" label, prompt, "Click to flip", previous and next arrows | inline-visuals-33-front.png |
| 97:77816 | Flashcard/34_Back | 1440x900 | Flashcard flipped to the back showing the answer | inline-visuals-34-back.png |
| 97:77841 | Recipe/35_Card | 1440x900 | Recipe card: ingredients with a servings stepper at 2, and a four-step method | inline-visuals-35-card.png |
| 97:77886 | Recipe/36_ScaledServings | 1440x900 | Recipe card with servings stepped up to 4 and ingredient amounts scaled | inline-visuals-36-scaled-servings.png |
| 97:77931 | CookingMode/37_Cover | 1440x900 | Cooking mode overlay, cover page: recipe title, "4 steps · 20 min", progress segments, summary line | inline-visuals-37-cover.png |
| 97:77965 | CookingMode/38_StepWithTimer | 1440x900 | Cooking mode step 1 of 4 with a timer at 8:00 and a "Start timer" button | inline-visuals-38-step-with-timer.png |
| 97:78000 | CookingMode/39_TimerRunning | 1440x900 | Cooking mode step 1 with the timer running at 6:42, a Pause button and an elapsed bar | inline-visuals-39-timer-running.png |
| 97:78037 | Recipe/40_UnitsMenu | 1440x900 | Recipe ingredients with the units menu open | inline-visuals-40-units-menu.png |
| 97:78065 | TranslationDraft/41_TranslationAndDraft | 1440x900 | Translation card (English and Portuguese blocks with copy) plus an email draft card with variant tabs | inline-visuals-41-translation-and-draft.png |
| 97:78103 | TranslationDraft/42_DraftVariantBSendMenu | 1440x900 | Email draft card on its second variant with the send menu open | inline-visuals-42-draft-variant-b-send-menu.png |
| 97:78134 | Weather/43_Card | 1440x900 | Weather card: 19° and Sunny, high, low and wind line, seven-day forecast row | inline-visuals-43-card.png |
| 97:78192 | Markdown/44_TableCodeMath | 1440x900 | Plain reply content: a Markdown table, a code block and a math block | inline-visuals-44-table-code-math.png |
