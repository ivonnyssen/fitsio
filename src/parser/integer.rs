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

pub fn integer(i: &[u8]) -> IResult<&[u8], Value, VerboseError<&[u8]>> {
    context(
        "integer",
        map(preceded(space0, terminated(i64, space0)), Value::Integer),
    )(i)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_integer() {
        assert_eq!(
            integer(b"+300                                                                  "),
            Ok((&b""[..], Value::Integer(300)))
        );
        assert_eq!(
            integer(b"  -300                                                                "),
            Ok((&b""[..], Value::Integer(-300)))
        );
        assert_eq!(
            integer(b" 300                                                                  "),
            Ok((&b""[..], Value::Integer(300)))
        );
        assert_eq!(
            integer(b"300                                                                   "),
            Ok((&b""[..], Value::Integer(300)))
        );
        assert_ne!(
            integer(b"+500                                                                  "),
            Ok((&b""[..], Value::Integer(300)))
        );
    }
}
