use pareg::Pareg;

use crate::{error::Result, solver::SolverType, tui::Theme};

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct GameArgs {
    pub size: Option<usize>,
    pub solver: Option<SolverType>,
    pub theme: Option<Theme>,
}

impl GameArgs {
    pub fn parse(args: &mut Pareg) -> Result<GameArgs> {
        let mut parsed = Self::default();
        while let Some(arg) = args.next() {
            match arg {
                "-s" | "--size" => parsed.size = args.next_arg()?,
                "--solver" => parsed.solver = args.next_arg()?,
                "-t" | "--theme" => parsed.theme = args.next_arg()?,
                arg => Err(format!("unexpected argument: '{arg}'"))?,
            }
        }
        Ok(parsed)
    }
}
