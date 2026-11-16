# Thread states: reply lifecycle

Each state of a thread, in order, from page "Session Lifecycle 0.2" (`97:40979`). Layout of the column is in `thread.md`; Markdown blocks in `message-content.md`. Feedback, run control and the states outside version 1 are in `thread-states-part-2.md`.

Every frame is 1440 x 900 with the same shell (`app_sidebar` 288, `session_header` 52, column 720 at x = 216, `composer_dock` 132) and shows exactly one user turn and one reply. Screenshots follow the pattern `../images/lifecycle-<nn>-<state>.png`. Type is size / line height / weight / face.

Unless a row says otherwise the user turn is `UserMessage` Default / Short, 454 x 78, text "Explain how public-key cryptography works, in plain terms.", and the reply starts at y = 134 in the column.

## State index

| # | Frame | Node | What changes |
| --- | --- | --- | --- |
| 01 | `SessionView/01_Streaming` | `97:40981` | partial paragraph + stream cursor; composer Working |
| 02 | `SessionView/02_StreamingStructured` | `97:40997` | headings and bullets arrive styled; cursor last |
| 03 | `SessionView/03_Complete` | `97:41023` | full reply + `ReplyActions`; composer Empty |
| 04 | `SessionView/04_Thinking` | `97:41050` | only a `ThinkingRow` (Working) |
| 05 | `SessionView/05_ThinkingExpanded` | `97:41065` | `ThinkingRow` (Expanded) + summary lines |
| 06 | `SessionView/06_ThinkingLogOpen` | `97:41084` | `ThinkingRow` (Expanded) + step log + reply |
| 07 | `SessionView/07_Stopped` | `97:41134` | cut-off reply + stopped note + `ReplyActions` |
| 08 | `SessionView/08_UserMessageHover` | `97:41158` | `UserMessage` Hover: actions visible |
| 09 | `SessionView/09_EditUserMessage` | `97:41184` | bubble replaced by an inline editor |
| 10, 11 | `Feedback/...` | `97:41214`, `97:41258` | part 2 |
| 12 | `SessionView/12_RetriedReply` | `97:41310` | version pager "2 / 2" before `ReplyActions` |
| 13, 14 | image sent, attachment viewer | `97:41333`, `97:41361` | part 2 (outside version 1) |
| 15 | `SessionView/15_ThinkingStateSources` | `107:9889` | `ThinkingState` panel, Working |
| 16 | `SessionView/16_ThinkingStateDone` | `107:11915` | `ThinkingState` panel, Done + reply |

## 01 Streaming

`assistant_message` (`97:40991`), 720 x 78, stack with gap 12:

| Child | Spec |
| --- | --- |
| `paragraph` (`97:40992`) | 16 / 24 / regular / interface sans, `#111827` (`gray/900`). Text is cut mid-word: "Public-key cryptography gives everyone two linked keys: a public key you can hand to anyone, and a private key you nev" |
| `stream_cursor` (`97:40993`) | rectangle 8 wide x 18 high, fill `#111827` (`gray/900`), radius 2. It is its own block: 12 below the last block, at the left edge of the column. It is not inline at the end of the text |

No `ReplyActions`, no thinking row, no label. Blink or other motion of the cursor is not shown in the design file.

Composer while streaming: `Composer` `state` = Working (`97:11378`). Component note: "Prompt box. Empty shows voice; Filled shows send; Working shows stop while the reply runs." What differs from Empty:

| Part | Empty (`97:11362`) | Working (`97:11378`) |
| --- | --- | --- |
| Last toolbar button | `voice`: 32 x 32, radius 16 (round), fill `#F3F4F6` (`gray/100`), `waveform` icon 16 | `stop`: 32 x 32, radius 16, fill `#111827` (`gray/900`), `icon_stop` 16 (drawn light on dark) |
| Everything else | same: placeholder "Reply to Isonomic…" in `#9CA3AF` (`gray/400`), `add` button, context ring "42%", `mic` button | same; the text field stays present |

The dock, disclaimer and sizes do not change.

## 02 Streaming structured

