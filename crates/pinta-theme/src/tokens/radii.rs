#[derive(Debug, Clone)]
pub struct Radii {
    pub sm: f32,
    pub md: f32,
    pub lg: f32,
}

impl Default for Radii {
    fn default() -> Self {
        Self {
            sm: 4.0,
            md: 6.0,
            lg: 8.0,
        }
    }
}
