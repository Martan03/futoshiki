use termint::geometry::Vec2;

use crate::{
    board::board_struct::Board,
    solver::{
        domain::{
            bit_domain::BitDomain, hash_domain::HashDomain, DomainTrait,
        },
        Solver,
    },
};

/// Implements forward checking solver.
pub struct FCSolver<'a, D>
where
    D: DomainTrait + Clone,
{
    board: &'a mut Board,
    values: Vec<D>,
}

impl<'a> FCSolver<'a, BitDomain> {
    /// Creates new AC3 solver with given board and bitmap domain
    pub fn bit(board: &'a mut Board) -> Self {
        let value = BitDomain::default(board.size());
        Self::new(board, value)
    }
}

impl<'a> FCSolver<'a, HashDomain> {
    /// Creates new AC3 solver with given board and hashset domain
    pub fn hash(board: &'a mut Board) -> Self {
        let value = HashDomain::default(board.size());
        Self::new(board, value)
    }
}

impl<'a, D> Solver<'a> for FCSolver<'a, D>
where
    D: DomainTrait + Clone,
{
    fn solve(&mut self) -> bool {
        let Some(Vec2 { x, y }) = self.find_cell() else {
            return true;
        };

        let id = x + y * self.board.size();
        for val in self.values[id].values() {
            let vals = self.values.clone();

            if self.assign(val, x, y).is_none() {
                self.board[id].set(0);
                self.values = vals;
                continue;
            }
            if self.solve() {
                return true;
            }
            self.board[id].set(0);
            self.values = vals;
        }
        false
    }
}

impl<'a, D> FCSolver<'a, D>
where
    D: DomainTrait + Clone,
{
    fn new(board: &'a mut Board, value: D) -> Self {
        let values = vec![value; board.size() * board.size()];
        let mut fc = Self { board, values };
        fc.generate();
        fc
    }
    /// Generates the initial domain state
    fn generate(&mut self) {
        for pos in self.board.rect() {
            let value = self.board[pos.x + pos.y * self.board.size()].value();
            if value == 0 {
                continue;
            }
            self.assign(value, pos.x, pos.y);
        }
    }

    /// Assigns given value to cell on given coordinates and removes the value
    /// from the neighbor domains
    fn assign(&mut self, val: usize, x: usize, y: usize) -> Option<()> {
        let id = x + y * self.board.size();
        self.board[id].set(val);

        for pos in 0..self.board.size() {
            self.remove_val(id, val, x, pos)?;
            self.remove_val(id, val, pos, y)?;
        }
        self.check_conds(val, x, y)
    }

    /// Finds unassigned cell with the smallest domain (least possible values)
    fn find_cell(&self) -> Option<Vec2> {
        let mut min_val = usize::MAX;
        let mut min = None;

        for pos in self.board.rect() {
            let id = pos.x + pos.y * self.board.size();
            if self.board[id].value() > 0 {
                continue;
            }

            let values = self.values[id].values().len();
            if values < min_val {
                min_val = values;
                min = Some(pos);
            }
        }
        min
    }

    /// Checks all conditions related to cell on given coordinates
    fn check_conds(&mut self, val: usize, x: usize, y: usize) -> Option<()> {
        let lsize = self.board.size().saturating_sub(1);

        if let Some(xs) = x.checked_sub(1) {
            let cond = self.board.hor_conds[xs + y * lsize];
            let id = xs + y * self.board.size();
            self.handle_cond(cond, val, id)?;
        }
        if let Some(ys) = y.checked_sub(1) {
            let id = x + ys * self.board.size();
            let cond = self.board.ver_conds[id];
            self.handle_cond(cond, val, id)?;
        }

        if x < lsize {
            let cond = self.board.hor_conds[x + y * lsize];
            let id = x + 1 + y * self.board.size();
            self.handle_cond(cond.map(|v| !v), val, id)?;
        }
        if y < lsize {
            let cond = self.board.ver_conds[x + y * self.board.size()];
            let id = x + (y + 1) * self.board.size();
            self.handle_cond(cond.map(|v| !v), val, id)?;
        }
        Some(())
    }

    /// Removes value from domain on given coordinates
    /// Returns Some when no change or domain not empty, else None
    fn remove_val(
        &mut self,
        cid: usize,
        val: usize,
        x: usize,
        y: usize,
    ) -> Option<bool> {
        let id = x + y * self.board.size();
        if id == cid {
            return Some(false);
        }
        self.values[id].remove(val)
    }

    /// Removes values that are in conflict with the inequality
    /// Returns Some when no change or domain not empty, else None
    fn handle_cond(
        &mut self,
        cond: Option<bool>,
        val: usize,
        id: usize,
    ) -> Option<bool> {
        match cond {
            Some(true) => self.values[id].remove_lower(val),
            Some(false) => self.values[id].remove_greater(val),
            None => Some(false),
        }
    }
}
