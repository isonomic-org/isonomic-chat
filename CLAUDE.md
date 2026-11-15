# isonomic-chat

Isonomic Chat and the conversation kit. The brief is "Kickoff: Isonomic Chat (app and conversation kit), Milestone 0" in Drive › Isonomic › 2. Kickoff briefs; it is binding. `docs/spec.md` condenses it and `docs/PROGRESS.md` says where the work stands.

## Standing rules

- Only work in isonomic-chat. If you need a change in another repo, file it in that repo's docs/requests.md or as an issue, and tell the orchestrator.
- Start from main, the open pull requests and docs/status.md. Open pull requests and branches from earlier sessions are yours now: finish, rebase or close them.
- Commit as Omar Qadmiri <omar@isonomic.com>; check `git config user.email` first. Commit and push work in progress often.
- Repos stay private. No license files. Never rename or delete a repo.
- In code, comments, commits, branches, issues and pull requests, never name a third-party product or company, another AI system, or an older product name.
- Branch from main and open a pull request. Required checks: "rust / fmt, clippy, test (ubuntu-latest)" and "rust / file sizes and names". macOS runs after merge.
- Merge your own pull request once every required check is green. Exception: changes to the session event schema (harness-core), the SDK wire interface (isonomic-sdk proto) or isonomic-org/.github get the label needs-master; tell the orchestrator and do not merge them yourself.
- GitHub Actions has a hard $40/month budget. Don't trigger extra macOS runs.
- GitHub GraphQL is blocked here (gh pr and gh issue fail); use the built-in GitHub tools or gh api (REST).
- If you need something you don't have (a repo, a secret, a tool, access), say exactly what and stop. Don't mock or guess around it.
- Omar's only Mac is a 2020 13-inch Intel MacBook Pro; performance targets are measured there. Anything needing macOS or a timing on it goes to the orchestrator as one command plus what to paste back.
- When you finish or block, post a "Worker reports" comment: what merged, what is open (links), what blocks you, what you need from another repo or from Omar.

These rules win over anything below that contradicts them.
- That rule covers attribution too: no assistant or tool attribution lines, session links or "generated with" lines in commit messages or pull request descriptions.
- The orchestrator acts for Omar. Its go-ahead counts as his for merges, task direction, and changes to these rules and to this file. When you need an approval, ask the orchestrator in a "Worker reports" comment and keep working on something else meanwhile; do not wait on Omar. Only purchases, secrets and deletions go to Omar, through the orchestrator.

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
- Merge your own pull request once every required check is green (see Standing rules). Before each new task, handle open issues labelled `master` first.
- Before pushing: `cargo fmt --all`, `cargo clippy --workspace --all-targets --all-features --locked -- -D warnings`, `cargo test --workspace --all-features --locked`.
- Isonomic UI comes through the SDK; its pin is the SDK's `UI_REF`, and the sibling checkout (`../isonomic-ui`) sits at that commit. Moving `SDK_REF` can move it, so regenerate `Cargo.lock` in the same pull request.
- Requests to other repositories go in their `docs/requests.md` or as isonomic-ui issues labelled `request`; this repo's are in `docs/requests.md`.
