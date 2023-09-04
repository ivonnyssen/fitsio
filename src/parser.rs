use std::u8;

use nom::{
    branch::alt,
    bytes::complete::{is_a, tag, take, take_while},
    character::complete::{i64, space0},
    character::is_digit,
    combinator::{map, opt},
    error::context,
    error::VerboseError,
    number::complete::double,
    sequence::{preceded, separated_pair, terminated},
    Err, IResult,
};

use crate::keywords::{Keyword, ParseKeywordError, ValueIndicator};
use crate::types::{KeywordRecord, Value};

fn keyword(i: &[u8]) -> IResult<&[u8], Result<Keyword, ParseKeywordError>, VerboseError<&[u8]>> {
    context("keyword", take(8u8))(i).map(|(i, res)| (i, res.try_into()))
}

fn value_indicator(i: &[u8]) -> IResult<&[u8], ValueIndicator, VerboseError<&[u8]>> {
    context("value_indicator", take(2u8))(i).map(|(i, res)| (i, res.into()))
}

/*fn value_with_optional_comment(i: &[u8]) -> IResult<&[u8], &[u8], VerboseError<&[u8]>> {
    context("value", take(70u8))(i)
        .pair(value, opt(comment))
        .map(|(i, res)| (i, res))
}*/

fn value(i: &[u8]) -> IResult<&[u8], Value, VerboseError<&[u8]>> {
    context(
        "value",
        alt((
            character_string,
            continued_string,
            logical,
            integer,
            real,
            complex_integer,
            complex_float,
            date,
        )),
    )(i)
}

fn character_string(i: &[u8]) -> IResult<&[u8], Value, VerboseError<&[u8]>> {
    context(
        "character_string",
        map(
            take_while(|c| is_ascii_text_char(c) && c != b'\''),
            |s: &[u8]| Value::CharacterString(std::str::from_utf8(s).unwrap()),
        ),
    )(i)
}

fn continued_string(i: &[u8]) -> IResult<&[u8], Value, VerboseError<&[u8]>> {
    todo!()
}

fn logical(i: &[u8]) -> IResult<&[u8], Value, VerboseError<&[u8]>> {
    context(
        "logical",
        map(alt((tag("T"), tag("F"))), |s: &[u8]| {
            Value::Logical(s == b"T")
        }),
    )(i)
}

fn integer(i: &[u8]) -> IResult<&[u8], Value, VerboseError<&[u8]>> {
    context("integer", map(preceded(space0, i64), Value::Integer))(i)
}

fn real(i: &[u8]) -> IResult<&[u8], Value, VerboseError<&[u8]>> {
    context("real", map(preceded(space0, double), Value::Real))(i)
}

fn complex_integer(i: &[u8]) -> IResult<&[u8], Value, VerboseError<&[u8]>> {
    context(
        "complex integer",
        map(
            separated_pair(
                preceded(tag("("), preceded(space0, i64)),
                tag(","),
                preceded(space0, terminated(i64, tag(")"))),
            ),
            Value::ComplexInteger,
        ),
    )(i)
}

fn complex_float(i: &[u8]) -> IResult<&[u8], Value, VerboseError<&[u8]>> {
    context(
        "complex integer",
        map(
            separated_pair(
                preceded(tag("("), double),
                tag(","),
                preceded(space0, terminated(double, tag(")"))),
            ),
            Value::ComplexFloat,
        ),
    )(i)
}

fn date(i: &[u8]) -> IResult<&[u8], Value, VerboseError<&[u8]>> {
    //CCYY-MM-DD[Thh:mm:ss[.s...]]
    context(
        "date",
        map(take_while(is_ascii_text_char), |s: &[u8]| {
            Value::Date(std::str::from_utf8(s).unwrap())
        }),
    )(i)
}

/*fn comment(i: &[u8]) -> IResult<&[u8], Option<&str>, VerboseError<&[u8]>> {
    tag("/")(i)
}*/

fn is_ascii_text_char(c: u8) -> bool {
    c >= 32u8 && c <= 126u8
}

