// Copyright 2026 Anud Inc.
// SPDX-License-Identifier: GPL-3.0-or-later

//! Renders Chat's shell and home offscreen, light and dark, to
//! `target/screenshots/<theme>/<state>.png` (or under `$SCREENSHOT_DIR`).
//!
//! Needs GPUI's offscreen renderer: Metal on macOS, WGPU on Linux (a software
//! Vulkan adapter such as Mesa lavapipe works). Opt-in:
//!
//! ```sh
//! cargo test -p chat --features test-support --test screenshots
//! ```

fn main() {
    #[cfg(any(target_os = "macos", target_os = "linux"))]
    screenshots::run();
    #[cfg(not(any(target_os = "macos", target_os = "linux")))]
    println!("screenshots: skipped; no headless renderer on this platform");
}

#[cfg(any(target_os = "macos", target_os = "linux"))]
#[path = "screenshots/scenes.rs"]
mod scenes;

#[cfg(any(target_os = "macos", target_os = "linux"))]
mod screenshots {
    use std::path::PathBuf;
    use std::sync::Arc;
    use std::time::Duration;

    use std::rc::Rc;

    use crate::scenes::*;
    use conversation_host::ScriptedHost;
    use isonomic_chat::copy::DayPart;
    use isonomic_chat::{ChatShell, Route};
    use ui::assets::AllAssets;
    use ui::component::{ActiveTheme as _, Theme, ThemeMode};
    use ui::test::TestWindowExt as _;
    use ui::{
        AnyWindowHandle, AppContext as _, Bounds, Entity, HeadlessAppContext, WindowBounds,
        WindowOptions, px, size,
    };

    /// The Figma frames are 1440 x 1024 including the desktop's 40 px rail.
    const WIDE: f32 = 1400.;
    /// A narrow window: the composer wraps and the panels give way.
    const NARROW: f32 = 720.;
    const HEIGHT: f32 = 1024.;
    /// The Mac's window at its usual size (2200 x 1442 at 2x).
    const MAC: f32 = 1100.;

