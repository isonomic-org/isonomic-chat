# Progress

Newest first. Updated at every task boundary so a new session can resume from this repository alone. Detail is in `status.md`.

## 2026-11-15 · Step 0

**Where the work is**

| Task | Pull request | State |
| --- | --- | --- |
| Step 0: workspace, `Cargo.lock`, inventory, name map | #2 | open; CI blocked by the organization's Actions billing |
| Chat shell and home with static data | next | |

**Notes for whoever resumes**

- The GitHub connector cannot create repositories, write `.github/workflows/`, or carry very large files such as `Cargo.lock`. The repository was created in the browser as @QadmiriOmar; large pushes go through git on Omar's Mac (`~/Code/isonomic/repos/isonomic-chat`), whose `gh` is signed in as @QadmiriOmar.
- Isonomic UI is checked out next to this repository at the commit in `UI_REF`.

**Next, in order**

1. Chat's shell and home from `ChatView/01` and `02`, static data.
2. `conversation-model` and `composer-model`, once the harness schema and fixtures merge.
3. The `ConversationHost` trait and the fixture player.
