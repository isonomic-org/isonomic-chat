# Thread (session) view: layout

Source: page "Session Lifecycle 0.2" (`97:40979`) of the design file. Every screen there is a 1440 x 900 frame. Reference frame for this file: `SessionView/03_Complete` (`97:41023`), screenshot `../images/lifecycle-03-complete.png`. States are in `thread-states.md`; Markdown blocks in `message-content.md`.

Values are design pixels at 1x. Colour tokens are the names the design file gives (for example `gray/900`). Type is written size / line height / weight / face.

## Frame tree

| Layer | Node (in `97:41023`) | x, y (in parent) | Size | Notes |
| --- | --- | --- | --- | --- |
| screen | `97:41023` | 0, 0 | 1440 x 900 | |
| `app_sidebar` (instance) | `97:41024` | 0, 0 | 288 x 900 | documented elsewhere |
| `main_panel` | `97:41025` | 288, 0 | 1152 x 900 | |
| `session_header` (instance) | `97:41026` | 0, 0 | 1152 x 52 | documented elsewhere; pinned |
| `session_body` | `97:41027` | 0, 52 | 1152 x 848 | |
| `conversation` | `97:41028` | 0, 0 | 1152 x 848 | fills `session_body` when no side panel |
| `transcript_scroll` | `97:41029` | 0, 0 | 1152 x 716 | the scrolling region |
| `transcript` | `97:41030` | 216, 0 | 720 x content height | the centred column |
| `composer_dock` | `97:41047` | 0, 716 | 1152 x 132 | pinned to the bottom |
| `Composer` (instance) | `97:41048` | 216, 0 | 720 x 96 | internals documented elsewhere |
| `disclaimer` (text) | `97:41049` | 382.5, 104 | 387 x 16 | centred |

Heights add up: 52 + 716 + 132 = 900.

## What scrolls, what is pinned

- Pinned: `app_sidebar` (left, 288), `session_header` (top, 52), `composer_dock` (bottom, 132).
- Scrolls: only the content of `transcript_scroll` (1152 x 716). The scroll region spans the full width of `conversation`; the 720 column is centred inside it.
- In every lifecycle frame the transcript is shorter than 716, so no frame shows scrolled content, a scrollbar, a fade under the header or above the dock, or a scroll-to-bottom control. Those are not shown in the design file.
- The reference code gives no fill for `main_panel`, `session_body`, `transcript_scroll` or `composer_dock`; the screenshot shows plain white behind all of them, with no border between transcript and dock.

## Transcript column (`transcript`)

| Property | Value |
| --- | --- |
| Width | 720, centred in 1152 (x = 216, 216 left over each side) |
| Layout | vertical stack, children aligned to the start (left), each child full width |
| Padding | 32 top, 32 bottom, 0 left and right |
| Gap between children | 24 |
| Children, in order | `user_turn`, then `assistant_message` (in frame 16 a `ThinkingState` instance sits between them as its own child, also 24 apart) |

In the reference frame: `user_turn` at y = 32 (height 78), `assistant_message` at y = 134 (32 + 78 + 24).

## User turn

`user_turn` (`97:41031`): 720 wide, horizontal row, content pushed to the right edge (justify end), aligned to the top, clips overflow. It holds one `UserMessage` instance (`97:41032`).

`UserMessage` component set `97:11440`. Variants: `state` = Default / Hover, `size` = Short / Long. Component note in the design file: "Right-aligned user bubble. Hover reveals time, retry, edit and copy. Long wraps at 560."

| Variant | Node |
| --- | --- |
| Default, Short | `97:11441` |
| Hover, Short | `97:11449` |
| Default, Long | `97:11457` |
| Hover, Long | `97:11465` |

Structure (vertical stack, gap 6, children aligned to the right):

| Part | Spec |
| --- | --- |
| `bubble` | fill `#F3F4F6` (`gray/100`); radius 18; padding 16 left and right, 10 top and bottom; no border, no shadow |
| bubble text | 16 / 24 / regular (400) / interface sans; colour `#111827` (`gray/900`) |
| Short | text on one line, bubble hugs it. Example text "Explain how public-key cryptography works, in plain terms." gives a 454 x 44 bubble (422 text + 32 padding) |
| Long | text box fixed at 528 wide and wraps; bubble is 560 wide (528 + 32); height grows by 24 per line |
| `actions` row | under the bubble, 28 high, horizontal, gap 2, centred vertically. Opacity 0 in Default, fully visible in Hover. It keeps its space when hidden |
| actions content, in order | time text, then three icon buttons: `icon_button_arrows_clockwise` (retry), `icon_button_pencil_simple` (edit), `icon_button_copy` (copy) |
| time text | 12 / 16 / regular / interface sans; `#9CA3AF` (`gray/400`); default "just now" |
| icon buttons | `IconButton` component (`97:12`), ghost: 28 x 28, radius 6, no fill, icon 16 x 16 centred |

Because the hidden actions row still takes space, a one-line user message always occupies 78 (44 bubble + 6 + 28). The visible gap from the bottom of the bubble to the first line of the reply is therefore 6 + 28 + 24 = 58.

Widths seen in screens: 454 (Short, frames 01 to 12), 476 (Markdown frame `97:78201`), 530 (frame 16, `107:12461`), 284 (frame 13). The maximum is the Long variant at 560. The edit box in frame 09 is also 560 wide.

## Assistant message

`assistant_message` (`97:41033`): 720 wide (the full column), vertical stack, children aligned left. No bubble, no fill, no border, no avatar, no name label.

