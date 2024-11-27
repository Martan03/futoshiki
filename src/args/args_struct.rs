use termint::{
    enums::Color,
    help,
    widgets::{Grad, StrSpanExtension},
};

use crate::{error::Error, solver::SolverType};

use super::{action::Action, bench_args::BenchArgs, game_args::GameArgs};

/// Parses given arguments and checks for arguments conditions
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Args {
    pub action: Action,
}

impl Args {
    /// Parses arguments
    pub fn parse(args: std::env::Args) -> Result<Args, Error> {
        let mut parsed = Self::default();

        let mut args_iter = args.into_iter().peekable();
        args_iter.next();
        while let Some(arg) = args_iter.peek() {
            match arg.as_str() {
                "bench" => {
                    args_iter.next();
                    let bench_args = BenchArgs::parse(&mut args_iter)?;
                    parsed.action = Action::Benchmark(bench_args);
                }
                "-h" | "--help" => {
                    args_iter.next();
                    parsed.action = Action::Help;
                }
                _ => {
                    let game_args = GameArgs::parse(&mut args_iter)?;
                    parsed.action = Action::Game(game_args);
                }
            }
        }

        Ok(parsed)
    }

    /// Displays help
    pub fn help() {
        println!(
            "Welcome to help for {} by {}\n",
            "futoshiki".fg(Color::Green),
            Grad::new("Martin Slezák", (0, 220, 255), (175, 80, 255))
        );
        help!(
            "Usage":
            "futoshiki" ["game_options"] =>
                "Starts TUI game according to options\n"
            "futoshiki bench" ["bench_options"] =>
                "Starts the benchmark according to options\n"
            "Game options":
            "-s  --size" ["num"] => "Sets size of the game (default 4)\n"
            "--solver" ["solver_type"] => "Sets the solver to be used\n"
            "Bench options":
            "-s  --size" ["num"] => {
                "Sets size of the game (default 4)",
                "Note: can be set multiple times\n"
            }
            "--solver" ["solver_type"] => {
                "Sets the solver to benchmark (default all)",
                "Note: can be set multiple times\n"
            }
            "-r  --repeats" ["num"] =>
                "Sets repeats of the benchmark (default 10)\n"
            "Solver types":
            "bt  backtrack  backtracking" =>
                "Backtracking solver implementation"
            "fc  forward-check  forward-checking" =>
                "Forward checking solver implementation using hash sets"
            "fcb  forward-check-bit  forward-checking-bit" =>
                "Forward checking solver implementation using bitmaps"
            "ac3  arc-cons3  arc-consistency3" =>
                "Arc Consistency #3 solver implementation using hash sets"
            "ac3b  arc-cons3-bit  arc-consistency3-bit" =>
                "Arc Consistency #3 solver implementation using bitmaps"
        );
    }

    /// Parses the solver from the given arguments
    pub fn parse_solver<T>(args: &mut T) -> Result<SolverType, Error>
    where
        T: Iterator<Item = String>,
    {
        match args.next().ok_or("missing arguments for solver")?.as_str() {
            "bt" | "backtrack" | "backtracking" => Ok(SolverType::Backtrack),
            "fcb" | "forward-check-bit" | "forward-checking-bit" => {
                Ok(SolverType::ForwardBitCheck)
            }
            "fc" | "forward-check" | "forward-checking" => {
                Ok(SolverType::ForwardCheck)
            }
            "ac3" | "arc-cons3" | "arc-consistency3" => {
                Ok(SolverType::ArcConsistency3)
            }
            "ac3b" | "arc-cons3-bit" | "arc-consistency3-bit" => {
                Ok(SolverType::ArcConsistency3Bit)
            }
            _ => Err("invalid solver option".into()),
        }
    }

    /// Gets number (usize) from args
    pub fn get_num<T>(args: &mut T) -> Result<usize, Error>
    where
        T: Iterator<Item = String>,
    {
        let Some(val) = args.next() else {
            return Err(Error::Msg("missing argument parameter".into()));
        };

        val.parse::<usize>()
            .map_err(|_| Error::Msg(format!("number expected, got '{val}'")))
    }
}
