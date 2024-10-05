use termint::{
    enums::Color,
    help,
    widgets::{Grad, StrSpanExtension},
};

use crate::{error::Error, solver::SolverType};

/// Parses given arguments and checks for arguments conditions
#[derive(Debug)]
pub struct Args {
    pub size: usize,
    pub solver: SolverType,
    pub help: bool,
}

impl Args {
    /// Parses arguments
    pub fn parse(args: std::env::Args) -> Result<Args, Error> {
        let mut parsed = Self::default();

        let mut args_iter = args.into_iter();
        args_iter.next();
        while let Some(arg) = args_iter.next() {
            match arg.as_str() {
                "-s" | "--size" => {
                    parsed.size = Self::get_num(&mut args_iter)?
                }
                "--solver" => parsed.parse_solver(&mut args_iter)?,
                "-h" | "--help" => parsed.help = true,
                arg => Err(format!("unexpected argument: '{arg}'"))?,
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
            "futoshiki" => "Opens 4x4 game\n"
            "futoshiki" ["options"] => "Behaves according to options\n"
            "Options":
            "-s  --size" ["num"] => "Sets size of the game\n"
            "--solver" ["solver_type"] => "Sets the solver to be used\n"
            "-h  --help" => "Prints this help"
        );
    }

    /// Parses the solver from the given arguments
    fn parse_solver<T>(&mut self, args: &mut T) -> Result<(), Error>
    where
        T: Iterator<Item = String>,
    {
        match args.next().ok_or("missing arguments for solver")?.as_str() {
            "bt" | "backtrack" | "backtracking" => {
                self.solver = SolverType::Backtrack;
            }
            _ => return Err("invalid solver option".into()),
        }
        Ok(())
    }

    /// Gets number (usize) from args
    fn get_num<T>(args: &mut T) -> Result<usize, Error>
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

impl Default for Args {
    fn default() -> Self {
        Self {
            size: 4,
            solver: SolverType::Backtrack,
            help: false,
        }
    }
}
