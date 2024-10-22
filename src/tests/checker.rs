use std::vec;

use termint::geometry::{Rect, Vec2};

use crate::board::{board_struct::Board, cell::Cell};

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
                let row_val = self.board[rid].value() - 1;
                if row[row_val] {
                    return false;
                }
                row[row_val] = true;

                let cid = y + x * self.board.size();
                let col_val = self.board[cid].value() - 1;
                if col[col_val] {
                    return false;
                }
                col[col_val] = true;
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

fn create_board<I>(cells: I, size: usize) -> Board
where
    I: IntoIterator,
    I::Item: Into<Cell>,
{
    let lsize = size.saturating_sub(1);
    Board {
        cells: cells.into_iter().map(|c| c.into()).collect(),
        hor_conds: vec![None; size * lsize],
        ver_conds: vec![None; size * lsize],
        selected: Vec2::new(0, 0),
        size,
    }
}

/// Tests [`Checker`] on correctly solved boards
#[test]
fn true_checker_4x4_test() {
    let cells = vec![2, 3, 4, 1, 1, 4, 3, 2, 3, 1, 2, 4, 4, 2, 1, 3];
    let board = create_board(cells, 4);
    assert!(Checker::check(&board));

    let cells = vec![3, 4, 2, 1, 4, 3, 1, 2, 1, 2, 4, 3, 2, 1, 3, 4];
    let mut board = create_board(cells, 4);
    board.hor_conds[7] = Some(false);
    board.ver_conds[2] = Some(true);
    board.ver_conds[7] = Some(false);
    board.ver_conds[11] = Some(false);
    assert!(Checker::check(&board));

    let cells = vec![4, 1, 2, 3, 2, 3, 4, 1, 1, 2, 3, 4, 3, 4, 1, 2];
    let mut board = create_board(cells, 4);
    board.hor_conds[8] = Some(false);
    board.hor_conds[11] = Some(false);
    board.ver_conds[0] = Some(true);
    board.ver_conds[4] = Some(true);
    board.ver_conds[5] = Some(true);
    board.ver_conds[8] = Some(false);
    board.ver_conds[9] = Some(false);
    assert!(Checker::check(&board));
}

/// Tests [`Checker`] on wrongly solved boards
#[test]
fn false_checker_4x4_test() {
    let cells = vec![1, 2, 3, 4, 2, 4, 1, 3, 3, 1, 3, 2, 4, 3, 2, 1];
    let mut board = create_board(cells, 4);
    board.hor_conds[0] = Some(false);
    board.hor_conds[1] = Some(false);
    board.hor_conds[4] = Some(true);
    board.hor_conds[6] = Some(true);
    board.ver_conds[3] = Some(true);
    board.ver_conds[7] = Some(true);
    board.ver_conds[10] = Some(false);
    assert!(!Checker::check(&board));

    let cells = vec![1, 2, 3, 4, 2, 4, 1, 3, 3, 1, 4, 2, 4, 3, 2, 1];
    let mut board = create_board(cells, 4);
    board.hor_conds[0] = Some(false);
    board.hor_conds[1] = Some(false);
    board.hor_conds[4] = Some(true);
    board.hor_conds[6] = Some(false);
    board.ver_conds[3] = Some(true);
    board.ver_conds[7] = Some(true);
    board.ver_conds[10] = Some(false);
    assert!(!Checker::check(&board));
}
