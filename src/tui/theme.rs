use ratatui::style::{Color, palette::tailwind};

pub struct Theme {
    pub bg: Color,
    pub highlight: Color,
    pub ink: Color,
    pub red_ink: Color,
    pub blue_ink: Color,
    pub green_ink: Color,
    pub yellow_ink: Color,
}

impl Default for Theme {
    fn default() -> Self {
        Self {
            bg: tailwind::ZINC.c400,
            highlight: tailwind::GRAY.c500,
            ink: tailwind::NEUTRAL.c100,
            red_ink: tailwind::RED.c200,
            blue_ink: tailwind::BLUE.c200,
            green_ink: tailwind::GREEN.c200,
            yellow_ink: tailwind::AMBER.c200,
        }
    }
}