/*fn keyword_record(
    i: &[u8],
) -> IResult<&[u8], Result<KeywordRecord, ParseKeywordError>, VerboseError<&[u8]>> {
    context(
        "keyword_record",
        tuple((keyword, value_indicator, value, comment)),
    )(i)
}*/

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_keyword() {
        assert_eq!(keyword(b"COMMENT "), Ok((&b""[..], Ok(Keyword::Comment))));
        assert_eq!(
            keyword(b"COMMENT-"),
            Ok((&b""[..], Err(ParseKeywordError::UnknownKeyword)))
        );
    }

    #[test]
    fn test_value_indicator() {
        assert_eq!(
            value_indicator(b"= "),
            Ok((&b""[..], ValueIndicator::EqualSpace))
        );
        assert_eq!(value_indicator(b"  "), Ok((&b""[..], ValueIndicator::None)));
        assert_ne!(
            value_indicator(b"xx"),
            Ok((&b""[..], ValueIndicator::EqualSpace))
        );
    }

    #[test]
    fn test_integer() {
        assert_eq!(integer(b"+300"), Ok((&b""[..], Value::Integer(300))));
        assert_eq!(integer(b"-300"), Ok((&b""[..], Value::Integer(-300))));
        assert_eq!(integer(b" 300"), Ok((&b""[..], Value::Integer(300))));
        assert_eq!(integer(b"300"), Ok((&b""[..], Value::Integer(300))));
        assert_ne!(integer(b"+500"), Ok((&b""[..], Value::Integer(300))));
    }

    #[test]
    fn test_real() {
        assert_eq!(real(b"+300.1"), Ok((&b""[..], Value::Real(300.1))));
        assert_eq!(real(b"-300.1"), Ok((&b""[..], Value::Real(-300.1))));
        assert_eq!(real(b" 300.1"), Ok((&b""[..], Value::Real(300.1))));
        assert_eq!(real(b"300.1"), Ok((&b""[..], Value::Real(300.1))));
        assert_ne!(real(b"+500.1"), Ok((&b""[..], Value::Real(300.1))));
    }

    #[test]
    fn test_complex_integer() {
        assert_eq!(
            complex_integer(b"( 123, 45)"),
            Ok((&b""[..], Value::ComplexInteger((123, 45))))
        );
        assert_eq!(
            complex_integer(b"(123, 45)"),
            Ok((&b""[..], Value::ComplexInteger((123, 45))))
        );
        assert_eq!(
            complex_integer(b"(-123,-45)"),
            Ok((&b""[..], Value::ComplexInteger((-123, -45))))
        );
        assert_eq!(
            complex_integer(b"(+123, +45)"),
            Ok((&b""[..], Value::ComplexInteger((123, 45))))
        );
        assert_ne!(
            complex_integer(b"(-500,-45)"),
            Ok((&b""[..], Value::ComplexInteger((-123, -45))))
        );
    }

    #[test]
    fn test_complex_float() {
        assert_eq!(
            complex_float(b"(123.23, -45.7)"),
            Ok((&b""[..], Value::ComplexFloat((123.23, -45.7))))
        );
        assert_eq!(
            complex_float(b"(+123.23, 45.7)"),
            Ok((&b""[..], Value::ComplexFloat((123.23, 45.7))))
        );
        assert_eq!(
            complex_float(b"(-123.23, +45.7)"),
            Ok((&b""[..], Value::ComplexFloat((-123.23, 45.7))))
        );
        assert_ne!(
            complex_float(b"(500.23, -45.7)"),
            Ok((&b""[..], Value::ComplexFloat((123.23, -45.7))))
        );
    }

    #[test]
    fn test_date() {
        assert_eq!(
            date(b"0000-01-01T00:00:00"),
            Ok((&b""[..], Value::Date("0000-01-01T00:00:00")))
        );
        assert_eq!(
            date(b"9999-12-31T23:59:59"),
            Ok((&b""[..], Value::Date("9999-12-31T23:59:59")))
        );
        assert_eq!(
            date(b"99999-01-01T00:00:00"),
            Ok((&b""[..], Value::Date("99999-01-01T00:00:00")))
        );
        assert_eq!(
            date(b"+99999-12-31T23:59:59"),
            Ok((&b""[..], Value::Date("+99999-12-31T23:59:59")))
        );
        assert_eq!(
            date(b"-04713-11-24T12:00:00"),
            Ok((&b""[..], Value::Date("-04713-11-24T12:00:00")))
        );
    }

    /*  #[test]
        fn test_keyword_record() {
            assert_eq!(
                keyword_record(
                    b"COMMENT     'This file is part of the EUVE Science Archive. It contains'        "
                ),
                Ok((
                    &b""[..],
                    (Ok(KeywordRecord::new(
                        Keyword::Comment,
                        Value::CharacterString(
                            "This file is part of the EUVE Science Archive. It contains"
                        ),
                        None
                    )))
                ))
            );

            assert_eq!(
                keyword_record(
                    b"SIMPLE  =                    T / FITS STANDARD                                  "
                ),
                Ok((
                    &b""[..],
                    (Ok(KeywordRecord::new(
                        Keyword::Simple,
                        Value::Logical(true),
                        Some("FITS STANDARD")
                    )))
                ))
            );
        }
    */
    /*
    "COMMENT     ' '                                                                 COMMENT     'This file is part of the EUVE Science Archive. It contains'        "
     */
}
