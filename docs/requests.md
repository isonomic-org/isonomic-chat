# Requests

What this repository needs from others, and what others asked of it. Newest first. Each entry says exactly what to add.

## Out

### O5 · Isonomic UI: sidebar labels

isonomic-ui #210. Long `SidebarMenuItem` labels clip without an ellipsis.

### O4 · Desktop: icons and linking

**To:** isonomic-desktop. **Status:** open.

1. Chat draws icons from Isonomic UI's full catalog (`isonomic_ui::assets::IconName`), not only the default set. The window registers `isonomic_ui::assets::AllAssets`, or an asset source that includes the icons in `chat::icons::ALL`.
2. Chat depends on Isonomic UI through `../isonomic-ui`, pinned by `UI_REF`, like the SDK. Linking `chat` at a pinned revision works when the window and Chat resolve the same `isonomic-ui` checkout; otherwise GPUI is built twice and Chat's views cannot be placed in the window's tabs.

### O3 · Isonomic UI: stacked list item

isonomic-ui #208. For the kit's `ContextItem`.

### O2 · Isonomic UI: Thinking indicator

isonomic-ui #207, from proposal #14. For the kit's `ThinkingIndicator` (story 3).

### O1 · Isonomic UI: Prompt composer

isonomic-ui #206. For the kit's `PromptComposer` (home and thread).

### O0 · `ci.yml`

**Status:** done in #2. The master session's branch `ci/shared-workflow` (issue #1) was merged into the Step 0 branch, and two inputs were added to its call of the shared workflow through the web editor: `ui: true` (Isonomic UI as a sibling checkout at `UI_REF`, which needs the `UI_DEPLOY_KEY` secret) and `features: --all-features --locked`. When `xtask` gains the product-term lint and the boundary check, `xtask: lint boundary` follows the same way.

## In

None yet.
