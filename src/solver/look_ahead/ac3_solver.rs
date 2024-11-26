use std::collections::HashSet;

use crate::board::board_struct::Board;

use super::la_solver::LASolver;

pub struct Ac3Solver<'a> {
    board: &'a mut Board,
    values: Vec<HashSet<usize>>,
}

impl<'a> Ac3Solver<'a> {
    pub fn solve(board: &'a mut crate::board::board_struct::Board) -> bool {
        let mut solver = Self {
            board,
            values: vec![],
        };
        solver.board.disable_vals();
        solver.gen_values() && solver.apply_conds() && solver.solve_inner()
    }
}

impl<'a> LASolver for Ac3Solver<'a> {
    fn board(&self) -> &Board {
        &self.board
    }

    fn board_mut(&mut self) -> &mut Board {
        &mut self.board
    }

    fn values(&self) -> &Vec<HashSet<usize>> {
        &self.values
    }

    fn values_mut(&mut self) -> &mut Vec<HashSet<usize>> {
        &mut self.values
    }

    fn set_values(&mut self, values: Vec<HashSet<usize>>) {
        self.values = values;
    }

    fn rem_val(&mut self, cid: usize, val: usize, x: usize, y: usize) -> bool {
        let id = x + y * self.board.size();
        if self.board[id].value() != 0 || !self.values[id].remove(&val) {
            return true;
        }
        self.check_conds(x, y) && (cid == id || !self.values[id].is_empty())
    }
}
