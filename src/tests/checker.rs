use termint::geometry::Vec2;

use crate::{
    board::{board_struct::Board, cell::Cell},
    checker::Checker,
};

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
