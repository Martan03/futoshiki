use std::{fmt::Display, time::Duration, u64};

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

    pub fn avg_time(&self) -> Duration {
        self.total_time / self.cnt
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
