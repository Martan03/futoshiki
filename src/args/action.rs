use super::{bench_args::BenchArgs, game_args::GameArgs};

/// App startup action set using the arguments. It allows to switch between
/// each functionality of the app.
#[derive(Debug, Clone, PartialEq)]
pub enum Action {
    // Open the game itself
    Game(GameArgs),
    // Starts solver algorithms benchmark
    Benchmark(BenchArgs),
    // Starts document related benchmarks
    Doc,
    // Opens config file
    Config,
    // Displays help
    Help,
}

impl Default for Action {
    fn default() -> Self {
        Self::Game(GameArgs::default())
    }
}
