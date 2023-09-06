use crate::types::Value;

use nom::{
    bytes::complete::take_while, character::complete::space0, combinator::map, error::context,
    error::VerboseError, sequence::preceded, IResult,
};

pub fn date(i: &[u8]) -> IResult<&[u8], Value, VerboseError<&[u8]>> {
    //CCYY-MM-DD[Thh:mm:ss[.s...]]
    context(
        "date",
        map(
            preceded(space0, take_while(super::is_allowed_ascii)),
            |s: &[u8]| Value::Date(std::str::from_utf8(s).unwrap().trim_end().to_string()),
        ),
    )(i)
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
