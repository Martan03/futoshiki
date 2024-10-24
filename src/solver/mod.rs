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
}
