# Attachments (design 0.2)

Outside version 1. Recorded so the design is not lost.

Attachments have no page of their own. This file gathers what the design shows across three pages: the composer page (`97:29007`), the session lifecycle page (`97:40979`) and the agent work page (`97:44917`). Component values come from the layer data. Screen geometry marked "seen" was read from the screenshot, because the layer tree of these five frames could not be listed (see the last section).

## Screens

| Frame | Node | Screenshot | What it shows |
| --- | --- | --- | --- |
| ComposerAttachment/16_ImageAttached | `97:29338` | `../images/composer-16-image-attached.png` | New-chat home; composer grown to hold an attachment tray above the text |
| ComposerAttachment/17_HoverRemove | `97:29349` | `../images/composer-17-hover-remove.png` | Same, pointer over the thumbnail: remove button and tooltip |
| ComposerAttachment/19_DropzoneUploading | `107:5416` | `../images/composer-19-dropzone-uploading.png` | Drop zone above the composer listing three files in three states |
| SessionView/13_ImageSentRetitled | `97:41333` | `../images/lifecycle-13-image-sent-retitled.png` | Sent user turn with the image above the bubble; session titled "Roadmap gap review" |
| AttachmentViewer/14_Open | `97:41361` | `../images/lifecycle-14-open.png` | Full-window viewer for the sent image |

### Composer with an attachment (frames 16 and 17)

| Part | Values |
| --- | --- |
| Home heading | `Evening thoughts` in the display serif with a sparkle icon, centred above the composer |
| Composer | 720 wide at x 504 (seen), about 204 high: white, 1 px border, radius about 16. Order from the top: attachment tray, text, control row |
| Attachment tray | starts 12 inside the composer's top-left corner; one `AttachmentThumb` 120 x 96; thumbs would sit side by side |
| Text | `What's missing from this roadmap?` 16/24, about 16 below the tray |
| Control row | plus button at the left; context ring `42%`, microphone and a dark round send button (arrow up) at the right |
| Footer row under the composer | `Project` with folder icon and caret; `[model name]` `Extra`; `Auto`; `Apps` pill |
| Thumb caption | `sdk-roadmap.png` |
| Frame 17 | thumb in Hover state; tooltip `Remove` above the remove button: dark fill, white 12 px medium text, radius about 6, about 60 x 26, centred on the button and 8 above it (seen) |

### Drop zone (frame 19)

| Part | Values (seen unless a component is named) |
| --- | --- |
| Drop zone | 720 x about 298 at x 504, directly above the composer with a 20 gap; 1 px dashed grey border, radius about 16, padding about 16 |
| Prompt | upload icon about 20 px; `Drop files to add them to this chat` 14/20 medium; `PDF, images, code and docs up to 30 MB` 12/16 grey; centred, about 8 apart |
| File list | three `DropzoneFile` rows, full width (about 686), 8 apart, in the states Uploading, Done, Error |
| Composer | empty, placeholder `How can I help you today?`; no context ring and no send button in this state |

All three rows use the same sample name, `isonomic-whitepaper-v0.3.pdf`.

### Sent image in a user turn (frame 13)

| Part | Values (seen) |
| --- | --- |
| User turn | right-aligned stack: the thumbnail (120 x 96, same look as `AttachmentThumb` Default, caption `sdk-roadmap.png`), then the bubble 8 below it |
| Bubble | `What's missing from this roadmap?`, standard user bubble (grey fill `#F3F4F6`, radius 18, padding 16 x 10, 16/24) |
| Reply | `Three gaps stand out in the roadmap image:` then a numbered list: `No docs milestone before the beta date, so early users have nothing to build from.`; `Inference capacity is assumed for week 4 but nothing tracks the cluster work it depends on.`; `There's no feedback loop after beta: add a triage owner and a weekly review.`; then `Want me to add these as tasks with owners?`; reply actions and `9:51 PM` |
| Header | title `Roadmap gap review` (the session was retitled after the first reply), `Share` button |

### Attachment viewer (frame 14)

| Part | Values (seen) |
| --- | --- |
| Scrim | dark grey-blue overlay at roughly 60% over the whole window, sidebar included |
| Top bar | file name `sdk-roadmap.png` at the top left (x 20, y 20, white, 14 px medium); at the top right `1440 × 900 · 214 KB` (12 px, light grey), then two 32 px square buttons with a translucent light fill and radius about 6: download and close |
| Image stage | the image on a white sheet 1000 x 625 at x 220, y 160, radius about 12, centred |
| Sample image content | `Isonomic SDK · 6-week roadmap` with four labelled bars: `Core API`, `Auth & keys`, `Examples`, `Beta launch` |

## Components

### AttachmentThumb (`97:11523`)

Description: file or image attached to the composer. Hover reveals the remove button.

| Part | Values |
| --- | --- |
| Frame | 120 x 96, background `#F3F4F6` (gray/100), 1 px border `#E5E7EB` (gray/200), radius 10, clips content |
| `preview` | 120 x 64, `#F3F4F6`; holds the image |
| `caption` | white strip, padding 8 x 6; file name 12/16 `#374151` (gray/700), one line |
| `remove` (Hover only) | 20 x 20 circle, `#111827` (gray/900), at x 93, y 5 (7 from the right edge, 5 from the top); `icon_x` 12 px, white |

