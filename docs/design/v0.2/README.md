# Design reference, version 0.2

A written copy of the latest version of the Isonomic Chat design file (key `A0H5z7vRJfA8J7HYFy1aKW`), for building without access to the design file. Read on 15 Nov 2026. The design file was only read, never edited.

Version 1 of the product is plain streamed chat: home, a thread with a composer, streamed Markdown replies, stop and continue, titles, a chat list in a sidebar, a model picker, first-run and empty states, error states. Everything else the design shows is recorded and marked "outside version 1".

## What is in this folder

| File | What it holds |
| --- | --- |
| `README.md` | this map, how the latest version was identified, the page index, the differences from the older docs |
| `screen-index-part-1.md` and following | every 0.2 screen: node id, name, size, state, picture |
| `tokens.md` | colours, type, radii, spacing, shadows |
| `components.md` | every 0.2 component, marked needed for version 1 or outside it |
| `screens/app-shell.md`, `screens/sidebar.md` | window frame, sidebar, collapsed rail, header |
| `screens/home.md` | home and its variants |
| `screens/composer.md`, `screens/composer-part-2.md` | composer, its footer and every composer menu |
| `screens/model-picker.md` | the model picker (two designs) |
| `screens/thread.md` | thread layout |
| `screens/thread-states.md`, `screens/thread-states-part-2.md` | streaming, thinking, stopped, edit, retry, feedback, run status |
| `screens/message-content.md` | Markdown inside a reply |
| `screens/first-run-and-system-states.md` | onboarding, page not found, usage warnings, empty state, toast |
| `screens/search-and-menus.md` | search palette, account menu, shortcuts dialog, shared menu parts |
| `screens/basic-chat-page.md` | the one 0.2 page that still uses the older shell |
| `screens/agent-work-and-side-panel.md`, `screens/run-control-and-approvals.md`, `screens/context-memory-and-research.md`, `screens/attachments.md` | outside version 1; recorded so the design is not lost |
| `images/` | one picture per screen, plus component sheets (`components-*.png`) |

Pictures are exports of the design and show its sample text as drawn, including names of outside products and models. The written files replace those names with `[model name]`, `[third-party service]` or `[older name]`. Build from the written files; use the pictures only to check.

## How the file is versioned

The design file is organised as pages. The page list reads, in order:

1. A divider page named "VERSION 0.2" (`97:2`).
2. Nineteen pages whose names end in "0.2".
3. A blank divider.
4. A divider page named "VERSION 0.1" (`3:5219`).
5. Sixteen pages whose names end in "0.1", plus one reference page of an outside product.
6. A blank divider and a "Scrap" page (`0:1`).

Version 0.2 is the latest. How this is known:

- No page or divider carries a version above 0.2.
- The 0.2 block is listed first, above the 0.1 block.
- Every 0.2 page has a 0.1 page of the same name, except three that exist only in 0.2: "Run Control and Approvals", "Context and Memory", "Home, Onboarding and Library".
- Sections inside 0.2 pages are named "... / Flow v2"; the repository's older read cites "Flow v1".
- Node ids on 0.2 pages start at 97 and go up to 109; ids on 0.1 pages run from 3 to 53. Ids are given out in order of creation, so the 0.2 content is newer.
- A memory note drawn inside the file says: "Versions are kept per whole flow as version pages."

The linked node `97:51083` is the page "Artifacts 0.2" itself. It sits in the 0.2 block and its ids are in the 0.2 range, so it belongs to version 0.2. It is an artifacts page, not a chat screen: the link identifies the version, and the version 1 screens are on other 0.2 pages (below).

The section the repository read before, `19:992` "Start a Basic Chat / Flow v1", is not on any 0.2 page: the "Start a Basic Chat 0.2" page holds only sections `97:22305` and `97:22356`. Its id range and name place it in version 0.1. It was not re-read.

## Pages of version 0.2

"V1" says whether the page holds screens version 1 needs.

