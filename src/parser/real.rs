use crate::types::Value;

use nom::{
    branch::alt,
    bytes::complete::{tag, take, take_while},
    character::complete::{i64, space0},
    combinator::{all_consuming, complete, map, opt},
    error::context,
    error::VerboseError,
    multi::many0,
    number::complete::double,
    sequence::{pair, preceded, separated_pair, terminated, tuple},
    IResult, Parser,
};

pub fn real(i: &[u8]) -> IResult<&[u8], Value, VerboseError<&[u8]>> {
    context(
        "real",
        map(preceded(space0, terminated(double, space0)), Value::Real),
    )(i)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_real() {
        assert_eq!(
            real(b"+300.1                                                                "),
            Ok((&b""[..], Value::Real(300.1)))
        );
        assert_eq!(
            real(b"-300.1                                                                "),
            Ok((&b""[..], Value::Real(-300.1)))
        );
        assert_eq!(
            real(b" 300.1                                                                "),
            Ok((&b""[..], Value::Real(300.1)))
        );
        assert_eq!(
            real(b"300.1                                                                 "),
            Ok((&b""[..], Value::Real(300.1)))
        );
        assert_ne!(
            real(b"+500.1                                                                "),
            Ok((&b""[..], Value::Real(300.1)))
        );
    }
}
