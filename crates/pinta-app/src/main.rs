mod app;
mod diagnostics;
mod message;
mod state;
mod tools;
mod view;
mod viewport;

use app::{title, update, view};
use diagnostics::CaptureRequest;
use iced::Task;
use iced::window;
use message::AppMessage;
use state::AppState;
use std::env;
use std::thread;
use std::time::Duration;

pub(crate) const DEFAULT_WINDOW_WIDTH: f32 = 1100.0;
pub(crate) const DEFAULT_WINDOW_HEIGHT: f32 = 750.0;
pub(crate) const MIN_WINDOW_WIDTH: f32 = 980.0;
pub(crate) const MIN_WINDOW_HEIGHT: f32 = 620.0;

fn main() -> iced::Result {
    iced::application(boot, update, view)
        .title(title)
        .theme(|state: &AppState| state.theme.iced_theme())
        .window(window::Settings {
            size: iced::Size::new(DEFAULT_WINDOW_WIDTH, DEFAULT_WINDOW_HEIGHT),
            min_size: Some(iced::Size::new(MIN_WINDOW_WIDTH, MIN_WINDOW_HEIGHT)),
            icon: Some(default_window_icon()),
            ..window::Settings::default()
        })
        .run()
}

fn default_window_icon() -> window::Icon {
    let bytes = include_bytes!("../../../../pinta-upstream/Pinta.Resources/icons/hicolor/32x32/apps/com.github.PintaProject.Pinta.png");
    let image = image::load_from_memory(bytes)
        .expect("failed to decode default Pinta PNG icon")
        .into_rgba8();
    let (width, height) = image.dimensions();

    window::icon::from_rgba(image.into_raw(), width, height)
        .expect("failed to build window icon from RGBA pixels")
}

fn boot() -> (AppState, Task<AppMessage>) {
    (AppState::default(), initial_task())
}

fn initial_task() -> Task<AppMessage> {
    let request = CaptureRequest {
        output_path: env::var("PINTA_MOCK_CAPTURE_PATH").ok(),
        diagnostics_root: env::var("PINTA_MOCK_DIAGNOSTICS_DIR").ok(),
        upstream_session_dir: env::var("PINTA_MOCK_UPSTREAM_SESSION_DIR").ok(),
    };

    if request.is_empty() {
        return Task::none();
    }

    Task::perform(
        async move {
            thread::sleep(Duration::from_millis(1200));
            request
        },
        AppMessage::CaptureRequested,
    )
}
