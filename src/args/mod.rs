use std::{fmt::Display, str::FromStr};

use pareg::{ArgErrCtx, ArgError, Pareg};

pub mod action;
pub mod args_struct;
pub mod bench_args;
pub mod game_args;

/// Reads next value greater then given value from arguments, returns error on
/// failure.
fn next_greater<T>(
    args: &mut Pareg,
    cmp: T,
) -> std::result::Result<T, ArgError>
where
    T: FromStr + PartialOrd + ToString + Copy,
    T::Err: Display,
{
    args.next_manual(|a| is_greater(a, cmp))
}

/// Parses given argument and compares it to the given value. Return the value
/// on success and when meeting the greater condition, otherwise returns error.
fn is_greater<T>(arg: &str, cmp: T) -> std::result::Result<T, ArgError>
where
    T: FromStr + PartialOrd + ToString,
    T::Err: Display,
{
    let val = arg
        .parse::<T>()
        .map_err(|e| ArgError::parse_msg(e.to_string(), arg.to_string()))?;

    match val > cmp {
        true => Ok(val),
        false => Err(ArgError::InvalidValue(Box::new(ArgErrCtx::from_inner(
            format!("value must be greater then {}", val.to_string()),
            val.to_string(),
        )))),
    }
}

#[cfg(test)]
pub mod tests {
    use super::is_greater;

    #[test]
    fn is_greater_usize() {
        assert!(is_greater("0", 0).is_err());
        assert!(is_greater("-10", 0).is_err());
        assert!(is_greater("4", 3).is_ok());
    }
}
