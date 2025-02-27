use std::{
    cell::RefCell,
    fmt::Display,
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

use crate::{
    args::game_args::GameArgs, board::board_struct::Board, config::Config,
    error::Error, solver::SolverType, tui::theme::Theme,
};

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
    #[default]
    Solver,
    SolverPicker,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum State {
    #[default]
    Playing,
    Solved,
    Unsolvable,
}

impl Display for State {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            State::Playing => Ok(()),
            State::Solved => write!(f, "Solved"),
            State::Unsolvable => write!(f, "Unsolvable"),
        }
    }
}

#[derive(Debug)]
pub struct App {
    pub board: Board,
    pub action: Action,
    pub solver: SolverType,
    pub state: State,
    pub term: Term,
    pub screen: Screen,
    pub theme: Theme,
    pub sp_state: Rc<RefCell<ListState>>,
}

impl App {
    /// Creates new [`App`]
    pub fn new(config: Config, args: GameArgs) -> Self {
        let solver = args.solver.unwrap_or(config.default_solver);
        let solver_id = solver.get_id();

        let theme = args.theme.unwrap_or(config.default_theme).get_theme();
        Self {
            board: Board::new(
                args.size.unwrap_or(config.default_size),
                theme.clone(),
            ),
            action: Default::default(),
            solver: solver,
            state: Default::default(),
            term: Term::new().small_screen(Self::small_screen()),
            screen: Default::default(),
            theme,
            sp_state: Rc::new(RefCell::new(ListState::selected(0, solver_id))),
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
            Screen::Solver => self.render_solver(),
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
            Screen::Solver => self.listen_solver(event),
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
            state: Default::default(),
            term: Term::new().small_screen(Self::small_screen()),
            screen: Default::default(),
            theme: Theme::default(),
            sp_state: Rc::new(RefCell::new(ListState::selected(0, 0))),
        }
    }
}
