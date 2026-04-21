use iced::{
    Font,
    font::{Family, Weight},
};

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
            title: 14.0,
            panel_title: 13.0,
            body: 12.0,
            caption: 11.0,
            toolbar: 12.0,
        }
    }
}

impl Typography {
    pub fn ui_regular(&self) -> Font {
        Font {
            family: Family::Name("Noto Sans"),
            weight: Weight::Normal,
            ..Font::DEFAULT
        }
    }

    pub fn ui_medium(&self) -> Font {
        Font {
            family: Family::Name("Noto Sans"),
            weight: Weight::Medium,
            ..Font::DEFAULT
        }
    }
}
