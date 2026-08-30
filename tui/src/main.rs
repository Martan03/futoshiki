use std::{
    env,
    fs::create_dir_all,
    process::{Command, ExitCode},
};

use app::App;
use args::{
    action::Action, args_struct::Args, bench_args::BenchArgs,
    game_args::GameArgs,
};
use bench::{doc_bench::DocBench, solver_bench::SolverBench};
use config::Config;
use pareg::Pareg;
use termint::{enums::Color, style::Stylize, term::Term};

use crate::error::Error;

mod app;
mod args;
mod bench;
mod config;
mod error;
mod solver_type;
mod tui;

fn main() -> ExitCode {
    // test_solver();
    // return ExitCode::SUCCESS;
    match run() {
        Ok(_) => ExitCode::SUCCESS,
        Err(e) => {
            eprintln!("{} {}", "Error:".fg(Color::Red), e);
            ExitCode::FAILURE
        }
    }
}

fn run() -> Result<(), Error> {
    let args = Args::parse(Pareg::args())?;
    match args.action {
        Action::Game(game_args) => run_app(game_args),
        Action::Benchmark(bench_args) => run_benchmark(bench_args),
        Action::Doc => run_doc_benchmark(),
        Action::Config => config(),
        Action::Help => {
            Args::help();
            Ok(())
        }
    }
}

fn run_app(args: GameArgs) -> Result<(), Error> {
    let config = Config::load();
    let mut app = App::new(config, args);
    Term::default()
        .setup()?
        .small_screen(App::small_screen())
        .run(&mut app)?;
    Ok(())
}

fn run_benchmark(args: BenchArgs) -> Result<(), Error> {
    SolverBench::run(args);
    Ok(())
}

fn run_doc_benchmark() -> Result<(), Error> {
    DocBench::run();
    Ok(())
}

fn config() -> Result<(), Error> {
    let editor = env::var("EDITOR").unwrap_or("vi".to_string());
    create_dir_all(Config::get_dir())?;
    let file = Config::get_path();
    if !file.exists() {
        Config::default().save()?;
    }

    Command::new(editor).arg(file).spawn()?.wait()?;
    Ok(())
}

// #[allow(unused)]
// fn test_solver() {
//     let mut board = Board::tricky();
//     let stat =
//         Bench::run(|| _ = AC3Solver::bit(&mut board.clone()).solve(), 10000);

//     println!(
//         "{}Forward Checking:\n\
//             {}└>\x1b[0m Time: [{}{:?} {}{:?} {}{:?}\x1b[0m]",
//         Color::Green.to_fg(),
//         Color::Gray.to_fg(),
//         Color::Gray.to_fg(),
//         stat.min_time,
//         Color::White.to_fg(),
//         stat.avg_time(),
//         Color::Gray.to_fg(),
//         stat.max_time
//     );
// }
