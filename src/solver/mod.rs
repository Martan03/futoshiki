use std::fmt::Display;

use bt_solver::BtSolver;
use look_ahead::la_solver::LASolver;
use look_ahead_bit::la_bit_solver::LABitSolver;

use crate::board::board_struct::Board;

pub mod bt_solver;
pub mod look_ahead;
pub mod look_ahead_bit;

pub struct ForwardCheck;
pub struct ArcConsistency3;

pub trait Solver<'a> {
    /// Solves given board and returns Some(board) when solvable, else None
    fn solve(board: &'a mut Board) -> bool;
}

/// Enum containing all available solver types
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SolverType {
    Backtrack,
    ForwardBitCheck,
    ForwardCheck,
    ArcConsistency3Bit,
    ArcConsistency,
}

impl SolverType {
    /// Solves the board using the corresponding solver algorithm
    pub fn solve(&self, board: &mut Board) -> bool {
        match self {
            SolverType::Backtrack => BtSolver::solve(board),
            SolverType::ForwardBitCheck => {
                LABitSolver::<ForwardCheck>::solve(board)
            }
            SolverType::ForwardCheck => LASolver::<ForwardCheck>::solve(board),
            SolverType::ArcConsistency3Bit => {
                LABitSolver::<ArcConsistency3>::solve(board)
            }
            SolverType::ArcConsistency => {
                LASolver::<ArcConsistency3>::solve(board)
            }
        }
    }

    /// Gets all solvers
    pub fn solvers() -> &'static [Self] {
        &[
            Self::Backtrack,
            Self::ForwardBitCheck,
            Self::ForwardCheck,
            Self::ArcConsistency3Bit,
            Self::ArcConsistency,
        ]
    }

    /// Gets solver type based on the given id
    pub fn get(id: usize) -> Self {
        match id {
            0 => Self::Backtrack,
            1 => Self::ForwardBitCheck,
            2 => Self::ForwardCheck,
            3 => Self::ArcConsistency3Bit,
            4 => Self::ArcConsistency,
            _ => panic!("Unknown solver ID"),
        }
    }

    /// Gets id of the current solver
    pub fn get_id(&self) -> usize {
        match self {
            SolverType::Backtrack => 0,
            SolverType::ForwardBitCheck => 1,
            SolverType::ForwardCheck => 2,
            SolverType::ArcConsistency3Bit => 3,
            SolverType::ArcConsistency => 4,
        }
    }
}

impl Display for SolverType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SolverType::Backtrack => write!(f, "Backtracking"),
            SolverType::ForwardBitCheck => write!(f, "Forward bit checking"),
            SolverType::ForwardCheck => write!(f, "Forward checking"),
            SolverType::ArcConsistency3Bit => {
                write!(f, "Bit Arc Consistency #3")
            }
            SolverType::ArcConsistency => write!(f, "Arc Consistency #3"),
        }
    }
}
