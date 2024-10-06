use std::usize;

use termint::geometry::{Rect, Vec2};

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

        for pos in self.board.rect().into_iter() {
            self.values.push(rows[pos.y] & cols[pos.x]);
        }

        let lsize = self.board.size().saturating_sub(1);
        let rect = Rect::new(0, 0, lsize, self.board.size());
        for pos in rect.into_iter() {
            let spos = Vec2::new(pos.x + 1, pos.y);
            match self.board.hor_conds[pos.x + pos.y * lsize] {
                Some(true) => self.assign_cond(pos, spos),
                Some(false) => self.assign_cond(spos, pos),
                None => {}
            }
            match self.board.ver_conds[pos.y + pos.x * self.board.size()] {
                Some(true) => self.assign_cond(pos.inverse(), spos.inverse()),
                Some(false) => self.assign_cond(spos.inverse(), pos.inverse()),
                None => {}
            }
        }
    }

    fn solve_inner(&mut self) -> bool {
        let Some(Vec2 { x, y }) = self.find_min() else {
            return true;
        };

        let id = x + y * self.board.size();
        let vals = self.values[id];

        for val in 0..self.board.size() {
            if (vals & (1 << val)) == 0 {
                continue;
            }

            let Some(changed) = self.assign(val + 1, x, y) else {
                return false;
            };
            if self.solve_inner() {
                return true;
            }
            self.unassign_to(val + 1, x, y, self.board.size(), changed);
        }
        false
    }

    /// Find unassigned value with the least possible numbers
    fn find_min(&self) -> Option<Vec2> {
        let mut min_val = u32::MAX;
        let mut min = None;

        for pos in self.board.rect().into_iter() {
            let id = pos.x + pos.y * self.board.size();
            if self.board[id].value() > 0 {
                continue;
            }
            let ones = self.values[id].count_ones();
            if ones < min_val {
                min_val = ones;
                min = Some(pos);
            }
        }
        min
    }

    fn assign(
        &mut self,
        val: usize,
        x: usize,
        y: usize,
    ) -> Option<Vec<usize>> {
        let id = x + y * self.board.size();
        self.board[id].set(val);
        let bval = 1 << (val - 1);

        let mut changed = vec![0; self.board.size()];
        for pos in 0..self.board.size() {
            let yid = x + pos * self.board.size();
            if self.board[yid].value() == 0 && self.values[yid] & bval != 0 {
                self.values[yid] &= !bval;
                changed[pos] &= 1 << pos;
            }

            let xid = pos + y * self.board.size();
            if self.board[xid].value() == 0 && self.values[xid] & bval != 0 {
                self.values[xid] &= !bval;
                changed[y] &= 1 << pos;
            }

            if (xid != id && self.values[xid] == 0)
                || (yid != id && self.values[yid] == 0)
            {
                self.unassign_to(val, x, y, pos, changed);
                return None;
            }
        }
        Some(changed)
    }

    fn unassign_to(
        &mut self,
        val: usize,
        x: usize,
        y: usize,
        to: usize,
        changed: Vec<usize>,
    ) {
        let id = x + y * self.board.size();
        self.board[id].set(0);

        for pos in 0..=to {
            if changed[pos] & (1 << pos) != 0 {
                self.values[x + pos * self.board.size()] |= 1 << (val - 1);
            }
            if changed[y] & (1 << pos) != 0 {
                self.values[pos + y * self.board.size()] |= 1 << (val - 1);
            }
        }
    }

    fn assign_cond(&mut self, fdir: Vec2, sdir: Vec2) {
        let max = usize::BITS
            - self.values[fdir.x + fdir.y * self.board.size()].leading_zeros();
        self.values[sdir.x + sdir.y * self.board.size()] &=
            (1 << max.saturating_sub(1)) - 1;

        let min =
            self.values[sdir.x + sdir.y * self.board.size()].trailing_zeros();
        self.values[fdir.x + fdir.y * self.board.size()] &=
            !((1 << (min.min(self.board.size() as u32) + 1)) - 1);
    }

    /// Converts cell on given coordinates to bitmap value
    fn cell_to_bit(&self, x: usize, y: usize) -> usize {
        let val = self.board[x + y * self.board.size()].value();
        (val > 0).then(|| 1 << (val - 1)).unwrap_or(0)
    }
}
