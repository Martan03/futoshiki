use crate::board::board_struct::Board;

use super::la_bit_solver::LABitSolver;

pub struct Ac3BitSolver<'a> {
    board: &'a mut Board,
    values: Vec<usize>,
}

impl<'a> Ac3BitSolver<'a> {
    pub fn solve(board: &'a mut Board) -> bool {
        let mut solver = Self {
            board,
            values: vec![],
        };
        solver.board.disable_vals();
        solver.gen_values() && solver.apply_conds() && solver.solve_inner()
    }
}

impl<'a> LABitSolver for Ac3BitSolver<'a> {
    fn board(&self) -> &Board {
        &self.board
    }

    fn board_mut(&mut self) -> &mut Board {
        &mut self.board
    }

    fn values(&self) -> &Vec<usize> {
        &self.values
    }

    fn values_mut(&mut self) -> &mut Vec<usize> {
        &mut self.values
    }

    fn set_values(&mut self, values: Vec<usize>) {
        self.values = values;
    }

    fn rem_val(&mut self, cid: usize, val: usize, x: usize, y: usize) -> bool {
        let id = x + y * self.board.size();
        if self.board[id].value() != 0 || self.values[id] & val == 0 {
            return true;
        }
        self.values[id] &= !val;
        self.check_conds(x, y) && (cid == id || self.values[id] != 0)
    }
}
