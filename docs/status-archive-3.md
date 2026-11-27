# Status archive 3

Older entries of `status.md`, newest first. Earlier ones are in `status-archive-2.md`.

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
