use crossterm::event::{KeyCode, KeyEvent};
use termint::{
    enums::Color,
    style::Style,
    widgets::{Block, Element, Layout, List, Overlay, StrSpanExtension},
};

use crate::{
    app::{App, Screen},
    error::Error,
    solver::SolverType,
};

impl App {
    /// Renders the solver picker
    pub fn render_sp(&mut self) -> Element {
        let solvers: Vec<String> = SolverType::solvers()
            .iter()
            .map(ToString::to_string)
            .collect();

        let list = List::new(solvers, self.sp_state.clone())
            .selected_style(Style::new().fg(Color::Cyan))
            .auto_scroll();
        let mut block = Block::vertical()
            .title("Solver picker".fg(Color::Default))
            .border_style(Style::new().fg(Color::Default))
            .style(Style::new().fg(Color::Default).bg(Color::Default));
        block.push(list, 0..);

        let mut wrapper = Layout::horizontal().center();
        wrapper.push(block, 23);
        let mut layout = Layout::vertical().center();
        layout.push(wrapper, 0..);

        let overlay = Overlay::new(vec![self.render_builder(), layout.into()]);
        Element::new(overlay)
    }

    /// Handles key events when in solver picker screens
    pub fn listen_sp(&mut self, event: KeyEvent) -> Result<(), Error> {
        match event.code {
            KeyCode::Enter => {
                self.sp_select_screen();
                self.screen = Screen::Builder;
                return Ok(self.render()?);
            }
            KeyCode::Up | KeyCode::Char('k') => self.sp_checked_sub(),
            KeyCode::Down | KeyCode::Char('j') => self.sp_checked_add(),
            KeyCode::Tab => {
                self.screen = Screen::Builder;
                return Ok(self.render()?);
            }
            KeyCode::Char('q') | KeyCode::Esc => return Err(Error::Exit),
            _ => return Ok(()),
        };
        Ok(self.term.rerender()?)
    }
}

impl App {
    /// Selects currently selected screen to be rendered next
    fn sp_select_screen(&mut self) {
        let state = self.sp_state.borrow();
        let Some(sel) = state.selected else {
            return;
        };

        self.solver = SolverType::get(sel);
    }

    /// Sets selected mode to the next value if next exists
    fn sp_checked_add(&mut self) {
        let mut state = self.sp_state.borrow_mut();
        let Some(sel) = state.selected else {
            return;
        };

        if sel + 1 < SolverType::solvers().len() {
            state.selected = Some(sel + 1);
        }
    }

    /// Sets selected mode to the previous value if previous exists
    fn sp_checked_sub(&mut self) {
        let mut state = self.sp_state.borrow_mut();
        let Some(sel) = state.selected else {
            return;
        };

        state.selected = Some(sel.saturating_sub(1));
    }
}
