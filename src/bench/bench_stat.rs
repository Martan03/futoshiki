use std::{
    fmt::Display,
    ops::{Add, AddAssign},
    time::Duration,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BenchStat {
    pub total_time: Duration,
    pub cnt: u32,
    pub min_time: Duration,
    pub max_time: Duration,
}

impl BenchStat {
    pub fn add(&mut self, time: Duration) {
        self.total_time += time;
        self.cnt += 1;
        self.min_time = self.min_time.min(time);
        self.max_time = self.max_time.max(time);
    }

    pub fn join(&mut self, other: Self) {
        self.total_time += other.total_time;
        self.cnt += other.cnt;
        self.min_time = self.min_time.min(other.min_time);
        self.max_time = self.max_time.max(other.max_time);
    }

    pub fn avg_time(&self) -> Duration {
        self.total_time / self.cnt
    }
}

impl Add for BenchStat {
    type Output = BenchStat;

    fn add(mut self, rhs: Self) -> Self::Output {
        self.join(rhs);
        self
    }
}

impl AddAssign for BenchStat {
    fn add_assign(&mut self, rhs: Self) {
        self.join(rhs)
    }
}

impl Default for BenchStat {
    fn default() -> Self {
        Self {
            total_time: Duration::from_secs(0),
            cnt: 0,
            min_time: Duration::from_secs(u64::MAX),
            max_time: Duration::from_secs(0),
        }
    }
}

impl Display for BenchStat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let avg = self.total_time / self.cnt;
        write!(f, "Avg. time: {:?}", avg)
    }
}
