use termint::geometry::{Rect, Vec2};

use crate::board::board_struct::Board;

use super::Solver;

/// Forward check solver that uses bitmaps for available numbers (domains).
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
        solver.board.disable_vals();

        solver.gen_values();
        solver.apply_conds();

        solver.solve_inner()
    }
}

impl<'a> FcBitSolver<'a> {
    /// Generates the cell domains (possible values for each cell)
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
            let val = match self.board[pos].enabled() {
                true => rows[pos.y] & cols[pos.x],
                false => self.cell_to_bit(pos.x, pos.y),
            };
            self.values.push(val);
        }
    }

    /// Applies all the conditions
    fn apply_conds(&mut self) -> bool {
        let lsize = self.board.size().saturating_sub(1);
        let rect = Rect::new(0, 0, lsize, self.board.size());
        for pos in rect.into_iter() {
            let mut changed = false;
            let spos = Vec2::new(pos.x + 1, pos.y);

            let cond = self.board.hor_conds[pos.x + pos.y * lsize];
            match self.check_cond(pos, spos, cond, true) {
                Some(c) => changed = changed || c,
                None => return false,
            }

            let cond = self.board.ver_conds[pos.y + pos.x * self.board.size()];
            match self.check_cond(pos.inverse(), spos.inverse(), cond, true) {
                Some(c) => changed = changed || c,
                None => return false,
            }

            if changed && !self.check_conds(pos.x, pos.y) {
                return false;
            }
        }
        true
    }

    /// Solves the board using the forward checking, returns true on success
    fn solve_inner(&mut self) -> bool {
        let Some(Vec2 { x, y }) = self.find_min() else {
            return true;
        };

        let id = x + y * self.board.size();
        for val in 0..self.board.size() {
            let vals = self.values.clone();
            let board = self.board.get_cells();

            if (self.values[id] & (1 << val)) == 0 {
                continue;
            }

            if !self.assign(val + 1, x, y) {
                return false;
            };
            if self.solve_inner() {
                return true;
            }
            self.board.cells(board);
            self.values = vals;
        }
        false
    }

    /// Finds unassigned cell with the smallest domain (least possible values)
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

    /// Assigns given value to cell on given coordinates and removes the value
    /// from the neighbor domains
    fn assign(&mut self, val: usize, x: usize, y: usize) -> bool {
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
        true
    }

    /// Removes the given value from the domain on given coordinates and
    /// checks/applies the conditions
    fn rem_val(&mut self, cid: usize, val: usize, x: usize, y: usize) -> bool {
        let id = x + y * self.board.size();
        if self.board[id].value() != 0 || self.values[id] & val == 0 {
            return true;
        }
        self.values[id] &= !val;
        self.check_conds(x, y) && (cid == id || self.values[id] != 0)
    }

    /// Unassignes the given value from given coordinates and adds it to
    /// neighbor domains which were affected
    fn _unassign_to(
        &mut self,
        val: usize,
        x: usize,
        y: usize,
        to: usize,
        changed: Vec<usize>,
    ) {
        let id = x + y * self.board.size();
        self.board[id].set(0);

        for pos in 0..=to.min(changed.len() - 1) {
            if changed[pos] & (1 << pos) != 0 {
                self.values[x + pos * self.board.size()] |= 1 << (val - 1);
            }
            if changed[y] & (1 << pos) != 0 {
                self.values[pos + y * self.board.size()] |= 1 << (val - 1);
            }
        }
    }

    /// Good idea, but have to differentiate horizontal conds and vertical
    fn check_conds(&mut self, x: usize, y: usize) -> bool {
        let pos = Vec2::new(x, y);
        let lsize = self.board.size().saturating_sub(1);

        let mut changed = false;
        if let Some(xs) = x.checked_sub(1) {
            let cond = self.board.hor_conds[xs + y * lsize];
            match self.check_cond(Vec2::new(xs, y), pos, cond, false) {
                Some(c) => changed = changed || c,
                None => return false,
            }
        }
        if let Some(ys) = y.checked_sub(1) {
            let cond = self.board.ver_conds[x + ys * self.board.size()];
            match self.check_cond(Vec2::new(x, ys), pos, cond, false) {
                Some(c) => changed = changed || c,
                None => return false,
            }
        }

        if x < lsize {
            let cond = self.board.hor_conds[x + y * lsize];
            match self.check_cond(pos, Vec2::new(x + 1, y), cond, true) {
                Some(c) => changed = changed || c,
                None => return false,
            }
        }
        if y < lsize {
            let cond = self.board.ver_conds[x + y * self.board.size()];
            match self.check_cond(pos, Vec2::new(x, y + 1), cond, true) {
                Some(c) => changed = changed || c,
                None => return false,
            }
        }

        !changed || self.check_conds(x, y)
    }

    /// Checks condition on given positions and with given condition
    fn check_cond(
        &mut self,
        fpos: Vec2,
        spos: Vec2,
        cond: Option<bool>,
        pos: bool,
    ) -> Option<bool> {
        let (f, s) = match cond {
            Some(true) => self.apply_cond(fpos, spos)?,
            Some(false) => self.apply_cond(spos, fpos)?,
            None => return Some(false),
        };
        (!s || ((!pos || self.check_conds(spos.x, spos.y))
            && (pos || self.check_conds(fpos.x, fpos.y))))
        .then_some(f)
    }

    /// Applies the condition, return whether the cells changed
    fn apply_cond(&mut self, fpos: Vec2, spos: Vec2) -> Option<(bool, bool)> {
        let fid = fpos.x + fpos.y * self.board.size();
        let sid = spos.x + spos.y * self.board.size();

        let fchange = self.apply_cond_mask(fid, self.get_min_mask(sid))?;
        let schange = self.apply_cond_mask(sid, self.get_max_mask(fid))?;
        Some((fchange, schange))
    }

    /// Applies the given mask to the cell, returns true when changed
    fn apply_cond_mask(&mut self, sid: usize, mask: usize) -> Option<bool> {
        if !self.board[sid].enabled() {
            return Some(false);
        }
        let val = self.values[sid] & mask;
        let change = self.values[sid] != val;
        self.values[sid] = val;
        (val != 0).then_some(change)
    }

    /// Gets max value mask
    fn get_max_mask(&self, id: usize) -> usize {
        let max_bit = usize::BITS - self.values[id].leading_zeros();
        (1 << max_bit.saturating_sub(1)) - 1
    }

    /// Gets min value mask
    fn get_min_mask(&self, id: usize) -> usize {
        let min_bit = self.values[id].trailing_zeros();
        !((1 << (min_bit.min(self.board.size() as u32) + 1)) - 1)
    }

    /// Converts cell on given coordinates to bitmap value
    fn cell_to_bit(&self, x: usize, y: usize) -> usize {
        let val = self.board[x + y * self.board.size()].value();
        (val > 0).then(|| 1 << (val - 1)).unwrap_or(0)
    }
}
