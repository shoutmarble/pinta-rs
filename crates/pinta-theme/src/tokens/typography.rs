#[derive(Debug, Clone)]
pub struct Typography {
    pub title: f32,
    pub panel_title: f32,
    pub body: f32,
    pub caption: f32,
    pub toolbar: f32,
}

impl Default for Typography {
    fn default() -> Self {
        Self {
            title: 16.0,
            panel_title: 13.0,
            body: 14.0,
            caption: 12.0,
            toolbar: 13.0,
        }
    }
}
