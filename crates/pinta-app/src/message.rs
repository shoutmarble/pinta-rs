use pinta_ui::widgets::canvas_viewport::CanvasAction;

use crate::diagnostics::CaptureRequest;
use crate::state::{PaletteColor, PaletteTarget, ToolKind};

#[derive(Debug, Clone)]
pub enum AppMessage {
    ToolSelected(ToolKind),
    PaletteColorSelected {
        color: PaletteColor,
        target: PaletteTarget,
    },
    PaletteSwapRequested,
    PaletteResetRequested,
    Canvas(CanvasAction),
    CaptureRequested(CaptureRequest),
    CaptureFinished,
}

impl From<CanvasAction> for AppMessage {
    fn from(value: CanvasAction) -> Self {
        Self::Canvas(value)
    }
}
