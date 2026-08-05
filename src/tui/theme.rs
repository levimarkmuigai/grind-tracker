use ratatui::style::{Color, palette::tailwind};

pub struct Theme {
    pub bg: Color,
    pub highlight: Color,
    pub ink: Color,
}

impl Default for Theme {
    fn default() -> Self {
        Self {
            bg: tailwind::ZINC.c400,
            highlight: tailwind::GRAY.c500,
            ink: tailwind::NEUTRAL.c100,
        }
    }
}
