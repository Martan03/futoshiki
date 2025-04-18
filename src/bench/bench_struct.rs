use std::time::{Duration, Instant};

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

    pub fn run_with_timeout<F>(
        func: F,
        cnt: usize,
        timeout: Duration,
    ) -> Option<BenchStat>
    where
        F: Fn() + Send + Clone + 'static,
    {
        let mut bench = Self::default();

        for _ in 0..cnt {
            let (sender, receiver) = std::sync::mpsc::channel();
            let func_clone = func.clone();
            std::thread::spawn(move || {
                let start = Instant::now();
                func_clone();
                let _ = sender.send(start.elapsed());
            });

            match receiver.recv_timeout(timeout) {
                Ok(duration) => bench.stat.add(duration),
                Err(_) => return None,
            }
        }
        Some(bench.stat)
    }

    fn run_one<F>(&mut self, func: &F)
    where
        F: Fn(),
    {
        let start = Instant::now();
        func();
        self.stat.add(start.elapsed());
    }
}
