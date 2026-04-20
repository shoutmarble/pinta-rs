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
use message::AppMessage;
use state::AppState;
use std::env;
use std::thread;
use std::time::Duration;

pub(crate) const DEFAULT_WINDOW_WIDTH: f32 = 1100.0;
pub(crate) const DEFAULT_WINDOW_HEIGHT: f32 = 750.0;

fn main() -> iced::Result {
    iced::application(boot, update, view)
        .title(title)
        .theme(|state: &AppState| state.theme.iced_theme())
        .window_size((DEFAULT_WINDOW_WIDTH, DEFAULT_WINDOW_HEIGHT))
        .run()
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
