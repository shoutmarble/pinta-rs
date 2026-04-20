use pinta_ui::widgets::canvas_viewport::CanvasAction;

use crate::state::ToolKind;

#[derive(Debug, Clone)]
pub enum AppMessage {
    ToolSelected(ToolKind),
    Canvas(CanvasAction),
    CaptureRequested(String),
    CaptureFinished,
}

impl From<CanvasAction> for AppMessage {
    fn from(value: CanvasAction) -> Self {
        Self::Canvas(value)
    }
}
