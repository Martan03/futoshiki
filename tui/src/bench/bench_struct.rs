use std::time::{Duration, Instant};

use super::bench_stat::BenchStat;

/// Implements methods for running the time complexity benchmark
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Bench {
    stat: BenchStat,
}

impl Bench {
    /// Runs the benchmark on given function given repeats
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

    /// Runs the benchmark on given function given repeats, but each function
    /// run has given timeout.
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

#[cfg(test)]
mod tests {
    use std::{thread::sleep, time::Duration};

    use super::Bench;

    #[test]
    fn run_with_timeout_times_out() {
        let func = || sleep(Duration::from_millis(10));

        let stat = Bench::run_with_timeout(func, 5, Duration::from_millis(1));

        assert_eq!(stat, None);
    }

    #[test]
    fn run_with_timeout_pass() {
        let func = || sleep(Duration::from_millis(1));

        let stat = Bench::run_with_timeout(func, 5, Duration::from_millis(2));

        assert!(stat.is_some());
    }
}
