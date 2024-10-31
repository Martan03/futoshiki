use crossterm::event::{KeyCode, KeyEvent};
use termint::{
    enums::Color,
    geometry::Constraint,
    paragraph,
    widgets::{Element, Layout, Paragraph, Spacer, StrSpanExtension},
};

use crate::{app::App, error::Error};

impl App {
    /// Renders the game screen
    pub fn render_game(&mut self) -> Element {
        let mut game = Layout::vertical().center();
        game.push(self.board.clone(), 0..);

        let mut wrapper = Layout::horizontal().center();
        wrapper.push(game, 0..);

        let mut main = Layout::vertical();
        main.push(Spacer::new(), Constraint::Fill(1));
        main.push(wrapper, 0..);
        main.push(Spacer::new(), Constraint::Fill(1));
        main.push(self.render_game_help(), 0..);

        main.into()
    }

    /// Handles key events while in game screen
    pub fn listen_game(&mut self, event: KeyEvent) -> Result<(), Error> {
        match event.code {
            KeyCode::Char('q') | KeyCode::Esc => return Err(Error::Exit),
            _ => return Ok(()),
        }
    }
}

impl App {
    /// Renders the game help screen
    fn render_game_help(&self) -> Paragraph {
        paragraph!(
            "[Arrows/hjkl]Move".fg(Color::Gray),
            "[Esc|q]Quit".fg(Color::Gray),
        )
        .separator(" ")
    }
}
