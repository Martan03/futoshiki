use termint::enums::Color;

#[derive(Debug, PartialEq, Clone)]
pub struct Theme {
    pub border: Color,
    pub background: Color,
    pub foreground: Color,
    pub help: Color,
    pub select: Color,
}

impl Theme {
    /// Gets dark theme - dark background, light foreground
    pub fn dark() -> Self {
        Self {
            border: Color::Gray,
            background: Color::Black,
            foreground: Color::White,
            help: Color::Gray,
            select: Color::Cyan,
        }
    }

    /// Gets light theme - light background, dark foreground
    pub fn light() -> Self {
        Self {
            border: Color::Gray,
            background: Color::White,
            foreground: Color::Black,
            help: Color::Gray,
            select: Color::Cyan,
        }
    }
}

impl Default for Theme {
    fn default() -> Self {
        Self {
            border: Color::Gray,
            background: Default::default(),
            foreground: Color::LightGray,
            help: Color::Gray,
            select: Color::Cyan,
        }
    }
}
