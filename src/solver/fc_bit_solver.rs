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

        for i in 0..self.board.size() {
            let val = vals & (1 << i);
            if val == 0 {
                continue;
            }

            self.assign_value(val, x, y);
            if self.solve_inner() {
                return true;
            }
            self.board[id].set(0);
        }
        false
    }

    /// Find unassigned value with the least possible numbers
    fn find_min(&self) -> Option<(usize, usize)> {
        let mut min_val = usize::MAX;
        let mut min = None;
        for y in 0..self.board.size() {
            for x in 0..self.board.size() {
                let id = x + y * self.board.size();
                if self.board[id].value() > 0 {
                    continue;
                }
                if self.values[id] < min_val {
                    min_val = self.values[id];
                    min = Some((x, y));
                }
            }
        }
        min
    }

    fn assign_value(&mut self, val: usize, x: usize, y: usize) {
        let id = x + y * self.board.size();
        self.board[id].set(val);

        for pos in 0..self.board.size() {
            self.values[x + pos * self.board.size()] &= !val;
            self.values[pos + y * self.board.size()] &= !val;
        }
    }

    /// Converts cell on given coordinates to bitmap value
    fn cell_to_bit(&self, x: usize, y: usize) -> usize {
        let val = self.board[x + y * self.board.size()].value();
        (val > 0).then(|| 1 << (val - 1)).unwrap_or(0)
    }
}
