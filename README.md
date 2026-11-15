# isonomic-chat

Isonomic Chat, the assistant you talk to, and the conversation kit every Isonomic app uses to show an agent conversation.

## Three layers

| Layer | Owns | Never contains |
| --- | --- | --- |
| Isonomic UI (isonomic-ui) | How things look: presentational components | Session or SDK types |
| The kit (`crates/conversation-*`) | What things mean: folding session events into models, binding them to Isonomic UI | Product terms, screens, navigation, policy |
| Chat (`crates/chat`) | What makes it Chat: layouts, home, settings, copy, its preset | Event matching or folding, tool rendering, approval logic |

The agent loop, tools, policy, approvals and storage run in the engine's host (isonomic-harness).

## Layout

| Path | What |
| --- | --- |
| `crates/ui` | The single re-export of Isonomic UI |
| `crates/chat` | The app: manifest, routes, state, `shell/`, `home/`, `thread/`, `settings/`, `preset/` |
| `crates/conversation-*` | The kit (lands in later pull requests) |
| `examples/dev` | Chat in a standalone window |
| `xtask` | Screenshots, report, product-term lint, boundary check |
| `docs/` | `spec.md`, `status.md`, `PROGRESS.md`, `promotions.md`, `name-map.md`, `design-diffs.md`, `requests.md` |

## Develop

Isonomic UI is a sibling checkout pinned to the commit in `UI_REF`:

```sh
git clone https://github.com/isonomic-org/isonomic-ui ../isonomic-ui
git -C ../isonomic-ui checkout "$(cat UI_REF)"
cargo run                 # Chat in a standalone window
cargo test --workspace --all-features --locked
# Offscreen screenshots of the shell and home, light and dark:
cargo test -p chat --features test-support --test screenshots
```

On Linux, GPUI needs `libxkbcommon-dev libxkbcommon-x11-dev libwayland-dev libasound2-dev libvulkan-dev libx11-xcb-dev libfontconfig-dev libssl-dev`.
