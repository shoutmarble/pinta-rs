use iced::Color;

#[derive(Debug, Clone)]
pub struct Colors {
    pub window_bg: Color,
    pub toolbar_bg: Color,
    pub sidebar_bg: Color,
    pub panel_bg: Color,
    pub panel_header_bg: Color,
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
    pub status_fg: Color,
    pub checker_light: Color,
    pub checker_dark: Color,
}

impl Default for Colors {
    fn default() -> Self {
        Self {
            window_bg: Color::from_rgb8(0x2B, 0x2A, 0x29),
            toolbar_bg: Color::from_rgb8(0x2B, 0x2A, 0x29),
            sidebar_bg: Color::from_rgb8(0x2B, 0x2A, 0x29),
            panel_bg: Color::from_rgb8(0x24, 0x23, 0x22),
            panel_header_bg: Color::from_rgb8(0x2B, 0x2A, 0x29),
            canvas_page_bg: Color::from_rgb8(0xD2, 0xCC, 0xC2),
            border_subtle: Color::from_rgb8(0x4C, 0x4A, 0x48),
            border_strong: Color::from_rgb8(0x66, 0x63, 0x60),
            text_primary: Color::from_rgb8(0xF2, 0xF0, 0xEE),
            text_muted: Color::from_rgb8(0xBC, 0xB8, 0xB4),
            icon_subtle: Color::from_rgb8(0xDD, 0xD9, 0xD5),
            icon_disabled: Color::from_rgb8(0x6F, 0x6C, 0x69),
            panel_icon: Color::from_rgb8(0xD0, 0xCC, 0xC8),
            hover_bg: Color::from_rgb8(0x3A, 0x38, 0x36),
            selected_bg: Color::from_rgb8(0x2D, 0x4F, 0x79),
            toolbox_selected_bg: Color::from_rgb8(0x44, 0x42, 0x40),
            toolbox_hover_bg: Color::from_rgb8(0x34, 0x32, 0x30),
            status_bg: Color::from_rgb8(0x24, 0x24, 0x24),
            status_fg: Color::from_rgb8(0xF2, 0xF2, 0xF2),
            checker_light: Color::from_rgb8(0xF4, 0xF4, 0xF4),
            checker_dark: Color::from_rgb8(0xE7, 0xE7, 0xE7),
        }
    }
}
