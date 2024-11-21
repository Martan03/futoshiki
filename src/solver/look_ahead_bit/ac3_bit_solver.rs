use termint::geometry::Vec2;

use crate::solver::{ArcConsistency3, Solver};

use super::la_bit_solver::LABitSolver;

impl<'a> Solver<'a> for LABitSolver<'a, ArcConsistency3> {
    fn solve(board: &'a mut crate::board::board_struct::Board) -> bool {
        let mut solver = Self {
            board,
            values: vec![],
            _technique: ArcConsistency3 {},
        };
        solver.board.disable_vals();
        solver.gen_values() && solver.apply_conds() && solver.solve_inner()
    }
}

impl<'a> LABitSolver<'a, ArcConsistency3> {
    /// Solves the board using ac3, returns true on success
    pub(super) fn solve_inner(&mut self) -> bool {
        let Some(Vec2 { x, y }) = self.find_min() else {
            return true;
        };

        let id = x + y * self.board.size();
        for val in 0..self.board.size() {
            if (self.values[id] & (1 << val)) == 0 {
                continue;
            }

            let vals = self.values.clone();

            if !self.assign(val + 1, x, y) {
                self.board[id].set(0);
                self.values = vals;
                // TODO: I think continue is right, but why did I put return?
                continue;
            };
            if self.solve_inner() {
                return true;
            }
            self.board[id].set(0);
            self.values = vals;
        }
        false
    }

    /// Assigns given value to cell on given coordinates and removes the value
    /// from the neighbor domains
    pub(super) fn assign(&mut self, val: usize, x: usize, y: usize) -> bool {
        let id = x + y * self.board.size();
        self.board[id].set(val);
        let bval = 1 << (val - 1);

        for pos in 0..self.board.size() {
            if !self.rem_val(id, bval, x, pos)
                || !self.rem_val(id, bval, pos, y)
            {
                return false;
            }
        }
        self.check_conds(x, y)
    }

    /// Removes the given value from the domain on given coordinates and
    /// checks/applies the conditions
    pub(super) fn rem_val(
        &mut self,
        cid: usize,
        val: usize,
        x: usize,
        y: usize,
    ) -> bool {
        let id = x + y * self.board.size();
        if self.board[id].value() != 0 || self.values[id] & val == 0 {
            return true;
        }
        self.values[id] &= !val;
        self.check_conds(x, y) && (cid == id || self.values[id] != 0)
    }
}
