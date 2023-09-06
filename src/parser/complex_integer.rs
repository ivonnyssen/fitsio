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

pub fn complex_integer(i: &[u8]) -> IResult<&[u8], Value, VerboseError<&[u8]>> {
    context(
        "complex integer",
        map(
            preceded(
                space0,
                terminated(
                    separated_pair(
                        preceded(tag("("), preceded(space0, i64)),
                        tag(","),
                        preceded(space0, terminated(i64, tag(")"))),
                    ),
                    space0,
                ),
            ),
            Value::ComplexInteger,
        ),
    )(i)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_complex_integer() {
        assert_eq!(
            complex_integer(
                b"( 123, 45)                                                            "
            ),
            Ok((&b""[..], Value::ComplexInteger((123, 45))))
        );
        assert_eq!(
            complex_integer(
                b"  (123, 45)                                                           "
            ),
            Ok((&b""[..], Value::ComplexInteger((123, 45))))
        );
        assert_eq!(
            complex_integer(
                b"(-123,-45)                                                            "
            ),
            Ok((&b""[..], Value::ComplexInteger((-123, -45))))
        );
        assert_eq!(
            complex_integer(
                b"(+123, +45)                                                           "
            ),
            Ok((&b""[..], Value::ComplexInteger((123, 45))))
        );
        assert_ne!(
            complex_integer(
                b"(-500,-45)                                                            "
            ),
            Ok((&b""[..], Value::ComplexInteger((-123, -45))))
        );
    }
}
