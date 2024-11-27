use super::{bench_args::BenchArgs, game_args::GameArgs};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Action {
    Game(GameArgs),
    Benchmark(BenchArgs),
    Help,
}

impl Default for Action {
    fn default() -> Self {
        Self::Game(GameArgs::default())
    }
}
