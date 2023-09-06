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

pub fn logical(i: &[u8]) -> IResult<&[u8], Value, VerboseError<&[u8]>> {
    context(
        "logical",
        map(
            preceded(space0, terminated(alt((tag("T"), tag("F"))), space0)),
            |s: &[u8]| Value::Logical(s == b"T"),
        ),
    )(i)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_logical() {
        assert_eq!(
            logical(b"                   T                                                  "),
            Ok((&b""[..], Value::Logical(true)))
        );
        assert_eq!(
            logical(b"                   F                                                  "),
            Ok((&b""[..], Value::Logical(false)))
        );
        assert_eq!(
            logical(b"T                                                                     "),
            Ok((&b""[..], Value::Logical(true)))
        );
        assert_eq!(
            logical(b" F                                                                    "),
            Ok((&b""[..], Value::Logical(false)))
        );
        assert_ne!(
            logical(b" T   /Test comment                                                    "),
            Ok((
                &b"/Test comment                                                    "[..],
                Value::Logical(false)
            ))
        );
    }
}
