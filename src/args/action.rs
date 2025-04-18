use super::{bench_args::BenchArgs, game_args::GameArgs};

#[derive(Debug, Clone, PartialEq)]
pub enum Action {
    Game(GameArgs),
    Benchmark(BenchArgs),
    Config,
    Help,
}

impl Default for Action {
    fn default() -> Self {
        Self::Game(GameArgs::default())
    }
}
