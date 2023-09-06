use std::u8;

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

use crate::keywords::Keyword;
use crate::types::{KeywordRecord, Value};

fn keyword(i: &[u8]) -> IResult<&[u8], Keyword, VerboseError<&[u8]>> {
    context("keyword", map(complete(take(8u8)), Keyword::from))(i)
}

//todo: parse the right value for each keyword
fn value(i: &[u8]) -> IResult<&[u8], (Value, Option<&[u8]>), VerboseError<&[u8]>> {
    context(
        "value",
        take(72u8).and_then(alt((
            all_consuming(pair(character_string, opt(value_comment))),
            //all_consuming(continued_string),
            all_consuming(pair(logical, opt(value_comment))),
            all_consuming(pair(integer, opt(value_comment))),
            all_consuming(pair(real, opt(value_comment))),
            all_consuming(pair(complex_integer, opt(value_comment))),
            all_consuming(pair(complex_float, opt(value_comment))),
            all_consuming(pair(date, opt(value_comment))),
        ))),
    )(i)
}

fn character_string(i: &[u8]) -> IResult<&[u8], Value, VerboseError<&[u8]>> {
    context(
        "character_string",
        map(
            preceded(
                tag(b"= "),
                preceded(
                    space0,
                    many0(preceded(tag(b"'"), terminated(no_single_quote, tag(b"'")))),
                ),
            ),
            |parts: Vec<&[u8]>| Value::CharacterString(u8vec_to_string(parts)),
        ),
    )(i)
}

fn u8vec_to_string(v: Vec<&[u8]>) -> String {
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
        map(
            preceded(
                tag("= "),
                preceded(space0, terminated(alt((tag("T"), tag("F"))), space0)),
            ),
            |s: &[u8]| Value::Logical(s == b"T"),
        ),
    )(i)
}

fn integer(i: &[u8]) -> IResult<&[u8], Value, VerboseError<&[u8]>> {
    context(
        "integer",
        map(preceded(tag("= "), preceded(space0, i64)), Value::Integer),
    )(i)
}

fn real(i: &[u8]) -> IResult<&[u8], Value, VerboseError<&[u8]>> {
    context(
        "real",
        map(preceded(tag("= "), preceded(space0, double)), Value::Real),
    )(i)
}

fn complex_integer(i: &[u8]) -> IResult<&[u8], Value, VerboseError<&[u8]>> {
    context(
        "complex integer",
        map(
            preceded(
                tag("= "),
                preceded(
                    space0,
                    separated_pair(
                        preceded(tag("("), preceded(space0, i64)),
                        tag(","),
                        preceded(space0, terminated(i64, tag(")"))),
                    ),
                ),
            ),
            Value::ComplexInteger,
        ),
    )(i)
}

fn complex_float(i: &[u8]) -> IResult<&[u8], Value, VerboseError<&[u8]>> {
    context(
        "complex integer",
        map(
            preceded(
                tag("= "),
                preceded(
                    space0,
                    separated_pair(
                        preceded(tag("("), double),
                        tag(","),
                        preceded(space0, terminated(double, tag(")"))),
                    ),
                ),
            ),
            Value::ComplexFloat,
        ),
    )(i)
}

fn date(i: &[u8]) -> IResult<&[u8], Value, VerboseError<&[u8]>> {
    //CCYY-MM-DD[Thh:mm:ss[.s...]]
    context(
        "date",
        map(
            preceded(tag("= "), preceded(space0, take_while(is_ascii_text_char))),
            |s: &[u8]| Value::Date(std::str::from_utf8(s).unwrap().trim_end().to_string()),
        ),
    )(i)
}

fn value_comment(i: &[u8]) -> IResult<&[u8], &[u8], VerboseError<&[u8]>> {
    context(
        "value_comment",
        map(
            preceded(space0, preceded(tag("/"), take_while(is_ascii_text_char))),
            |s: &[u8]| std::str::from_utf8(s).unwrap().trim_end().as_bytes(),
        ),
    )(i)
}

fn is_ascii_text_char(c: u8) -> bool {
    (32u8..=126u8).contains(&c)
}

