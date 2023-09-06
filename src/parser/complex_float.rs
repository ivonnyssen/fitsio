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

pub fn complex_float(i: &[u8]) -> IResult<&[u8], Value, VerboseError<&[u8]>> {
    context(
        "complex integer",
        map(
            preceded(
                space0,
                terminated(
                    separated_pair(
                        preceded(tag("("), double),
                        tag(","),
                        preceded(space0, terminated(double, tag(")"))),
                    ),
                    space0,
                ),
            ),
            Value::ComplexFloat,
        ),
    )(i)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_complex_float() {
        assert_eq!(
            complex_float(
                b"(123.23, -45.7)                                                       "
            ),
            Ok((&b""[..], Value::ComplexFloat((123.23, -45.7))))
        );
        assert_eq!(
            complex_float(
                b" (+123.23, 45.7)                                                      "
            ),
            Ok((&b""[..], Value::ComplexFloat((123.23, 45.7))))
        );
        assert_eq!(
            complex_float(
                b"(-123.23, +45.7)                                                      "
            ),
            Ok((&b""[..], Value::ComplexFloat((-123.23, 45.7))))
        );
        assert_ne!(
            complex_float(
                b"(500.23, -45.7)                                                       "
            ),
            Ok((&b""[..], Value::ComplexFloat((123.23, -45.7))))
        );
    }
}
