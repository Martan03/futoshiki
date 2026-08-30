use pareg::Pareg;

use crate::{error::Result, solver_type::SolverType, tui::ThemeType};

use super::next_greater;

/// Game arguments helper struct.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct GameArgs {
    pub size: Option<usize>,
    pub solver: Option<SolverType>,
    pub theme: Option<ThemeType>,
}

impl GameArgs {
    /// Parses the game arguments.
    pub fn parse(args: &mut Pareg) -> Result<GameArgs> {
        let mut parsed = Self::default();
        while let Some(arg) = args.next() {
            match arg.as_str() {
                "-s" | "--size" => parsed.size = Some(next_greater(args, 1)?),
                "--solver" => parsed.solver = args.next_arg()?,
                "-t" | "--theme" => parsed.theme = args.next_arg()?,
                arg => Err(format!("unexpected argument: '{arg}'"))?,
            }
        }
        Ok(parsed)
    }
}

#[cfg(test)]
mod tests {
    use pareg::Pareg;

    use crate::{
        args::game_args::GameArgs, solver_type::SolverType, tui::ThemeType,
    };

    #[test]
    fn game_args_parse() {
        let args = vec![
            "-s", "10", "-s", "5", "--solver", "fc", "-t", "light",
            "--solver", "ac3",
        ];
        let mut pareg =
            Pareg::new(args.iter().map(|v| v.to_string()).collect());

        let res = GameArgs::parse(&mut pareg);

        let expected = GameArgs {
            size: Some(5),
            solver: Some(SolverType::AC3),
            theme: Some(ThemeType::Light),
        };
        assert_eq!(res.unwrap(), expected);
    }
}
