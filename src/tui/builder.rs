use crossterm::event::{KeyCode, KeyEvent};
use termint::{geometry::Vec2, widgets::Element};

use crate::{
    app::{Action, App, Screen},
    board::board_struct::Board,
    error::Error,
};

impl App {
    pub fn render_builder(&mut self) -> Element {
        self.render_game()
    }

    pub fn listen_builder(&mut self, event: KeyEvent) -> Result<(), Error> {
        match event.code {
            KeyCode::Up | KeyCode::Char('k') => self.move_neg((0, 1)),
            KeyCode::Down | KeyCode::Char('j') => self.move_pos((0, 1)),
            KeyCode::Right | KeyCode::Char('l') => self.move_pos((1, 0)),
            KeyCode::Left | KeyCode::Char('h') => self.move_neg((1, 0)),
            KeyCode::Char('s') => _ = self.solver.solve(&mut self.board),
            KeyCode::Char('>') => self.action = Action::Greater,
            KeyCode::Char('<') => self.action = Action::Lower,
            KeyCode::Char('c') => self.action = Action::Clear,
            KeyCode::Char(c) if c.is_numeric() => {
                if let Some(val) = c.to_digit(10) {
                    self.board.push(val as usize);
                }
            }
            KeyCode::Backspace => self.board.pop(),
            KeyCode::Delete => self.board.clear(),
            KeyCode::Tab => self.screen = Screen::SolverPicker,
            KeyCode::Char('r') => self.board.reset(),
            KeyCode::Char('d') => self.board = Board::default(),
            KeyCode::Char('q') | KeyCode::Esc => return Err(Error::Exit),
            _ => return Ok(()),
        }
        self.render()
    }
}

impl App {
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
