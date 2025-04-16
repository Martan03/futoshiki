use std::fmt::Display;

use ac3_solver::AC3Solver;
use bt_solver::BtSolver;
use fc_solver::FCSolver;
use pareg::FromArg;
use serde::{Deserialize, Serialize};

use crate::board::board_struct::Board;

pub mod ac3;
pub mod ac3_solver;
pub mod bt_solver;
pub mod domain;
pub mod fc_solver;
pub mod values;

pub trait Solver<'a> {
    /// Solves given board and returns true if the solution was found
    fn solve(&mut self) -> bool;
}

/// Enum containing all available solver types
#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Default,
    Hash,
    FromArg,
    Serialize,
    Deserialize,
)]
pub enum SolverType {
    #[arg("backtracking")]
    BT,
    #[arg("bit-ac3-bt" | "bit-ac3-backtracking")]
    BitAC3BT,
    #[arg("ac3-bt" | "ac3-backtracking")]
    AC3BT,
    #[arg("bit-fc" | "bit-forward-checking")]
    BitFC,
    #[arg("forward-checking")]
    FC,
    #[arg("bit-ac3" | "bit-arc-consistency3")]
    #[default]
    BitAC3,
    #[arg("arc-consistency3")]
    AC3,
}

impl SolverType {
    /// Solves the board using the corresponding solver algorithm
    pub fn solve(&self, board: &mut Board) -> bool {
        match self {
            SolverType::BT => BtSolver::new(board).solve(),
            SolverType::BitAC3BT => BtSolver::bit(board).solve(),
            SolverType::AC3BT => BtSolver::hash(board).solve(),
            SolverType::BitFC => FCSolver::bit(board).solve(),
            SolverType::FC => FCSolver::hash(board).solve(),
            SolverType::BitAC3 => AC3Solver::bit(board).solve(),
            SolverType::AC3 => AC3Solver::hash(board).solve(),
        }
    }

    /// Gets all solvers
    pub fn solvers() -> &'static [Self] {
        &[
            Self::BT,
            Self::BitAC3BT,
            Self::AC3BT,
            Self::BitFC,
            Self::FC,
            Self::BitAC3,
            Self::AC3,
        ]
    }

    /// Gets solver type based on the given id
    pub fn get(id: usize) -> Self {
        Self::solvers().get(id).copied().expect("Unknown solver ID")
    }

    /// Gets id of the current solver
    pub fn get_id(&self) -> usize {
        for (i, solver) in Self::solvers().iter().enumerate() {
            if solver == self {
                return i;
            }
        }
        panic!("Unknown solver type");
    }
}

impl Display for SolverType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SolverType::BT => write!(f, "Backtracking"),
            SolverType::BitAC3BT => {
                write!(f, "Bit Arc Consistency #3 Backtracking")
            }
            SolverType::AC3BT => write!(f, "Arc Consistency #3 Backtracking"),
            SolverType::BitFC => write!(f, "Bit Forward Checking"),
            SolverType::FC => write!(f, "Forward Checking"),
            SolverType::BitAC3 => write!(f, "Bit Arc Consistency #3"),
            SolverType::AC3 => write!(f, "Arc Consistency #3"),
        }
    }
}
