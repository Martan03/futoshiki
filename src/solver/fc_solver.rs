use std::collections::HashSet;

use termint::geometry::{Rect, Vec2};

use crate::board::board_struct::Board;

use super::Solver;

/// Forward checking solver that uses 2D array for the available numbers
/// (domains).
#[allow(unused)]
pub struct FcSolver<'a> {
    board: &'a mut Board,
    values: Vec<HashSet<usize>>,
}

impl<'a> Solver<'a> for FcSolver<'a> {
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

impl<'a> FcSolver<'a> {
    /// Generates the cell domains (possible values for each cell)
    fn gen_values(&mut self) -> bool {
        let mut rows = vec![];
        let mut cols = vec![];

        for y in 0..self.board.size() {
            let mut row: HashSet<usize> = (1..=self.board.size()).collect();
            let mut col: HashSet<usize> = (1..=self.board.size()).collect();

            for x in 0..self.board.size() {
                row.remove(&self.board[x + y * self.board.size()].value());
                col.remove(&self.board[y + x * self.board.size()].value());
            }

            rows.push(row);
            cols.push(col);
        }

        for pos in self.board.rect() {
            let mut val: HashSet<usize> = match self.board[pos].enabled() {
                true => {
                    rows[pos.y].intersection(&cols[pos.x]).cloned().collect()
                }
                false => {
                    let id = pos.x + pos.y * self.board.size();
                    [self.board[id].value()].iter().cloned().collect()
                }
            };
            if val.is_empty() {
                return false;
            }
            val.shrink_to_fit();
            self.values.push(val);
        }
        true
    }

    /// Applies all the conditions
    fn apply_conds(&mut self) -> bool {
        let lsize = self.board.size().saturating_sub(1);
        for pos in Rect::new(0, 0, lsize, self.board.size()) {
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
        let values: Vec<usize> = self.values[id].iter().cloned().collect();
        for val in values {
            let vals = self.values.clone();

            if !self.assign(val, x, y) {
                self.board[id].set(0);
                self.values = vals;
                return false;
            }
            if self.solve_inner() {
                return true;
            }
            self.board[id].set(0);
            self.values = vals;
        }
        false
    }

    /// Finds unassigned cell with the smallest domain (least possible values)
    fn find_min(&self) -> Option<Vec2> {
        let mut min_val = usize::MAX;
        let mut min = None;

        for pos in self.board.rect() {
            let id = pos.x + pos.y * self.board.size();
            if self.board[id].value() > 0 {
                continue;
            }

            let values = self.values[id].len();
            if values < min_val {
                min_val = values;
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

        for pos in 0..self.board.size() {
            if !self.rem_val(id, val, x, pos) || !self.rem_val(id, val, pos, y)
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
        if self.board[id].value() != 0 || !self.values[id].remove(&val) {
            return true;
        }
        self.check_conds(x, y) && (cid == id || !self.values[id].is_empty())
    }

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

        let min = self.values[sid].iter().min().copied().unwrap_or(0);
        let fchange = self.rem_lower_vals(fid, min)?;

        let max = self.values[fid].iter().max().copied().unwrap_or(0);
        let schange = self.rem_greater_vals(sid, max)?;

        Some((fchange, schange))
    }

    /// Removes values greater or equal to given value
    fn rem_greater_vals(&mut self, id: usize, val: usize) -> Option<bool> {
        if !self.board[id].enabled() {
            return Some(false);
        }
        let len = self.values[id].len();
        self.values[id].retain(|&v| v < val);
        (!self.values[id].is_empty()).then_some(self.values[id].len() != len)
    }

    /// Removes values lower or equal to given value
    fn rem_lower_vals(&mut self, id: usize, val: usize) -> Option<bool> {
        if !self.board[id].enabled() {
            return Some(false);
        }
        let len = self.values[id].len();
        self.values[id].retain(|&v| v > val);
        (!self.values[id].is_empty()).then_some(self.values[id].len() != len)
    }
}
