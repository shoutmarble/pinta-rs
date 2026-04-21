use glam::DVec2;
use iced::{Element, Task, window};
use pinta_ui::widgets::canvas_viewport::CanvasAction;

use crate::diagnostics;
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
        AppMessage::PaletteColorSelected { color, target } => {
            state.set_palette_color(target, color)
        }
        AppMessage::PaletteSwapRequested => state.swap_palette_colors(),
        AppMessage::PaletteResetRequested => state.reset_palette_colors(),
        AppMessage::CaptureFinished => {}
        AppMessage::CaptureRequested(request) => {
            let capture_request = request.clone();
            let capture_state = state.clone();

            return window::latest().then(move |maybe_id| {
                let Some(id) = maybe_id else {
                    return Task::none();
                };

                let request_for_screenshot = capture_request.clone();
                let state_for_screenshot = capture_state.clone();

                window::screenshot(id).then(move |screenshot| {
                    let capture_request = request_for_screenshot.clone();
                    let capture_state = state_for_screenshot.clone();

                    Task::perform(
                        async move {
                            let mut rgba = screenshot.rgba.to_vec();
                            unpremultiply_rgba(&mut rgba);
                            diagnostics::save_artifacts(
                                &capture_request,
                                &capture_state,
                                &rgba,
                                screenshot.size.width,
                                screenshot.size.height,
                            );
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
            CanvasAction::Scrolled {
                delta_lines,
                cursor,
            } => {
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

fn unpremultiply_rgba(rgba: &mut [u8]) {
    for pixel in rgba.chunks_exact_mut(4) {
        let alpha = pixel[3];

        if alpha == 0 {
            pixel[0] = 0;
            pixel[1] = 0;
            pixel[2] = 0;
            continue;
        }

        if alpha == u8::MAX {
            continue;
        }

        let alpha = u16::from(alpha);

        for channel in &mut pixel[..3] {
            let value = (u16::from(*channel) * u16::from(u8::MAX) + alpha / 2) / alpha;
            *channel = value.min(u16::from(u8::MAX)) as u8;
        }
    }
}

fn on_canvas_pressed(state: &mut AppState, screen: DVec2) {
    let image = state.viewport.screen_to_image(screen);
    if !state.lock_status_cursor {
        state.cursor_text = format!("{:.0}, {:.0}", image.x, image.y);
    }

    if state.active_tool == ToolKind::Pencil {
        state.pencil_session = Some(PencilSession {
            points: vec![image],
        });
        pencil::begin_stroke(state, image);
    }
}

fn on_canvas_moved(state: &mut AppState, screen: DVec2) {
    let image = state.viewport.screen_to_image(screen);
    state.viewport.hovered_image_pos = Some(image);
    if !state.lock_status_cursor {
        state.cursor_text = format!("{:.0}, {:.0}", image.x, image.y);
    }

    if state.active_tool == ToolKind::Pencil {
        pencil::extend_stroke(state, image);
    }
}

fn on_canvas_released(state: &mut AppState, screen: DVec2) {
    let image = state.viewport.screen_to_image(screen);
    if !state.lock_status_cursor {
        state.cursor_text = format!("{:.0}, {:.0}", image.x, image.y);
    }

    if state.active_tool == ToolKind::Pencil {
        pencil::end_stroke(state, image);
    }
}