| Page | Node | Screens | V1 | Written up in |
| --- | --- | --- | --- | --- |
| Start a Basic Chat | `97:22304` | 6 | partly (older shell, see below) | `screens/basic-chat-page.md` |
| Navigation, Sidebar and Search | `97:25384` | 16 | yes | `screens/app-shell.md`, `sidebar.md`, `search-and-menus.md`, `home.md` |
| Composer and Session Setup | `97:29007` | 21 | yes (composer, model picker); menus mostly outside | `screens/composer.md`, `composer-part-2.md`, `model-picker.md` |
| Session Lifecycle | `97:40979` | 16 | yes | `screens/thread.md`, `thread-states.md`, `thread-states-part-2.md` |
| System States | `97:126642` | 3 | yes | `screens/first-run-and-system-states.md` |
| Home, Onboarding and Library | `101:60682` | 10 | partly (onboarding, start page) | `screens/home.md`, `first-run-and-system-states.md` |
| Run Control and Approvals | `101:60680` | 14 | only the run status bar | `screens/run-control-and-approvals.md` |
| Agentic Work and the Side Panel | `97:44917` | 14 | no | `screens/agent-work-and-side-panel.md` |
| Context and Memory | `101:60681` | 8 | no | `screens/context-memory-and-research.md` |
| Research and Web Search | `97:117938` | 6 | no | `screens/context-memory-and-research.md` |
| Inline Visuals and Widgets | `97:76658` | 44 | only `Markdown/44_TableCodeMath` | `screens/message-content.md` |
| Artifacts | `97:51083` | 34 | no | index only |
| Projects | `97:87205` | 17 | no | index only |
| Customize: Connectors, Skills and Plugins | `97:91329` | 14 | no | index only |
| Scheduled Tasks | `97:95802` | 13 | no | index only |
| Session Management | `97:119518` | 8 | not decided (rename, delete, share of a chat) | index only |
| Settings | `97:121605` | 13 | no | index only |
| Devices and Phone | `97:124264` | 9 | no | index only |
| Components | `97:3` | 27 sheets | yes | `components.md` |

266 screens in all. "Index only" pages have a row and a picture per screen in the screen index, and no written layout.

## What 0.2 changes against the repository's docs

The repository's `docs/name-map.md` and `docs/design-diffs.md` describe version 0.1 (five `ChatView` frames). Against them:

### Changed or new

| # | Older docs say | Version 0.2 shows |
| --- | --- | --- |
| 1 | Shell: a 272 px `chat_sidebar` with a 40 px `app_rail`, an 848 px chat panel, an always-open 320 px `ContextSidebar` | A 288 px `AppSidebar` that collapses to a 56 px `AppSidebarRail`, and a main panel of 1152 (1384 collapsed). No always-open context panel; side panels open per task and are outside version 1 |
| 2 | Sidebar holds collapse, new chat, a chat list, saved prompts, pinned models, Projects | Wordmark and collapse button; nav items New chat, Search, Projects, Artifacts, Scheduled, Customize; a "Pinned" group and a "Recents" group, both collapsible; chat rows with a status dot (Running, Unread, Attention) and a hover "more" button; an account row pinned at the bottom |
| 3 | `PromptComposer` 800 x 144 with Add files, Voice, model selector and Send inside it | `Composer` 720 x 96 (radius 16, grows by line) with a plus button, dictation, and one round button that is voice when empty, send when filled, stop while working. The model control moved out, into a `ComposerFooter` row under the composer, drawn on home only |
| 4 | Home: greeting with the logo, composer, five `SuggestionChips` | Home: a display-serif greeting 34 / 42 with a sparkle icon, composer, footer. No chips on the plain home; suggestion cards appear only on the start-page variant |
| 5 | Thread "arrives with the bindings"; `ChatMessage` rows with avatar, name and time for both roles; bodies in the display serif | The thread is fully drawn: a right-aligned grey user bubble, an unboxed reply across a 720 column, 16 / 24 interface sans, a block cursor while streaming, a thinking row, a "You stopped this response" note, inline edit of a user message, a version pager, a feedback dialog. No avatars in messages |
| 6 | `MessageActions`: copy, regenerate, version switch, rating | `ReplyActions`: copy, read aloud, thumbs up, thumbs down, retry, then the time. User message actions (time, retry, edit, copy) show on hover |
| 7 | Accent blue for Send and the selected tab | No blue anywhere in the 0.2 shell. Send is `gray/900`; selection is a grey fill |
| 8 | `ChatHeader` 54 high: "New chat" muted, or title, caret, Share | `SessionHeader` 52 high, no bottom border. Kind Home: only an incognito icon button at the right. Kind Session: title with caret, a device indicator, a panel toggle, Share |
| 9 | `ModelSelector` is a labelled button in the composer | A full model picker menu, drawn twice: a 300 px menu of two-line rows with Effort and More models submenus, and a later 380 px menu with speed, quality and cost marks, a thinking toggle and a thinking budget bar |
| 10 | Disclaimer under the docked composer (older wording) | "Isonomic is AI and can make mistakes. Please double-check responses." |
| 11 | `ThinkingIndicator` with orbs | `ThinkingRow` (collapsed, expanded summary, step log) and a separate `ThinkingState` block. The orbs are not used on any screen of the new shell |
| 12 | Nothing on first run, search, shortcuts, usage limits | Onboarding steps, a setup checklist, a search palette, a keyboard shortcuts dialog, usage warning banners, page not found, `EmptyState`, `Toast` |

### Lines in the older docs that 0.2 makes wrong

