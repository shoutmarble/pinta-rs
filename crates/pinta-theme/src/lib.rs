pub mod tokens;

use iced::{Color, Theme, theme};

pub use tokens::colors::Colors;
pub use tokens::radii::Radii;
pub use tokens::sizing::Sizing;
pub use tokens::spacing::Spacing;
pub use tokens::typography::Typography;

#[derive(Debug, Clone, Default)]
pub struct PintaTheme {
    pub colors: Colors,
    pub spacing: Spacing,
    pub radii: Radii,
    pub typography: Typography,
    pub sizing: Sizing,
}

impl PintaTheme {
    pub fn iced_theme(&self) -> Theme {
        Theme::custom(
            "Pinta".to_string(),
            theme::Palette {
                background: self.colors.window_bg,
                text: self.colors.text_primary,
                primary: self.colors.selected_bg,
                success: Color::from_rgb8(0x6A, 0x90, 0x4A),
                warning: Color::from_rgb8(0xC2, 0x8A, 0x3B),
                danger: Color::from_rgb8(0xB4, 0x4E, 0x46),
            },
        )
    }
}
