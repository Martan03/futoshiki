use termint::geometry::{Rect, Vec2};

use crate::board::board_struct::Board;

pub struct Checker<'a> {
    board: &'a Board,
}

impl<'a> Checker<'a> {
    /// Checks whether given board is correctly solved
    pub fn check(board: &'a Board) -> bool {
        let checker = Self { board };
        checker.check_cells() && checker.check_conds()
    }

    /// Checks the cell restrictions
    fn check_cells(&self) -> bool {
        for y in 0..self.board.size() {
            let mut row = vec![false; self.board.size()];
            let mut col = vec![false; self.board.size()];

            for x in 0..self.board.size() {
                let rid = x + y * self.board.size();
                let Some(rval) = self.board[rid].value().checked_sub(1) else {
                    return false;
                };

                let cid = y + x * self.board.size();
                let Some(cval) = self.board[cid].value().checked_sub(1) else {
                    return false;
                };

                if row[rval] || col[cval] {
                    return false;
                }
                row[rval] = true;
                col[cval] = true;
            }
        }
        true
    }

    /// Checks the conditions
    fn check_conds(&self) -> bool {
        let lsize = self.board.size().saturating_sub(1);
        for pos in Rect::new(0, 0, lsize, self.board.size()) {
            let spos = Vec2::new(pos.x + 1, pos.y);

            let cond = self.board.hor_conds[pos.x + pos.y * lsize];
            if !self.check_cond(pos, spos, cond) {
                return false;
            }

            let cond = self.board.ver_conds[pos.y + pos.x * (lsize + 1)];
            if !self.check_cond(pos.inverse(), spos.inverse(), cond) {
                return false;
            }
        }
        true
    }

    /// Checks the given condition between the given positions
    fn check_cond(&self, fpos: Vec2, spos: Vec2, cond: Option<bool>) -> bool {
        match cond {
            Some(true) => self.board[fpos].value() > self.board[spos].value(),
            Some(false) => self.board[fpos].value() < self.board[spos].value(),
            None => true,
        }
    }
}
