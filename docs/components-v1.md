# Agent components for version 1: fit against the kit

Checked against isonomic-ui `2e3116b` (the SDK's `UI_REF` at `439a069`), through `isonomic_app::extra`. Version 1 is plain chat: tools, approvals, questions, reasoning steps and memory stay out. Which component each screen uses comes from the version 0.2 layout reference (`docs/design/v0.2/`), not from this note.

| Component | What it needs | What the kit has | Gap |
| --- | --- | --- | --- |
| Streamed answer | The question, the answer text, streaming on or off, optional sources and follow-ups; emits stop, regenerate, copy, feedback | `Turn::prompt`, `Turn::reply()`, `TurnState`; stop maps to `cancel` | Its markup is a subset: paragraphs, "- " lists, `**bold**`, `[n]` citations. No numbered lists, headings, links, inline code or code fences, which the Markdown text view renders today. It also draws the question above the answer, so the prompt bubble would show twice. |
| Message action bar | The text to copy; regenerate with a count of versions and the current one; votes; read aloud | The reply text | Regenerate is `sessions.fork` with reason `regenerate`: `ConversationHost` has no `fork`, and `Thread` does not know sibling versions (a fork is another session). Votes have nowhere to go in the SDK; read aloud has no speech service. Version 1 can offer Copy now, and "try again" once `fork` is in the kit. |
| Model picker | Models: id, name, optional tagline, five-step ratings under named columns, a note; settings with switches and levels | `ModelCatalog`: id, name, remote, availability, recommended | No honest data for taglines or ratings (`ModelInfo` has name, context length, remote, recommended), so those stay empty. The note can say Recommended, Not installed or Downloading. A reasoning switch needs `sessions.create` to take a reasoning level, which it does not. Not-installed models need a disabled or "Install" state in the picker (to confirm). |
| Code block | Code, language, optional filename, copy | Fenced code inside the reply's Markdown | The Markdown text view draws fences itself. Using this component means a fence parser and a named block renderer on the text view (its hooks allow it); whether those hooks keep the tail-only re-parse while a reply streams must be checked. Otherwise, a request to Isonomic UI to let the host render fences. |
| Usage meter | A plan's allowance, parts, limits, renewal | Nothing of the kind | Not a fit: it is a billing card. Left out. |
| Context usage grid | The model's context limit, and tokens per part | `ModelInfo.context_length`; `Usage` on `assistant` and `turn.end` | The engine reports no token use for local models, so the grid cannot be filled honestly. Left out until it does. |

## Kit work these imply

1. `ConversationHost::fork(session, at, reason)` over `sessions.fork`, and sibling versions in the model (which sessions branch from which turn), for regenerate and the version switcher.
2. A fence parser and renderer that put `extra::code_block::CodeBlock` inside replies, with a test that streaming still re-parses only the tail.
3. The picker's options built from `ModelCatalog`, with notes from availability and the recommendation.
