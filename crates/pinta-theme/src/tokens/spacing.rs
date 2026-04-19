#[derive(Debug, Clone)]
pub struct Spacing {
    pub xxs: u16,
    pub xs: u16,
    pub sm: u16,
    pub md: u16,
    pub lg: u16,
    pub xl: u16,
}

impl Default for Spacing {
    fn default() -> Self {
        Self {
            xxs: 2,
            xs: 4,
            sm: 6,
            md: 8,
            lg: 12,
            xl: 16,
        }
    }
}
