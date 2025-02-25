use pareg::FromArg;
use serde::{Deserialize, Serialize};
use termint::enums::Color;

pub mod builder;
pub mod solver_picker;

#[derive(
    Debug, Clone, PartialEq, Eq, FromArg, Serialize, Deserialize, Default,
)]
pub enum Theme {
    Dark,
    Light,
    #[default]
    Default,
}

impl Theme {
    pub fn get_color(&self) -> Color {
        match self {
            Theme::Dark => Color::Default,
            Theme::Light => Color::White,
            Theme::Default => Color::Default,
        }
    }
}
