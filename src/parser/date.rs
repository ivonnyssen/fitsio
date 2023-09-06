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

pub fn date(i: &[u8]) -> IResult<&[u8], Value, VerboseError<&[u8]>> {
    //CCYY-MM-DD[Thh:mm:ss[.s...]]
    context(
        "date",
        map(
            preceded(space0, take_while(is_ascii_text_char)),
            |s: &[u8]| Value::Date(std::str::from_utf8(s).unwrap().trim_end().to_string()),
        ),
    )(i)
}

fn is_ascii_text_char(c: u8) -> bool {
    (32u8..=126u8).contains(&c)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_date() {
        assert_eq!(
            date(b"0000-01-01T00:00:00                                                   "),
            Ok((&b""[..], Value::Date("0000-01-01T00:00:00".to_string())))
        );
        assert_eq!(
            date(b"                                                   9999-12-31T23:59:59"),
            Ok((&b""[..], Value::Date("9999-12-31T23:59:59".to_string())))
        );
        assert_eq!(
            date(b"99999-01-01T00:00:00                                                  "),
            Ok((&b""[..], Value::Date("99999-01-01T00:00:00".to_string())))
        );
        assert_eq!(
            date(b"+99999-12-31T23:59:59                                                 "),
            Ok((&b""[..], Value::Date("+99999-12-31T23:59:59".to_string())))
        );
        assert_eq!(
            date(b"-04713-11-24T12:00:00                                                 "),
            Ok((&b""[..], Value::Date("-04713-11-24T12:00:00".to_string())))
        );
    }
}
