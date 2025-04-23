use std::{
    fmt::Display,
    ops::{Add, AddAssign},
    time::Duration,
};

/// Time complexity benchmark statistics
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BenchStat {
    pub total_time: Duration,
    pub cnt: u32,
    pub min_time: Duration,
    pub max_time: Duration,
}

impl BenchStat {
    /// Adds time to the current benchmark statistics
    pub fn add(&mut self, time: Duration) {
        self.total_time += time;
        self.cnt += 1;
        self.min_time = self.min_time.min(time);
        self.max_time = self.max_time.max(time);
    }

    /// Joins given statistic to the current statistic
    pub fn join(&mut self, other: Self) {
        self.total_time += other.total_time;
        self.cnt += other.cnt;
        self.min_time = self.min_time.min(other.min_time);
        self.max_time = self.max_time.max(other.max_time);
    }

    /// Gets the average time
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

#[cfg(test)]
mod tests {
    use std::time::Duration;

    use super::BenchStat;

    fn get_stat(total: u64, cnt: u32, min: u64, max: u64) -> BenchStat {
        BenchStat {
            total_time: Duration::from_secs(total),
            cnt,
            min_time: Duration::from_secs(min),
            max_time: Duration::from_secs(max),
        }
    }

    #[test]
    fn bench_stat_add() {
        let mut stat = BenchStat::default();

        stat.add(Duration::from_secs(5));
        stat.add(Duration::from_secs(3));

        assert_eq!(stat, get_stat(8, 2, 3, 5));
    }

    #[test]
    fn bench_stat_join() {
        let mut stat = get_stat(8, 2, 3, 5);
        let another = get_stat(9, 3, 2, 4);

        stat.join(another.clone());
        assert_eq!(stat, get_stat(17, 5, 2, 5));

        stat += another;
        assert_eq!(stat, get_stat(26, 8, 2, 5));
    }

    #[test]
    fn bench_stat_avg_time() {
        let stat = get_stat(16, 5, 3, 5);

        assert_eq!(stat.avg_time(), Duration::from_secs_f64(3.2));
        assert_eq!(stat.to_string(), String::from("Avg. time: 3.2s"));
    }
}
