# Progress

Newest first. Updated at every task boundary so a new session can resume from this repository alone. Detail is in `status.md`.

## 2026-11-27 · The gallery, after the launch decisions

**Where the work is**

| Task | Where | State |
| --- | --- | --- |
| Launch decisions (#49) | issue | closed: every decision carried out |
| The kit's gallery, the fixture player, the boundary and product-term checks | `kit-gallery` pull request | open |
| Approvals live in the window | isonomic-sdk #64 (O20) | waiting on the SDK |
| Ask widgets (questions, choices) | spec, then the kit | next |
| Tool bindings in the kit (#57) | after harness #50 brings `kits-files-read` | next |
| Display widgets (#51) | after the harness display specs and fixture merge | queued |

**Notes for whoever resumes**

- Check out isonomic-sdk next to this repository at `SDK_REF`, and isonomic-ui at the SDK's `UI_REF`. The harness comes by git over SSH; with HTTPS only, point `ssh://git@github.com/` at `https://github.com/` with git's `insteadOf` and set `CARGO_NET_GIT_FETCH_WITH_CLI=true`.
- Linux needs the xkbcommon development libraries to link, and a Vulkan driver (a software one works) for the gallery and Chat's screenshots.
- The gallery: `cargo test -p conversation-ui --features test-support --test gallery`. Chat's screens: `cargo test -p isonomic-chat --features test-support --test screenshots`.