`assistant_message` (`97:41007`), 720 x 284. Blocks arrive already styled. Order and y: paragraph 0 (48), heading "The two jobs it does" 60, bullet list 100 (two items, 78), heading "Why it’s safe" 190, one-line paragraph 230 (24 high, text not read), `stream_cursor` 266. The cursor is again a separate block under the last block.

## 03 Complete

See `thread.md` for the full spec. Reply ends with `ReplyActions` (time "9:41 PM"). Composer returns to `state` = Empty. Screenshot `../images/lifecycle-03-complete.png`.

## 04 Thinking (collapsed)

`assistant_message` (`97:41060`) holds one child: `ThinkingRow`, `state` = Working (`97:11475`), 20 high.

`ThinkingRow` component set `97:11474`. Note: "Collapsible thinking summary above a reply. Working animates the orb and counts seconds." Row: horizontal, gap 8, centred vertically.

| Variant | Node | Children in order |
| --- | --- | --- |
| Working | `97:11475` | `AgentOrb` 20 x 20 (animated); label 14 / 20 / regular, `#4B5563` (`gray/600`); seconds text "12s" 14 / 20 / regular, `#9CA3AF` (`gray/400`); `icon_caret_right` 14 x 14 |
| Done | `97:11480` | `orb` 16 x 16 (static glyph; the component list names a `lightbulb` icon); label 14 / 20 / regular, `#6B7280` (`gray/500`); `icon_caret_right` 14 x 14. No seconds text |
| Expanded | `97:11484` | `orb` 16 x 16 (static); label as Done; `icon_caret_down` 14 x 14 |

Label text: default "Checking the formats to demonstrate"; in frame 04 "Picking an analogy that holds up".

`AgentOrb` (`97:4006`): 20 x 20 dot orb, one ink colour (`gray/900`) with per-dot opacity. Its note says the preview is a 26-frame flipbook at 10 frames a second on a 2.6 s loop, that the coded component should run continuously, and that with reduced motion it holds the first frame.

The composer state in frame 04 was not read; the other in-progress frames (01, 15) show Working.

## 05 Thinking expanded (summary)

`assistant_message` (`97:41075`), 720 x 116, gap 12:

| Child | Spec |
| --- | --- |
| `ThinkingRow` Expanded (`97:41076`) | label "Thinking about the explanation", 241 x 20 |
| `thinking_summary` (`97:41077`) | 698 x 84. Left border 2 `#E5E7EB` (`gray/200`), no other border, no fill. Padding-left 16, padding 4 top and bottom. Lines stacked with gap 8. Text 14 / 20 / regular / interface sans, `#6B7280` (`gray/500`), 680 wide |

Summary lines: "The user wants plain terms, so lead with the two-key idea before any math." / "A padlock analogy works for encryption; signatures need the reverse direction, so call that out separately." / "Mention factoring only as the intuition for why it’s hard to reverse."

No reply text and no `ReplyActions` in this frame.

## 06 Thinking log open

`assistant_message` (`97:41094`), gap 12: `ThinkingRow` Expanded (`97:41095`, label "Thought for 14s", 146 wide), then `thinking_log`, then the reply (paragraph, heading, two bullets). No `ReplyActions` and no cursor in this frame.

`thinking_log` (`97:41096`), 660 x 280 (narrower than the column, left aligned):

| Property | Value |
| --- | --- |
| Box | fill `#F9FAFB` (`gray/50`), border 1 `#E5E7EB` (`gray/200`), radius 12, padding 16 left and right, 14 top and bottom |
| Steps | stacked, gap 10; each `log_step` is a row, top aligned, gap 10 |
| Step icon | `icon_check_circle` 16 x 16; the style list for the frame carries `green/600` (`#059669`) |
| Step title | 14 / 20 / medium (500) / interface sans, `#1F2937` (`gray/800`) |
| Step detail | 2 below the title; 14 / 20 / regular, `#6B7280` (`gray/500`); 600 wide |

Steps (title, then detail):

