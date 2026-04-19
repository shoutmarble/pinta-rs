mod app;
mod message;
mod state;
mod tools;
mod view;
mod viewport;

use app::{title, update, view};
use iced::Task;
use message::AppMessage;
use state::AppState;
use std::env;
use std::process::Command;
use std::thread;
use std::time::Duration;

fn main() -> iced::Result {
    iced::application(title, update, view)
        .window_size((1100.0, 750.0))
        .run_with(|| (AppState::default(), initial_task()))
}

fn initial_task() -> Task<AppMessage> {
    let Some(output_path) = env::var("PINTA_MOCK_CAPTURE_PATH").ok() else {
        return Task::none();
    };

    Task::perform(
        async move {
            thread::sleep(Duration::from_millis(1200));

            let _ = Command::new("spectacle")
                .args([
                    "--background",
                    "--nonotify",
                    "--activewindow",
                    "--delay",
                    "150",
                    "--output",
                    &output_path,
                ])
                .status();
        },
        |_| AppMessage::CaptureFinished,
    )
}
