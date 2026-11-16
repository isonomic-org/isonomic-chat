# Status

Newest first. Every deviation from the brief is recorded here.

## 2026-11-16 · How long ago, in search

- Each chat keeps when it was last active: the newest change in any of its versions, from the host's list, or now for a chat started here. Search shows it on the right of the row in the design's short form ("now", "20m", "2h", "3d", "2w", "4mo", "1y").
- The rest of `docs/design-diffs.md` is either fixed, decided, or with Isonomic UI (#385: a chat icon on palette rows, list indent, icon size apart from button size). The serif wordmark and greeting wait on whether the theme has a serif family.

## 2026-11-16 · SDK at 72aca92: Isonomic UI's new options in use

- `SDK_REF` moves to `72aca92` (`UI_REF` `4f1207b`, isonomic-ui #381); harness unchanged. `Cargo.lock` needs no change.
- Sidebar: the design's padding and 1 px gaps, 32 px rows that take focus (Tab, Enter, Space, Up, Down), Pinned folds. The header title truncates and keeps its caret; Delete is destructive in both menus.
- Search: 640 wide, "Search or ask…", Esc in the field, 36 px rows, groups kept while filtering, the design's hints, nothing selected until the person types or moves, "No chats match.", focus held inside.
- Composer: the comfortable size, round 32 px send and stop. Model picker: "Model" header, name-only trigger, trailing check, chosen-row fill, the design's row padding, no footer.
- Thread: list indent and marker gap; the reply bar's pager first and "Try again". Buttons under a prompt and a reply stay 24 px so their icons stay 16 px.
- Still open: palette chat icon and meta, hints as key chips, list text at 28, icon size apart from button size (`docs/design-diffs.md`).

## 2026-11-16 · First run, versions on a real engine, keys, and the aligned status bar

- **First run:** home leaves Downloading the moment the model is ready, with no restart. The kit host gains `availability(model)` over `models.info`; Chat asks it every second while a model is downloading or has been asked of the window, and stops when it settles (or, if the window never starts the download, after a minute, offering Install again). Tested against the SDK test engine with a scripted download held at a gate.
- **Versions on a real engine:** the SDK adapter pages through the session list until it is complete (the engine sends 50 at a time; tested with 60), and Chat lists every session. Opening a chat reads each earlier session only up to its fork point, for at most ten seconds; a log that ends early or stalls shows "This chat couldn't load." with Try again, never an endless load (tested with a stalled log).
- **Keys:** Mod-B toggles the sidebar too (Mod-. stays, the design's key; on macOS Cmd-. means cancel). Alt-Up and Alt-Down step through chats with focus in the composer (tested).
- **SDK at `aea8ba1`** (`UI_REF` `66e93d3`): the status bar is the aligned run status bar as the design draws it: Stop while running; Dismiss, Retry and Resume when stopped; Dismiss and Try again on an error; Dismiss and Continue when cut short. Steer is left out. Dismiss hides it until the turn changes. Tests find its buttons with `ConversationView::status_button`.

## 2026-11-16 · Audit of the version 1 screens against design 0.2

- Every screen and state was read against `docs/design/v0.2/` in both themes; the result is `docs/design-diffs.md` (one table per screen, each row fixed, needs Isonomic UI, or accepted) and `docs/accessibility.md` (keyboard path and accessible names). The 0.1 tables moved to `docs/design-diffs-v01.md`.
- Fixed: the thread's spacing (the scroller's own insets doubled the gaps), the prompt bubble (16 / 24, 16 / 10 padding, 560), one heading size, the prompt's Copy and Edit reachable by Tab and shown on focus, the edit box (16 px text, 32 buttons, shadow, Escape cancels, named). Home's footer height and padding, the picker at 380, an empty state to the design's sizes with an icon, Downloading after Install, a failed model list with Try again, a chosen model that is downloading or unavailable no longer asks for an install. The rail centred with a 28 toggle and no selection, row padding, regular weight for a selected chat, the status dot box, menus with Open and icons at 220, the title button and rename field (300, Escape cancels), the palette centred with Recents and the New chat shortcut, focus back after search.
- Keyboard: Chat opens on the composer; toggle sidebar moves to Mod-. (the design's key); Alt-Up and Alt-Down step through chats. Names: the toggle by state, More by chat, running chats, the title button, the rename and edit fields, code-block copy.
- Screenshot states added: thinking before text, Markdown blocks, a failed send with Try again (19 states).

## 2026-11-16 · SDK at abe8fe4: the run status bar and no attaching

- `SDK_REF` moves to `abe8fe4` (its `UI_REF` is isonomic-ui `b4c9670`); harness unchanged at `ae4f1a7`. `Cargo.lock` needs no change.
- The kit's status bar is now Isonomic UI's `RunStatusBar`: stopped (with Continue, passed as its action label), cut short (Continue), failed (Try again), reconnecting (also for an app away) and closed. Its button is the child `"action"` of the bar's id (`conversation_ui::action_button`).
- Both composers turn attaching off (`set_attach(false)`): no plus button, no dropped files, no pasted images.
- The `gpui` alias stays: `#[ui::test]` still resolves through it.
- Chat's icon list gains the bar's icons.

## 2026-11-16 · The chat list by newest version

- A chat is as new as its newest version: the list orders chats by the latest change in any of their sessions (`conversation_model::newest_first`), pins still on top in pin order. A fork whose first session is not listed (an archived chat) never shows as a chat.
- Tested in the model and against the SDK test engine: an older chat with a version made after a newer chat is listed first, and is not without the change.

## 2026-11-16 · Editing a prompt

- Edit (under a prompt, on hover) opens Isonomic UI's textarea in place of the bubble, with Cancel and Save; Mod-Enter saves. Save forks before that turn with reason edit and sends the new prompt there; it is a new version of the turn, in the same pager as try again. An empty or unchanged prompt does nothing.
- Tested on the scripted host (cancel keeps the prompt; save makes a version with reason edit; the pager brings the first prompt back) and on the SDK test engine (the fork carries reason edit and runs the new prompt). Screenshot state `thread-editing`.
- Chat's icon list now has the icons try again, the pager and edit draw.

## 2026-11-16 · Try again and versions

- Try again on any finished reply forks the session before that turn (`ConversationHost::fork`, over `sessions.fork` with reason regenerate) and sends the prompt again in the fork. The fork shares every turn before it; the turns after it belong to the old version.
- The session and its forks at one point are that turn's versions. The pager under the reply switches between them, and later turns follow the version shown. Trying a version again adds a sibling, never a fork of a fork at the same turn.
- A conversation is a tree of sessions; what shows is the path to the session most recently changed (`conversation_model::plan`). Opening a chat folds each session on the path up to where the next forked, then follows the last live.
- Chat's list leaves forks out: a version is inside its chat. Rename, pin and delete act on the chat's first session.
- Tested on the scripted host (last reply, an earlier reply, reopening with nested versions) and on the SDK's real host loop (a fork's own log holds only `session.start` and `forked`; Chat lists one chat and opens the newer version).
- Not built: editing a prompt (the same fork with reason edit, once there is an edit box), and ordering the chat list by a chat's newest version.

## 2026-11-16 · Thread to design 0.2

- The thread scrolls across the whole panel, with its turns in a centred 720 column: 32 above the first and below the last, 24 between them. The composer and the status bar sit in the same column, with the disclaimer under them.
- A prompt is a muted bubble on the right, with Copy under it on hover. A reply is unboxed Markdown, with Copy under it once it ends. Each code fence has its own copy button.
- The status bar above the composer is the one place for reconnecting, a closed conversation, and how the last reply ended: stopped or cut short (with Continue), failed (in the danger colours, with Try again). Earlier turns keep a quiet note. It is a placeholder until Isonomic UI has a run status bar.
- Code block decision: the text view's own fences, because a fence renderer that keeps the streaming re-parse tail-only is not proven (request O9).
- Differences: `docs/design-diffs.md`, T1 to T10. Requests to Isonomic UI: O9 to O12.

Older entries: `docs/status-archive.md`.
