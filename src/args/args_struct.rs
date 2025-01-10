use pareg::Pareg;
use termint::{
    enums::Color,
    help,
    widgets::{Grad, StrSpanExtension},
};

use crate::error::Result;

use super::{action::Action, bench_args::BenchArgs, game_args::GameArgs};

/// Parses given arguments and checks for arguments conditions
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Args {
    pub action: Action,
}

impl Args {
    /// Parses arguments
    pub fn parse(mut args: Pareg) -> Result<Args> {
        let mut parsed = Self::default();

        while let Some(arg) = args.peek() {
            match arg {
                "bench" => {
                    args.next();
                    let bench_args = BenchArgs::parse(&mut args)?;
                    parsed.action = Action::Benchmark(bench_args);
                }
                "-h" | "--help" => {
                    args.next();
                    parsed.action = Action::Help;
                }
                _ => {
                    let game_args = GameArgs::parse(&mut args)?;
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
}
