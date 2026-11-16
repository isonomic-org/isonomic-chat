# Status

Newest first. Every deviation from the brief is recorded here.

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

## 2026-11-16 · Home and composer to design 0.2

- Home is the greeting (sparkles and the time of day), the 720 composer ("How can I help you today?") and a footer with Isonomic UI's model picker, lifted 140 px above centre. No suggestion chips.
- The picker lists the catalog's models with their state only (Recommended, Not installed, Downloading, Unavailable). Choosing one that can answer runs the next chat with it; choosing one that is not installed asks the window to install it (`ChatShell::choose_model`, tested).
- The thread's composer no longer has a model menu (`ConversationView::set_model_menu`, a kit switch, default on). Its placeholder is "Reply to Isonomic…" and the disclaimer is the 0.2 wording.
- With no chat model, home is Isonomic UI's empty state with one button that installs the recommended model.
- Differences: `docs/design-diffs.md`, H1 to H7.

## 2026-11-16 · Shell and sidebar to design 0.2

- The context panel is gone. The app sidebar (288 px) has the toggle and the wordmark, the nav (New chat, Search), and the Pinned and Recents groups; each row has a chat icon, a running dot while its reply is written, and a menu (Pin or Unpin, Rename, Delete). Collapsed, it is Isonomic UI's icon rail: the toggle and the nav only. Below 1,040 px the window collapses it.
- The header is 52 px: on a chat, its title with a menu (Rename, Pin or Unpin, Delete); on home, nothing.
- Search (`cmd-k`, or the nav item) opens Isonomic UI's command palette over the shell, with the chats by title and the New chat action; Enter opens, Escape or a click outside closes.
- Any chat can be deleted from its row (`ChatShell::delete_session`).
- The list's empty and failed states, the live-data rules (no invented name, no example data on a backend) and the screenshot harness come from the earlier layout work. Differences from 0.2: `docs/design-diffs.md`, S1 to S10.

## 2026-11-16 · Final layouts of the version 1 screens

Built from the components Chat already uses, with Isonomic UI's spacing, type and theme tokens; nothing restyled, no new widgets, no new agent components.

- **Home:** Isonomic UI's prompt composer, the same one the thread uses, with the model menu from the catalog; the picked model runs every new chat (it was only applied at the view level before, and is now tested end to end). With no chat model installed, one button asks the window to install the recommended one.
- **Thread:** the divider of a compaction now sits after the last turn the host confirmed, never after a send in flight (it had landed under the new prompt).
- **Narrow and wide:** below 1,100 px the context panel gives way, below 860 px the sidebar too; the header's toggle still opens it, and both come back when the window widens. The header title truncates and Share keeps only its icon.
- **Live versus examples:** on a backend, the greeting names no one and the context panel shows its empty state instead of example connections.
- **Chat list states:** "Chats you start appear here." and a failed list with its reason and Try again (`ChatShell::reload`); a failed write says so.
- **Rail icon:** `chat` from Isonomic UI's icon catalog (was `message-circle`, which is not in it), with a test that it resolves.
- **Screenshots:** `tests/screenshots.rs` now renders 13 states in both themes, including live ones on the scripted host (thread, narrow thread, streaming, errors, live home, models, failed list, install a model). All were looked at.
- **Keyboard:** every control is an Isonomic UI button or field, so Tab reaches it; the sidebar, context panel and new chat keep their shortcuts.
## 2026-11-16 · Fixes found while laying out the screens

- Rail icon: the manifest names `chat`, which Isonomic UI's icon catalog has (`message-circle` is not in it, so the window's rail showed a "C"). A test checks that `icons/<name>.svg` resolves.
- A model picked in one chat now runs every new chat. The shell kept the pick but never handed it to a new conversation, and a new conversation's catalog arrives after its first send could read it. The view now keeps a pick until its catalog has loaded. Tested end to end.
- A compaction's divider sits after the last turn the host confirmed, never after a send in flight.
- The final layouts wait for the version 0.2 layout reference (`docs/design/v0.2/`).