| Where | Line | Why it is wrong now |
| --- | --- | --- |
| `name-map.md` intro | names pages "Start a Basic Chat 0.1" and "Components 0.1" as the source | the source is the 0.2 block |
| `name-map.md` | `ChatSidebar`, `ContextSidebar`, `ChatHeader`, `PromptComposer`, `ChatMessage`, `MessageActions`, `ThinkingIndicator`, `ChatGreeting`, `SuggestionChips`, `IconButton`, `AppRailItem` rows | replaced; see the last table of `components.md` |
| `name-map.md` | "`Avatar` ... owner `Message`" | messages have no avatar in 0.2; the only avatar is in the sidebar account row |
| `design-diffs.md` header | cites section `19:992`, frames `ChatView/01` to `05` | version 0.1 |
| `design-diffs.md` D1 | serif for message bodies | bodies are interface sans 16 / 24; the serif is only for the wordmark, greeting, large titles and math |
| `design-diffs.md` D3, H3 | blue Send, blue at 40% when disabled | no blue; the design now agrees with what was built (ink) |
| `design-diffs.md` D5 | "Body and sidebar rows 14 px; composer text 16 px" | sidebar rows 14; message bodies and composer 16 |
| `design-diffs.md` S1, S2 | 272 px sidebar with a 40 px rail; rail holds collapse and new chat | 288 px sidebar, 56 px rail when collapsed; New chat is a nav item |
| `design-diffs.md` S3 | saved prompts and pinned models in the sidebar | gone; a "Pinned" group holds pinned projects and chats |
| `design-diffs.md` S5 | avatar at the foot of the rail, not shown | an account row with name, plan and menu is part of `AppSidebar` |
| `design-diffs.md` S6 | first row has a logo and a pin | rows have a leading icon by kind and a trailing status dot; no logo, no pin |
| `design-diffs.md` S8 to S11 | context panel, its tabs, connections list, context rows | the always-open context panel does not exist in 0.2 |
| `design-diffs.md` H1, H2 | composer toolbar with model selector | see change 3 |
| `design-diffs.md` H4, H5 | greeting mark; five chips | sparkle icon; no chips |
| `design-diffs.md` Thread | "arrives with the bindings" | fully drawn; see `screens/thread.md` |
| `design-diffs.md` Icon matches | 16 icons | 0.2 uses many more; each screen file lists its icons |

### Still true

- D2 (the design file's icon set differs from Isonomic UI's), D4 (the design's grey scale) and D6 (light only) still hold.
- S4 still holds in spirit: Projects is shown, and is outside version 1. The same now goes for Artifacts, Scheduled and Customize.

## Open points for the owner

1. The page "Start a Basic Chat 0.2" still uses the version 0.1 shell and components (272 px sidebar, 320 px context sidebar, 800 x 144 composer, chips, blue streaming dot). Every other 0.2 page uses the new shell. This reference treats the new shell as version 0.2 and records the older page in `screens/basic-chat-page.md`. Its captions are the only place the file states Enter to send, Shift+Enter for a new line, and when a title appears.
2. Two model picker designs exist. The 380 px one has later node ids; that it supersedes the other is an inference.
3. The design does not show: an error inside the transcript, continue after stop (only retry), auto-scroll or a jump-to-latest button, a thread with more than one turn, loading an existing thread, narrow or wide window behaviour (every screen is 1440 x 900), focus rings, or dark mode. The full list is in the "Not shown" section of `screens/thread-states.md`.
4. The frames named Stopped, ToolError and Done on the run control page all place the status bar in its Running variant; the Stopped and Error variants exist only on the component sheet.
5. Both approval variants draw "Allow once" as the filled button and "Deny" as outline. Rule 7 of this repository (deny-safe) is unchanged; the difference is recorded in `screens/run-control-and-approvals.md`.
6. The status of incognito, the Pinned group, search, the account row and the context percentage chip in version 1 is not decided; each is flagged where it is described.

## How it was read, and limits

- Structure came from the design file's layer metadata for all nineteen 0.2 pages. Sizes, colours, type and text for the version 1 screens came from the design file's per-node code export, read on components and overlays rather than whole screens.
- The design tool limited the number of calls while this was read. Where a value came from a picture or from layer geometry instead of the export, the screen file says "seen" or names the gap.
- Screen states on "index only" pages are taken from frame and layer names and were mostly not checked against the pictures.
- Four component sheets are wider or taller than 1600 px and were scaled down (Session, Shell, Run and Approvals, Workspace and Context). The loose component frames are drawn on a dark canvas, so the two orb sheets are nearly unreadable.
- No animation data was read.

## Pictures

The screenshots of the design file are not in this repository: their sample text shows names this repository must not carry, and they are large. The orchestrator keeps them. Every layout here is written out in words and numbers so it can be built without them; where a value was read from a picture it is marked "seen".
