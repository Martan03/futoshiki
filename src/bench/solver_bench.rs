use termint::enums::Color;

use crate::{
    args::bench_args::BenchArgs, board::board_gen::BoardGen,
    solver::SolverType,
};

use super::{bench_stat::BenchStat, bench_struct::Bench, charter::Charter};

#[derive(Debug, Clone)]
pub struct SolverBench {
    repeats: usize,
    solvers: Vec<SolverType>,
    charter: Charter,
}

impl SolverBench {
    pub fn run(args: BenchArgs) {
        let mut solvers: Vec<SolverType> =
            args.solvers.iter().copied().collect();
        if solvers.is_empty() {
            solvers = SolverType::solvers().to_vec();
        }

        let mut bench = Self {
            repeats: args.repeats,
            solvers,
            charter: Charter::empty("Solver benchmark"),
        };

        let mut sizes: Vec<_> = args.sizes.iter().copied().collect();
        sizes.sort();
        for size in sizes {
            println!("{size}x{size} board, {} repeats", bench.repeats);
            bench.test_board(size);
        }

        _ = bench.charter.plot("benchmark.png");
    }

    fn test_board(&mut self, size: usize) {
        let board = BoardGen::generate(size);

        let mut stats = vec![];
        for solver in self.solvers.iter() {
            let stat = Bench::run(
                || _ = solver.solve(&mut board.clone()),
                self.repeats,
            );
            stats.push((solver, stat));
        }

        stats.sort_by_key(|(_, stat)| stat.total_time);

        for (i, (solver, stat)) in stats.iter().enumerate() {
            print!("{}{}. ", Color::Gray.to_fg(), i + 1);

            let secs = stat.avg_time().as_secs_f64();
            self.charter.push(solver, size as i32, secs);
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
