# Composer

The prompt box, its footer of pickers, and the small parts that sit in or around it. Menus that open from the composer are in `composer-part-2.md`; the model picker is in `model-picker.md`. Pixel values are design pixels at 1x. Token names in brackets are the variable names attached to the colour in the design file.

Source page: "Composer and Session Setup 0.2" (`97:29007`). Components page: `97:3`.

## Version 1 scope

| Part | Version 1 |
| --- | --- |
| `Composer` box, text, placeholder, send and stop | version 1 |
| `ComposerFooter` model control | version 1 |
| Docked composer and disclaimer line | version 1 |
| `ContextRing` chip | V1? (shown in the docked composer; needs context accounting) |
| Plus button and its menu | outside version 1 (no attachments, tools or connectors) |
| Dictation and voice buttons | outside version 1 |
| Footer project picker, mode control, `Apps` chip | outside version 1 |
| `ModeSwitch`, work hint | outside version 1 |
| Attachment tray, dropzone | outside version 1 |

With the outside parts removed, the empty state of the version 1 composer has no trailing button drawn for it in the design file (Empty shows only the voice button there). What the Empty state shows instead is a build decision to record.

## `Composer` (`97:11361`)

Component note in the file: "Prompt box. Empty shows voice; Filled shows send; Working shows stop while the reply runs."

Variants: state Empty `97:11362`, Filled `97:11370`, Working `97:11378`. Properties: `text`, `showContext` (boolean, default on).

### Box

| Property | Value |
| --- | --- |
| Width | 720 (fixed in every screen) |
| Height | hugs content: 96 with one text line, 144 with three lines |
| Fill | `#FFFFFF` (white) |
| Border | 1 px `#E5E7EB` (gray/200) |
| Radius | 16 |
| Padding | top 14, right 12, bottom 10, left 16 |
| Layout | column, gap 14: text, then toolbar |
| Shadow | style "shadow/sm": 0 1 2 `#0000000D` |

Height sum for one line: 1 + 14 + 24 + 14 + 32 + 10 + 1 = 96.

### Text area

| Property | Value |
| --- | --- |
| Type | interface sans, 16 / 24, weight 400 |
| Width | full inner width (690) |
| Placeholder colour | `#9CA3AF` (gray/400) |
| Typed text colour | `#111827` (gray/900) |
| Placeholder on home | `How can I help you today?` |
| Placeholder in a thread | `Reply to Isonomic…` |

Text wraps and the box grows by 24 per line. The tallest draft drawn is three lines (144). A maximum height, scrolling inside the box, a caret, text selection and a focus ring are not shown in the design file.

### Toolbar (layer `toolbar`)

Row, gap 4, items centred, full inner width, 32 high. Order, left to right:

| # | Layer | Size | Detail |
| --- | --- | --- | --- |
| 1 | `add` | 28 x 28, radius 6 | ghost icon button (instance of `IconButton` `97:12`), icon `plus` 16 x 16. Opens the plus menu |
| 2 | `spacer` | fills | |
| 3 | `ContextRing` | hugs, 24 high | only when `showContext` is on |
| 4 | `mic` | 28 x 28, radius 6 | ghost icon button, icon `microphone` 16 x 16. Dictation |
| 5 | trailing button | 32 x 32, radius 16 (circle) | changes with state, see below |

Ghost icon buttons have no fill at rest. Their icons are vectors; the component's style list includes `#6B7280` (gray/500), which matches the mid-gray seen in the pictures. `IconButton` has Default and Hover variants (documented with the basics).

### States

| State | Text shown | Trailing button | Layer | Fill | Icon |
| --- | --- | --- | --- | --- | --- |
| Empty | placeholder, `#9CA3AF` | voice | `voice` `97:11368` | `#F3F4F6` (gray/100) | `waveform` 16, dark |
| Filled | typed text, `#111827` | send | `send` `97:11376` | `#111827` (gray/900) | `arrow-up` 16, white |
| Working | placeholder, `#9CA3AF` (the box is empty again) | stop | `stop` `97:11384` | `#111827` (gray/900) | `stop` 16, white |

Everything else is identical across the three states: same box, same plus, same dictation button.

When each trailing button shows:

- Send shows as soon as there is text and no reply is running.
- Stop replaces it while a reply is running.
- Voice shows only when the box is empty and nothing is running.
- Typing while a reply runs (queued messages), and a disabled send, are not drawn on the component. Queued messages are covered in `thread-states.md` (outside version 1).
- Hover, pressed and focus looks for send, stop and voice are not shown in the design file.

### `ContextRing` (`100:4529`)

Component note in the file: "Context usage ring for the composer and header; orange from 80%."

| Property | Value |
| --- | --- |
| Layout | row, gap 6, items centred, padding 3 top/bottom, 8 left/right |
| Border / radius | 1 px `#E5E7EB` (gray/200), 999 (pill); no fill |
| Ring | layer `ring`, 16 x 16 vector: a partial arc on a light track |
| Text | percent value, default `42%`; 12 / 16, weight 500, interface sans |

