use termint::enums::Color;

use crate::{board::board_gen::BoardGen, solver::SolverType};

use super::{bench_stat::BenchStat, bench_struct::Bench};

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

        let mut stats = vec![];
        for solver in SolverType::solvers() {
            let stat = Bench::run(
                || _ = solver.solve(&mut board.clone()),
                self.repeats,
            );
            stats.push((solver, stat));
        }

        stats.sort_by_key(|(_, stat)| stat.total_time);
        for (i, (solver, stat)) in stats.iter().enumerate() {
            print!("{}{}. ", Color::Gray.to_fg(), i + 1);
            Self::print_stat(solver, stat);
        }
    }

    fn print_stat(solver: &SolverType, stat: &BenchStat) {
        println!(
            "{}{solver}:\n\
            {}└>\x1b[0m Time: [{}{:?} {}{:?} {}{:?}\x1b[0m]",
            Color::Green.to_fg(),
            Color::Gray.to_fg(),
            Color::Gray.to_fg(),
            stat.min_time,
            Color::White.to_fg(),
            stat.avg_time(),
            Color::Gray.to_fg(),
            stat.max_time
        );
    }
}
