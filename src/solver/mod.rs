use std::fmt::Display;

use bt_solver::BtSolver;
use fc_bit_solver::FcBitSolver;
use fc_solver::FcSolver;

use crate::board::board_struct::Board;

pub mod bt_solver;
pub mod fc_bit_solver;
pub mod fc_solver;

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
}

impl SolverType {
    /// Solves the board using the corresponding solver algorithm
    pub fn solve(&self, board: &mut Board) -> bool {
        match self {
            SolverType::Backtrack => BtSolver::solve(board),
            SolverType::ForwardBitCheck => FcBitSolver::solve(board),
            SolverType::ForwardCheck => FcSolver::solve(board),
        }
    }

    /// Gets all solvers
    pub fn solvers() -> &'static [Self] {
        &[Self::Backtrack, Self::ForwardBitCheck, Self::ForwardCheck]
    }

    /// Gets solver type based on the given id
    pub fn get(id: usize) -> Self {
        match id {
            0 => Self::Backtrack,
            1 => Self::ForwardBitCheck,
            2 => Self::ForwardCheck,
            _ => panic!("Unknown solver ID"),
        }
    }

    /// Gets id of the current solver
    pub fn get_id(&self) -> usize {
        match self {
            SolverType::Backtrack => 0,
            SolverType::ForwardBitCheck => 1,
            SolverType::ForwardCheck => 2,
        }
    }
}

impl Display for SolverType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SolverType::Backtrack => write!(f, "Backtracking"),
            SolverType::ForwardBitCheck => write!(f, "Forward bit checking"),
            SolverType::ForwardCheck => write!(f, "Forward checking"),
        }
    }
}