fn keyword_record(i: &[u8]) -> IResult<&[u8], KeywordRecord, VerboseError<&[u8]>> {
    context(
        "keyword_record",
        map(tuple((keyword, value)), |(keyword, value)| {
            KeywordRecord::new(
                keyword,
                value.0,
                value.1.map(|s| std::str::from_utf8(s).unwrap()),
            )
        }),
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
    fn test_value_with_optional_comment() {
        assert_eq!(
            value(b"=                    T / FITS STANDARD                                  "),
            Ok((
                &b""[..],
                (Value::Logical(true), Some(&b" FITS STANDARD"[..]))
            ))
        );
        assert_eq!(
            value(b"=                    T                                                  "),
            Ok((&b""[..], (Value::Logical(true), None)))
        );
    }

    #[test]
    fn test_character_string() {
        assert_eq!(
            character_string(b"=   'This file is part of the EUVE Science Archive. It contains'"),
            Ok((
                &b""[..],
                Value::CharacterString(
                    "This file is part of the EUVE Science Archive. It contains".to_string()
                )
            ))
        );
        assert_eq!(
            character_string(b"= 'String with single quote '' 123.45 , _ + - '"),
            Ok((
                &b""[..],
                Value::CharacterString("String with single quote ' 123.45 , _ + - ".to_string())
            ))
        );
    }

    #[test]
    fn test_logical() {
        assert_eq!(
            logical(b"=                    T"),
            Ok((&b""[..], Value::Logical(true)))
        );
        assert_eq!(
            logical(b"=                    F"),
            Ok((&b""[..], Value::Logical(false)))
        );
        assert_eq!(logical(b"= T"), Ok((&b""[..], Value::Logical(true))));
        assert_eq!(logical(b"=  F"), Ok((&b""[..], Value::Logical(false))));
        assert_ne!(logical(b"= T"), Ok((&b""[..], Value::Logical(false))));
    }

    #[test]
    fn test_integer() {
        assert_eq!(integer(b"= +300"), Ok((&b""[..], Value::Integer(300))));
        assert_eq!(integer(b"=   -300"), Ok((&b""[..], Value::Integer(-300))));
        assert_eq!(integer(b"=  300"), Ok((&b""[..], Value::Integer(300))));
        assert_eq!(integer(b"= 300"), Ok((&b""[..], Value::Integer(300))));
        assert_ne!(integer(b"= +500"), Ok((&b""[..], Value::Integer(300))));
    }

    #[test]
    fn test_real() {
        assert_eq!(real(b"= +300.1"), Ok((&b""[..], Value::Real(300.1))));
        assert_eq!(real(b"= -300.1"), Ok((&b""[..], Value::Real(-300.1))));
        assert_eq!(real(b"=  300.1"), Ok((&b""[..], Value::Real(300.1))));
        assert_eq!(real(b"= 300.1"), Ok((&b""[..], Value::Real(300.1))));
        assert_ne!(real(b"= +500.1"), Ok((&b""[..], Value::Real(300.1))));
    }

    #[test]
    fn test_complex_integer() {
        assert_eq!(
            complex_integer(b"= ( 123, 45)"),
            Ok((&b""[..], Value::ComplexInteger((123, 45))))
        );
        assert_eq!(
            complex_integer(b"=   (123, 45)"),
            Ok((&b""[..], Value::ComplexInteger((123, 45))))
        );
        assert_eq!(
            complex_integer(b"= (-123,-45)"),
            Ok((&b""[..], Value::ComplexInteger((-123, -45))))
        );
        assert_eq!(
            complex_integer(b"= (+123, +45)"),
            Ok((&b""[..], Value::ComplexInteger((123, 45))))
        );
        assert_ne!(
            complex_integer(b"= (-500,-45)"),
            Ok((&b""[..], Value::ComplexInteger((-123, -45))))
        );
    }

    #[test]
    fn test_complex_float() {
        assert_eq!(
            complex_float(b"= (123.23, -45.7)"),
            Ok((&b""[..], Value::ComplexFloat((123.23, -45.7))))
        );
        assert_eq!(
            complex_float(b"=  (+123.23, 45.7)"),
            Ok((&b""[..], Value::ComplexFloat((123.23, 45.7))))
        );
        assert_eq!(
            complex_float(b"= (-123.23, +45.7)"),
            Ok((&b""[..], Value::ComplexFloat((-123.23, 45.7))))
        );
        assert_ne!(
            complex_float(b"= (500.23, -45.7)"),
            Ok((&b""[..], Value::ComplexFloat((123.23, -45.7))))
        );
    }

    #[test]
    fn test_date() {
        assert_eq!(
            date(b"= 0000-01-01T00:00:00"),
            Ok((&b""[..], Value::Date("0000-01-01T00:00:00".to_string())))
        );
        assert_eq!(
            date(b"=   9999-12-31T23:59:59"),
            Ok((&b""[..], Value::Date("9999-12-31T23:59:59".to_string())))
        );
        assert_eq!(
            date(b"= 99999-01-01T00:00:00"),
            Ok((&b""[..], Value::Date("99999-01-01T00:00:00".to_string())))
        );
        assert_eq!(
            date(b"= +99999-12-31T23:59:59"),
            Ok((&b""[..], Value::Date("+99999-12-31T23:59:59".to_string())))
        );
        assert_eq!(
            date(b"= -04713-11-24T12:00:00"),
            Ok((&b""[..], Value::Date("-04713-11-24T12:00:00".to_string())))
        );
    }

    #[test]
    fn test_keyword_record() {
        assert_eq!(
            keyword_record(
                b"COMMENT     'This file is part of the EUVE Science Archive. It contains'        "
            ),
            Ok((
                &b""[..],
                (KeywordRecord::new(
                    Keyword::Comment,
                    Value::CharacterString(
                        "This file is part of the EUVE Science Archive. It contains".to_string()
                    ),
                    None
                ))
            ))
        );

        assert_eq!(
            keyword_record(
                b"SIMPLE  =                    T / FITS STANDARD                                  "
            ),
            Ok((
                &b""[..],
                (KeywordRecord::new(
                    Keyword::Simple,
                    Value::Logical(true),
                    Some(" FITS STANDARD")
                ))
            ))
        );
    }

    /*
    "COMMENT     ' '                                                                 COMMENT     'This file is part of the EUVE Science Archive. It contains'        "
     */
}
