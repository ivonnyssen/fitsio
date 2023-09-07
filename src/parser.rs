use std::u8;

use nom::{
    bytes::complete::{tag, take, take_while},
    character::complete::space0,
    combinator::{complete, map, opt},
    error::context,
    error::VerboseError,
    multi::many0,
    sequence::{pair, preceded},
    IResult, Parser,
};

use crate::keywords::Keyword;
use crate::keywords::ValueType;
use crate::types::KeywordRecord;

mod character_string;
mod complex_float;
mod complex_integer;
mod continued_string;
mod date;
mod integer;
mod logical;
mod real;

use crate::parser::{
    character_string::character_string, complex_float::complex_float,
    complex_integer::complex_integer, continued_string::continued_string, date::date,
    integer::integer, logical::logical, real::real,
};

fn is_allowed_ascii(c: u8) -> bool {
    (32u8..=126u8).contains(&c)
}

fn keyword(i: &[u8]) -> IResult<&[u8], Keyword, VerboseError<&[u8]>> {
    context("keyword", map(complete(take(8u8)), Keyword::from))(i)
}

fn value_comment(i: &[u8]) -> IResult<&[u8], &[u8], VerboseError<&[u8]>> {
    context(
        "value_comment",
        map(
            preceded(space0, preceded(tag("/"), take_while(is_allowed_ascii))),
            |s: &[u8]| std::str::from_utf8(s).unwrap().trim_end().as_bytes(),
        ),
    )(i)
}

fn keyword_record(i: &[u8]) -> IResult<&[u8], KeywordRecord, VerboseError<&[u8]>> {
    let (i, key) = keyword(i)?;
    let (_, remainder) = take(72u8).parse(i)?;
    match key {
        Keyword::Simple => parse_value_and_comment(key, remainder, ValueType::Logical),
        Keyword::Comment => parse_value_and_comment(key, remainder, ValueType::CharacterString),
        _ => parse_value_and_comment(key, remainder, ValueType::Unknown),
    }
}

fn parse_value_and_comment(
    key: Keyword,
    remainder: &[u8],
    value_type: ValueType,
) -> Result<(&[u8], KeywordRecord<'_>), nom::Err<VerboseError<&[u8]>>> {
    match value_type {
        ValueType::CharacterString => map(
            pair(character_string, opt(value_comment)),
            |(value, comment)| {
                KeywordRecord::new(key, value, comment.map(|s| std::str::from_utf8(s).unwrap()))
            },
        )(remainder),
        ValueType::ComplexFloat => map(
            pair(complex_float, opt(value_comment)),
            |(value, comment)| {
                KeywordRecord::new(key, value, comment.map(|s| std::str::from_utf8(s).unwrap()))
            },
        )(remainder),
        ValueType::ComplexInteger => map(
            pair(complex_integer, opt(value_comment)),
            |(value, comment)| {
                KeywordRecord::new(key, value, comment.map(|s| std::str::from_utf8(s).unwrap()))
            },
        )(remainder),
        ValueType::Date => map(pair(date, opt(value_comment)), |(value, comment)| {
            KeywordRecord::new(key, value, comment.map(|s| std::str::from_utf8(s).unwrap()))
        })(remainder),
        ValueType::Integer => map(pair(integer, opt(value_comment)), |(value, comment)| {
            KeywordRecord::new(key, value, comment.map(|s| std::str::from_utf8(s).unwrap()))
        })(remainder),
        ValueType::Logical => map(pair(logical, opt(value_comment)), |(value, comment)| {
            KeywordRecord::new(key, value, comment.map(|s| std::str::from_utf8(s).unwrap()))
        })(remainder),
        ValueType::Real => map(pair(real, opt(value_comment)), |(value, comment)| {
            KeywordRecord::new(key, value, comment.map(|s| std::str::from_utf8(s).unwrap()))
        })(remainder),
        ValueType::Unknown => todo!(),
        ValueType::ContinuedString => map(
            pair(continued_string, opt(value_comment)),
            |(value, comment)| {
                KeywordRecord::new(key, value, comment.map(|s| std::str::from_utf8(s).unwrap()))
            },
        )(remainder),
    }
}

fn hdu(i: &[u8]) -> IResult<&[u8], Vec<KeywordRecord>, VerboseError<&[u8]>> {
    context("hdu", many0(keyword_record))(i)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::Value;

    #[test]
    fn test_keyword() {
        assert_eq!(keyword(b"COMMENT "), Ok((&b""[..], Keyword::Comment)));
        assert_eq!(keyword(b"COMMENT-"), Ok((&b""[..], Keyword::Unknown)));
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

    fn test_hdu() {
        assert_eq!(
            hdu(
                b"SIMPLE  =                    T / FITS STANDARD                                  COMMENT     'This file is part of the EUVE Science Archive. It contains'        "
            ),
            Ok((
                &b""[..],
                vec![KeywordRecord::new(
                    Keyword::Simple,
                    Value::Logical(true),
                    Some(" FITS STANDARD")
                ),KeywordRecord::new(
                    Keyword::Comment,
                    Value::CharacterString("This file is part of the EUVE Science Archive. It contains".to_string()),
                    None
                )]
            ))
        );
    }
}