| Variant | Node | Ring arc | Text colour |
| --- | --- | --- | --- |
| level=Normal | `100:4527` | dark (`#111827` gray/900 is in the style list) | `#4B5563` (gray/600) |
| level=Warn | `100:4528` | `#F97316` (orange/500) | `#EA580C` (orange/600) |

Where it shows: in the docked composer of a thread, on the incognito home and in the attachment screens. It is hidden on the plain home, the start page, and the long-draft screens. What clicking it does is not shown on this page.

## Composer on home

Instance state Empty, `showContext` off, 96 high, at x 216, y 313 in the 1152 x 848 content area, with the greeting 20 above and `ComposerFooter` 20 below. See `home.md`. Picture: `../images/navigation-01-home.png`.

## Composer docked in a thread (`composer_dock` `97:41047` in `SessionView/03_Complete`)

| Property | Value |
| --- | --- |
| Frame | full width of the session body (1152), 132 high, at the bottom of the body |
| Layout | column, gap 8, centred horizontally, bottom padding 12, no top padding |
| Children | `Composer` (720 x 96), then the disclaimer line |

- Composer: state Empty, placeholder `Reply to Isonomic…`, `showContext` on (`42%`), plus, dictation and voice buttons.
- Disclaimer `97:41049`: `Isonomic is AI and can make mistakes. Please double-check responses.`; interface sans, 12 / 16, weight 400, `#9CA3AF` (gray/400), one centred line.
- There is no `ComposerFooter` in the dock. The model, project and mode controls are only drawn on home. How the model is changed mid-thread is not shown on this page.
- A fill or top border behind the dock, and a fade over the transcript, are not present on the frame.
- With a usage banner the dock grows to 188: banner (48), gap 8, composer, gap 8, disclaimer. See `first-run-and-system-states.md`.

Picture: `../images/lifecycle-03-complete.png`.

## Taller draft (`ComposerMenu/14_LongDraftModeSwitch` `97:29316`)

Composer `97:29322`: state Filled, `showContext` off, 720 x 144. The text is three lines:

`Draft a launch plan for the Isonomic SDK: audience, milestones for the next six weeks, the docs we need before beta, and a short risk list. Keep it to one page and flag anything that depends on the inference cluster being ready. Then turn the milestones into tasks with owners.`

The home group re-centres around the taller box: greeting at y 227, composer at y 289, footer at y 453 (same gap 20, bottom padding 140 rule).

## `ComposerFooter` (`97:19517`)

Component note in the file: "Pickers under the composer: project, model + effort, permission mode."

| Property | Value |
| --- | --- |
| Size | 720 x 28 |
| Layout | row, gap 4, items centred, padding 0 top/bottom, 8 left/right |
| Fill / border | none |
| Properties | `project`, `model`, `effort`, `mode` (texts); `showProject`, `showMode`, `showApps` (booleans, all on by default) |

Controls, left to right:

| # | Layer | Version 1 | Layout | Content |
| --- | --- | --- | --- | --- |
| 1 | `project` `97:19518` | outside | row, gap 6, padding 4 / 8, radius 6 | icon `folder-simple` 14; label `Project` (or a project name, for example `Isonomic`), 14 / 20, weight 500, `#4B5563` (gray/600); icon `caret-down` 12 |
| 2 | `spacer` `97:19522` | | fills | |
| 3 | `model` `97:19523` | version 1 | row, gap 6, padding 4 / 8, radius 6 | `[model name]`, 14 / 20, weight 500, `#374151` (gray/700); then effort `Extra`, 14 / 20, weight 400, `#9CA3AF` (gray/400). No icon |
| 4 | `mode` `97:19526` | outside | row, padding 4 / 8, radius 6 | `Auto`, 14 / 20, weight 500, `#374151` |
| 5 | `apps` `101:5848` | outside | row, gap 6, padding 3 / 8, radius 999, 1 px border `#E5E7EB` | three overlapping marks, then `Apps`, 12 / 16, weight 500, `#374151` |

`app_marks` `101:5849`: three circles 18 x 18, fill `#FFFFFF`, 1 px border `#E5E7EB`, radius 9, each overlapping the next by 6; icons 10 x 10: `envelope-simple`, `calendar`, `folder`.

What each control opens:

| Control | Menu (see `composer-part-2.md`) |
| --- | --- |
| `project` | project picker, or the work-in-folder menu |
| `model` | model picker (`model-picker.md`); its `Effort` row opens the effort menu |
| `mode` | permission mode menu |
| `apps` | connected apps menu |

Only the resting look of each control is drawn. Hover, pressed, open and focus looks are not shown in the design file. With only the model control (version 1), it stays right-aligned because the spacer is before it.

## `ModeSwitch` (`97:11536`, outside version 1)

Component note in the file: "Chat / Work switch shown in the composer once a draft is long or work is implied."

