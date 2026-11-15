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

    use isonomic_chat::ChatShell;
    use isonomic_chat::copy::DayPart;
    use ui::assets::AllAssets;
    use ui::component::{ActiveTheme as _, Theme, ThemeMode};
    use ui::test::TestWindowExt as _;
    use ui::{
        AnyWindowHandle, AppContext as _, Bounds, Entity, HeadlessAppContext, WindowBounds,
        WindowOptions, px, size,
    };

    /// The Figma frames are 1440 x 1024 including the desktop's 40 px rail.
    const WIDTH: f32 = 1400.;
    const HEIGHT: f32 = 1024.;

    type Setup = fn(&mut ChatShell, &mut ui::Window, &mut ui::Context<ChatShell>);

    const STATES: &[(&str, Setup)] = &[
        ("01-home", |_, _, _| {}),
        ("02-composing", |shell, window, cx| {
            shell.set_draft(
                "Explain how public-key cryptography works, in plain terms.",
                window,
                cx,
            )
        }),
        ("04-titled", |shell, window, cx| {
            shell.open_session("s2".into(), window, cx)
        }),
        ("sidebar-hidden", |shell, window, cx| {
            shell.open_session("s2".into(), window, cx);
            shell.set_panels(false, false, cx);
        }),
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
            for (name, setup) in STATES {
                cx.update(|cx| Theme::change(mode, None, cx));
                let (handle, shell) = open(&mut cx);
                cx.update_window(handle, |_, window, cx| {
                    shell.update(cx, |shell, cx| {
                        shell.set_day_part(DayPart::Morning, cx);
                        setup(shell, window, cx);
                    });
                    window.render_frame(cx);
                })
                .unwrap();
                cx.run_until_parked();
                cx.advance_clock(Duration::from_millis(500));
                cx.run_until_parked();
                cx.update_window(handle, |_, window, cx| window.render_frame(cx))
                    .unwrap();
                let is_dark = cx.update(|cx| cx.theme().is_dark());
                assert_eq!(is_dark, mode.is_dark(), "{name} drawn in the wrong theme");

                let image = cx
                    .capture_screenshot(handle)
                    .expect("offscreen rendering must be available");
                let path = dir.join(format!("{name}.png"));
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

    fn open(cx: &mut HeadlessAppContext) -> (AnyWindowHandle, Entity<ChatShell>) {
        cx.update(|cx| {
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
                |window, cx| cx.new(|cx| ChatShell::new(None, window, cx)),
            )
        })
        .expect("open the headless window")
    }
}
