# Page "Start a Basic Chat 0.2": the older shell

Page node `97:22304` of the design file. This page draws the first chat flow in a different shell from every other 0.2 page. This file records both shells side by side. It does not decide which one wins.

Screenshots: `../images/basic-chat-01-new-chat.png`, `../images/basic-chat-02-composing.png`, `../images/basic-chat-03-thinking.png`, `../images/basic-chat-04-streaming.png`, `../images/basic-chat-05-complete.png`, `../images/basic-chat-00-chat-screenshot.png`.

Values are design pixels at 1x. Type is size / line height / weight / face.

## Frames on the page

All frames are 1440 x 1024 (the other 0.2 pages use 1440 x 900).

| Frame | Node | Section |
| --- | --- | --- |
| `ChatView/01_NewChat` | `97:22357` | "Start a Basic Chat / Flow v2" (`97:22356`) |
| `ChatView/02_Composing` | `97:22367` | same |
| `ChatView/03_Thinking` | `97:22377` | same |
| `ChatView/04_Streaming` | `97:22389` | same |
| `ChatView/05_Complete` | `97:22401` | same |
| `Chat-Screenshot` | `97:22306` | "Chat Screenshots v3" (`97:22305`) |

## Shell layout (frames 01 to 05)

| Region | Node (frame 01) | x | Size | Notes |
| --- | --- | --- | --- | --- |
| `chat_sidebar` (instance) | `97:22358` | 0 | 272 x 1024 | a 40 wide icon rail plus a list; shows items named after models, "Projects", "Chats" |
| `chat_panel` | `97:22359` | 272 | 848 x 1024 | |
| `chat_header` (instance of `ChatHeader` `97:27`) | `97:22360` | 0 in panel | 848 x 54 | states Untitled and Titled; bottom border line visible |
| `empty_state` (frames 01, 02) | `97:22361` | 0, y = 54 | 848 x 970 | |
| `message_list` (frames 04, 05) | `97:22393` / `97:22405` | 0, y = 54 | 848 x 790 | |
| `composer_dock` (frames 04, 05) | `97:22396` / `97:22408` | 0, y = 844 | 848 x 180 | |
| `context_sidebar` (instance) | `97:22365` | 1120 | 320 x 1024 | always shown: tabs "Context", "Tools", "More"; sections "Files", "Connections" with four `[third-party service]` rows |

Widths: 272 + 848 + 320 = 1440. Frame 03's children were not listed by the design file (collapsed in the page listing); the screenshot shows the same layout as frame 04.

## Empty state (frames 01 and 02)

`empty_state`: vertical stack, centred both ways, gap 28, padding-bottom 96.

| Child | Node | Position in `empty_state` | Size | Spec |
| --- | --- | --- | --- | --- |
| `chat_greeting` (`ChatGreeting` `97:19465`) | `97:22362` | 254, 302 | 340 x 36 | row, gap 12: `logo` 28 x 28, then heading "Good morning, Omar" 30 / 36 / regular / display serif, `#111827` (`gray/900`). Note: "Empty-state greeting, time-of-day aware. Disappears on first send." |
| `prompt_composer` (`PromptComposer` `97:73`) | `97:22363` | 24, 366 | 800 x 144 | see below |
| `suggestion_chips` (`SuggestionChips` `97:19468`) | `97:22364` | 171.5, 538 | 505 x 34 | row, gap 8, five chips |

Chips, in order (each a white button: border 1 `#D1D5DB` `gray/300`, radius 6, small shadow, padding 11 left / 13 right / 9 top and bottom, icon 16 + gap 8 + label 14 / 16 / medium / interface sans `#374151` `gray/700`):

| # | Layer | Icon | Label |
| --- | --- | --- | --- |
| 1 | `chip_write` | `pencil-simple-line` | "Write" |
| 2 | `chip_learn` | `graduation-cap` | "Learn" |
| 3 | `chip_code` | `code` | "Code" |
| 4 | `chip_life` | `coffee` | "Life stuff" |
| 5 | `chip_brainstorm` | `lightbulb` | "Brainstorm" |

