use std::{fmt::Display, time::Duration};

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct BenchStat {
    pub times: Vec<Duration>,
}

impl Display for BenchStat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let sum: Duration = self.times.iter().sum();
        let avg = sum / self.times.len();
        write!(f, "Avg. time: {}", avg)
    }
}
