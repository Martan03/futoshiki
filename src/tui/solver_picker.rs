use crossterm::event::{KeyCode, KeyEvent};
use termint::{
    style::Style,
    widgets::{Block, Element, Layout, List, StrSpanExtension},
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
            .selected_style(Style::new().fg(self.theme.select))
            .auto_scroll();
        let mut block = Block::vertical()
            .title("Solver picker".fg(self.theme.border))
            .border_style(Style::new().fg(self.theme.border))
            .style(Style::new().fg(self.theme.foreground));
        block.push(list, 0..);

        let mut wrapper = Layout::horizontal().center();
        wrapper.push(block, 37);
        let mut layout = Layout::vertical().center().bg(self.theme.background);
        layout.push(wrapper, 0..);

        Element::new(layout)
    }

    /// Handles key events when in solver picker screens
    pub fn listen_sp(&mut self, event: KeyEvent) -> Result<(), Error> {
        match event.code {
            KeyCode::Enter => {
                self.sp_select_screen();
                self.screen = Screen::Solver;
                return self.render();
            }
            KeyCode::Up | KeyCode::Char('k') => self.sp_checked_sub(),
            KeyCode::Down | KeyCode::Char('j') => self.sp_checked_add(),
            KeyCode::Tab => {
                self.screen = Screen::Solver;
                return self.render();
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
