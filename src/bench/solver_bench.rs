use std::{
    collections::{HashMap, HashSet},
    time::Duration,
};

use termint::enums::Color;

use crate::{
    args::bench_args::BenchArgs,
    board::{board_gen::BoardGen, board_struct::Board},
    solver::SolverType,
};

use super::{bench_stat::BenchStat, bench_struct::Bench, charter::Charter};

#[derive(Debug, Clone)]
pub struct SolverBench {
    repeats: usize,
    boards: usize,
    timeout: Duration,
    solvers: Vec<SolverType>,
    disqualified: HashSet<SolverType>,
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
            boards: args.boards,
            timeout: Duration::from_secs_f64(args.timeout),
            solvers,
            disqualified: HashSet::new(),
            charter: Charter::empty("Solver benchmark"),
        };

        let mut sizes: Vec<_> = args.sizes.iter().copied().collect();
        sizes.sort();
        for size in sizes {
            println!("{size}x{size} board, {} repeats", bench.repeats);
            bench.test_boards(size);
        }

        _ = bench.charter.plot("benchmark.png");
    }

    fn test_boards(&mut self, size: usize) {
        let mut stats: HashMap<SolverType, BenchStat> = HashMap::new();
        for _ in 0..self.boards {
            let board = BoardGen::generate(size);
            self.test_solvers(&mut stats, board);
        }

        let mut stats: Vec<_> = stats.iter().collect();
        stats.sort_by_key(|(_, stat)| stat.total_time);
        for (i, (solver, stat)) in stats.iter().enumerate() {
            print!("{}{}. ", Color::Gray.to_fg(), i + 1);

            let secs = stat.avg_time().as_secs_f64();
            self.charter.push(solver, size as i32, secs);
            Self::print_stat(solver, stat);
        }
    }

    fn test_solvers(
        &mut self,
        stats: &mut HashMap<SolverType, BenchStat>,
        board: Board,
    ) {
        for solver in self.solvers.iter() {
            if self.disqualified.contains(solver) {
                continue;
            }

            let solver = *solver;
            let board = board.clone();
            let stat = Bench::run_with_timeout(
                move || _ = solver.solve(&mut board.clone()),
                self.repeats,
                self.timeout,
            );
            let Some(stat) = stat else {
                self.disqualified.insert(solver);
                continue;
            };
            stats
                .entry(solver)
                .and_modify(|existing_stat| *existing_stat += stat.clone())
                .or_insert(stat.clone());
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
