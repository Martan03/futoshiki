use std::fmt::Display;

use bt_solver::BtSolver;
use look_ahead::{ac3_solver::Ac3Solver, fc_solver::FcSolver};
use look_ahead_bit::{
    ac3_bit_solver::Ac3BitSolver, fc_bit_solver::FcBitSolver,
};
use pareg::FromArg;
use serde::{Deserialize, Serialize};

use crate::board::board_struct::Board;

pub mod bt_solver;
pub mod look_ahead;
pub mod look_ahead_bit;

pub trait Solver<'a> {
    /// Solves given board and returns Some(board) when solvable, else None
    fn solve(board: &'a mut Board) -> bool;
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
    #[arg("bt" | "backtracking")]
    Backtrack,
    #[arg("fcb" | "forward-check-bit" | "forward-checking-bit")]
    ForwardBitCheck,
    #[arg("fc" | "forwarch-check" | "forward-checking")]
    ForwardCheck,
    #[arg("ac3" | "arc-cons3" | "arc-consistency3")]
    #[default]
    ArcConsistency3Bit,
    #[arg("ac3b" | "arc-cons3-bit" | "arc-consistency3-bit")]
    ArcConsistency3,
}

impl SolverType {
    /// Solves the board using the corresponding solver algorithm
    pub fn solve(&self, board: &mut Board) -> bool {
        match self {
            SolverType::Backtrack => BtSolver::solve(board),
            SolverType::ForwardBitCheck => FcBitSolver::solve(board),
            SolverType::ForwardCheck => FcSolver::solve(board),
            SolverType::ArcConsistency3Bit => Ac3BitSolver::solve(board),
            SolverType::ArcConsistency3 => Ac3Solver::solve(board),
        }
    }

    /// Gets all solvers
    pub fn solvers() -> &'static [Self] {
        &[
            Self::Backtrack,
            Self::ForwardBitCheck,
            Self::ForwardCheck,
            Self::ArcConsistency3Bit,
            Self::ArcConsistency3,
        ]
    }

    /// Gets solver type based on the given id
    pub fn get(id: usize) -> Self {
        match id {
            0 => Self::Backtrack,
            1 => Self::ForwardBitCheck,
            2 => Self::ForwardCheck,
            3 => Self::ArcConsistency3Bit,
            4 => Self::ArcConsistency3,
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
            SolverType::ArcConsistency3 => 4,
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
            SolverType::ArcConsistency3 => write!(f, "Arc Consistency #3"),
        }
    }
}