1. "Understand the ask" / "Plain-language explanation; no prior crypto knowledge assumed."
2. "Choose the frame" / "Two linked keys; one locks, the other unlocks."
3. "Cover both uses" / "Encryption and signatures run in opposite directions."
4. "Explain the safety" / "One-way math: multiplying primes vs. factoring the product."
5. "Ground it" / "Close with everyday places it shows up."

Frames 05 and 06 show two different expanded bodies (plain summary lines, boxed step log) under the same `ThinkingRow` variant. Which one opens when is not shown in the design file.

## 15 and 16: ThinkingState panel

`ThinkingState` component set `100:3242` (note: "AI working panel: live timer, searched sources (+N), thinking steps, done label."). In frames 15 and 16 it is a direct child of the transcript column (not inside `assistant_message`), 720 x 154 at y = 134, with the usual 24 gap before the reply. Component default width is 640; in the frames it stretches to 720. Screenshots `../images/lifecycle-15-thinking-state-sources.png`, `../images/lifecycle-16-thinking-state-done.png`. User text: "What are the main options for confidential computing on cloud GPUs?" (`UserMessage` 530 wide).

| Part | Spec |
| --- | --- |
| Box | fill `#F9FAFB` (`gray/50`), border 1 `#E5E7EB` (`gray/200`), radius 12, padding 12 left and right, 10 top and bottom, children stacked with gap 8 |
| `header` | row, gap 8: leading mark 16 x 16; label "Thinking" 14 / 20 / medium, `#1F2937` (`gray/800`); timer "0:08" 14 / 20 / regular, `#9CA3AF` (`gray/400`); spacer; `icon_caret_up` 14 x 14 at the right edge |
| `steps` | stacked, gap 8, padding-left 24 |
| `search_row` | row, gap 6: `icon_globe` 14; "Searched" 14 / 20 / regular, `#4B5563` (`gray/600`); two `source_chip`s; one `more_chip` |
| chips | fill `#F3F4F6` (`gray/100`), radius 6, padding 7 left and right, 2 top and bottom, text 12 / 16 / medium, `#374151` (`gray/700`). Texts: "cloudsec.example.com", "[third-party service].example.org", "+3 more" |
| `step_row` | row, gap 6: `icon_check` 14; text 14 / 20 / regular, `#4B5563`. Texts: "Compared attestation flows", "Checked throughput benchmarks" |

How the two states differ:

| | 15 Working | 16 Done (`100:3241`) |
| --- | --- | --- |
| Header mark | animated dot orb | green check circle |
| Last row | spinner ring + "Drafting the answer..." in regular weight | `done_row`: `icon_check_circle` 14 + "Finished thinking" 14 / 20 / medium, `#1F2937` |
| Reply | none | paragraph + `ReplyActions` (time "11:02 AM") |
| Composer | Working (stop button) | Empty (voice button) |

Frame 15 was read from its screenshot only (the design file returned no child layers for it). Reply text in 16: "Three practical routes today: GPUs with a confidential mode, attested confidential VMs with GPU passthrough, and confidential containers on top of either. Expect a 5–15% throughput cost." A collapsed look of this panel is not shown in the frames. Source chips come from web search, which is outside version 1.

## 07 Stopped

`assistant_message` (`97:41144`), gap 12: paragraph, heading "The two jobs it does", one bullet (the "Encryption: …" item), then:

| Child | Spec |
| --- | --- |
| `stopped_note` (`97:41151`) | 186 x 20, row, gap 6, centred vertically: `icon_stop` 14 x 14, then text "You stopped this response" 14 / 20 / regular / interface sans, `#6B7280` (`gray/500`). The style list carries `gray/400` (`#9CA3AF`), most likely the icon |
| `ReplyActions` (`97:41154`) | the normal row, all five buttons and the time, 12 below the note |

The kept text ends at a block boundary. No stream cursor. No "Continue" control anywhere in the frame; the only way forward shown is the retry button in `ReplyActions`.

## 08 User message hover

