# Keyboard and accessible names

How a keyboard reaches every control in the version 1 screens, and what assistive technology hears for each control Chat or the conversation kit creates. Audited 16 Nov 2026 from the code; Tab follows paint order (Isonomic UI's root maps Tab and Shift-Tab to the next and previous focusable control).

## Shortcuts

Bound on Chat's root, so they work from any control inside it (`crates/chat/src/lib.rs`). On macOS Mod is Cmd; elsewhere Ctrl.

| Keys | Does |
| --- | --- |
| Mod-Shift-O | New chat: home, composer focused |
| Mod-K | Search: the palette, its field focused |
| Mod-., Mod-B | Expand or collapse the sidebar: Mod-. is the design's key; Mod-B does the same, since on macOS Cmd-. also means cancel |
| Alt-Up, Alt-Down | Open the chat above or below the open one, in sidebar order (from home, Down opens the first); they work with focus in the composer (tested) |
| Enter, Shift-Enter | In the composer: send, new line |
| Escape | Composer while a reply is written: stop. Palette: close, focus back where it was. Rename field and edit box: cancel |
| Mod-Enter | Edit box: save |

## Path through each screen

- **Opening Chat:** focus starts in home's composer, so the first key typed is the prompt.
- **Sidebar:** Tab reaches the toggle, New chat, Search, then each chat row and its More button. Rows show a focus ring; Enter or Space opens one; Up and Down move between rows. Pinned folds with its caret. More's menu holds Open, Pin or Unpin, Rename and Delete.
- **Header:** Tab reaches the title button; its menu holds Pin or Unpin, Rename and Delete. Rename opens a field: Enter keeps the new title, Escape leaves it as it was.
- **Search palette:** opens on its field with nothing selected; typing or an arrow selects; Enter opens, Escape closes; Tab stays inside while it is open. Focus returns to where it was.
- **Home:** composer, then Send once there is text, then the model picker. The picker opens with Enter or Space onto the chosen model; Up, Down, Home and End move; Enter picks. When models could not be listed, Try again sits where the picker would be. With no model installed, Tab reaches Install, which then reads Downloading and is disabled.
- **Thread:** the composer and its Send or Stop; the status bar's buttons (Stop; Dismiss, Retry, Resume; Dismiss, Try again); under each finished reply, Copy, Try again and the pager's arrows; under each prompt, Copy and Edit, which show while one of them has focus; each code block's Copy. Edit opens a field with focus in it; Tab reaches Cancel and Save.
- **Not reached:** turns scrolled out of the virtual list are not laid out, so their buttons are not Tab stops until scrolled into view.

## Accessible names

| Control | Where | Name |
| --- | --- | --- |
| Sidebar toggle | `shell/sidebar.rs` | "Collapse sidebar" or "Expand sidebar", by state |
| New chat, Search | `shell/sidebar.rs` | Their labels |
| Chat row | `shell/sidebar.rs` | The title; "{title}, writing a reply" while one runs |
| Row More | `shell/sidebar.rs` | "More actions for {title}" |
| Row and title menu items | `shell/sidebar.rs`, `shell/header.rs` | Open, Pin or Unpin, Rename, Delete |
| List Try again | `shell/sidebar.rs` | "Try loading chats again" |
| Title button | `shell/header.rs` | "{title}, chat options" |
| Rename field | `shell/header.rs` | "Chat title" |
| Palette rows | Isonomic UI | The chat's title, or New chat |
| Composer, Send, Stop | Isonomic UI | "Message", "Send", "Stop the answer" |
| Model picker | Isonomic UI | "Model: {name}"; rows are list box options named by model and note |
| Install | `home.rs` | "Install {model}", then "Downloading" |
| Models Try again | `home.rs` | "Try again" |
| Prompt Copy, Edit | `conversation-ui/turn.rs` | "Copy prompt", "Edit" |
| Edit field, Cancel, Save | `conversation-ui/turn.rs` | "Edit message", "Cancel", "Save" |
| Code block Copy | `conversation-ui/turn.rs` | "Copy code" |
| Reply Copy, Try again, pager | Isonomic UI's action bar | "Copy", "Try again", "Previous version", "Next version" |
| Status bar | Isonomic UI's run status bar | Role status, named by its words; its action by its label |

## Requests to Isonomic UI

The four asks from the audit (focusable sidebar rows, focus held in the palette, named picker rows, action bar labels) landed in isonomic-ui #381 and are in use.
