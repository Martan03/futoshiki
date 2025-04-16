use termint::geometry::Vec2;

use crate::{
    board::{board_struct::Board, cell::Cell},
    checker::Checker,
    solver::{
        ac3::AC3,
        bt_solver::BtSolver,
        domain::{bit_domain::BitDomain, DomainTrait},
        look_ahead::{ac3_solver::Ac3Solver, fc_solver::FcSolver},
        look_ahead_bit::{
            ac3_bit_solver::Ac3BitSolver, fc_bit_solver::FcBitSolver,
        },
        values::DomainValues,
        Solver,
    },
    tui::theme::Theme,
};

/// Gets tricky futoshiki board
fn get_tricky() -> Board {
    let cells = vec![0, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    let mut board = Board {
        cells: cells.into_iter().map(Cell::from).collect(),
        hor_conds: vec![None; 12],
        ver_conds: vec![None; 12],
        selected: Vec2::new(0, 0),
        size: 4,
        theme: Theme::dark(),
    };
    board.hor_conds[2] = Some(true);
    board.hor_conds[4] = Some(true);
    board.hor_conds[8] = Some(true);
    board.hor_conds[9] = Some(true);
    board.ver_conds[11] = Some(true);
    board
}

#[test]
fn bt_solver_test() {
    let mut board = get_tricky();
    assert!(BtSolver::new(&mut board).solve());
    assert!(Checker::check(&board));
}

#[test]
fn bt_solver_domain_test() {
    let mut board = get_tricky();
    let size = board.size();

    let values = AC3::generate(
        &mut board,
        vec![Box::new(BitDomain((1 << size) - 1)); size * size],
    );
    let mut backtracking =
        BtSolver::new(&mut board).values(Box::new(DomainValues::new(values)));

    assert!(backtracking.solve());
    assert!(Checker::check(&board));
}

#[test]
fn fc_bit_solver_test() {
    let mut board = get_tricky();
    assert!(FcBitSolver::solve(&mut board));
    assert!(Checker::check(&board));
}

#[test]
fn fc_solver_test() {
    let mut board = get_tricky();
    assert!(FcSolver::solve(&mut board));
    assert!(Checker::check(&board));
}

#[test]
fn ac3_bit_solver_test() {
    let mut board = get_tricky();
    assert!(Ac3BitSolver::solve(&mut board));
    assert!(Checker::check(&board));
}

#[test]
fn ac3_solver_test() {
    let mut board = get_tricky();
    assert!(Ac3Solver::solve(&mut board));
    assert!(Checker::check(&board));
}

#[test]
fn ac3_test() {
    let mut board = get_tricky();
    let values = vec![
        Box::new(BitDomain((1 << board.size()) - 1))
            as Box<dyn DomainTrait>;
        board.size() * board.size()
    ];

    let values = AC3::generate(&mut board, values);

    let expected = vec![
        vec![1, 3, 4],
        vec![1, 2, 3, 4],
        vec![2, 3, 4],
        vec![1, 2, 3],
        vec![1, 2, 3, 4],
        vec![3, 4],
        vec![1, 3],
        vec![1, 3, 4],
        vec![1, 3, 4],
        vec![1, 2, 3, 4],
        vec![3, 4],
        vec![2, 3],
        vec![3, 4],
        vec![1, 2, 3],
        vec![1, 2, 3, 4],
        vec![1, 2],
    ];
    for (domain, expected) in values.iter().zip(expected.iter()) {
        assert_eq!(domain.values(), *expected);
    }
}
