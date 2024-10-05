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
    geometry::{Constraint, TextAlign},
    paragraph,
    term::Term,
    widgets::{Layout, Paragraph, StrSpanExtension},
};

use crate::{board::board_struct::Board, error::Error, solver::SolverType};

pub struct App {
    board: Board,
    solver: SolverType,
    term: Term,
}

impl App {
    /// Creates new [`App`]
    pub fn new(size: usize, solver: SolverType) -> Self {
        Self {
            board: Board::new(size),
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
        game.add_child(self.board.clone(), Constraint::Min(0));

        let mut wrapper = Layout::horizontal().center();
        wrapper.add_child(game, Constraint::Min(0));

        let mut main = Layout::vertical();
        main.add_child(wrapper, Constraint::Fill);
        main.add_child(self.render_help(), Constraint::Min(0));

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
            KeyCode::Up | KeyCode::Char('k') => self.board.up(),
            KeyCode::Down | KeyCode::Char('j') => self.board.down(),
            KeyCode::Right | KeyCode::Char('l') => self.board.right(),
            KeyCode::Left | KeyCode::Char('h') => self.board.left(),
            KeyCode::Char('s') => _ = self.solver.solve(&mut self.board),
            KeyCode::Char(c) if c.is_numeric() => match c.to_digit(10) {
                Some(val) => self.board.push(val as usize),
                _ => {}
            },
            KeyCode::Char('q') | KeyCode::Esc => return Err(Error::Exit),
            _ => return Ok(()),
        }
        self.render()
    }

    /// Small screen to be displayed, when game can't fit
    fn small_screen() -> Layout {
        let mut layout = Layout::vertical().center();
        layout.add_child(
            "Terminal too small!"
                .modifier(Modifier::BOLD)
                .align(TextAlign::Center),
            Constraint::Min(0),
        );
        layout.add_child(
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
            solver: SolverType::ForwardBitCheck,
            term: Term::new().small_screen(Self::small_screen()),
        }
    }
}