## 2026-11-16 · SDK 439a069: the QA'd components

- `SDK_REF` moves to `439a069` (its `UI_REF` is isonomic-ui `2e3116b`, with all 86 version 1 components through QA); harness unchanged at `ae4f1a7`. `Cargo.lock` needs no change.
- Behaviour from the components, checked against Chat: Tab skips disabled text fields (the composer disabled on a closed session); icon-only buttons are named by their tooltip (the sidebar and context panel toggles); a focused toggle draws a focus ring; the prompt composer wraps its controls below about 430 px. Every test passes unchanged.

## 2026-11-16 · SDK 99630cc: preset version 3, compaction, a conversation too large

- `SDK_REF` moves to `99630cc`; every harness crate is at `ae4f1a7`, the SDK's string character for character.
- Chat preset version 3 caps replies at 1,024 tokens. A reply cut at the limit is `Truncated` and offers Continue, which finishes it in the same turn. Tested with the scripted host's `cut_after`.
- `compacted` with a summary is a quiet divider after the last turn (`Turn::divider_after`, or `Thread::divider` before any turn), showing the summary's note. It is never a message. Tested on the scripted host and on the `autonomous-run` fixture.
- `turn.end` `failed` with `context.too_large` and no steps shows its `reason` as written; the composer stays usable, and Try again is not offered, since the same prompt would not fit either (`Turn::error_code`, `TOO_LARGE`).
- `scripted.rs` stays under 20 KB: `ScriptedReply` moves to `scripted/reply.rs`.

## 2026-11-16 · SDK 12ddf2f: the picked model, stream ends, installing a model

- `SDK_REF` moves to `12ddf2f`; every harness crate is at `9a3809f`, the SDK's URL and rev character for character. `Cargo.lock` regenerated.
- A new chat passes the picked model to `sessions.create`; the session's model shows in `SessionEntry.model` (from the summary, or from `session.start` in the scripted host). An app without the `models` capability that names a model gets `capability_denied`, which is tested.
- The catalog's recommendation comes from `ModelInfo.recommended`.
- Stream ends: `SessionUpdate::Gap` ends the kit's stream, so the view follows again from its last `seq` and reads what was skipped. `SessionUpdate::Ended` (a purge) becomes `Update::Ended`, which closes the thread, so the view stops following.
- Installing a model: `Backend::install_model` is the window's install step, wired in `Chat::build` to `AppContext::request(WindowRequest::InstallModel { model })`. Chat calls it on `ModelUnavailable`, and from home: when no chat model can answer, home shows one button that asks to install the recommended model, in place of a composer that cannot reply.
- Chat preset version 2: no test here asserts the preset version or its text, so nothing changed.

## 2026-11-16 · Stop and continue, checked after the first run on the Mac

- From the Intel Mac run (a reply stopped at its first character, then continue left it `Stopped` with no new text): the kit's path is correct. With the scripted host, a reply stopped at its first delta continues in the same turn; on the real harness, a late or doubled cancel does not stop the continue that follows. Both are tested now.
- Fixed: a continue the host refused was dropped silently, so the turn stayed `Stopped` with nothing said, which matches what was seen. The reason now shows on the turn (`Turn::refusal`) and the view reports `HostError`. The refusal clears when the host opens the turn again.

## 2026-11-15 · Retry and error states

**Built**

- Model: a turn shows `retrying` while the host retries a transient error, cleared when the reply or the turn's end arrives. `Thread::awaiting_client` follows `session.status`. `Thread::take_retry` gives the prompt of a failed or rejected last turn; a rejected send the host never opened is dropped first, so trying again replaces it.
- View: "Try again" under a failed or rejected last turn. A note while the host retries. A line above the composer while reconnecting, while a tool's app is away, or once the session is closed, with the composer disabled when closed.
- Reconnect: when a session's stream ends, the view follows it again from the last `seq` it saw, waiting 250 ms and doubling to at most 8 s. Nothing is missed or shown twice. A refusal other than `disconnected` stops it, and so does a closed session.
- `ScriptedHost::disconnect` ends every open stream, for tests.
- CLAUDE.md: the budget rule (one paid check run per push).

