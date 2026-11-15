# isonomic-chat

Isonomic Chat and the conversation kit. The brief is "Kickoff: Isonomic Chat (app and conversation kit), Milestone 0" in Drive › Isonomic › 2. Kickoff briefs; it is binding. `docs/spec.md` condenses it and `docs/PROGRESS.md` says where the work stands.

## Rules

1. **Never build a presentational component that Isonomic UI has or has queued** (isonomic-ui #1). If one is missing, file an isonomic-ui issue labelled `request`, use a plain placeholder, and track it in `docs/name-map.md`.
2. **Kit by default.** Before writing a type, helper or binding, ask whether Agents, an in-app assistant or the Activity tab could use it. Yes: the kit, with no product terms, behind a slot or registry. Only Chat: `crates/chat`, behind a module boundary. Policy, storage or security: a request to the host. Record every call in `docs/promotions.md`.
3. **Chat never matches on event variants, folds events, renders a tool call or decides an approval.** The boundary check enforces it.
4. **Kit crates never depend on `crates/chat`** and their public types carry no product terms ("chat", "agent", "project", app names). The lint enforces it.
5. **Only `crates/ui` names Isonomic UI or GPUI.** A crate that uses GPUI's macros adds `extern crate ui as gpui;` at its root.
6. **Tokens only.** No raw colours, sizes or durations outside the theme and the layout metrics modules. Light and dark, fully keyboard accessible.
7. **Approvals are deny-safe.** No binding defaults to allow; an ambiguous answer is a denial.
8. **Design differences are recorded,** not resolved: Isonomic UI's tokens and icons win until Omar decides; every visible difference from the Figma file goes in `docs/design-diffs.md`.
9. **Small files.** Every source and doc file stays under 20 KB (`Cargo.lock` is exempt).
10. **Names.** Work is authored as Omar Qadmiri <omar@isonomic.com>. Name no third party anywhere: not in code, docs, issues or pull requests.

## Workflow

- One task per branch and pull request. End every pull request with `ready`, `blocked: …`, `failed: …` or `waiting: …`.
- Merge your own pull request once every check is green. A change to a shared contract (the frozen event schema, the SDK wire interface, isonomic-org/.github) is labelled `needs-master` and waits for the master session. Before each new task, handle open issues labelled `master` first.
- Before pushing: `cargo fmt --all`, `cargo clippy --workspace --all-targets --all-features --locked -- -D warnings`, `cargo test --workspace --all-features --locked`.
- Isonomic UI is a sibling checkout (`../isonomic-ui`) at the commit in `UI_REF`. Bumping it means regenerating `Cargo.lock` in the same pull request.
- Requests to other repositories go in their `docs/requests.md` or as isonomic-ui issues labelled `request`; this repo's are in `docs/requests.md`.
