# Design differences

Every visible difference between the design and what this repository draws. The design is version 0.2, written up in `docs/design/v0.2/`; where 0.2 and Isonomic UI differ, the build follows the layout and structure of 0.2 and takes colours, type, icons and components from Isonomic UI, and each difference is recorded here. Screenshots: `cargo test -p isonomic-chat --features test-support --test screenshots` (or under `$SCREENSHOT_DIR`).

## Version 0.2: shell and sidebar

| # | Design 0.2 | Built | Why |
| --- | --- | --- | --- |
| S1 | Collapsed rail 56 px | Isonomic UI's icon rail, 48 px | The sidebar component's collapsed width |
| S2 | "Pinned" group collapses with its caret | Pinned and Recents groups, not collapsible | Isonomic UI's sidebar group has no collapse |
| S3 | A row's "more" button shows on hover only | Shown on every row | The row suffix cannot tell hover; keeping it visible keeps it reachable by keyboard |
| S4 | Status dots: running, unread, attention | Running only (grey), while a chat's reply is written | Unread and attention are outside version 1 |
| S5 | Account row at the foot | Left out | The window does not hand Chat the person's ID yet |
| S6 | Search palette: "Search or ask…", tabs, filters, compose mode, groups by status | Isonomic UI's command palette: "Type a command", chats by title and the New chat action | Version 1 searches chats by title; the palette's placeholder is fixed |
| S7 | Wordmark in the display serif, 20 / 28 semibold | The theme's sans, large, semibold | No fonts added |
| S8 | Header: device, side panel, Share; home: incognito | Title with its menu (Rename, Pin or Unpin, Delete) on a chat; nothing on home | Outside version 1 |
| S9 | Narrow windows not shown | Below 1,040 px the sidebar collapses to its rail; it expands again when the window widens | Derived from the collapse rule |
| S10 | Grey palette (`gray/50` sidebar, `gray/200` selected row) | Isonomic UI's sidebar and theme tokens, light and dark | Tokens come from Isonomic UI |

## Version 0.2: home and composer

| # | Design 0.2 | Built | Why |
| --- | --- | --- | --- |
| H1 | Greeting mark is the brand face, 3xl serif text | Isonomic UI's `sparkles` icon at 32 px and the theme's sans, 3xl | No brand mark or fonts in Isonomic UI |
| H2 | Composer 720 × 96, radius 16; empty state shows voice | Isonomic UI's prompt composer at 720 wide; empty shows a disabled send | Voice is outside version 1 |
| H3 | No attach button | The composer's plus is shown | Isonomic UI's prompt composer cannot hide it yet (request filed) |
| H4 | Model control in the footer, with taglines and ratings | Isonomic UI's model picker, right-aligned; each model carries only Recommended, Not installed, Downloading or Unavailable, in the tagline slot | The catalog's honest data only |
| H5 | Choosing a model that is not installed is not shown | Choosing it asks the window to install it | Decision for version 1 |
| H6 | No-model state not shown | Isonomic UI's empty state: icon, "Install a model to start", one line, one primary button | `first-run-and-system-states.md` uses the empty state |
| H7 | Greeting names the person | No name on a live backend; the example name on example data | The window does not hand Chat the person's name yet |

## Version 0.2: thread

| # | Design 0.2 | Built | Why |
| --- | --- | --- | --- |
| T1 | Prompt bubble wraps at 560 | Isonomic UI's message bubble, which wraps at 80 % of the column (576 at 720) | The component's own limit |
| T2 | Prompt hover actions: time, retry, edit, copy | Copy only, shown on hover; the row keeps its space | Try again sits on the reply; edit is not built yet; turns carry no time |
| T3 | Reply actions: copy, read aloud, thumbs up and down, retry, time; version pager beside them | Isonomic UI's message action bar: Copy and Try again, always shown; the pager ("2 / 2") beside Try again once a reply has versions | Votes and read aloud have no service; no time; the pager is the action bar's own |
| T4 | Stream cursor block under the text | None; the thinking indicator shows until text arrives | The text view streams with its own fade |
| T5 | Thinking row: orb, label, seconds, caret; expands to a summary or log | Isonomic UI's thinking indicator: orb and the last line of thinking, not expandable, no seconds | No expandable thinking row in Isonomic UI |
| T6 | Stopped note "You stopped this response" in the reply | On the last turn, said by the status bar with Continue; on earlier turns, the note under the reply | The status bar is the one stop and error bar |
| T7 | Run status bar: 44 high, radius 10, Steer, Stop, Dismiss, Resume, Retry | A plain placeholder on theme tokens: icon, words, Try again and Continue; no Dismiss, no Steer; stop stays on the composer | No run status bar in Isonomic UI yet (O12); steering is outside version 1 |
| T8 | Errors: only the status bar's Error variant | Every failure of the last turn in the bar, in the danger colours, with Try again where a resend can work; reconnecting and closed there too | Version 1 keeps errors |
| T9 | Code blocks: `CodeBlock` | The text view's fences with a copy button | Tail-only re-parse with a fence renderer is unproven (O9) |
| T10 | Page not found | Not built | No route in version 1 reaches a missing page; a deleted chat returns home |

## Version 0.1 (superseded by 0.2)

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
