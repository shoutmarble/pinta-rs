use glam::DVec2;
use iced::{Element, Task, window};
use image::{ImageBuffer, ImageFormat, Rgba};
use pinta_ui::widgets::canvas_viewport::CanvasAction;

use crate::message::AppMessage;
use crate::state::{AppState, PencilSession, ToolKind};
use crate::tools::pencil;
use crate::view::main_window;

pub fn title(state: &AppState) -> String {
    format!("{} - Pinta", state.document_name)
}

pub fn update(state: &mut AppState, message: AppMessage) -> Task<AppMessage> {
    match message {
        AppMessage::ToolSelected(tool) => state.active_tool = tool,
        AppMessage::CaptureFinished => {}
        AppMessage::CaptureRequested(output_path) => {
            return window::get_latest().then(move |maybe_id| {
                let Some(id) = maybe_id else {
                    return Task::none();
                };

                let capture_path = output_path.clone();

                window::screenshot(id).then(move |screenshot| {
                    let output_path = capture_path.clone();

                    Task::perform(
                        async move {
                            save_window_capture(output_path, screenshot);
                        },
                        |_| AppMessage::CaptureFinished,
                    )
                })
            });
        }
        AppMessage::Canvas(action) => match action {
            CanvasAction::CursorMoved(screen) => on_canvas_moved(state, screen),
            CanvasAction::Pressed(screen) => on_canvas_pressed(state, screen),
            CanvasAction::Released(screen) => on_canvas_released(state, screen),
            CanvasAction::Scrolled { delta_lines, cursor } => {
                let next_zoom = (state.viewport.zoom + delta_lines * 0.1).clamp(0.1, 16.0);
                state.viewport.zoom_about_screen_point(cursor, next_zoom);
                state.zoom_percent = (state.viewport.zoom * 100.0).round() as u32;
            }
        },
    }

    Task::none()
}

pub fn view(state: &AppState) -> Element<'_, AppMessage> {
    main_window::view(state)
}

fn save_window_capture(output_path: String, screenshot: window::Screenshot) {
    eprintln!(
        "capture screenshot size={}x{} bytes={}",
        screenshot.size.width,
        screenshot.size.height,
        screenshot.bytes.len()
    );

    let Some(image) = ImageBuffer::<Rgba<u8>, _>::from_raw(
        screenshot.size.width,
        screenshot.size.height,
        screenshot.bytes.to_vec(),
    ) else {
        eprintln!("capture failed: screenshot bytes did not match image dimensions");
        return;
    };

    let temp_path = format!("{output_path}.tmp");
    let ready_path = format!("{output_path}.ready");

    if image.save_with_format(&temp_path, ImageFormat::Png).is_ok() {
        if std::fs::rename(&temp_path, &output_path).is_ok() {
            let _ = std::fs::write(&ready_path, b"ok");
            eprintln!("capture saved to {output_path}");
        }
    } else {
        eprintln!("capture failed: could not encode png to {temp_path}");
    }
}

fn on_canvas_pressed(state: &mut AppState, screen: DVec2) {
    let image = state.viewport.screen_to_image(screen);
    state.cursor_text = format!("{:.0}, {:.0}", image.x, image.y);

    if state.active_tool == ToolKind::Pencil {
        state.pencil_session = Some(PencilSession { points: vec![image] });
        pencil::begin_stroke(state, image);
    }
}

fn on_canvas_moved(state: &mut AppState, screen: DVec2) {
    let image = state.viewport.screen_to_image(screen);
    state.viewport.hovered_image_pos = Some(image);
    state.cursor_text = format!("{:.0}, {:.0}", image.x, image.y);

    if state.active_tool == ToolKind::Pencil {
        pencil::extend_stroke(state, image);
    }
}

fn on_canvas_released(state: &mut AppState, screen: DVec2) {
    let image = state.viewport.screen_to_image(screen);
    state.cursor_text = format!("{:.0}, {:.0}", image.x, image.y);

    if state.active_tool == ToolKind::Pencil {
        pencil::end_stroke(state, image);
    }
}
