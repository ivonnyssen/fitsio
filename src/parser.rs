use std::u8;

use nom::{
    branch::alt,
    bytes::complete::{tag, take, take_while},
    character::complete::{i64, space0},
    combinator::{map, opt},
    error::context,
    error::VerboseError,
    multi::many0,
    number::complete::double,
    sequence::{preceded, separated_pair, terminated, tuple},
    IResult,
};

use crate::keywords::{Keyword, ValueIndicator};
use crate::types::{KeywordRecord, Value};

fn keyword(i: &[u8]) -> IResult<&[u8], Keyword, VerboseError<&[u8]>> {
    context("keyword", take(8u8))(i).map(|(i, res)| (i, res.into()))
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
            many0(preceded(tag(b"'"), terminated(no_single_quote, tag(b"'")))),
            |parts: Vec<&[u8]>| Value::CharacterString(u8vec_to_str(parts)),
        ),
    )(i)
}

fn u8vec_to_str(v: Vec<&[u8]>) -> String {
    let mut it = v.iter().peekable();
    let mut acc = String::new();
    while let Some(part) = it.next() {
        acc.push_str(std::str::from_utf8(part).unwrap());
        match it.peek().is_some() {
            true => acc.push('\''),
            false => (),
        }
    }
    acc
}

fn no_single_quote(i: &[u8]) -> IResult<&[u8], &[u8], VerboseError<&[u8]>> {
    context("no_single_quote", take_while(|c| c != b'\''))(i)
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
            Value::Date(std::str::from_utf8(s).unwrap().to_string())
        }),
    )(i)
}

fn comment(i: &[u8]) -> IResult<&[u8], &[u8], VerboseError<&[u8]>> {
    context(
        "keyword comment",
        map(
            preceded(tag("/"), take_while(is_ascii_text_char)),
            |s: &[u8]| s,
        ),
    )(i)
}

fn is_ascii_text_char(c: u8) -> bool {
    (32u8..=126u8).contains(&c)
}

fn keyword_record(i: &[u8]) -> IResult<&[u8], KeywordRecord, VerboseError<&[u8]>> {
    context(
        "keyword_record",
        map(
            tuple((
                keyword,
                value_indicator,
                value,
                opt(preceded(space0, comment)),
            )),
            |(keyword, value_indicator, value, comment)| {
                KeywordRecord::new(
                    keyword,
                    value,
                    comment.map(|s| std::str::from_utf8(s).unwrap()),
                )
            },
        ),
    )(i)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_keyword() {
        assert_eq!(keyword(b"COMMENT "), Ok((&b""[..], Keyword::Comment)));
        assert_eq!(keyword(b"COMMENT-"), Ok((&b""[..], Keyword::Unknown)));
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
    fn test_character_string() {
        assert_eq!(
            character_string(b"'This file is part of the EUVE Science Archive. It contains'"),
            Ok((
                &b""[..],
                Value::CharacterString(
                    "This file is part of the EUVE Science Archive. It contains".to_string()
                )
            ))
        );
        assert_eq!(
            character_string(b"'String with single quote '' 123.45 , _ + - '"),
            Ok((
                &b""[..],
                Value::CharacterString("String with single quote ' 123.45 , _ + - ".to_string())
            ))
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
            Ok((&b""[..], Value::Date("0000-01-01T00:00:00".to_string())))
        );
        assert_eq!(
            date(b"9999-12-31T23:59:59"),
            Ok((&b""[..], Value::Date("9999-12-31T23:59:59".to_string())))
        );
        assert_eq!(
            date(b"99999-01-01T00:00:00"),
            Ok((&b""[..], Value::Date("99999-01-01T00:00:00".to_string())))
        );
        assert_eq!(
            date(b"+99999-12-31T23:59:59"),
            Ok((&b""[..], Value::Date("+99999-12-31T23:59:59".to_string())))
        );
        assert_eq!(
            date(b"-04713-11-24T12:00:00"),
            Ok((&b""[..], Value::Date("-04713-11-24T12:00:00".to_string())))
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