Component note: "Starter categories under the empty-state composer. Each opens a short list of example prompts; picking one fills the composer."

## PromptComposer (`97:73`, 800 x 144)

States: Empty (`97:74`), Filled, Streaming. Component note: "Prompt input. Empty: send disabled. Filled: send enabled, Enter sends, Shift+Enter adds a line. Streaming: send becomes Stop; typing stays enabled."

| Part | Spec |
| --- | --- |
| Box | white, border 1 `#D1D5DB` (`gray/300`), radius 6, two-layer drop shadow (0 4 3 at 10% black, 0 2 2 at 6% black), padding-bottom 12 |
| `textarea` | 94 high, text inset 9 top and bottom, 13 left and right; placeholder 16 / 24 / regular / interface sans, `#6B7280` (`gray/500`) |
| Placeholder text | "How can I help you today?" on the empty screen; "Reply..." once a thread exists (from the screenshots) |
| `toolbar` | 36 high, padding 12 left and right, gap 8, centred vertically |
| Toolbar, left | `attach_button`: `paperclip` icon + "Add files"; `voice_button`: `microphone` icon + "Voice" (both white bordered buttons like the chips) |
| Toolbar, right | spacer, then `ModelSelector` (`3:1427`): model logo 16 + "[model name]" + `caret-down` 16; then `send_button` |
| `send_button` | fill `#0161FE` (`blue/600`), white label "Send message" 14 / 16 / medium, radius 6, padding 13 / 9; opacity 40% when empty |
| Streaming | the send button is replaced by a white bordered button with a filled square icon and the label "Stop" (from the screenshot of frames 03 and 04) |

## Message list (frames 03 to 05)

`message_list`: vertical stack, gap 24, padding 24 left and right, 16 top and bottom. Messages are 800 wide.

`ChatMessage` (`97:41`): properties `role` (User, Assistant), `state` (Thinking, Streaming, Complete), `author`, `timestamp`, `body`, `showActions`. Note: "One turn in the MessageList. role: User | Assistant. state: Thinking (before first token), Streaming, Complete. message_actions is always on for Assistant/Complete and hover-only for User."

| Part | Spec |
| --- | --- |
| Row | avatar 40 x 40 on the left (round, radius 20; a photo for the user, the model logo for the assistant), gap 12, then `content` |
| `content` | vertical stack, gap 8 |
| `meta` | row, gap 12: author 14 / 20 / medium / interface sans `#111827` (`gray/900`); timestamp 14 / 20 / regular `#6B7280` (`gray/500`) |
| Author text | "You" for the user; "[model name]" for the assistant |
| Timestamp | "10:14 AM" on both |
| Body | 14 / 20 / regular / display serif, `#374151` (`gray/700`); both roles left aligned, no bubble |
| `MessageActions` (`97:17`) | row, gap 2, ghost icon buttons 28 x 28 with 16 icons. Assistant: copy, thumbs-up, thumbs-down, retry. User: edit, copy (hover only). No read-aloud button and no time in the row |

Sizes in frames: `user_message` 800 x 48 at (24, 16); `assistant_message` 800 x 112 (streaming) or 800 x 220 (complete) at (24, 88).

Texts: user "Explain how public-key cryptography works, in plain terms." Assistant (complete), three paragraphs: "Public-key cryptography gives you two keys that work as a pair: a public key you can hand to anyone, and a private key you never share." / "Anything locked with your public key can only be opened with your private key, so someone can send you a secret without the two of you ever agreeing on a password first. It works in reverse too: when you sign something with your private key, anyone holding your public key can check that it really came from you and was not altered." / "The math behind it is easy to do in one direction and impractical to undo, which is why publishing the public key gives nothing away about the private one."

