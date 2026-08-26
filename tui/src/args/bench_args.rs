use std::collections::HashSet;

use pareg::Pareg;

use crate::{error::Result, solver_type::SolverType};

use super::next_greater;

/// Benchmark arguments helper struct.
#[derive(Debug, Clone, PartialEq)]
pub struct BenchArgs {
    pub sizes: HashSet<usize>,
    pub solvers: HashSet<SolverType>,
    pub repeats: usize,
    pub boards: usize,
    pub timeout: Option<f64>,
}

impl BenchArgs {
    /// Parses the benchmark arguments.
    pub fn parse(args: &mut Pareg) -> Result<BenchArgs> {
        let mut parsed = Self::default();
        while let Some(arg) = args.next() {
            match arg.as_str() {
                "-s" | "--size" => {
                    _ = parsed.sizes.insert(next_greater(args, 1)?)
                }
                "--solver" => _ = parsed.solvers.insert(args.next_arg()?),
                "-r" | "--repeats" => parsed.repeats = next_greater(args, 0)?,
                "-b" | "--boards" => parsed.boards = next_greater(args, 0)?,
                "-t" | "--timeout" => {
                    parsed.timeout = Some(next_greater(args, 0.)?)
                }
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

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use pareg::Pareg;

    use crate::solver_type::SolverType;

    use super::BenchArgs;

    #[test]
    fn bench_args_parse() {
        let args = vec![
            "-s", "10", "-s", "5", "--solver", "fc", "-r", "3", "-b", "5",
            "-t", "10", "--solver", "ac3",
        ];
        let mut pareg =
            Pareg::new(args.iter().map(|v| v.to_string()).collect());

        let res = BenchArgs::parse(&mut pareg);

        let expected = BenchArgs {
            sizes: HashSet::from([10, 5]),
            solvers: HashSet::from([SolverType::FC, SolverType::AC3]),
            repeats: 3,
            boards: 5,
            timeout: Some(10.),
        };
        assert_eq!(res.unwrap(), expected);
    }
}
