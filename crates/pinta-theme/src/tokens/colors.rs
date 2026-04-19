use iced::Color;

#[derive(Debug, Clone)]
pub struct Colors {
    pub window_bg: Color,
    pub toolbar_bg: Color,
    pub panel_bg: Color,
    pub panel_header_bg: Color,
    pub canvas_surround_bg: Color,
    pub canvas_page_bg: Color,
    pub border_subtle: Color,
    pub border_strong: Color,
    pub text_primary: Color,
    pub text_muted: Color,
    pub hover_bg: Color,
    pub selected_bg: Color,
    pub toolbox_selected_bg: Color,
    pub toolbox_hover_bg: Color,
    pub status_bg: Color,
    pub checker_light: Color,
    pub checker_dark: Color,
}

impl Default for Colors {
    fn default() -> Self {
        Self {
            window_bg: Color::from_rgb8(0xF6, 0xF6, 0xF7),
            toolbar_bg: Color::from_rgb8(0xF7, 0xF7, 0xF8),
            panel_bg: Color::from_rgb8(0xFB, 0xFB, 0xFC),
            panel_header_bg: Color::from_rgb8(0xF4, 0xF4, 0xF5),
            canvas_surround_bg: Color::from_rgb8(0xF3, 0xF3, 0xF4),
            canvas_page_bg: Color::from_rgb8(0xFF, 0xFF, 0xFF),
            border_subtle: Color::from_rgb8(0xD8, 0xD8, 0xDB),
            border_strong: Color::from_rgb8(0xB9, 0xB9, 0xBF),
            text_primary: Color::from_rgb8(0x2A, 0x2A, 0x2D),
            text_muted: Color::from_rgb8(0x6E, 0x6E, 0x75),
            hover_bg: Color::from_rgb8(0xEB, 0xEB, 0xEE),
            selected_bg: Color::from_rgb8(0xCF, 0xE2, 0xF8),
            toolbox_selected_bg: Color::from_rgb8(0xE4, 0xE7, 0xEC),
            toolbox_hover_bg: Color::from_rgb8(0xEF, 0xF0, 0xF3),
            status_bg: Color::from_rgb8(0xF5, 0xF5, 0xF6),
            checker_light: Color::from_rgb8(0xF4, 0xF4, 0xF4),
            checker_dark: Color::from_rgb8(0xE7, 0xE7, 0xE7),
        }
    }
}
