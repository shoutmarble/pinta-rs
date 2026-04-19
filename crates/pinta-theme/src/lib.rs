pub mod tokens;

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
