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
mod screenshots {
    use std::path::PathBuf;
    use std::sync::Arc;
    use std::time::Duration;

    use std::rc::Rc;

    use conversation_host::{ScriptedHost, ScriptedReply};
    use conversation_model::{Availability, ModelCatalog, ModelOption};
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

    /// What a state needs: the window, the shell and, when live, its host.
    struct Scene<'a> {
        cx: &'a mut HeadlessAppContext,
        handle: AnyWindowHandle,
        shell: Entity<ChatShell>,
        host: Option<ScriptedHost>,
    }

    impl Scene<'_> {
        fn shell(
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
        fn send(&mut self, text: &str) {
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
        fn reply(&mut self, text: &str) {
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
        fn again(&mut self, index: usize) {
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
        fn begin_edit(&mut self, index: usize) {
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

        fn host(&self) -> &ScriptedHost {
            self.host.as_ref().expect("a live state")
        }

        fn settle(&mut self) {
            self.cx.run_until_parked();
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

    const ANSWER: &str = "Public-key cryptography uses **two keys**: one you share and one you keep.\n\n1. Anyone can lock a message with your public key.\n2. Only your private key unlocks it.\n\nThat is how a browser and a server agree on a secret without ever sending it.";

    /// A finished exchange, a second reply stopped part way (continue shows),
    /// and the quiet divider of a compaction between them.
    fn thread(scene: &mut Scene) {
        scene
            .host()
            .script(ScriptedReply::text(ANSWER).with_title("Public-key cryptography"));
        scene.host().script(ScriptedReply::text("A digital signature is the reverse: you lock with your private key, and anyone can check it with your public key."));
        scene.send("Explain how public-key cryptography works, in plain terms.");
        scene.host().finish();
        scene.settle();
        let session = scene.host().sessions()[0].clone();
        scene
            .host()
            .compact(&session, "Earlier messages are not shown to the model.");
        scene.reply("And signatures?");
        scene.host().pump(9);
        scene.settle();
        scene.shell(|shell, _, cx| {
            if let Route::Session(id) = shell.route().clone()
                && let Some(thread) = shell.thread(&id).cloned()
            {
                thread.update(cx, |thread, cx| thread.stop(cx));
            }
        });
        scene.settle();
    }

    /// A reply part way through streaming, with reasoning done.
    fn streaming(scene: &mut Scene) {
        scene
            .host()
            .script(ScriptedReply::text(ANSWER).with_reasoning("Plain terms, one analogy, short."));
        scene.send("Explain how public-key cryptography works, in plain terms.");
        scene.host().pump(20);
        scene.settle();
    }

    /// A reply written twice: the second shows, with the pager.
    fn versions(scene: &mut Scene) {
        scene.host().script(
            ScriptedReply::text("Try a short walk after lunch.").with_title("Afternoon energy"),
        );
        scene.send("One tip for afternoon energy?");
        scene.host().finish();
        scene.settle();
        scene.host().script(ScriptedReply::text(
            "Drink a glass of water before you reach for coffee.",
        ));
        scene.again(0);
        scene.host().finish();
        scene.settle();
    }

    /// A sent prompt open for editing, its reply below.
    fn editing(scene: &mut Scene) {
        scene.host().script(
            ScriptedReply::text("Try a short walk after lunch.").with_title("Afternoon energy"),
        );
        scene.send("One tip for afternoon energy?");
        scene.host().finish();
        scene.settle();
        scene.begin_edit(0);
    }

    /// A failed send with try again, and a conversation too large.
    fn errors(scene: &mut Scene) {
        scene.host().script(ScriptedReply::text("Sure."));
        scene.host().script(ScriptedReply::too_large(
            "This conversation is too long for the model. Start a new chat to keep going.",
        ));
        scene.send("Hello");
        scene.host().finish();
        scene.settle();
        scene.reply("One more question");
        scene.settle();
    }

    /// Thinking before any words of the reply.
    fn thinking(scene: &mut Scene) {
        scene.host().script(
            ScriptedReply::text("Here is a plan.")
                .with_reasoning("Picking an analogy that holds up"),
        );
        scene.send("Explain public-key cryptography with one analogy.");
        scene.host().pump(5);
        scene.settle();
    }

    /// A reply with a heading, a list and a code block.
    fn markdown(scene: &mut Scene) {
        scene.host().script(
            ScriptedReply::text(
                "Signing takes three steps.\n\n## The two jobs it does\n\n- Encryption: anyone can lock a message with your public key.\n- Signatures: you sign with your private key.\n\n```rust\nfn sign(message: &[u8], key: &PrivateKey) -> Signature {\n    key.sign(message)\n}\n```",
            )
            .with_title("Signing"),
        );
        scene.send("How do I sign a message?");
        scene.host().finish();
        scene.settle();
    }

    /// A send the host refused: the bar says so and offers Try again.
    fn failed(scene: &mut Scene) {
        scene
            .host()
            .script(ScriptedReply::text("Sure.").with_title("Hello"));
        scene.send("Hello");
        scene.host().finish();
        scene.settle();
        // Nothing is scripted, so the host refuses this one.
        scene.reply("One more question");
        scene.settle();
    }

    /// Two installed models and one to download; the recommended one first.
    fn models(host: &ScriptedHost) {
        let option = |id: &str, name: &str, availability| ModelOption {
            id: id.into(),
            name: name.into(),
            remote: false,
            availability,
        };
        host.set_models(ModelCatalog::new(
            vec![
                option("small", "Small", Availability::Installed),
                option("medium", "Medium", Availability::Installed),
                option("large", "Large", Availability::NotInstalled),
            ],
            Some("medium".into()),
        ));
    }

    /// No chat model installed: home asks to install the recommended one.
    fn install(host: &ScriptedHost) {
        let option = |id: &str, name: &str| ModelOption {
            id: id.into(),
            name: name.into(),
            remote: false,
            availability: Availability::NotInstalled,
        };
        host.set_models(ModelCatalog::new(
            vec![option("small", "Small"), option("medium", "Medium")],
            Some("medium".into()),
        ));
    }

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