**Not yet**

- The first-run hand-off from the window: its shape is not specified here (asked in Worker reports).

## 2026-11-15 · Model choice

**Built**

- Kit: `ModelCatalog` (chat models only; the recommended one first, then those that can answer, then downloading, not installed and unavailable) with `choose`, which names a listed model that can answer, or nothing. A model is never assumed to exist. `ConversationHost::models` loads it: `SdkHost` reads `models.list` plus each model's `models.info` state, and a model whose state cannot be read is `Unknown`, never installed.
- `ConversationView` offers the catalog in the composer's model menu. A pick applies to the session the view creates (`NewSession.model`) and is reported as `ModelSelected`. `capability_denied`, `unknown_model` and `model_unavailable` read in plain words (`ConversationCopy`); `model_unavailable` also reports `ModelUnavailable` so the app can offer the download.
- Chat applies the last picked model to every new chat. The manifest asks for `models` with `any: true`, local only.

**Waiting on the SDK**

- `ModelInfo.recommended` and `sessions.create { model }` are in review (needs-master). Until they merge, the catalog has no recommendation and the preset's model runs. Switching model inside a chat is not in version 1.

## 2026-11-15 · Everything through the SDK

- `SDK_REF` moves to `c0d2bf7`; every harness crate is pinned at `f061211`, the SDK's URL and rev character for character.
- The conversation components come through the SDK (`isonomic_app::extra`); no crate depends on isonomic-ui directly. The `../isonomic-ui` checkout stays for the macros patch.
- Continuing a first reply after its title step works (harness #37) and is tested.

## 2026-11-15 · One UI pin

- `UI_REF` is gone. The shared workflow reads the pin from the SDK checkout (isonomic-org/.github #8), so the SDK's `UI_REF` is the one pin.

## 2026-11-15 · Chats that persist

**Built**

- Kit: `ConversationHost` gains `list` (newest first, archived or not, by title text, with a limit) and `archive`, as `SessionEntry` and `SessionQuery`. `SdkHost` maps them onto `sessions.list` and `sessions.archive`; `ScriptedHost` keeps them in memory.
- Chat: with a `Backend` (a host and a store), the sidebar lists the host's chats instead of the examples. Opening one replays its history; rename goes to `sessions.rename`; delete archives it and drops its pin. A new chat takes the host's session id as soon as the host creates it. A list that arrives after a chat was started keeps that chat.
- Chat's own state is only the pins, in its schema `chat` (`store.rs`): a `pins` table of `session text` and `pinned_at timestamptz`, one statement per call. The manifest now asks for `sql` and declares the schema. `MemoryStore` serves tests and the dev window. Writes in flight are counted (`is_saving`), and the last failed write is kept (`save_error`).
- Tests: the mounted app lists the host's chats with their pins, and writes a pin in Chat's schema; a stored chat reopens with its history, and rename and delete reach the host.

**Notes**

- The SDK testkit's database is scripted and runs no SQL, so the tests check the statements Chat sends and the rows it reads back, not a real database.
- History lives in the host's session log, so `blobs` is not used yet; attachments will use it.

## 2026-11-15 · Isonomic UI through the SDK

- `SDK_REF` moves to `c68bf0d`. `crates/ui` re-exports `isonomic_app::ui` instead of depending on isonomic-ui, so the SDK's `UI_REF` is the pin. The conversation components still come from `isonomic-ui-extra` directly until the SDK re-exports them (requests O8). `UI_REF` stays until the shared workflow reads the pin from the SDK checkout.
- The tokio workaround for the SDK's `TestHost` is gone (isonomic-sdk #22).

Earlier entries: `status-archive.md`.
