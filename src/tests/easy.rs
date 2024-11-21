use termint::geometry::Vec2;

use crate::{
    board::{board_struct::Board, cell::Cell},
    checker::Checker,
    solver::{
        bt_solver::BtSolver, look_ahead::la_solver::LASolver,
        look_ahead_bit::la_bit_solver::LABitSolver, ArcConsistency3,
        ForwardCheck, Solver,
    },
};

/// Gets easy futoshiki board
fn get_easy() -> Board {
    let cells = vec![0, 0, 0, 0, 0, 1, 0, 2, 0, 0, 0, 0, 0, 0, 0, 0];
    let mut board = Board {
        cells: cells.into_iter().map(Cell::from).collect(),
        hor_conds: vec![None; 12],
        ver_conds: vec![None; 12],
        selected: Vec2::new(0, 0),
        size: 4,
    };
    board.ver_conds[0] = Some(true);
    board.ver_conds[8] = Some(true);
    board.ver_conds[10] = Some(true);
    board
}

#[test]
fn bt_solver_test() {
    let mut board = get_easy();
    assert!(BtSolver::solve(&mut board));
    assert!(Checker::check(&board));
}

#[test]
fn fc_bit_solver_test() {
    let mut board = get_easy();
    assert!(LABitSolver::<ForwardCheck>::solve(&mut board));
    assert!(Checker::check(&board));
}

#[test]
fn fc_solver_test() {
    let mut board = get_easy();
    assert!(LASolver::<ForwardCheck>::solve(&mut board));
    assert!(Checker::check(&board));
}

#[test]
fn ac3_bit_solver_test() {
    let mut board = get_easy();
    assert!(LABitSolver::<ArcConsistency3>::solve(&mut board));
    assert!(Checker::check(&board));
}

#[test]
fn ac3_solver_test() {
    let mut board = get_easy();
    assert!(LASolver::<ArcConsistency3>::solve(&mut board));
    assert!(Checker::check(&board));
}
