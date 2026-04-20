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
    pub icon_subtle: Color,
    pub icon_disabled: Color,
    pub panel_icon: Color,
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
            window_bg: Color::from_rgb8(0xFA, 0xFA, 0xFA),
            toolbar_bg: Color::from_rgb8(0xFA, 0xFA, 0xFA),
            panel_bg: Color::from_rgb8(0xFF, 0xFF, 0xFF),
            panel_header_bg: Color::from_rgb8(0xFA, 0xFA, 0xFA),
            canvas_surround_bg: Color::from_rgb8(0xD7, 0xD1, 0xC7),
            canvas_page_bg: Color::from_rgb8(0xFF, 0xFF, 0xFF),
            border_subtle: Color::from_rgb8(0xE6, 0xE6, 0xE6),
            border_strong: Color::from_rgb8(0xB9, 0xB9, 0xBF),
            text_primary: Color::from_rgb8(0x2A, 0x2A, 0x2D),
            text_muted: Color::from_rgb8(0x6E, 0x6E, 0x75),
            icon_subtle: Color::from_rgb8(0x4A, 0x4A, 0x50),
            icon_disabled: Color::from_rgba8(0x6E, 0x6E, 0x75, 0.22),
            panel_icon: Color::from_rgb8(0x55, 0x55, 0x5A),
            hover_bg: Color::from_rgb8(0xEB, 0xEB, 0xEE),
            selected_bg: Color::from_rgb8(0xCF, 0xE2, 0xF8),
            toolbox_selected_bg: Color::from_rgb8(0xE4, 0xE7, 0xEC),
            toolbox_hover_bg: Color::from_rgb8(0xEF, 0xF0, 0xF3),
            status_bg: Color::from_rgb8(0xFA, 0xFA, 0xFA),
            checker_light: Color::from_rgb8(0xF4, 0xF4, 0xF4),
            checker_dark: Color::from_rgb8(0xE7, 0xE7, 0xE7),
        }
    }
}
