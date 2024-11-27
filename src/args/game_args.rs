use crate::{args::args_struct::Args, error::Error, solver::SolverType};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GameArgs {
    pub size: usize,
    pub solver: SolverType,
}

impl GameArgs {
    pub fn parse<T>(args: &mut T) -> Result<GameArgs, Error>
    where
        T: Iterator<Item = String>,
    {
        let mut parsed = Self::default();
        while let Some(arg) = args.next() {
            match arg.as_str() {
                "-s" | "--size" => parsed.size = Args::get_num(args)?,
                "--solver" => parsed.solver = Args::parse_solver(args)?,
                arg => Err(format!("unexpected argument: '{arg}'"))?,
            }
        }
        Ok(parsed)
    }
}

impl Default for GameArgs {
    fn default() -> Self {
        Self {
            size: 4,
            solver: Default::default(),
        }
    }
}
