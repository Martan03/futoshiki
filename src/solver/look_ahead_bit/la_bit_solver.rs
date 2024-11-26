use termint::geometry::{Rect, Vec2};

use crate::board::board_struct::Board;

pub trait LABitSolver {
    fn board(&self) -> &Board;

    fn board_mut(&mut self) -> &mut Board;

    fn values(&self) -> &Vec<usize>;

    fn values_mut(&mut self) -> &mut Vec<usize>;

    fn set_values(&mut self, values: Vec<usize>);

    /// Removes the given value from the domain on given coordinates and
    /// checks/applies the conditions
    fn rem_val(&mut self, cid: usize, val: usize, x: usize, y: usize) -> bool;

    /// Generates the cell domains (possible values for each cell)
    fn gen_values(&mut self) -> bool {
        let mut rows = vec![];
        let mut cols = vec![];

        let starter = (1 << self.board().size()) - 1;
        for y in 0..self.board().size() {
            let mut row = starter;
            let mut col = starter;

            for x in 0..self.board().size() {
                row &= !self.cell_to_bit(x, y);
                col &= !self.cell_to_bit(y, x);
            }

            rows.push(row);
            cols.push(col);
        }

        for pos in self.board().rect() {
            let val = match self.board()[pos].enabled() {
                true => rows[pos.y] & cols[pos.x],
                false => self.cell_to_bit(pos.x, pos.y),
            };
            if val == 0 {
                return false;
            }
            self.values_mut().push(val);
        }
        true
    }

    /// Applies all the conditions
    fn apply_conds(&mut self) -> bool {
        let lsize = self.board().size().saturating_sub(1);
        for pos in Rect::new(0, 0, lsize, self.board().size()) {
            let mut changed = false;
            let spos = Vec2::new(pos.x + 1, pos.y);

            let cond = self.board().hor_conds[pos.x + pos.y * lsize];
            match self.check_cond(pos, spos, cond, true) {
                Some(c) => changed = changed || c,
                None => return false,
            }

            let cond =
                self.board().ver_conds[pos.y + pos.x * self.board().size()];
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

    /// Solves the board using ac3, returns true on success
    fn solve_inner(&mut self) -> bool {
        let Some(Vec2 { x, y }) = self.find_min() else {
            return true;
        };

        let id = x + y * self.board().size();
        for val in 0..self.board().size() {
            if (self.values()[id] & (1 << val)) == 0 {
                continue;
            }

            let vals = self.values().clone();

            if !self.assign(val + 1, x, y) {
                self.board_mut()[id].set(0);
                self.set_values(vals);
                // TODO: I think continue is right, but why did I put return?
                continue;
            };
            if self.solve_inner() {
                return true;
            }
            self.board_mut()[id].set(0);
            self.set_values(vals);
        }
        false
    }

    /// Assigns given value to cell on given coordinates and removes the value
    /// from the neighbor domains
    fn assign(&mut self, val: usize, x: usize, y: usize) -> bool {
        let id = x + y * self.board().size();
        self.board_mut()[id].set(val);
        let val = 1 << (val - 1);

        for pos in 0..self.board().size() {
            if !self.rem_val(id, val, x, pos) || !self.rem_val(id, val, pos, y)
            {
                return false;
            }
        }
        self.check_conds(x, y)
    }

    /// Finds unassigned cell with the smallest domain (least possible values)
    fn find_min(&self) -> Option<Vec2> {
        let mut min_val = u32::MAX;
        let mut min = None;

        for pos in self.board().rect() {
            let id = pos.x + pos.y * self.board().size();
            if self.board()[id].value() > 0 {
                continue;
            }
            let ones = self.values()[id].count_ones();
            if ones < min_val {
                min_val = ones;
                min = Some(pos);
            }
        }
        min
    }

    /// Good idea, but have to differentiate horizontal conds and vertical
    fn check_conds(&mut self, x: usize, y: usize) -> bool {
        let pos = Vec2::new(x, y);
        let lsize = self.board().size().saturating_sub(1);

        let mut changed = false;
        if let Some(xs) = x.checked_sub(1) {
            let cond = self.board().hor_conds[xs + y * lsize];
            match self.check_cond(Vec2::new(xs, y), pos, cond, false) {
                Some(c) => changed = changed || c,
                None => return false,
            }
        }
        if let Some(ys) = y.checked_sub(1) {
            let cond = self.board().ver_conds[x + ys * self.board().size()];
            match self.check_cond(Vec2::new(x, ys), pos, cond, false) {
                Some(c) => changed = changed || c,
                None => return false,
            }
        }

        if x < lsize {
            let cond = self.board().hor_conds[x + y * lsize];
            match self.check_cond(pos, Vec2::new(x + 1, y), cond, true) {
                Some(c) => changed = changed || c,
                None => return false,
            }
        }
        if y < lsize {
            let cond = self.board().ver_conds[x + y * self.board().size()];
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
        let fid = fpos.x + fpos.y * self.board().size();
        let sid = spos.x + spos.y * self.board().size();

        let fchange = self.apply_cond_mask(fid, self.get_min_mask(sid))?;
        let schange = self.apply_cond_mask(sid, self.get_max_mask(fid))?;
        Some((fchange, schange))
    }

    /// Applies the given mask to the cell, returns true when changed
    fn apply_cond_mask(&mut self, sid: usize, mask: usize) -> Option<bool> {
        if !self.board()[sid].enabled() {
            return Some(false);
        }
        let val = self.values()[sid] & mask;
        let change = self.values()[sid] != val;
        self.values_mut()[sid] = val;
        (val != 0).then_some(change)
    }

    /// Gets max value mask
    fn get_max_mask(&self, id: usize) -> usize {
        let max_bit = usize::BITS - self.values()[id].leading_zeros();
        (1 << max_bit.saturating_sub(1)) - 1
    }

    /// Gets min value mask
    fn get_min_mask(&self, id: usize) -> usize {
        let min_bit = self.values()[id].trailing_zeros();
        !((1 << (min_bit.min(self.board().size() as u32) + 1)) - 1)
    }

    /// Converts cell on given coordinates to bitmap value
    fn cell_to_bit(&self, x: usize, y: usize) -> usize {
        let val = self.board()[x + y * self.board().size()].value();
        (val > 0).then(|| 1 << (val - 1)).unwrap_or(0)
    }
}
