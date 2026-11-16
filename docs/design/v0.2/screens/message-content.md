# Message content: Markdown inside a reply

How each Markdown block looks inside `assistant_message`. Sources: frame `Markdown/44_TableCodeMath` (`97:78192`, on page "Inline Visuals and Widgets 0.2" `97:76658`; its reply is `97:78202`) and the lifecycle frames on page `97:40979` (`97:41033` for paragraphs, headings and bullets; `97:41344` for the numbered list).

Values are design pixels at 1x. Type is size / line height / weight / face. Token names are the design file's.

## Container

| Property | Value |
| --- | --- |
| Container | `assistant_message`, 720 wide, vertical stack, left aligned |
| Gap between blocks | 12, the same between every pair of blocks. Headings get no extra space above or below |
| Background | none (text sits on the white panel) |
| Last child | `ReplyActions` row, also 12 below the last block |

## Block summary

| Block | Layer name | Type | Text colour | Box |
| --- | --- | --- | --- | --- |
| Paragraph | `paragraph` | 16 / 24 / regular (400) / interface sans | `#111827` (`gray/900`) | 720 wide, no box |
| Heading | `heading` | 18 / 28 / semibold (600) / interface sans | `#111827` (`gray/900`) | 720 wide, no box |
| Bullet list | `bullet_list` | 16 / 24 / regular / interface sans | marker `#6B7280` (`gray/500`), text `#111827` | see below |
| Numbered list | `numbered_list` | 16 / 24 (item height 24) | not read, see below | see below |
| Table | `md_table` | header 12 / 16 / medium (500); cells 14 / 20 / regular; interface sans | header `#6B7280` (`gray/500`), cells `#1F2937` (`gray/800`) | border, radius 8 |
| Code block | `code_block` | 13 / 20 / regular / mono face | `#1F2937` (`gray/800`) | fill, border, radius 10 |
| Math block | `math_block` | 20 / 28 / semibold (600) / display serif | `#111827` (`gray/900`) | fill, radius 8 |

Only one heading level is shown (18 / 28 semibold). Other heading levels are not shown in the design file.

## Paragraph

- 16 / 24 / regular / interface sans, `#111827`, full 720 width, wraps.
- Two lines = 48 high, one line = 24.
- Paragraph to paragraph spacing is the container gap, 12.

## Heading

- 18 / 28 / semibold / interface sans, `#111827`, one line = 28 high.
- 12 above and 12 below, like every other block.
- Examples: "The two jobs it does", "Why it’s safe".

## Bullet list (`bullet_list`, `97:41036`)

| Property | Value |
| --- | --- |
| List box | 720 wide, vertical stack, padding-left 4, gap 6 between items |
| Item (`list_item`) | horizontal row, top aligned, gap 10 between marker and text |
| Marker | the character "•", 16 / 24 / regular, `#6B7280` (`gray/500`), 7 wide, at x = 4 from the column edge |
| Item text | starts at x = 21 from the column edge (4 + 7 + 10); text box 690 wide; `#111827`; wraps, second line aligns with the first line of text (hanging indent) |
| Item heights | 24 per line; a one-line item is 24, a two-line item 48 |

Nested lists are not shown in the design file.

## Numbered list (`numbered_list`, `97:41346`, frame 13)

