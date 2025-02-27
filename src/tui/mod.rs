use pareg::FromArg;
use serde::{Deserialize, Serialize};
use theme::Theme;

pub mod game;
pub mod solver_picker;
pub mod theme;

#[derive(
    Debug, Clone, PartialEq, Eq, FromArg, Serialize, Deserialize, Default,
)]
pub enum ThemeType {
    Dark,
    Light,
    #[default]
    Default,
}

impl ThemeType {
    pub fn get_theme(&self) -> Theme {
        match self {
            ThemeType::Dark => Theme::dark(),
            ThemeType::Light => Theme::light(),
            ThemeType::Default => Theme::default(),
        }
    }
}
