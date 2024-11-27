use std::collections::HashSet;

use crate::{args::args_struct::Args, error::Error, solver::SolverType};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BenchArgs {
    pub sizes: HashSet<usize>,
    pub solvers: HashSet<SolverType>,
    pub repeats: usize,
}

impl BenchArgs {
    pub fn parse<T>(args: &mut T) -> Result<BenchArgs, Error>
    where
        T: Iterator<Item = String>,
    {
        let mut parsed = Self::default();
        while let Some(arg) = args.next() {
            match arg.as_str() {
                "-s" | "--size" => {
                    _ = parsed.sizes.insert(Args::get_num(args)?)
                }
                "--solver" => {
                    _ = parsed.solvers.insert(Args::parse_solver(args)?)
                }
                "-r" | "--repeats" => parsed.repeats = Args::get_num(args)?,
                arg => Err(format!("unexpected argument: '{arg}'"))?,
            }
        }
        Ok(parsed)
    }
}

impl Default for BenchArgs {
    fn default() -> Self {
        Self {
            sizes: HashSet::new(),
            solvers: HashSet::new(),
            repeats: 10,
        }
    }
}
