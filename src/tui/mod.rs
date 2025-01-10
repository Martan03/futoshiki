use pareg::FromArg;
use termint::enums::Color;

pub mod builder;
pub mod solver_picker;

#[derive(Debug, Clone, PartialEq, Eq, FromArg)]
pub enum Theme {
    Dark,
    Light,
}

impl Theme {
    pub fn get_color(&self) -> Color {
        match self {
            Theme::Dark => Color::Default,
            Theme::Light => Color::White,
        }
    }
}
