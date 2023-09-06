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

mod character_string;
mod complex_float;
mod complex_integer;
mod date;
mod integer;
mod logical;
mod real;

use crate::parser::{
    character_string::character_string, complex_float::complex_float,
    complex_integer::complex_integer, date::date, integer::integer, logical::logical, real::real,
};

fn keyword(i: &[u8]) -> IResult<&[u8], Keyword, VerboseError<&[u8]>> {
    context("keyword", map(complete(take(8u8)), Keyword::from))(i)
}

fn value(i: &[u8]) -> IResult<&[u8], (Value, Option<&[u8]>), VerboseError<&[u8]>> {
    context(
        "value",
        take(72u8).and_then(preceded(
            alt((tag("= "), tag("  "))),
            alt((
                all_consuming(pair(character_string, opt(value_comment))),
                //all_consuming(continued_string),
                all_consuming(pair(logical, opt(value_comment))),
                all_consuming(pair(integer, opt(value_comment))),
                all_consuming(pair(real, opt(value_comment))),
                all_consuming(pair(complex_integer, opt(value_comment))),
                all_consuming(pair(complex_float, opt(value_comment))),
                all_consuming(pair(date, opt(value_comment))),
            )),
        )),
    )(i)
}

fn continued_string(i: &[u8]) -> IResult<&[u8], Value, VerboseError<&[u8]>> {
    todo!()
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