    /// What a state needs: the window, the shell and, when live, its host.
    pub(crate) struct Scene<'a> {
        cx: &'a mut HeadlessAppContext,
        handle: AnyWindowHandle,
        shell: Entity<ChatShell>,
        host: Option<ScriptedHost>,
    }

    impl Scene<'_> {
        pub(crate) fn shell(
            &mut self,
            f: impl FnOnce(&mut ChatShell, &mut ui::Window, &mut ui::Context<ChatShell>),
        ) {
            let shell = self.shell.clone();
            self.cx
                .update_window(self.handle, |_, window, cx| {
                    shell.update(cx, |shell, cx| f(shell, window, cx))
                })
                .unwrap();
            self.cx.run_until_parked();
        }

        /// Type a prompt on home and send it.
        pub(crate) fn send(&mut self, text: &str) {
            let shell = self.shell.clone();
            let text = text.to_owned();
            self.cx
                .update_window(self.handle, |_, window, cx| {
                    shell.update(cx, |shell, cx| shell.set_draft(&text, window, cx));
                    window.render_frame(cx);
                    window.press("enter", cx);
                })
                .unwrap();
            self.cx.run_until_parked();
        }

        /// Type into the open conversation's composer and send it.
        pub(crate) fn reply(&mut self, text: &str) {
            let shell = self.shell.clone();
            let text = text.to_owned();
            self.cx
                .update_window(self.handle, |_, _, cx| {
                    let id = match shell.read(cx).route() {
                        Route::Session(id) => id.clone(),
                        Route::Home => return,
                    };
                    let thread = shell.read(cx).thread(&id).cloned();
                    if let Some(thread) = thread {
                        thread.update(cx, |thread, cx| thread.send(text, cx));
                    }
                })
                .unwrap();
            self.cx.run_until_parked();
        }

        /// Write the open conversation's reply at `index` again.
        pub(crate) fn again(&mut self, index: usize) {
            let shell = self.shell.clone();
            self.cx
                .update_window(self.handle, |_, _, cx| {
                    let Route::Session(id) = shell.read(cx).route().clone() else {
                        return;
                    };
                    if let Some(thread) = shell.read(cx).thread(&id).cloned() {
                        thread.update(cx, |thread, cx| thread.regenerate(index, cx));
                    }
                })
                .unwrap();
            self.cx.run_until_parked();
        }

        /// Open the edit field on the prompt at `index`.
        pub(crate) fn begin_edit(&mut self, index: usize) {
            let shell = self.shell.clone();
            self.cx
                .update_window(self.handle, |_, window, cx| {
                    let Route::Session(id) = shell.read(cx).route().clone() else {
                        return;
                    };
                    if let Some(thread) = shell.read(cx).thread(&id).cloned() {
                        thread.update(cx, |thread, cx| thread.begin_edit(index, window, cx));
                    }
                })
                .unwrap();
            self.cx.run_until_parked();
        }

        pub(crate) fn host(&self) -> &ScriptedHost {
            self.host.as_ref().expect("a live state")
        }

        pub(crate) fn settle(&mut self) {
            self.cx.run_until_parked();
        }

        /// Draw one frame, as the window does between streamed pieces.
        pub(crate) fn frame(&mut self) {
            self.cx
                .update_window(self.handle, |_, window, cx| window.render_frame(cx))
                .unwrap();
        }
    }

    struct State {
        name: &'static str,
        width: f32,
        live: bool,
        /// Runs on the host before the shell opens (the shell loads the
        /// models and the chat list when it opens).
        before: fn(&ScriptedHost),
        setup: fn(&mut Scene),
    }

    fn nothing(_: &ScriptedHost) {}

    const STATES: &[State] = &[
        State {
            name: "01-home",
            width: WIDE,
            live: false,
            before: nothing,
            setup: |_| {},
        },
        State {
            name: "02-composing",
            width: WIDE,
            live: false,
            before: nothing,
            setup: |scene| {
                scene.shell(|shell, window, cx| {
                    shell.set_draft(
                        "Explain how public-key cryptography works, in plain terms.",
                        window,
                        cx,
                    )
                })
            },
        },
        State {
            name: "04-titled",
            width: WIDE,
            live: false,
            before: nothing,
            setup: |scene| {
                scene.shell(|shell, window, cx| shell.open_session("s2".into(), window, cx))
            },
        },
        State {
            name: "sidebar-hidden",
            width: WIDE,
            live: false,
            before: nothing,
            setup: |scene| {
                scene.shell(|shell, window, cx| {
                    shell.open_session("s2".into(), window, cx);
                    shell.set_sidebar_open(false, cx);
                })
            },
        },
        State {
            name: "search",
            width: WIDE,
            live: false,
            before: nothing,
            setup: |scene| scene.shell(|shell, window, cx| shell.open_search(window, cx)),
        },
        State {
            name: "delete-confirm",
            width: WIDE,
            live: false,
            before: nothing,
            setup: |scene| {
                scene.shell(|shell, window, cx| {
                    shell.open_session("s2".into(), window, cx);
                    shell.confirm_delete(&"s2".into(), window, cx);
                })
            },
        },
        State {
            name: "home-narrow",
            width: NARROW,
            live: false,
            before: nothing,
            setup: |_| {},
        },
        State {
            name: "thread",
            width: WIDE,
            live: true,
            before: nothing,
            setup: thread,
        },
        State {
            name: "thread-versions",
            width: WIDE,
            live: true,
            before: nothing,
            setup: versions,
        },
        State {
            name: "thread-editing",
            width: WIDE,
            live: true,
            before: nothing,
            setup: editing,
        },
        State {
            name: "thread-thinking",
            width: WIDE,
            live: true,
            before: nothing,
            setup: thinking,
        },
        State {
            name: "thread-markdown",
            width: WIDE,
            live: true,
            before: nothing,
            setup: markdown,
        },
        State {
            name: "thread-failed",
            width: WIDE,
            live: true,
            before: nothing,
            setup: failed,
        },
        State {
            name: "thread-narrow",
            width: NARROW,
            live: true,
            before: nothing,
            setup: thread,
        },
        State {
            name: "thread-streaming",
            width: WIDE,
            live: true,
            before: nothing,
            setup: streaming,
        },
        State {
            name: "thread-errors",
            width: WIDE,
            live: true,
            before: nothing,
            setup: errors,
        },
        State {
            name: "home-live",
            width: WIDE,
            live: true,
            before: nothing,
            setup: |_| {},
        },
        State {
            name: "chats-failed",
            width: WIDE,
            live: true,
            before: |host| {
                host.refuse_list(conversation_host::HostError::new(
                    "disconnected",
                    "Not connected to the engine.",
                ))
            },
            setup: |_| {},
        },
        State {
            name: "home-mac",
            width: MAC,
            live: true,
            before: models,
            setup: |_| {},
        },
        State {
            name: "thread-mac",
            width: MAC,
            live: true,
            before: nothing,
            setup: thread,
        },
        State {
            name: "thread-earlier-reply",
            width: MAC,
            live: true,
            before: nothing,
            setup: earlier_reply,
        },
        State {
            name: "home-models",
            width: WIDE,
            live: true,
            before: models,
            setup: |_| {},
        },
        State {
            name: "home-install-model",
            width: WIDE,
            live: true,
            before: install,
            setup: |_| {},
        },
    ];

    pub fn run() {
        let out = PathBuf::from(
            std::env::var("SCREENSHOT_DIR").unwrap_or_else(|_| "target/screenshots".into()),
        );
        let mut cx = HeadlessAppContext::with_platform(
            ui::platform::current_platform(true).text_system(),
            Arc::new(AllAssets),
            ui::platform::current_headless_renderer,
        );
        cx.allow_parking();
        cx.update(|cx| {
            ui::init(cx);
            isonomic_chat::init(cx);
        });

        let mut written = 0;
        for (mode, dir) in [(ThemeMode::Light, "light"), (ThemeMode::Dark, "dark")] {
            let dir = out.join(dir);
            std::fs::create_dir_all(&dir).expect("create the screenshot directory");
            for state in STATES {
                cx.update(|cx| Theme::change(mode, None, cx));
                let host = state.live.then(ScriptedHost::new);
                if let Some(host) = &host {
                    (state.before)(host);
                }
                let (handle, shell) = open(&mut cx, host.clone(), state.width);
                let mut scene = Scene {
                    cx: &mut cx,
                    handle,
                    shell: shell.clone(),
                    host,
                };
                scene.shell(|shell, _, cx| shell.set_day_part(DayPart::Morning, cx));
                (state.setup)(&mut scene);
                cx.update_window(handle, |_, window, cx| window.render_frame(cx))
                    .unwrap();
                cx.run_until_parked();
                cx.advance_clock(Duration::from_millis(500));
                cx.run_until_parked();
                cx.update_window(handle, |_, window, cx| window.render_frame(cx))
                    .unwrap();
                let is_dark = cx.update(|cx| cx.theme().is_dark());
                assert_eq!(
                    is_dark,
                    mode.is_dark(),
                    "{} drawn in the wrong theme",
                    state.name
                );

                let image = cx
                    .capture_screenshot(handle)
                    .expect("offscreen rendering must be available");
                let path = dir.join(format!("{}.png", state.name));
                image.save(&path).expect("write the screenshot");
                println!("wrote {}", path.display());
                written += 1;
                cx.update_window(handle, |_, window, _| window.remove_window())
                    .unwrap();
                cx.run_until_parked();
            }
        }
        println!("screenshots: {written} images in {}", out.display());
    }

    fn open(
        cx: &mut HeadlessAppContext,
        host: Option<ScriptedHost>,
        width: f32,
    ) -> (AnyWindowHandle, Entity<ChatShell>) {
        let backend = host.map(|host| isonomic_chat::Backend {
            host: Rc::new(host),
            store: Rc::new(isonomic_chat::store::MemoryStore::default()),
            install_model: None,
        });
        cx.update(|cx| {
            ui::open_window(
                WindowOptions {
                    window_bounds: Some(WindowBounds::Windowed(Bounds {
                        origin: Default::default(),
                        size: size(px(width), px(HEIGHT)),
                    })),
                    focus: false,
                    show: false,
                    ..Default::default()
                },
                cx,
                |window, cx| cx.new(|cx| ChatShell::new(backend, window, cx)),
            )
        })
        .expect("open the headless window")
    }
}
