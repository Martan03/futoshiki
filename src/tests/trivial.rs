use termint::geometry::Vec2;

use crate::{
    board::{board_struct::Board, cell::Cell},
    solver::{
        bt_solver::BtSolver, fc_bit_solver::FcBitSolver, fc_solver::FcSolver,
        Solver,
    },
    tests::checker::Checker,
};

/// Gets trivial futoshiki board
fn get_trivial() -> Board {
    let cells = vec![2, 0, 0, 1, 0, 0, 0, 0, 1, 0, 0, 0, 3, 1, 0, 2];
    Board {
        cells: cells.into_iter().map(Cell::from).collect(),
        hor_conds: vec![None; 12],
        ver_conds: vec![None; 12],
        selected: Vec2::new(0, 0),
        size: 4,
    }
}

#[test]
fn bt_solver_test() {
    let mut board = get_trivial();
    assert!(BtSolver::solve(&mut board));
    assert!(Checker::check(&board));
}

#[test]
fn fc_bit_solver_test() {
    let mut board = get_trivial();
    assert!(FcBitSolver::solve(&mut board));
    assert!(Checker::check(&board));
}

#[test]
fn fc_solver_test() {
    let mut board = get_trivial();
    assert!(FcSolver::solve(&mut board));
    assert!(Checker::check(&board));
}
