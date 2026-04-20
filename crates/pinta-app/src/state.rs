use glam::DVec2;
use pinta_theme::PintaTheme;
use pinta_ui::widgets::canvas_viewport::ViewportState;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ToolKind {
    Pencil,
    Paintbrush,
    Eraser,
    Zoom,
}

impl ToolKind {}

#[derive(Debug, Clone)]
pub struct PencilSession {
    pub points: Vec<DVec2>,
}

#[derive(Debug, Clone)]
pub struct AppState {
    pub theme: PintaTheme,
    pub document_name: String,
    pub active_tool: ToolKind,
    pub brush_width: u32,
    pub zoom_percent: u32,
    pub viewport: ViewportState,
    pub cursor_text: String,
    pub image_text: String,
    pub selection_text: String,
    pub history: Vec<String>,
    pub layers: Vec<String>,
    pub pencil_session: Option<PencilSession>,
}

impl Default for AppState {
    fn default() -> Self {
        Self {
            theme: PintaTheme::default(),
            document_name: "sample-input.png".to_string(),
            active_tool: ToolKind::Paintbrush,
            brush_width: 2,
            zoom_percent: 86,
            viewport: ViewportState::default(),
            cursor_text: "0, 0".to_string(),
            image_text: "800, 600".to_string(),
            selection_text: "0, 0, 0, 0".to_string(),
            history: vec!["Open Image".to_string()],
            layers: vec!["sample-input.png".to_string()],
            pencil_session: None,
        }
    }
}
