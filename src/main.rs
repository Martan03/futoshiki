use std::{
    env,
    fs::create_dir_all,
    io::{stdout, Write},
    panic::{set_hook, take_hook},
    process::{Command, ExitCode},
};

use app::App;
use args::{
    action::Action, args_struct::Args, bench_args::BenchArgs,
    game_args::GameArgs,
};
use bench::solver_bench::SolverBench;
use board::{board_struct::Board, cell::Cell};
use config::Config;
use crossterm::terminal::{disable_raw_mode, is_raw_mode_enabled};
use error::Error;
use pareg::Pareg;
use solver::{ac3_solver::AC3Solver, Solver};
use termint::{
    buffer::Buffer,
    enums::Color,
    geometry::{Rect, Vec2},
    widgets::{StrSpanExtension, Widget},
};
use tui::theme::Theme;

mod app;
mod args;
mod bench;
mod board;
mod checker;
mod config;
mod error;
mod solver;
mod tests;
mod tui;

fn main() -> ExitCode {
    match run() {
        Ok(_) => ExitCode::SUCCESS,
        Err(e) => {
            eprintln!("{} {}", "Error:".fg(Color::Red), e);
            ExitCode::FAILURE
        }
    }
}

fn run() -> Result<(), Error> {
    register_panic_hook();

    let args = Args::parse(Pareg::args())?;
    match args.action {
        Action::Game(game_args) => run_app(game_args),
        Action::Benchmark(bench_args) => run_benchmark(bench_args),
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
    app.run()
}

fn run_benchmark(args: BenchArgs) -> Result<(), Error> {
    SolverBench::run(args);
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

fn register_panic_hook() {
    let hook = take_hook();
    set_hook(Box::new(move |pi| {
        if is_raw_mode_enabled().unwrap_or_default() {
            // Restores screen
            print!("\x1b[?1049l\x1b[?25h");
            _ = stdout().flush();
            _ = disable_raw_mode();
        }
        hook(pi);
    }));
}

#[allow(unused)]
fn test_solver() -> Result<(), Error> {
    let mut board = Board {
        cells: vec![Cell::empty(); 16],
        hor_conds: vec![None; 12],
        ver_conds: vec![None; 12],
        selected: Vec2::new(0, 0),
        size: 4,
        theme: Theme::dark(),
    };
    board.hor_conds[1] = Some(true);
    board.hor_conds[2] = Some(true);
    board.hor_conds[11] = Some(true);
    board.ver_conds[0] = Some(false);
    board.ver_conds[4] = Some(true);
    board.ver_conds[5] = Some(false);
    board.ver_conds[6] = Some(false);

    if AC3Solver::bit(&mut board).solve() {
        println!("Solved!");
        let mut buffer = Buffer::empty(Rect::new(1, 1, 20, 10));
        board.render(&mut buffer);
        buffer.render();
        Ok(())
    } else {
        Err(Error::Msg("not solved".to_string()))
    }
}
