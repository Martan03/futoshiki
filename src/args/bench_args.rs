use std::collections::HashSet;

use pareg::Pareg;

use crate::{error::Result, solver::SolverType};

#[derive(Debug, Clone, PartialEq)]
pub struct BenchArgs {
    pub sizes: HashSet<usize>,
    pub solvers: HashSet<SolverType>,
    pub repeats: usize,
    pub boards: usize,
    pub timeout: Option<f64>,
}

impl BenchArgs {
    pub fn parse(args: &mut Pareg) -> Result<BenchArgs> {
        let mut parsed = Self::default();
        while let Some(arg) = args.next() {
            match arg {
                "-s" | "--size" => _ = parsed.sizes.insert(args.next_arg()?),
                "--solver" => _ = parsed.solvers.insert(args.next_arg()?),
                "-r" | "--repeats" => parsed.repeats = args.next_arg()?,
                "-b" | "--boards" => parsed.boards = args.next_arg()?,
                "-t" | "--timeout" => parsed.timeout = Some(args.next_arg()?),
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
            boards: 1,
            timeout: None,
        }
    }
}
