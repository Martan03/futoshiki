use std::{cell::RefCell, fmt::Display, rc::Rc};

use futoshiki_core::board::board_struct::Board;
use termint::{
    enums::Modifier,
    geometry::{Constraint, TextAlign},
    term::{Application, Frame, backend::Event},
    widgets::{Element, Layout, ListState, ToSpan},
};

use crate::{
    args::game_args::GameArgs, config::Config, solver_type::SolverType,
    tui::theme::Theme,
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
    pub board: Rc<RefCell<Board>>,
    pub action: Action,
    pub solver: SolverType,
    pub state: State,
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
            board: Rc::new(RefCell::new(Board::new(
                args.size.unwrap_or(config.default_size),
            ))),
            action: Default::default(),
            solver,
            state: Default::default(),
            screen: Default::default(),
            theme,
            sp_state: Rc::new(RefCell::new(ListState::selected(0, solver_id))),
        }
    }

    /// Small screen to be displayed, when game can't fit
    pub fn small_screen() -> Layout {
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

impl Application for App {
    type Message = ();

    fn view(&self, _frame: &Frame) -> Element<Self::Message> {
        match self.screen {
            Screen::Builder => self.render_builder(),
            Screen::Solver => self.render_solver(),
            Screen::SolverPicker => self.render_sp(),
        }
    }

    fn event(&mut self, event: Event) -> termint::prelude::Action {
        let Event::Key(e) = event else {
            return termint::prelude::Action::NONE;
        };

        match self.screen {
            Screen::Builder => self.listen_builder(e),
            Screen::Solver => self.listen_solver(e),
            Screen::SolverPicker => self.listen_sp(e),
        }
    }
}

impl Default for App {
    fn default() -> Self {
        Self {
            board: Default::default(),
            action: Default::default(),
            solver: SolverType::BitAC3,
            state: Default::default(),
            screen: Default::default(),
            theme: Theme::default(),
            sp_state: Rc::new(RefCell::new(ListState::selected(0, 0))),
        }
    }
}
