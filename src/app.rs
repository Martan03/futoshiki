use std::{
    cell::RefCell,
    io::{stdout, Write},
    rc::Rc,
    time::Duration,
};

use crossterm::{
    event::{poll, read, Event, KeyEvent},
    terminal::{disable_raw_mode, enable_raw_mode},
};
use termint::{
    enums::Modifier,
    geometry::{Constraint, TextAlign},
    term::Term,
    widgets::{Layout, ListState, StrSpanExtension},
};

use crate::{board::board_struct::Board, error::Error, solver::SolverType};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Action {
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Screen {
    Builder,
    Game,
    #[default]
    ModePicker,
    SolverPicker,
}

#[derive(Debug)]
pub struct App {
    pub board: Board,
    pub action: Action,
    pub solver: SolverType,
    pub screen: Screen,
    pub term: Term,

    pub mp_state: Rc<RefCell<ListState>>,
    pub sp_state: Rc<RefCell<ListState>>,
}

impl App {
    /// Creates new [`App`]
    pub fn new(size: usize, solver: SolverType) -> Self {
        Self {
            board: Board::new(size),
            action: Default::default(),
            solver,
            screen: Default::default(),
            term: Term::new().small_screen(Self::small_screen()),

            mp_state: Rc::new(RefCell::new(ListState::selected(0, 0))),
            sp_state: Rc::new(RefCell::new(ListState::selected(0, 0))),
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
    pub fn render(&mut self) -> Result<(), Error> {
        let screen = match self.screen {
            Screen::Builder => self.render_builder(),
            Screen::Game => self.render_game(),
            Screen::ModePicker => self.render_mp(),
            Screen::SolverPicker => self.render_sp(),
        };
        self.term.render(screen)?;
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
        match self.screen {
            Screen::Builder => self.listen_builder(event),
            Screen::Game => self.listen_game(event),
            Screen::ModePicker => self.listen_mp(event),
            Screen::SolverPicker => self.listen_sp(event),
        }
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
}

impl Default for App {
    fn default() -> Self {
        Self {
            board: Default::default(),
            action: Default::default(),
            solver: SolverType::ForwardBitCheck,
            screen: Default::default(),
            term: Term::new().small_screen(Self::small_screen()),

            mp_state: Rc::new(RefCell::new(ListState::selected(0, 0))),
            sp_state: Rc::new(RefCell::new(ListState::selected(0, 0))),
        }
    }
}
