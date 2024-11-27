use crate::{board::board_gen::BoardGen, solver::SolverType};

use super::bench_struct::Bench;

pub struct SolverBench {
    repeats: usize,
    board_size: usize,
}

impl SolverBench {
    pub fn run(board_size: usize, repeats: usize) {
        let bench = Self {
            repeats,
            board_size,
        };

        bench.test_board();
    }

    fn test_board(&self) {
        let board = BoardGen::generate(self.board_size);

        for solver in SolverType::solvers() {
            let stat = Bench::run(
                || _ = solver.solve(&mut board.clone()),
                self.repeats,
            );
        }
    }
}
