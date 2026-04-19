#[derive(Debug, Clone)]
pub struct Typography {
    pub title: u16,
    pub panel_title: u16,
    pub body: u16,
    pub caption: u16,
    pub toolbar: u16,
}

impl Default for Typography {
    fn default() -> Self {
        Self {
            title: 16,
            panel_title: 13,
            body: 14,
            caption: 12,
            toolbar: 13,
        }
    }
}
