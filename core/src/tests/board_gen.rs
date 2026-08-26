use crate::{
    board::board_gen::BoardGen,
    solver::{ac3_solver::AC3Solver, Solver},
};

const REP_CNT: usize = 10;

fn board_gen_n_times(size: usize) {
    for _ in 0..REP_CNT {
        let mut board = BoardGen::generate(size);
        assert!(AC3Solver::bit(&mut board).solve());
    }
}

#[test]
fn board_4_gen_n_times_test() {
    board_gen_n_times(4);
}

#[test]
fn board_6_gen_n_times_test() {
    board_gen_n_times(6);
}

#[test]
fn board_8_gen_n_times_test() {
    board_gen_n_times(8);
}
