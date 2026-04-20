use glam::DVec2;
use pinta_theme::PintaTheme;
use pinta_ui::widgets::{canvas_viewport::ViewportState, icon::IconKind};
use std::env;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ToolKind {
    MoveSelectedPixels,
    MoveSelection,
    Zoom,
    Pan,
    RectSelect,
    EllipseSelect,
    LassoSelect,
    MagicWandSelect,
    Paintbrush,
    Pencil,
    Eraser,
    PaintBucket,
    Gradient,
    ColorPicker,
    Text,
    LineCurve,
    Rectangle,
    RoundedRectangle,
    Ellipse,
    FreeformShape,
    CloneStamp,
    Recolor,
}

impl ToolKind {
    pub fn toolbox_order() -> &'static [Self] {
        &[
            Self::MoveSelectedPixels,
            Self::MoveSelection,
            Self::Zoom,
            Self::Pan,
            Self::RectSelect,
            Self::EllipseSelect,
            Self::LassoSelect,
            Self::MagicWandSelect,
            Self::Paintbrush,
            Self::Pencil,
            Self::Eraser,
            Self::PaintBucket,
            Self::Gradient,
            Self::ColorPicker,
            Self::Text,
            Self::LineCurve,
            Self::Rectangle,
            Self::RoundedRectangle,
            Self::Ellipse,
            Self::FreeformShape,
            Self::CloneStamp,
            Self::Recolor,
        ]
    }

    pub fn icon_kind(self) -> IconKind {
        match self {
            Self::MoveSelectedPixels => IconKind::MovePixels,
            Self::MoveSelection => IconKind::MoveSelection,
            Self::Zoom => IconKind::Zoom,
            Self::Pan => IconKind::Pan,
            Self::RectSelect => IconKind::RectSelect,
            Self::EllipseSelect => IconKind::EllipseSelect,
            Self::LassoSelect => IconKind::LassoSelect,
            Self::MagicWandSelect => IconKind::MagicWand,
            Self::Paintbrush => IconKind::Paintbrush,
            Self::Pencil => IconKind::Pencil,
            Self::Eraser => IconKind::Eraser,
            Self::PaintBucket => IconKind::PaintBucket,
            Self::Gradient => IconKind::Gradient,
            Self::ColorPicker => IconKind::ColorPicker,
            Self::Text => IconKind::Text,
            Self::LineCurve => IconKind::LineCurve,
            Self::Rectangle => IconKind::Rectangle,
            Self::RoundedRectangle => IconKind::RoundedRectangle,
            Self::Ellipse => IconKind::Ellipse,
            Self::FreeformShape => IconKind::Freeform,
            Self::CloneStamp => IconKind::CloneStamp,
            Self::Recolor => IconKind::Recolor,
        }
    }

    pub fn from_env(value: &str) -> Option<Self> {
        match value.trim().to_ascii_lowercase().as_str() {
            "move-selected-pixels" | "move_pixels" | "moveselectedpixels" => {
                Some(Self::MoveSelectedPixels)
            }
            "move-selection" | "move_selection" | "moveselection" => Some(Self::MoveSelection),
            "zoom" => Some(Self::Zoom),
            "pan" => Some(Self::Pan),
            "rectangle-select" | "rect-select" | "rectselect" => Some(Self::RectSelect),
            "ellipse-select" | "ellipseselect" => Some(Self::EllipseSelect),
            "lasso-select" | "lassoselect" => Some(Self::LassoSelect),
            "magic-wand-select" | "magicwand" | "magic-wand" => Some(Self::MagicWandSelect),
            "paintbrush" => Some(Self::Paintbrush),
            "pencil" => Some(Self::Pencil),
            "eraser" => Some(Self::Eraser),
            "paint-bucket" | "paintbucket" | "bucket" => Some(Self::PaintBucket),
            "gradient" => Some(Self::Gradient),
            "color-picker" | "colorpicker" => Some(Self::ColorPicker),
            "text" => Some(Self::Text),
            "line-curve" | "linecurve" => Some(Self::LineCurve),
            "rectangle" => Some(Self::Rectangle),
            "rounded-rectangle" | "roundedrectangle" => Some(Self::RoundedRectangle),
            "ellipse" => Some(Self::Ellipse),
            "freeform-shape" | "freeformshape" | "freeform" => Some(Self::FreeformShape),
            "clone-stamp" | "clonestamp" => Some(Self::CloneStamp),
            "recolor" => Some(Self::Recolor),
            _ => None,
        }
    }
}

#[derive(Debug, Clone)]
pub struct PencilSession {
    pub points: Vec<DVec2>,
}

#[derive(Debug, Clone)]
pub struct MockScenario {
    pub active: bool,
}

impl Default for MockScenario {
    fn default() -> Self {
        Self { active: false }
    }
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
    pub lock_status_cursor: bool,
    pub history: Vec<String>,
    pub layers: Vec<String>,
    pub pencil_session: Option<PencilSession>,
    pub mock_scenario: MockScenario,
}

impl Default for AppState {
    fn default() -> Self {
        let lock_status_cursor = env::var("PINTA_MOCK_DIAGNOSTICS_DIR").is_ok()
            || env::var("PINTA_MOCK_CAPTURE_PATH").is_ok();
        let viewport = ViewportState::default();

        Self {
            theme: PintaTheme::default(),
            document_name: "sample-input.png".to_string(),
            active_tool: env::var("PINTA_MOCK_ACTIVE_TOOL")
                .ok()
                .as_deref()
                .and_then(ToolKind::from_env)
                .unwrap_or(ToolKind::Paintbrush),
            brush_width: 2,
            zoom_percent: (viewport.zoom * 100.0).round() as u32,
            viewport,
            cursor_text: "0, 0".to_string(),
            image_text: "800, 600".to_string(),
            selection_text: "0, 0, 0, 0".to_string(),
            lock_status_cursor,
            history: vec!["Open Image".to_string()],
            layers: vec!["sample-input.png".to_string()],
            pencil_session: None,
            mock_scenario: MockScenario {
                active: matches!(env::var("PINTA_MOCK_TOOL_SCENARIO"), Ok(value) if value == "1"),
            },
        }
    }
}