| Variant | Node | Difference |
| --- | --- | --- |
| Default | `97:11524` | no remove button |
| Hover | `97:11530` | remove button shown |

Default name `screenshot.png`.

### DropzoneFile (`101:5724`)

Description: file in a dropzone: type badge, progress, retry on error. 420 wide by default (fills the drop zone), 1 px border, radius 8, padding 10 x 8, gap 10.

| Part | Values |
| --- | --- |
| Type badge | 32 x 32, `#F3F4F6`, radius 6, label `PDF` 12/16 semibold `#4B5563` |
| Name | 14/20 medium `#111827` |
| Second line | 4 below the name; see variants |
| Trailing control | see variants |

| State | Node | Border | Second line | Trailing control |
| --- | --- | --- | --- | --- |
| Uploading | `101:5721` | `#E5E7EB` | progress bar 300 x 4, radius 2, track `#F3F4F6`, fill `#111827` (180 wide in the sample, 60%) | `icon_button_x` 28 px ghost button (cancel) |
| Done | `101:5722` | `#E5E7EB` | `2.4 MB · ready` 12/16 `#6B7280` | `icon_button_x` 28 px ghost button (remove) |
| Error | `101:5723` | `#FEE2E2` (red/100) | `Upload failed · file over 30 MB` 12/16 `#DC2626` (red/600) | small outline button `Retry` (white, 1 px border `#D1D5DB`, radius 4, padding 11 x 7, 12/16 medium `#374151`) |

Default name `isonomic-whitepaper-v0.3.pdf`.

### FileCard (`97:19577`)

Description: file produced or shared in the transcript, with an Open action. Used in the agent work flow (frames 08, 09, 11 to 14 of that page), where `Open` shows the file preview pane described in `agent-work-and-side-panel.md`.

| Part | Values |
| --- | --- |
| Card | 720 wide by default (fills the transcript column; 528 beside the preview pane), 64 high, white, 1 px border `#E5E7EB`, radius 12, padding 14 x 12, gap 12 |
| `file_icon` | 36 x 36, `#F3F4F6`, radius 8; `icon` 18 px `file-text`, gray/600 `#4B5563` |
| Name | 14/20 medium `#111827` |
| Meta | 12/16 `#6B7280`, 2 below the name |
| `action` (optional, `showAction`) | button `Open`: white, 1 px border `#D1D5DB`, radius 6, padding 13 x 9, shadow sm (0 1 2 at 5% black), 14/16 medium `#374151` |

Defaults and sample: name `palette-notes.md`, meta `Document · MD`.

### ArtifactCard (`97:19584`)

Description: artifact chip in the transcript; opens the artifact beside the session.

| Part | Values |
| --- | --- |
| Card | 360 wide, 64 high, background `#F9FAFB` (gray/50), 1 px border `#E5E7EB`, radius 12, padding 14 x 12, gap 12 |
| `artifact_icon` | 32 x 32, `#EFF5FF` (blue/50), radius 8; `icon` 16 px `file-text` in blue/600 `#0161FE` |
| Title | 14/20 medium `#111827` |
| Meta | 12/16 `#6B7280`, 2 below the title |

Defaults and sample: title `Isonomic palette note`, meta `Docs · Only you`. There is no action button; the whole card is the target.

## How the pieces relate

| Moment | Component | Where |
| --- | --- | --- |
| File chosen or pasted, not yet sent | `AttachmentThumb` | tray inside the composer |
| Files dragged over the window, uploading | `DropzoneFile` | drop zone above the composer |
| Image sent by the user | thumbnail above the user bubble | user turn; a click opens the attachment viewer |
| File written by the assistant | `FileCard` | transcript; `Open` shows the file preview pane |
| Artifact created by the assistant | `ArtifactCard` | transcript; opens the artifact beside the session |

## Not read and oddities

- The layer trees of the five frames above were not listed: the design tool's rate limit was shared with other work, and a per-frame structure read returned only the frame itself. Sizes and positions in the screen tables are therefore read from the 1440 x 900 screenshots and are approximate to a few pixels. The four components are exact.
- Frame 19 uses the same file name on all three rows, so one file appears as uploading, ready and failed at once; it is a state sampler, not a real moment.
- Frame 19 says files may be up to 30 MB and the error row says the file is over 30 MB, while the ready row reports the same file as 2.4 MB.
- The thumbnail preview area is an empty grey block in frames 16, 17 and 13; no image is drawn inside it, though the viewer in frame 14 shows the picture.
- No hover, focus or selected state is drawn for `FileCard` or `ArtifactCard`, and no non-image attachment (for example a document chip in a user turn) is drawn.
- The viewer has no zoom, next or previous controls.
- Sidebar sample titles in these frames name outside products; treat them as `[third-party service]`. The composer footer names a model; written as `[model name]`.
