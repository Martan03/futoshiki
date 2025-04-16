use termint::geometry::Vec2;

use crate::{
    board::{board_struct::Board, cell::Cell},
    checker::Checker,
    solver::{
        ac3::AC3,
        ac3_solver::AC3Solver,
        bt_solver::BtSolver,
        domain::{bit_domain::BitDomain, Domains},
        fc_solver::FCSolver,
        Solver,
    },
    tui::theme::Theme,
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
        theme: Theme::dark(),
    }
}

#[test]
fn bt_solver_test() {
    let mut board = get_trivial();
    assert!(BtSolver::new(&mut board).solve());
    assert!(Checker::check(&board));
}

#[test]
fn bt_solver_bit_domain_test() {
    let mut board = get_trivial();
    let mut backtracking = BtSolver::bit(&mut board);

    assert!(backtracking.solve());
    assert!(Checker::check(&board));
}

#[test]
fn bt_solver_hash_domain_test() {
    let mut board = get_trivial();
    let mut backtracking = BtSolver::hash(&mut board);

    assert!(backtracking.solve());
    assert!(Checker::check(&board));
}

#[test]
fn fc_bit_solver_test() {
    let mut board = get_trivial();
    assert!(FCSolver::bit(&mut board).solve());
    assert!(Checker::check(&board));
}

#[test]
fn fc_solver_test() {
    let mut board = get_trivial();
    assert!(FCSolver::hash(&mut board).solve());
    assert!(Checker::check(&board));
}

#[test]
fn ac3_bit_solver_test() {
    let mut board = get_trivial();
    assert!(AC3Solver::bit(&mut board).solve());
    assert!(Checker::check(&board));
}

#[test]
fn ac3_solver_test() {
    let mut board = get_trivial();
    assert!(AC3Solver::hash(&mut board).solve());
    assert!(Checker::check(&board));
}

#[test]
fn ac3_test() {
    let mut board = get_trivial();
    let mut values: Domains = vec![
        Box::new(BitDomain((1 << board.size()) - 1));
        board.size() * board.size()
    ];

    AC3::generate(&mut board, &mut values);

    let expected = vec![
        vec![1, 2, 3, 4],
        vec![3, 4],
        vec![3, 4],
        vec![1, 2, 3, 4],
        vec![4],
        vec![2, 3, 4],
        vec![1, 2, 3, 4],
        vec![3, 4],
        vec![1, 2, 3, 4],
        vec![2, 3, 4],
        vec![2, 3, 4],
        vec![3, 4],
        vec![1, 2, 3, 4],
        vec![1, 2, 3, 4],
        vec![4],
        vec![1, 2, 3, 4],
    ];
    for (domain, expected) in values.iter().zip(expected.iter()) {
        assert_eq!(domain.values(), *expected);
    }
}
