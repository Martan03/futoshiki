use naive::naive_solver::NaiveSolver;

use crate::board::board_struct::Board;

pub mod naive;

pub trait Solver<'a> {
    /// Solves given board and returns Some(board) when solvable, else None
    fn solve(board: &'a mut Board) -> bool;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SolverType {
    Naive,
}

impl SolverType {
    pub fn solve(&self, board: &mut Board) -> bool {
        match self {
            SolverType::Naive => NaiveSolver::solve(board),
        }
    }
}

// 2 1
// 1 1 hor
// 2 0 ver
// 2 1 hor
// 2 1 ver

// 1 4 2 3
// 4 3 1 2
// 3 2 4 1
// 2 1 3 4
