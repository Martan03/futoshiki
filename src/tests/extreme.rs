use termint::geometry::Vec2;

use crate::{
    board::{board_struct::Board, cell::Cell},
    checker::Checker,
    solver::{
        bt_solver::BtSolver, fc_bit_solver::FcBitSolver, fc_solver::FcSolver,
        Solver,
    },
};

/// Gets tricky futoshiki board
fn get_extreme() -> Board {
    let mut board = Board {
        cells: vec![Cell::empty(); 16],
        hor_conds: vec![None; 12],
        ver_conds: vec![None; 12],
        selected: Vec2::new(0, 0),
        size: 4,
    };
    board.hor_conds[1] = Some(true);
    board.hor_conds[2] = Some(true);
    board.hor_conds[11] = Some(true);
    board.ver_conds[0] = Some(false);
    board.ver_conds[4] = Some(true);
    board.ver_conds[5] = Some(false);
    board.ver_conds[6] = Some(false);
    board
}

#[test]
fn bt_solver_test() {
    let mut board = get_extreme();
    assert!(BtSolver::solve(&mut board));
    assert!(Checker::check(&board));
}

#[test]
fn fc_bit_solver_test() {
    let mut board = get_extreme();
    assert!(FcBitSolver::solve(&mut board));
    assert!(Checker::check(&board));
}

#[test]
fn fc_solver_test() {
    let mut board = get_extreme();
    assert!(FcSolver::solve(&mut board));
    assert!(Checker::check(&board));
}
