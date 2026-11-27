// Copyright 2026 Anud Inc.
// SPDX-License-Identifier: GPL-3.0-or-later

//! The kit's gallery: every recorded fixture played through the conversation
//! view, light and dark, drawn offscreen to
//! `target/gallery/<theme>/<fixture>-<session>.png` (or under
//! `$GALLERY_DIR`). Each fixture is drawn twice: opened whole, and replayed
//! word by word with a frame per update, stopped half way through
//! (`-replay`). Replay also reports the slowest frame per fixture.
//!
//! Needs GPUI's offscreen renderer: Metal on macOS, WGPU on Linux (a software
//! Vulkan adapter such as Mesa lavapipe works). Opt-in:
//!
//! ```sh
//! cargo test -p conversation-ui --features test-support --test gallery
//! ```
//!
//! `GALLERY_FIXTURE=<name>` draws one fixture only.

fn main() {
    #[cfg(any(target_os = "macos", target_os = "linux"))]
    gallery::run();
    #[cfg(not(any(target_os = "macos", target_os = "linux")))]
    println!("gallery: skipped; no headless renderer on this platform");
}

#[cfg(any(target_os = "macos", target_os = "linux"))]
mod gallery {
    use conversation_host::{ConversationHost, FixtureHost};
    use conversation_model::schema::SessionId;
    use conversation_ui::{ConversationCopy, ConversationView};
    use harness_testkit::fixtures;
    use std::path::{Path, PathBuf};
    use std::rc::Rc;
    use std::sync::Arc;
    use std::time::{Duration, Instant};
    use ui::assets::AllAssets;
    use ui::component::{ActiveTheme as _, Theme, ThemeMode};
    use ui::test::TestWindowExt as _;
    use ui::{
        AnyWindowHandle, AppContext as _, Bounds, HeadlessAppContext, WindowBounds, WindowOptions,
        px, size,
    };

    const WIDTH: f32 = 960.;
    const HEIGHT: f32 = 900.;
    /// Updates released per frame while replaying the 10,000-event thread.
    const LONG_STEP: usize = 200;

    pub fn run() {
        let out = PathBuf::from(std::env::var("GALLERY_DIR").unwrap_or_else(|_| {
            concat!(env!("CARGO_MANIFEST_DIR"), "/../../target/gallery").into()
        }));
        let only = std::env::var("GALLERY_FIXTURE").ok();
        let mut cx = HeadlessAppContext::with_platform(
            ui::platform::current_platform(true).text_system(),
            Arc::new(AllAssets),
            ui::platform::current_headless_renderer,
        );
        cx.allow_parking();
        cx.update(ui::init);

        let mut written = 0;
        for (mode, theme) in [(ThemeMode::Light, "light"), (ThemeMode::Dark, "dark")] {
            let dir = out.join(theme);
            std::fs::create_dir_all(&dir).expect("create the gallery directory");
            cx.update(|cx| Theme::change(mode, None, cx));
            for fixture in fixtures::all() {
                if only.as_deref().is_some_and(|name| name != fixture.name) {
                    continue;
                }
                let host = FixtureHost::new(&fixture.events);
                for session in host.sessions() {
                    let name = format!("{}-{}", fixture.name, session.as_str());
                    let whole = open(&mut cx, &host, &session);
                    settle(&mut cx, whole);
                    assert_eq!(cx.update(|cx| cx.theme().is_dark()), mode.is_dark());
                    shoot(&mut cx, whole, &dir, &name);
                    written += 1;

                    let replay = FixtureHost::new(&fixture.events);
                    replay.replay();
                    let handle = open(&mut cx, &replay, &session);
                    let total = replay_len(&replay, &session);
                    let step = if fixture.events.len() > 2_000 {
                        LONG_STEP
                    } else {
                        1
                    };
                    let (mut sent, mut slowest) = (0, Duration::ZERO);
                    while sent < total / 2 && replay.advance(step) {
                        sent += step;
                        cx.run_until_parked();
                        let start = Instant::now();
                        frame(&mut cx, handle);
                        slowest = slowest.max(start.elapsed());
                    }
                    shoot(&mut cx, handle, &dir, &format!("{name}-replay"));
                    written += 1;
                    println!(
                        "{theme}/{name}: replayed {sent} of {total} updates, slowest frame {} ms",
                        slowest.as_millis()
                    );
                }
            }
        }
        println!("gallery: {written} images in {}", out.display());
    }

    /// How many updates replay sends for a session: its events plus one
    /// delta per word of each reply.
    fn replay_len(host: &FixtureHost, session: &SessionId) -> usize {
        use conversation_model::schema::{Block, Event};
        host.events(session)
            .iter()
            .map(|e| match &e.event {
                Event::Assistant(reply) => {
                    1 + reply
                        .content
                        .iter()
                        .map(|b| match b {
                            Block::Text { text } | Block::Reasoning { text, .. } => {
                                text.split_inclusive(' ').count()
                            }
                            _ => 0,
                        })
                        .sum::<usize>()
                }
                _ => 1,
            })
            .sum()
    }

    fn open(
        cx: &mut HeadlessAppContext,
        host: &FixtureHost,
        session: &SessionId,
    ) -> AnyWindowHandle {
        let shared: Rc<dyn ConversationHost> = Rc::new(host.clone());
        let session = session.clone();
        let (handle, _) = cx
            .update(|cx| {
                ui::open_window(
                    WindowOptions {
                        window_bounds: Some(WindowBounds::Windowed(Bounds {
                            origin: Default::default(),
                            size: size(px(WIDTH), px(HEIGHT)),
                        })),
                        focus: false,
                        show: false,
                        ..Default::default()
                    },
                    cx,
                    move |window, cx| {
                        cx.new(|cx| {
                            ConversationView::open(
                                shared,
                                session,
                                ConversationCopy::default(),
                                window,
                                cx,
                            )
                        })
                    },
                )
            })
            .expect("open the headless window");
        cx.run_until_parked();
        handle
    }

    fn frame(cx: &mut HeadlessAppContext, handle: AnyWindowHandle) {
        cx.update_window(handle, |_, window, cx| window.render_frame(cx))
            .unwrap();
    }

    fn settle(cx: &mut HeadlessAppContext, handle: AnyWindowHandle) {
        frame(cx, handle);
        cx.run_until_parked();
        cx.advance_clock(Duration::from_millis(500));
        cx.run_until_parked();
        frame(cx, handle);
    }

    fn shoot(cx: &mut HeadlessAppContext, handle: AnyWindowHandle, dir: &Path, name: &str) {
        settle(cx, handle);
        let image = cx
            .capture_screenshot(handle)
            .expect("offscreen rendering must be available");
        let path = dir.join(format!("{name}.png"));
        image.save(&path).expect("write the gallery image");
        println!("wrote {}", path.display());
        cx.update_window(handle, |_, window, _| window.remove_window())
            .unwrap();
        cx.run_until_parked();
    }
}
