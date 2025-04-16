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

/// Gets tricky futoshiki board
fn get_extreme() -> Board {
    let mut board = Board {
        cells: vec![Cell::empty(); 16],
        hor_conds: vec![None; 12],
        ver_conds: vec![None; 12],
        selected: Vec2::new(0, 0),
        size: 4,
        theme: Theme::dark(),
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
    assert!(BtSolver::new(&mut board).solve());
    assert!(Checker::check(&board));
}

#[test]
fn bt_solver_bit_domain_test() {
    let mut board = get_extreme();
    let mut backtracking = BtSolver::bit(&mut board);

    assert!(backtracking.solve());
    assert!(Checker::check(&board));
}

#[test]
fn bt_solver_hash_domain_test() {
    let mut board = get_extreme();
    let mut backtracking = BtSolver::hash(&mut board);

    assert!(backtracking.solve());
    assert!(Checker::check(&board));
}

#[test]
fn fc_bit_solver_test() {
    let mut board = get_extreme();
    assert!(FCSolver::bit(&mut board).solve());

    for y in 0..board.size() {
        for x in 0..board.size() {
            print!("{} ", board[x + y * board.size()].value());
        }
        println!();
    }

    assert!(Checker::check(&board));
}

#[test]
fn fc_solver_test() {
    let mut board = get_extreme();
    assert!(FCSolver::hash(&mut board).solve());
    assert!(Checker::check(&board));
}

#[test]
fn ac3_bit_solver_test() {
    let mut board = get_extreme();
    assert!(AC3Solver::bit(&mut board).solve());
    assert!(Checker::check(&board));
}

#[test]
fn ac3_solver_test() {
    let mut board = get_extreme();
    assert!(AC3Solver::hash(&mut board).solve());
    assert!(Checker::check(&board));
}

#[test]
fn ac3_test() {
    let mut board = get_extreme();
    let mut values: Domains = vec![
        Box::new(BitDomain((1 << board.size()) - 1));
        board.size() * board.size()
    ];
    AC3::generate(&mut board, &mut values);

    let expected = vec![
        vec![1, 2, 3],
        vec![3, 4],
        vec![2, 3],
        vec![1, 2],
        vec![2, 3, 4],
        vec![1, 2, 3],
        vec![1, 2, 3],
        vec![1, 2, 3, 4],
        vec![1, 2, 3],
        vec![2, 3, 4],
        vec![2, 3, 4],
        vec![1, 2, 3, 4],
        vec![1, 2, 3, 4],
        vec![1, 2, 3, 4],
        vec![2, 3, 4],
        vec![1, 2, 3],
    ];
    for (domain, expected) in values.iter().zip(expected.iter()) {
        assert_eq!(domain.values(), *expected);
    }
}
