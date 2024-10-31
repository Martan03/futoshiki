use crossterm::event::{KeyCode, KeyEvent};
use termint::{
    enums::Color,
    style::Style,
    widgets::{Block, Element, Layout, List},
};

use crate::{
    app::{App, Screen},
    error::Error,
};

impl App {
    /// Renders the mode picker screen
    pub fn render_mp(&mut self) -> Element {
        // let game = self.render_game();

        let list = List::new(["Build", "Play"], self.mp_state.clone())
            .selected_style(Style::new().fg(Color::Cyan));
        let mut block = Block::vertical().title("Mode picker");
        block.push(list, 0..);

        let mut wrapper = Layout::horizontal().center();
        wrapper.push(block, 0..);
        let mut layout = Layout::vertical().center();
        layout.push(wrapper, 0..);

        layout.into()
    }

    /// Handles key events when in mode picker screen
    pub fn listen_mp(&mut self, event: KeyEvent) -> Result<(), Error> {
        match event.code {
            KeyCode::Enter => {
                self.mp_select_screen();
                return Ok(self.render()?);
            }
            KeyCode::Up | KeyCode::Char('k') => self.mp_checked_sub(),
            KeyCode::Down | KeyCode::Char('j') => self.mp_checked_add(2),
            KeyCode::Char('q') | KeyCode::Esc => return Err(Error::Exit),
            _ => return Ok(()),
        }
        Ok(self.term.rerender()?)
    }
}

impl App {
    /// Selects currently selected screen to be rendered next
    fn mp_select_screen(&mut self) {
        let state = self.mp_state.borrow();
        let Some(sel) = state.selected else {
            return;
        };

        match sel {
            0 => self.screen = Screen::Builder,
            _ => self.screen = Screen::Game,
        }
    }

    /// Sets selected mode to the next value if next exists
    fn mp_checked_add(&mut self, ceil: usize) {
        let mut state = self.mp_state.borrow_mut();
        let Some(sel) = state.selected else {
            return;
        };

        if sel + 1 < ceil {
            state.selected = Some(sel + 1);
        }
    }

    /// Sets selected mode to the previous value if previous exists
    fn mp_checked_sub(&mut self) {
        let mut state = self.mp_state.borrow_mut();
        let Some(sel) = state.selected else {
            return;
        };

        state.selected = Some(sel.saturating_sub(1));
    }
}