| Property | Value |
| --- | --- |
| Size | 101 x 26 |
| Track | fill `#F3F4F6` (gray/100), radius 8, padding 2, row gap 2 |
| Option | padding 3 top/bottom, 10 left/right, radius 6; text 12 / 16, weight 500 |
| Selected option | fill `#FFFFFF`, shadow/sm, text `#111827` (gray/900) |
| Other option | no fill, text `#6B7280` (gray/500) |
| Options | `Chat`, `Work` |
| Variants | value=Chat `97:11537`, value=Work `97:11542` |

Placement: an overlay on the composer's toolbar row, directly right of the plus button (x 550 on screen; the plus button ends at 549). It is placed by hand: its top is 10 below the toolbar top, so it is not vertically centred on the toolbar.

| Screen | Value | Extra |
| --- | --- | --- |
| `14_LongDraftModeSwitch` `97:29316` | Chat | `Tooltip` `97:29325` 8 below the composer at x 524: `This looks like a task. Switch to Work?` |
| `15_WorkMode` `97:29326` | Work | greeting `What should we get done?`; footer project reads `Isonomic`; `work_hint` under the footer |

Tooltip style: fill `#111827`, radius 6, padding 5 top/bottom, 8 left/right, text 12 / 16, weight 500, `#FFFFFF`.

`work_hint` `97:29334` (481 x 30, 20 below the footer, centred): fill `#F9FAFB` (gray/50), 1 px border `#E5E7EB`, radius 8, padding 6 top/bottom, 10 left/right, row gap 6; icon `lightning` 14; text `Work mode runs multi-step tasks with tools and shows progress in the side panel.`, 12 / 16, weight 400, `#111827`.

## Attachment tray (outside version 1, brief)

Screens `ComposerAttachment/16_ImageAttached` `97:29338` and `17_HoverRemove` `97:29349`. Full detail belongs in `attachments.md`.

- `composer_with_attachments` (`97:29344`, `97:29355`): 720 x 204, fill white, 1 px border `#E5E7EB`, radius 16, clips content, shadow/sm. It wraps a tray on top of a borderless `Composer`.
- `attachment_tray`: row, padding 12 top, 12 left/right, 0 bottom.
- `AttachmentThumb` (`97:11523`, states Default and Hover): 120 x 96, fill `#F3F4F6`, 1 px border `#E5E7EB`, radius 10, clipped. Preview area 64 high; caption strip fill white, padding 6 / 8, file name 12 / 16, weight 400, `#374151`. Hover adds a remove button: 20 x 20 circle, fill `#111827`, icon `x` 12 white, at left 93, top 5.
- Screen 17 adds a `Tooltip` `97:29360` (61 x 26) under the remove button; its text was not read.
- The composer in these screens is Filled with `What’s missing from this roadmap?`, shows the context chip, and the send button.

## Dropzone (outside version 1, brief)

Screen `ComposerAttachment/19_DropzoneUploading` `107:5416`. The dropzone `107:5639` (720 x 298) sits between the greeting and the composer, 20 from each.

- Box: fill `#F9FAFB` (gray/50), 1 px dashed border `#9CA3AF` (gray/400), radius 16, padding 16, column gap 8, centred.
- Icon `upload-simple` 22; title `Drop files to add them to this chat`, 14 / 20, weight 500, `#1F2937` (gray/800); note `PDF, images, code and docs up to 30 MB`, 12 / 16, `#6B7280`.
- Three `DropzoneFile` rows (`101:5724`), full width (686): 1 px border `#E5E7EB` (Error: `#FEE2E2` red/100), radius 8, padding 8 / 10, gap 10; type badge 32 x 32, fill `#F3F4F6`, radius 6, text `PDF` 12 / 16, weight 600, `#4B5563`; file name 14 / 20, weight 500, `#111827`.

| State | Second line | Trailing |
| --- | --- | --- |
| Uploading `101:5721` | progress track 300 x 4, `#F3F4F6`, fill 180 wide `#111827`, radius 2 | ghost `x` button 28 |
| Done `101:5722` | `2.4 MB · ready`, 12 / 16, `#6B7280` | ghost `x` button 28 |
| Error `101:5723` | `Upload failed · file over 30 MB`, 12 / 16, `#DC2626` (red/600) | small button `Retry` |

## Not shown in the design file

- Maximum composer height and scrolling for long text.
- Focus ring, caret and selection colours.
- Disabled composer (for example offline or limit reached).
- Composer widths other than 720, and narrow windows.
- Keyboard hints (send key, newline key) anywhere on or near the composer.
- Hover and pressed looks of the trailing buttons and footer controls.

## Oddities

- The shadow is written two ways: the component uses a drop shadow of 0 1 1 at 5 percent in the reference code while its style is named shadow/sm (0 1 2 `#0000000D`); the attachment wrapper uses 0 1 2 directly. Treat both as shadow/sm.
- The context chip shows `42%` on the incognito home (an empty new chat) but is hidden on the plain home.
- `ModeSwitch` is placed by hand over the toolbar instead of being a toolbar child, so its offset is not a layout rule.
- The Working state shows the home placeholder text, not a "reply running" hint.
