use crate::types::Value;

use nom::{
    bytes::complete::tag,
    character::complete::space0,
    combinator::map,
    error::context,
    error::VerboseError,
    number::complete::double,
    sequence::{preceded, separated_pair, terminated},
    IResult,
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
