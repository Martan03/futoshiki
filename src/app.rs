use std::{
    io::{stdout, Write},
    time::Duration,
};

use crossterm::{
    event::{poll, read, Event, KeyCode, KeyEvent},
    terminal::{disable_raw_mode, enable_raw_mode},
};
use termint::{
    enums::{Color, Modifier},
    geometry::{Constraint, TextAlign, Vec2},
    paragraph,
    term::Term,
    widgets::{Layout, Paragraph, Spacer, StrSpanExtension},
};

use crate::{board::board_struct::Board, error::Error, solver::SolverType};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
enum Action {
    Greater,
    Lower,
    Clear,
    #[default]
    None,
}

impl Action {
    pub fn inverse(self) -> Self {
        match self {
            Action::Greater => Action::Lower,
            Action::Lower => Action::Greater,
            act => act,
        }
    }
}

#[derive(Debug)]
pub struct App {
    board: Board,
    action: Action,
    solver: SolverType,
    term: Term,
}

impl App {
    /// Creates new [`App`]
    pub fn new(size: usize, solver: SolverType) -> Self {
        Self {
            board: Board::new(size),
            action: Default::default(),
            solver,
            term: Term::new().small_screen(Self::small_screen()),
        }
    }

    /// Runs the [`App`]
    pub fn run(&mut self) -> Result<(), Error> {
        // Saves screen, clears screen and hides cursor
        print!("\x1b[?1049h\x1b[2J\x1b[?25l");
        _ = stdout().flush();
        enable_raw_mode()?;

        let res = self.main_loop();

        disable_raw_mode()?;
        // Restores screen
        print!("\x1b[?1049l\x1b[?25h");
        _ = stdout().flush();

        match res {
            Err(Error::Exit) => Ok(()),
            _ => res,
        }
    }

    /// Main loop of the [`App`]
    fn main_loop(&mut self) -> Result<(), Error> {
        self.render()?;
        loop {
            if poll(Duration::from_millis(100))? {
                self.event()?;
            }
        }
    }

    /// Renders the [`App`]
    fn render(&mut self) -> Result<(), Error> {
        let mut game = Layout::vertical().center();
        game.push(self.board.clone(), Constraint::Min(0));

        let mut wrapper = Layout::horizontal().center();
        wrapper.push(game, Constraint::Min(0));

        let mut main = Layout::vertical();
        main.push(Spacer::new(), Constraint::Fill(1));
        main.push(wrapper, Constraint::Min(0));
        main.push(Spacer::new(), Constraint::Fill(1));
        main.push(self.render_help(), Constraint::Min(0));

        self.term.render(main)?;
        Ok(())
    }

    /// Handles key listening of the [`App`]
    fn event(&mut self) -> Result<(), Error> {
        match read()? {
            Event::Key(e) => self.key_handler(e),
            Event::Resize(_, _) => self.render(),
            _ => Ok(()),
        }
    }

    /// Handles key events
    fn key_handler(&mut self, event: KeyEvent) -> Result<(), Error> {
        match event.code {
            KeyCode::Up | KeyCode::Char('k') => self.move_neg((0, 1)),
            KeyCode::Down | KeyCode::Char('j') => self.move_pos((0, 1)),
            KeyCode::Right | KeyCode::Char('l') => self.move_pos((1, 0)),
            KeyCode::Left | KeyCode::Char('h') => self.move_neg((1, 0)),
            KeyCode::Char('s') => _ = self.solver.solve(&mut self.board),
            KeyCode::Char('>') => self.action = Action::Greater,
            KeyCode::Char('<') => self.action = Action::Lower,
            KeyCode::Char('c') => self.action = Action::Clear,
            KeyCode::Char(c) if c.is_numeric() => match c.to_digit(10) {
                Some(val) => self.board.push(val as usize),
                _ => {}
            },
            KeyCode::Backspace => self.board.pop(),
            KeyCode::Delete => self.board.clear(),
            KeyCode::Char('q') | KeyCode::Esc => return Err(Error::Exit),
            _ => return Ok(()),
        }
        self.render()
    }

    /// Small screen to be displayed, when game can't fit
    fn small_screen() -> Layout {
        let mut layout = Layout::vertical().center();
        layout.push(
            "Terminal too small!"
                .modifier(Modifier::BOLD)
                .align(TextAlign::Center),
            Constraint::Min(0),
        );
        layout.push(
            "You have to increase terminal size".align(TextAlign::Center),
            Constraint::Min(0),
        );
        layout
    }

    /// Renders the help screen
    fn render_help(&self) -> Paragraph {
        paragraph!(
            "[Arrows/hjkl]Move".fg(Color::Gray),
            "[Esc|q]Quit".fg(Color::Gray),
        )
        .separator(" ")
    }
}

impl Default for App {
    fn default() -> Self {
        Self {
            board: Default::default(),
            action: Default::default(),
            solver: SolverType::ForwardBitCheck,
            term: Term::new().small_screen(Self::small_screen()),
        }
    }
}

impl App {
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

    fn move_neg<T>(&mut self, dir: T)
    where
        T: Into<Vec2>,
    {
        let dir = dir.into();
        self.action = self.action.inverse();
        match self.board.selected.checked_sub(dir) {
            Some(mpos) => self.board_move(mpos, dir),
            None => {}
        }
        self.action = Action::None;
    }

    fn board_move(&mut self, mpos: Vec2, dir: Vec2) {
        match self.action {
            Action::Greater => self.set_cond(mpos, dir, Some(true)),
            Action::Lower => self.set_cond(mpos, dir, Some(false)),
            Action::Clear => self.set_cond(mpos, dir, None),
            Action::None => {}
        }
        self.board.set_selected(mpos);
    }

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
            _ => return,
        }
    }
}
