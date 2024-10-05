use std::usize;

use crate::board::board_struct::Board;

use super::Solver;

/// Forward check solver that uses bitmaps for available numbers.
/// Supports boards smaller then 64
#[derive(Debug, PartialEq)]
pub struct FcBitSolver<'a> {
    board: &'a mut Board,
    values: Vec<usize>,
}

impl<'a> Solver<'a> for FcBitSolver<'a> {
    fn solve(board: &'a mut Board) -> bool {
        let mut solver = Self {
            board,
            values: vec![],
        };
        solver.gen_values();
        solver.solve_inner()
    }
}

impl<'a> FcBitSolver<'a> {
    fn gen_values(&mut self) {
        let mut rows = vec![];
        let mut cols = vec![];

        let starter = (1 << self.board.size()) - 1;
        for y in 0..self.board.size() {
            let mut row = starter;
            let mut col = starter;

            for x in 0..self.board.size() {
                row &= !self.cell_to_bit(x, y);
                col &= !self.cell_to_bit(y, x);
            }

            rows.push(row);
            cols.push(col);
        }

        for y in 0..self.board.size() {
            for x in 0..self.board.size() {
                self.values.push(rows[y] & cols[x]);
            }
        }
    }

    fn solve_inner(&mut self) -> bool {
        let Some((x, y)) = self.find_min() else {
            return true;
        };

        let id = x + y * self.board.size();
        let vals = self.values[id];

        for val in 0..self.board.size() {
            if (vals & (1 << val)) == 0 {
                continue;
            }

            let Some((cx, cy)) = self.assign(val + 1, x, y) else {
                return false;
            };
            if self.solve_inner() {
                return true;
            }
            self.unassign_to(val + 1, x, y, self.board.size(), cx, cy);
        }
        false
    }

    /// Find unassigned value with the least possible numbers
    fn find_min(&self) -> Option<(usize, usize)> {
        let mut min_val = u32::MAX;
        let mut min = None;
        for y in 0..self.board.size() {
            for x in 0..self.board.size() {
                let id = x + y * self.board.size();
                if self.board[id].value() > 0 {
                    continue;
                }
                let ones = self.values[id].count_ones();
                if ones < min_val {
                    min_val = ones;
                    min = Some((x, y));
                }
            }
        }
        min
    }

    fn assign(
        &mut self,
        val: usize,
        x: usize,
        y: usize,
    ) -> Option<(usize, usize)> {
        let id = x + y * self.board.size();
        self.board[id].set(val);
        let bval = 1 << (val - 1);

        let mut changed_x = 0;
        let mut changed_y = 0;
        for pos in 0..self.board.size() {
            let xid = x + pos * self.board.size();
            if self.board[xid].value() == 0 && self.values[xid] & bval != 0 {
                self.values[xid] &= !bval;
                changed_x &= 1 << pos;
            }

            let yid = pos + y * self.board.size();
            if self.board[yid].value() == 0 && self.values[yid] & bval != 0 {
                self.values[yid] &= !bval;
                changed_y &= 1 << pos;
            }

            if (xid != id && self.values[xid] == 0)
                || (yid != id && self.values[yid] == 0)
            {
                self.unassign_to(val, x, y, pos, changed_x, changed_y);
                return None;
            }
        }
        Some((changed_x, changed_y))
    }

    fn unassign_to(
        &mut self,
        val: usize,
        x: usize,
        y: usize,
        to: usize,
        changed_x: usize,
        changed_y: usize,
    ) {
        let id = x + y * self.board.size();
        self.board[id].set(0);

        for pos in 0..=to {
            if changed_x & (1 << pos) != 0 {
                self.values[x + pos * self.board.size()] |= 1 << (val - 1);
            }
            if changed_y & (1 << pos) != 0 {
                self.values[pos + y * self.board.size()] |= 1 << (val - 1);
            }
        }
    }

    /// Converts cell on given coordinates to bitmap value
    fn cell_to_bit(&self, x: usize, y: usize) -> usize {
        let val = self.board[x + y * self.board.size()].value();
        (val > 0).then(|| 1 << (val - 1)).unwrap_or(0)
    }
}
