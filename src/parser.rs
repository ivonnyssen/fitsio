use std::u8;

use nom::{
    bytes::complete::{tag, take, take_while},
    character::complete::space0,
    combinator::{complete, map, map_parser, opt},
    error::context,
    error::VerboseError,
    multi::many0,
    sequence::{pair, preceded},
    IResult,
};

use crate::keywords::Keyword;
use crate::keywords::ValueType;
use crate::types::KeywordRecord;
use crate::types::Value;

mod value;

use crate::parser::value::{
    character_string, complex_float, complex_integer, continued_string, date, integer, logical,
    real,
};

fn is_allowed_ascii(c: u8) -> bool {
    (32u8..=126u8).contains(&c)
}

fn keyword(i: &[u8]) -> IResult<&[u8], Keyword, VerboseError<&[u8]>> {
    context("keyword", map(complete(take(8u8)), Keyword::from))(i)
}

fn value_comment(i: &[u8]) -> IResult<&[u8], &str, VerboseError<&[u8]>> {
    context(
        "value_comment",
        map(
            preceded(space0, preceded(tag("/"), take_while(is_allowed_ascii))),
            |s: &[u8]| std::str::from_utf8(s).unwrap().trim_end(),
        ),
    )(i)
}

fn keyword_record(i: &[u8]) -> IResult<&[u8], KeywordRecord, VerboseError<&[u8]>> {
    let (i, key) = keyword(i)?;
    match key {
        Keyword::Simple => parse(i, key, logical),
        Keyword::Comment => parse(i, key, character_string),
        Keyword::BitPix => parse(i, key, integer),
        _ => map(take(72u8), |value: &[u8]| {
            KeywordRecord::new(
                key,
                Value::Unknown(std::str::from_utf8(value).unwrap().to_string()),
                None,
            )
        })(i),
    }
}

fn parse(
    i: &[u8],
    key: Keyword,
    parser: fn(i: &[u8]) -> IResult<&[u8], Value, VerboseError<&[u8]>>,
) -> IResult<&[u8], KeywordRecord, VerboseError<&[u8]>> {
    map_parser(
        take(72u8),
        map(pair(parser, opt(value_comment)), |(value, comment)| {
            KeywordRecord::new(key, value, comment)
        }),
    )(i)
}

pub fn header(i: &[u8]) -> IResult<&[u8], Vec<KeywordRecord>, VerboseError<&[u8]>> {
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
    fn test_parse() {
        assert_eq!(
            parse(
                b"    'This file is part of the EUVE Science Archive. It contains'        ",
                Keyword::Comment,
                character_string
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
            parse(
                b"=                    T / FITS STANDARD                                  ",
                Keyword::Simple,
                logical
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

    #[test]
    fn test_header() {
        assert_eq!(
            header(
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
        assert_eq!(
            header(
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
