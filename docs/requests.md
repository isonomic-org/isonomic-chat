# Requests

What this repository needs from others, and what others asked of it. Newest first. Each entry says exactly what to add.

## Out

### O4 · Desktop: icons and linking

**To:** isonomic-desktop. **Status:** open.

1. Chat draws icons from Isonomic UI's full catalog (`isonomic_ui::assets::IconName`), not only the default set. The window registers `isonomic_ui::assets::AllAssets`, or an asset source that includes the icons Chat lists in one module.
2. Chat depends on Isonomic UI through `../isonomic-ui`, pinned by `UI_REF`, like the SDK. Linking `chat` at a pinned revision works when the window and Chat resolve the same `isonomic-ui` checkout; otherwise GPUI is built twice and Chat's views cannot be placed in the window's tabs.

### O3 · Isonomic UI: stacked list item

isonomic-ui #208. For the kit's `ContextItem`.

### O2 · Isonomic UI: Thinking indicator

isonomic-ui #207, from proposal #14. For the kit's `ThinkingIndicator` (story 3).

### O1 · Isonomic UI: Prompt composer

isonomic-ui #206. For the kit's `PromptComposer` (home and thread).

### O0 · Master session: add `ci.yml`

**To:** the master session. **Status:** open. The connector cannot write workflow files; please add this file through the web editor as `.github/workflows/ci.yml`:

```yaml
name: ci

on:
  push:
    branches: [main]
  pull_request:

concurrency:
  group: ci-${{ github.ref }}
  cancel-in-progress: ${{ github.event_name == 'pull_request' }}

permissions:
  contents: read

jobs:
  rust:
    uses: isonomic-org/.github/.github/workflows/rust-ci.yml@main
    secrets: inherit
    with:
      # --locked rides on the features input so clippy and test both use Cargo.lock.
      features: --all-features --locked
      # GPUI's system libraries on Linux.
      linux-gui-libs: true
      # Isonomic UI as a sibling checkout at the commit in UI_REF (needs UI_DEPLOY_KEY).
      ui: true
```

When `xtask` gains the product-term lint and the boundary check, a later request adds `xtask: lint boundary`.

## In

None yet.