| Property | Value |
| --- | --- |
| Gap between blocks | 12 (between every pair of children: paragraphs, headings, lists, tables, code, the thinking row, the stopped note, the stream cursor and the reply action row) |
| Paragraph | 16 / 24 / regular / interface sans, `#111827` (`gray/900`), width 720 |
| Heading | 18 / 28 / semibold (600) / interface sans, `#111827`, width 720 |
| Other blocks | see `message-content.md` |

Block positions in the reference frame (y inside `assistant_message`): paragraph 0 (48 high), heading 60, bullet list 100 (78 high), heading 190, paragraph 230, paragraph 290, `ReplyActions` 350. Each step is the previous height plus 12.

Text in the reference frame, in order:

1. Paragraph: "Public-key cryptography gives everyone two linked keys: a public key you can hand to anyone, and a private key you never share. What one key locks, only the other can unlock."
2. Heading: "The two jobs it does"
3. Bullets: "Encryption: anyone can lock a message with your public key, but only your private key opens it." and "Signatures: you sign with your private key, and anyone can check the signature with your public key."
4. Heading: "Why it’s safe"
5. Paragraph: "The keys are tied together by math that is easy in one direction and practically impossible to reverse, like multiplying two huge primes versus factoring the result."
6. Paragraph: "That’s why your browser, your wallet and your messaging apps can talk securely with servers they have never met before."

## Reply action row (`ReplyActions`)

Component `97:19545`. Component note: "Action row under an assistant reply: copy, read aloud, good, bad, retry, time." It is the last child of `assistant_message`, 12 below the last content block, left aligned. Always visible in the frames that show a finished reply (it is not hover-only).

| Property | Value |
| --- | --- |
| Layout | horizontal, gap 2, centred vertically, 28 high |
| Width | hugs content: 195 with "9:41 PM" (5 x 28 + 5 x 2 + 45 text); 203 in frame 16; 196 in the Markdown frame |
| Properties | `showTime` (default true), `time` (default "just now") |

Children in order:

| # | Layer | Node | Icon (16 x 16, in a 28 x 28 ghost button, radius 6) | Meaning |
| --- | --- | --- | --- | --- |
| 1 | `copy` | `97:19546` | `copy` | copy the reply |
| 2 | `speaker_high` | `97:19547` | `speaker-high` | read aloud (outside version 1) |
| 3 | `thumbs_up` | `97:19548` | `thumbs-up` | good response |
| 4 | `thumbs_down` | `97:19549` | `thumbs-down` | bad response; opens the feedback dialog |
| 5 | `arrows_clockwise` | `97:19550` | `arrows-clockwise` | retry |
| 6 | time text | `97:19551` | none | 12 / 16 / regular / interface sans, `#9CA3AF` (`gray/400`); "9:41 PM" in the reference frame |

Icon colour: the icons render mid grey in the screenshot and the node's style list carries `gray/500` (`#6B7280`); the reference code does not attach the colour to a specific layer. Hover, pressed, selected (for example a filled thumb after rating) and disabled looks of these buttons are not shown in the design file.

## Composer dock

`composer_dock` (`97:41047`): 1152 x 132, vertical stack, children centred horizontally, gap 8, padding 12 bottom, 0 elsewhere.

| Child | Spec |
| --- | --- |
| `Composer` instance | 720 x 96 at x = 216 (same edges as the transcript column), y = 0. Component set `97:11361`; `state` = Empty in finished frames, Working while a reply runs. Placeholder text in the thread: "Reply to Isonomic…" |
| `disclaimer` | text "Isonomic is AI and can make mistakes. Please double-check responses."; 12 / 16 / regular / interface sans; `#9CA3AF` (`gray/400`); one line, centred; 8 below the composer |

Heights: 96 + 8 + 16 + 12 = 132. On the Run Control page the dock grows to 184 when a `RunStatusBar` sits above the composer and `transcript_scroll` shrinks to 664 (see `thread-states-part-2.md`).

## Column widths when a side panel is open

When a side panel opens, `conversation` narrows and the panel takes the rest of `session_body` (the two always add up to 1152). `transcript_scroll` (716 high) and `composer_dock` (132 high) take the new conversation width. The column stays centred, and the `Composer` is always as wide as the column and at the same x. Numbers read from the layer geometry:

| Case | Source frame | `conversation` | Panel | Column width | Column x | Side margin |
| --- | --- | --- | --- | --- | --- | --- |
| No panel | `SessionView/03_Complete` `97:41023` | 1152 | none | 720 | 216 | 216 |
| Work side panel | `AgentWork/03_WorkSummaryDone` `97:44974` | 832 | `side_panel` 320 at x = 832 | 720 | 56 | 56 |
| File preview | `FilePreview/11_Open` `97:45245` | 592 | `file_preview_pane` 560 at x = 592 | 528 | 32 | 32 |
| Checkpoints panel | `Checkpoints/11_Panel` `102:10377` | 772 | `side_panel` 380 at x = 772 | 448 | 162 | 162 |

Notes:

- The first three rows fit one rule: column = min(720, conversation width - 64), centred.
- The Checkpoints frame does not fit that rule: at 772 wide the column is only 448 with 162 on each side. In that frame the `UserMessage` fills the whole 448 column (its bubble wraps at the column width instead of 560).
- The disclaimer keeps its natural 387 width and stays centred in the dock in all four cases.
- Side panels are outside version 1.

## Not covered here

`app_sidebar`, `session_header` and the inside of `Composer` are documented in their own files.