Read from the layer geometry only (the styled reference for this frame was not fetched; the design file's read limit was reached).

| Property | Value |
| --- | --- |
| List box | 720 wide; items start at x = 4 (same 4 padding-left as bullets) |
| Item gap | 6 (items at y = 0, 30, 60 with height 24) |
| Marker | "1.", "2.", "3."; 14 wide, 24 high |
| Marker to text gap | 10 (text starts at x = 24 inside the item, 28 from the column edge) |
| Item text box | 690 wide, 24 line height |
| Colours | not read; expected to match the bullet list (marker `gray/500`, text `gray/900`) but not confirmed |

Item texts: "No docs milestone before the beta date, so early users have nothing to build from." / "Inference capacity is assumed for week 4 but nothing tracks the cluster work it depends on." / "There’s no feedback loop after beta: add a triage owner and a weekly review."

## Table (`md_table`, `97:78204`)

| Property | Value |
| --- | --- |
| Outer box | 720 wide, border 1 `#E5E7EB` (`gray/200`), radius 8, clips its content, no outer fill |
| Rows | vertical stack, no gap; each row is a horizontal row of equal-width cells (three columns share the width equally, 231.33 each inside 12 padding) |
| Header row | fill `#F9FAFB` (`gray/50`); bottom border 1 `#F3F4F6` (`gray/100`); padding 12 left and right, 8 top and bottom; text 12 / 16 / medium / interface sans, `#6B7280` (`gray/500`); row height 33 (16 + 16 + 1 border) |
| Body rows | no fill; bottom border 1 `#F3F4F6` (`gray/100`) on every row including the last; padding 12 left and right, 8 top and bottom; text 14 / 20 / regular / interface sans, `#1F2937` (`gray/800`); row height 37 |
| Alignment | all cells left aligned, including numbers; no vertical dividers between columns |
| Total height | 146 for one header and three body rows (2 border + 33 + 3 x 37) |

Content shown:

| Broker | Calls / day | p95 latency |
| --- | --- | --- |
| Models | 48,200 | 420 ms |
| Data & connections | 12,900 | 180 ms |
| Egress | 6,300 | 95 ms |

Column alignment markers, wide tables that overflow 720, and horizontal scrolling are not shown in the design file.

## Code block (`code_block`, `97:78222`)

| Property | Value |
| --- | --- |
| Box | 720 wide; fill `#F9FAFB` (`gray/50`); border 1 `#E5E7EB` (`gray/200`); radius 10; padding 14 left and right, 12 top and bottom |
| Text | 13 / 20 / regular / mono face; `#1F2937` (`gray/800`); whitespace kept (pre-wrap); text box 690 wide |
| Height | 146 for six lines (6 x 20 + 24 padding + 2 border) |
| Header | none: no language label, no copy button, no line numbers |
| Syntax colours | none: the whole block is one colour |

Content shown (two-space indentation; the model name is replaced by a placeholder):

```
brokers:
  models:
    route: cluster
    fallback: local-[model name]
  egress:
    allowlist: [api.isonomic.dev]
```

A copy control for code, a language label, syntax highlighting and long-line behaviour (wrap or scroll) are not shown in the design file. The text style is pre-wrap, so long lines would wrap as drawn.

## Math block (`math_block`, `97:78225`)

| Property | Value |
| --- | --- |
| Box | 720 wide; fill `#F9FAFB` (`gray/50`); no border; radius 8; padding 14 top and bottom; content centred horizontally |
| Text | 20 / 28 / semibold / display serif; `#111827` (`gray/900`); one line, not wrapped |
| Height | 56 (28 + 28) |

Content shown: "cost = Σ (tokensᵢ × priceᵢ) + nodes × hours × rate". The formula is plain text with subscript characters; no typeset fractions or stacked layout are shown.

## Inline code

Not shown in the design file. No frame read for this document has an inline code span inside a paragraph. The nearest evidence is the code block style (mono face 13 / 20, `gray/800` on `gray/50`).

## Links

Not shown in the design file inside reply text. No paragraph read for this document contains a link, so link colour, underline and hover are unknown.

## Bold and italic

Not shown in the design file inside reply text. Paragraphs are all regular weight; semibold appears only in headings.

## Block quote, horizontal rule, images, task lists

Not shown in the frames read for this document.

## Paragraphs around blocks (frame `97:78202`)

Order and y positions inside the reply, showing the constant 12 gap:

| y | Height | Block | Text |
| --- | --- | --- | --- |
| 0 | 24 | paragraph | "Here’s the summary for the last 24 hours:" |
| 36 | 146 | table | see above |
| 194 | 24 | paragraph | "The broker config that produced it:" |
| 230 | 146 | code block | see above |
| 388 | 24 | paragraph | "Daily cost is computed as:" |
| 424 | 56 | math block | see above |
| 492 | 28 | `ReplyActions` | time "9:05 AM" |

The user message in this frame is a `UserMessage` 476 wide (`97:78201`); its text was not read.

## Streaming and Markdown

While a reply streams, blocks appear in their final style as they arrive (frame `SessionView/02_StreamingStructured`, `97:40997`: paragraph, heading, two bullets, heading, a one-line paragraph), and the `stream_cursor` (8 x 18, `#111827`, radius 2) sits as its own block 12 below the last block, at the left edge. How a half-received table or code block looks is not shown in the design file.

## Colour and radius index

| Token | Hex | Used for |
| --- | --- | --- |
| `gray/900` | `#111827` | paragraph, heading, list text, math text, stream cursor |
| `gray/800` | `#1F2937` | table cells, code text |
| `gray/500` | `#6B7280` | bullet marker, table header text |
| `gray/200` | `#E5E7EB` | table border, code block border |
| `gray/100` | `#F3F4F6` | table row dividers |
| `gray/50` | `#F9FAFB` | table header fill, code block fill, math block fill |

| Radius | Used for |
| --- | --- |
| 8 | table, math block |
| 10 | code block |
| 2 | stream cursor |
