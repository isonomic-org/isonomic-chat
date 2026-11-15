# Design differences

Every visible difference between the Isonomic Chat Figma file and what this repository draws. Until Omar decides which source wins, the build uses Isonomic UI's tokens and icons, adds no fonts or icon sets, and records each difference here. When he decides, these become one task.

Figma: `A0H5z7vRJfA8J7HYFy1aKW`, section "Start a Basic Chat / Flow v1" (`19:992`), frames `ChatView/01`–`05`. Isonomic UI at `UI_REF`. Screenshots: `cargo test -p chat --features test-support --test screenshots` writes `crates/chat/target/screenshots/{light,dark}/`.

## Across the file

| # | Figma | Built | Why |
| --- | --- | --- | --- |
| D1 | IBM Plex Serif for the greeting and message bodies | IBM Plex Sans (the bundled interface face) everywhere | No fonts added; Isonomic UI bundles Sans and Mono only |
| D2 | Phosphor icons, regular weight | Isonomic UI's icon set, nearest match per icon (table below) | No icon sets added |
| D3 | Blue accent: selected tab text and underline, Send (`#0161FE`, `#3280FF`) | Ink: the theme's primary for Send and the tab underline | Isonomic UI's design language: the primary action is ink |
| D4 | The kit's gray scale (`gray/200`…`gray/900`) | Theme neutrals: border, input border, muted foreground, foreground, sidebar | Tokens only |
| D5 | Body and sidebar rows 14 px; composer text 16 px | The theme's default text size (larger than 14 px in rows and tabs) | Tokens only; Isonomic UI's 13 px scale is specified but not yet applied |
| D6 | Light only | Light and dark from the theme | The brief requires both |

## Shell

| # | Figma | Built | Why |
| --- | --- | --- | --- |
| S1 | `chat_sidebar` is 272 px including a 40 px `app_rail` | Chat's sidebar is 232 px with no rail | The rail belongs to the desktop shell |
| S2 | Rail top holds "collapse sidebar" and "new chat" | Both sit at the top of Chat's sidebar; when the sidebar is hidden, the toggle moves into the header | Chat owns those actions; the rail does not |
| S3 | Sidebar shows saved prompts and a pinned-models group | Not shown | Not in Milestone 0 |
| S4 | Sidebar shows a Projects group | Not shown | Projects are out of scope for Milestone 0 |
| S5 | User avatar at the foot of the rail | Not shown | The desktop shell owns the account |
| S6 | Chat list rows end in an ellipsis; the first row has a logo and a pin | Long titles are clipped without an ellipsis; pinned rows show a pin and no logo | Isonomic UI's sidebar item does not ellipsize yet (isonomic-ui #210); sessions have no logo |
| S7 | Header Untitled: muted "New chat". Titled: title, caret, Share | Same; the caret opens Rename and Delete; Share is disabled with a tooltip | Sharing lands later, as an outward action through host policy |
| S8 | No control for the context panel | A context-panel toggle at the right of the header | Chat owns when the panel opens |
| S9 | Context panel tabs: Context, Tools, More | Context and Tools | Nothing is defined for More yet |
| S10 | Connections list four named third-party services with their logos | Four generic placeholder connections with icons | No third-party names; real connections come from the desktop's connection settings |
| S11 | Context rows are the kit's stacked-list item | A plain icon-and-two-lines row | Isonomic UI has no stacked list item yet (isonomic-ui #208) |

## Home

| # | Figma | Built | Why |
| --- | --- | --- | --- |
| H1 | Composer toolbar: "Add files", "Voice", model selector, "Send message" | Stand-in: "Add files" (disabled), the model label (disabled, caret on the right), "Send" | Prompt composer not built yet (isonomic-ui #206); voice is out of scope |
| H2 | Model selector shows a third-party model name and logo | "Local model", no logo | No third-party names |
| H3 | Send disabled: blue at 40% opacity | The theme's disabled primary | D3 |
| H4 | Greeting mark: the old logo glyph | The Isonomic mark from the icon set | The icon set has it |
| H5 | Chips: Write, Learn, Code, Life stuff, Brainstorm | Same labels and order, nearest icons; each opens three example prompts | D2 |

## Thread (frames 03–05)

The thread view is the kit's `ConversationView` and arrives with the bindings. Until then, opening a chat shows the titled header, one muted line where the conversation will be, and the docked composer with the disclaimer under it, as in `ChatView/04` and `05`.

## Icon matches

| Figma (Phosphor) | Isonomic UI |
| --- | --- |
| sidebar-simple | panel-left |
| note-pencil | pen |
| push-pin | pin |
| stack (Context tab) | layers |
| wrench (Tools tab) | tools |
| plus | plus |
| paperclip | attachment |
| caret-down | the button's dropdown caret |
| share | share |
| pencil-simple-line (Write) | text-creation |
| graduation-cap (Learn) | education |
| code (Code) | code |
| coffee (Life stuff) | cafe |
| lightbulb (Brainstorm) | idea |
| greeting logo | isonomic |
| Connection logos | folder, notebook, layers, chat |
