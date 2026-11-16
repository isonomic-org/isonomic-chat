# Design differences, version 0.1 (superseded)

Kept for history; `docs/design-diffs.md` holds the version 0.2 tables. Several rows here no longer hold (the context panel, Share and the 1,100 / 860 breakpoints are gone).

The tables below compare the older version 0.1 reference ("Start a Basic Chat / Flow v1", `19:992`) and are kept until the home and thread layouts move to 0.2.

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
| H1 | Composer toolbar: "Add files", "Voice", model selector, "Send message" | Isonomic UI's prompt composer: the plus menu, the model menu (when models are known), and the send arrow, which becomes stop | Isonomic UI as built is the design source; dictation is off and voice is out of scope |
| H2 | Model selector shows a third-party model name and logo | "Local model", no logo | No third-party names |
| H3 | Send disabled: blue at 40% opacity | The theme's disabled primary | D3 |
| H4 | Greeting mark: the old logo glyph | The Isonomic mark from the icon set | The icon set has it |
| H5 | Chips: Write, Learn, Code, Life stuff, Brainstorm | Same labels and order, nearest icons; each opens three example prompts | D2 |

## Thread (frames 03–05)

The thread is the kit's `ConversationView`: the prompt on the trailing side on a muted bubble, the reply as Markdown, notes and Continue or Try again under a turn, a quiet divider for a compaction, and the composer with the disclaimer under it. Example chats with no conversation behind them show one muted line.

| Id | Figma | Built | Why |
| --- | --- | --- | --- |
| L1 | Fixed three-column layout | Below 1,100 px the context panel gives way, below 860 px the sidebar too (the header's toggle opens it); both come back when the window widens | Narrow windows keep the conversation readable |
| L2 | Empty sidebar, no error state | "Chats you start appear here." and a failed list with Try again, in the sidebar's footer | Isonomic UI's sidebar takes only menus in its body; its footer is the slot for a note |
| L3 | Greeting with the person's name | No name when Chat runs on a backend: the SDK has no identity in version 1 | Never invent a name |

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
