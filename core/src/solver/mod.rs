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