`UserMessage` `state` = Hover, `size` = Short (`97:11449`), instance `97:41167`. Same 454 x 78 box as Default; the `actions` row under the bubble goes from opacity 0 to fully visible. Row content, right aligned under the bubble, gap 2: time "just now" (12 / 16, `#9CA3AF`), then retry (`arrows-clockwise`), edit (`pencil-simple`), copy (`copy`), each a 28 x 28 ghost icon button with a 16 icon. The bubble itself does not change. Nothing else on the screen changes.

## 09 Edit user message

`user_turn` (`97:41192`) grows to 122 high and holds `edit_message` (`97:41193`) in place of the bubble; the reply below shifts down (starts at y = 178) and keeps its `ReplyActions`.

| Part | Spec |
| --- | --- |
| Box | 560 x 122, right aligned in the column (x = 160). Fill white, border 2 `#111827` (`gray/900`), radius 16, small drop shadow (0 1 1 at 5% black). Padding 14 left and right, 12 top and bottom; children stacked with gap 12 |
| `input` | text 16 / 24 / regular / interface sans, `#111827`, 530 wide: "Explain how public-key cryptography works, in plain terms, with one everyday analogy." (two lines) |
| `dialog_footer` | row, right aligned, gap 8, 34 high |
| "Cancel" | 70 x 34: fill white, border 1 `#D1D5DB` (`gray/300`), radius 6, `shadow/sm` (0 1 2 at 5% black); label 14 / 16 / medium, `#374151` (`gray/700`); padding 13 / 9 including the border |
| "Save" | 57 x 34: fill `#111827`, same radius, shadow and padding; label white |

No hover actions row while editing. A text caret, a disabled Save, and what the thread looks like after Save are not shown in the design file.

## 12 Retried reply

`assistant_message` (`97:41320`): three paragraphs (texts not read), then `reply_footer` (`97:41324`), 289 x 28: a row with gap 8 holding `version_pager` and then the normal `ReplyActions` (time "9:43 PM").

`version_pager` (`97:41325`), 86 x 28, row, gap 2:

| Child | Spec |
| --- | --- |
| `icon_button_caret_left` | ghost icon button 28 x 28, `caret-left` 16; enabled |
| text "2 / 2" | 12 / 16 / medium / interface sans, `#4B5563` (`gray/600`) |
| `icon_button_caret_right` | ghost icon button 28 x 28, `caret-right` 16; opacity 40% (disabled, already on the last version) |

The pager appears only on a reply with more than one version. Version "1 / 2" and a pager on an edited user message are not shown in the design file.

## Not shown

Version 1 needs these, and the pages read for this document do not show them. Do not guess; decide them outside the design file.

| Gap | Detail |
| --- | --- |
| Error in the transcript | no frame shows a failed reply inline (network, model or limit error), its text, or a retry there. The only error surface found is the `RunStatusBar` Error variant, and no frame uses it |
| Continue after stop | no "Continue" control. Frame 07 offers only retry. `RunStatusBar` Stopped has "Resume", but no frame shows that variant in place |
| Auto-scroll | whether the view follows the stream, and when it stops following |
| Scroll-to-bottom button | none drawn |
| Scrolled transcript | no scrollbar, no long transcript, no fade or shadow at the header or dock edge |
| More than one turn | every frame has one user turn and one reply; the gap between a reply and the next user turn is assumed to be the same 24 |
| Waiting before thinking | the moment after send and before any thinking row or text |
| Thinking to streaming | whether the thinking row stays above the text once text starts; the collapsed Done row is defined in the component but not placed in a frame read here |
| Loading an existing thread | no skeleton or spinner |
| Empty thread | covered by the home screen, not here |
| Cursor motion | blink or pulse of `stream_cursor` |
| Reply action states | hover, pressed, focus, tooltip, "copied" confirmation, selected thumbs, read-aloud playing |
| After editing | what Save does to the thread, disabled Save, pager on the user message |
| Long user messages | more than a few lines, collapse or expand |
| Partial Markdown | half-received table or code block while streaming |
| Title generation | the lifecycle frames always show a final title; the moment it appears is only described in a caption on the basic-chat page |
| Keyboard focus and dark mode | no focus rings, no dark frames |
