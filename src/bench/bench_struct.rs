use std::time::Instant;

use super::bench_stat::BenchStat;

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Bench {
    stat: BenchStat,
}

impl Bench {
    pub fn run<F>(func: F, cnt: usize) -> BenchStat
    where
        F: Fn(),
    {
        let mut bench = Self::default();

        for _ in 0..cnt {
            bench.run_one(&func);
        }
        bench.stat
    }

    pub fn run_one<F>(&mut self, func: &F)
    where
        F: Fn(),
    {
        let start = Instant::now();
        func();
        self.stat.add(start.elapsed());
    }
}