- Thinking (frame 03, screenshot): under the assistant's name line, a small faint dotted orb and the grey text "Thinking..." (`ThinkingIndicator` `97:19462`; not read in detail).
- Streaming (frame 04, screenshot): partial text, then a small blue filled dot (about 8 across) on its own line under the text, at the left edge of the body. The lifecycle page uses a dark 8 x 18 bar instead.
- Header: "New chat" with no Share button in frames 01 to 03; the generated title "Public-key cryptography explained" with a caret and a "Share" button from frame 04.

## Composer dock (frames 03 to 05)

`composer_dock` 848 x 180: `prompt_composer` 800 x 144 at (24, 0); `disclaimer` at y = 152, 365 x 16, centred, small grey text: "Isonomic Chat can make mistakes. Please double-check responses." Heights: 144 + 8 + 16 + 12 = 180.

## Captions under the frames

Each is 14 / 20 / regular / interface sans, `#6B7280` (`gray/500`), 24 below its frame.

| Node | Under | Text |
| --- | --- | --- |
| `97:22366` | 01 | "Greeting and composer centred. Send is disabled until there is text. Chips open example prompts." |
| `97:22376` | 02 | "Text entered, Send enabled. Enter sends, Shift+Enter adds a line." |
| `97:22388` | 03 | "Message posts, composer docks to the bottom, Send becomes Stop. The chat appears in the sidebar as “New chat”." |
| `97:22400` | 04 | "Reply streams in. A generated title replaces “New chat” in the header and sidebar; Share appears." |
| `97:22412` | 05 | "Stop reverts to Send (disabled until text). Actions sit under the reply; edit and copy show on hover over your message." |

These captions are the only place the file states this behaviour in words: when Send is enabled, the Enter and Shift+Enter keys, when the title is generated, and when Share appears.

## `Chat-Screenshot` (`97:22306`)

Same three-column shell: `Chat Sidebar` 272, a middle column 848 ("Chat / Conversation and Composer Flow": a conversation viewport 848 x 859 over a composer footer 848 x 165 holding the 800 x 144 composer at x = 24), and `ContextSidebar` 320. It shows a longer reply with metric cards and numbered findings. It is a still picture of an analysis answer, outside version 1.

## The two shells compared

| Item | This page (older shell) | Every other 0.2 page |
| --- | --- | --- |
| Frame size | 1440 x 1024 | 1440 x 900 |
| Left sidebar | `chat_sidebar`, 272 wide | `app_sidebar`, 288 wide |
| Main column | `chat_panel`, 848 wide | `main_panel`, 1152 wide |
| Header | `chat_header`, 54 high | `session_header`, 52 high |
| Right column | `context_sidebar`, 320 wide, always on | none by default; side panels open on demand |
| Composer | `PromptComposer`, 800 x 144, radius 6, labelled buttons, blue "Send message" | `Composer`, 720 x 96, radius 16, icon buttons, dark round send / stop |
| Composer inset | 24 from the panel edges | centred 720 column (x = 216) |
| Model picker | inside the composer toolbar | not in the thread composer (see `model-picker.md`) |
| Messages | avatar + name + time rows, both left aligned, body 14 / 20 display serif | user bubble right aligned, reply unboxed, body 16 / 24 interface sans |
| Reply actions | copy, thumbs-up, thumbs-down, retry | copy, read aloud, thumbs-up, thumbs-down, retry, time |
| Streaming mark | small blue dot | dark bar 8 x 18 |
| Thinking mark | orb + "Thinking..." | `ThinkingRow` with label, seconds and caret |
| Empty screen | greeting + composer + `suggestion_chips` | greeting + composer (see `home.md`) |
| Disclaimer | "Isonomic Chat can make mistakes. Please double-check responses." | "Isonomic is AI and can make mistakes. Please double-check responses." |
| Dock height | 180 | 132 |

## Read notes

- `ChatHeader` (`97:27`), `ThinkingIndicator` (`97:19462`) and the Filled and Streaming variants of `PromptComposer` were described from the screenshots and the page listing, not from styled reference output: the design file's read limit was reached.
- The sidebar in these frames lists items named after models and an older product name; they are omitted here.
