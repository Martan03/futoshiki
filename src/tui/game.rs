use crossterm::event::{KeyCode, KeyEvent};
use termint::{
    geometry::{Constraint, Vec2},
    paragraph,
    widgets::{Element, Layout, Paragraph, Spacer, StrSpanExtension},
};

use crate::{
    app::{Action, App, Screen, State},
    board::{board_gen::BoardGen, board_struct::Board},
    checker::Checker,
    error::Error,
};

impl App {
    /// Renders the game in builder mode
    pub fn render_builder(&mut self) -> Element {
        let mut layout = self.render_game(true);
        layout.push(self.render_builder_help(), 0..);
        layout.into()
    }

    /// Renders the game in solver mode
    pub fn render_solver(&mut self) -> Element {
        let mut layout = self.render_game(false);
        layout.push(self.render_solver_help(), 0..);
        layout.into()
    }

    /// Handles key events while in builder screen
    pub fn listen_builder(&mut self, event: KeyEvent) -> Result<(), Error> {
        match event.code {
            KeyCode::Char('>') => self.action = Action::Greater,
            KeyCode::Char('<') => self.action = Action::Lower,
            KeyCode::Char('c') => self.action = Action::Clear,
            KeyCode::Char('b') => {
                self.board.disable_vals();
                self.screen = Screen::Solver;
            }
            KeyCode::Enter => {
                self.board = BoardGen::generate(self.board.size);
                self.board.theme = self.theme.clone();
                self.state = State::Playing;
            }
            KeyCode::Char('r') => self.board.reset(),
            KeyCode::Char('d') => self.board = Board::default(),
            _ => return self.game_listener(event),
        }
        self.render()
    }

    /// Handles key events while in solver screen
    pub fn listen_solver(&mut self, event: KeyEvent) -> Result<(), Error> {
        match event.code {
            KeyCode::Char('s') => {
                self.state = match self.solver.solve(&mut self.board) {
                    true => State::Solved,
                    false => State::Unsolvable,
                };
            }
            KeyCode::Tab => {
                self.sp_state.borrow_mut().selected =
                    Some(self.solver.get_id());
                self.screen = Screen::SolverPicker;
            }
            KeyCode::Char('b') => {
                self.board.enable_vals();
                self.screen = Screen::Builder;
            }
            KeyCode::Enter => {
                self.board = BoardGen::generate(self.board.size);
                self.board.theme = self.theme.clone();
                self.board.disable_vals();
                self.state = State::Playing;
            }
            KeyCode::Char('r') => self.board.reset(),
            KeyCode::Char('d') => self.board = Board::default(),
            _ => return self.game_listener(event),
        }
        self.render()
    }
}

impl App {
    /// Renders the game and adds builder label when in builder mode
    fn render_game(&mut self, builder: bool) -> Layout {
        let mut game = Layout::vertical().center();
        game.push(self.state.to_string(), 0..);
        game.push(self.board.clone(), 0..);

        let mut wrapper = Layout::horizontal().center();
        wrapper.push(game, 0..);

        let mut main = Layout::vertical().bg(self.theme.background);
        if builder {
            main.push("Builder".fg(self.theme.help), 1);
        }

        main.push(Spacer::new(), Constraint::Fill(1));
        main.push(wrapper, 0..);
        main.push(Spacer::new(), Constraint::Fill(1));
        main
    }

    /// Handles shared key events between builder and solver mode
    fn game_listener(&mut self, event: KeyEvent) -> Result<(), Error> {
        match event.code {
            KeyCode::Up | KeyCode::Char('k') => self.move_neg((0, 1)),
            KeyCode::Down | KeyCode::Char('j') => self.move_pos((0, 1)),
            KeyCode::Right | KeyCode::Char('l') => self.move_pos((1, 0)),
            KeyCode::Left | KeyCode::Char('h') => self.move_neg((1, 0)),
            KeyCode::Char(c) if c.is_numeric() => {
                if let Some(val) = c.to_digit(10) {
                    self.board.push(val as usize);
                    if Checker::check(&self.board) {
                        self.state = State::Solved;
                    }
                }
            }
            KeyCode::Backspace => self.board.pop(),
            KeyCode::Delete => self.board.clear(),
            KeyCode::Char('q') | KeyCode::Esc => return Err(Error::Exit),
            _ => return Ok(()),
        }
        self.render()
    }

    /// Renders the game help screen in builder mode
    fn render_builder_help(&self) -> Paragraph {
        paragraph!(
            "[Arrows/hjkl]Move".fg(self.theme.help),
            "[</>+Arrows]Add cond.".fg(self.theme.help),
            "[c+Arrows]Remove cond.".fg(self.theme.help),
            "[Numbers]Place num.".fg(self.theme.help),
            "[Del]Clear cell".fg(self.theme.help),
            "[Enter]Scramble".fg(self.theme.help),
            "[b]Solve mode".fg(self.theme.help),
            "[Esc|q]Quit".fg(self.theme.help),
        )
        .separator(" ")
    }

    /// Renders the game help screen in solver mode
    fn render_solver_help(&self) -> Paragraph {
        paragraph!(
            "[Arrows/hjkl]Move".fg(self.theme.help),
            "[Numbers]Place num.".fg(self.theme.help),
            "[Del]Clear cell".fg(self.theme.help),
            "[Enter]Scramble".fg(self.theme.help),
            "[s]Solve".fg(self.theme.help),
            "[Tab]Solver sel.".fg(self.theme.help),
            "[b]Builder mode".fg(self.theme.help),
            "[Esc|q]Quit".fg(self.theme.help),
        )
        .separator(" ")
    }

    /// Moves in positive direction with action check
    fn move_pos<T>(&mut self, dir: T)
    where
        T: Into<Vec2>,
    {
        let dir = dir.into();
        match self.board.selected + dir {
            pos if pos.x < self.board.size() && pos.y < self.board.size() => {
                self.board_move(pos, dir);
            }
            _ => {}
        }
        self.action = Action::None;
    }

    /// Moves in negative direction with action check
    fn move_neg<T>(&mut self, dir: T)
    where
        T: Into<Vec2>,
    {
        let dir = dir.into();
        self.action = self.action.inverse();
        if let Some(mpos) = self.board.selected.checked_sub(dir) {
            self.board_move(mpos, dir);
        }
        self.action = Action::None;
    }

    /// Moves selection to given position, sets the condition based on action
    fn board_move(&mut self, mpos: Vec2, dir: Vec2) {
        match self.action {
            Action::Greater => self.set_cond(mpos, dir, Some(true)),
            Action::Lower => self.set_cond(mpos, dir, Some(false)),
            Action::Clear => self.set_cond(mpos, dir, None),
            Action::None => {}
        }
        self.board.set_selected(mpos);
    }

    /// Sets the condition to given value between the two positions given
    fn set_cond(&mut self, mpos: Vec2, dir: Vec2, cond: Option<bool>) {
        let mut cpos = self.board.selected;
        if mpos.x <= cpos.x && mpos.y <= cpos.y {
            cpos = mpos;
        }

        match dir {
            Vec2 { x, y: 0 } if x != 0 => {
                let id = cpos.x + cpos.y * self.board.size().saturating_sub(1);
                self.board.hor_conds[id] = cond;
            }
            Vec2 { x: 0, y } if y != 0 => {
                let id = cpos.x + cpos.y * self.board.size();
                self.board.ver_conds[id] = cond;
            }
            _ => (),
        }
    }
}
