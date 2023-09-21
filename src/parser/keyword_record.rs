use nom::{
    branch::alt,
    bytes::complete::{tag, take, take_while},
    character::complete::space0,
    combinator::{all_consuming, complete, map, map_parser, opt, success},
    error::{context, VerboseError},
    sequence::{pair, preceded},
    IResult,
};

use tracing::trace;

use crate::parser::value::{
    character_string, complex_float, complex_integer, continued_string, date, integer, logical,
    real, unknown,
};

use crate::types::keyword::Keyword;
use crate::types::keyword_record::KeywordRecord;

fn keyword(i: &[u8]) -> IResult<&[u8], Keyword, VerboseError<&[u8]>> {
    context("keyword", map(complete(take(8u8)), Keyword::from))(i)
}

pub fn keyword_record(i: &[u8]) -> IResult<&[u8], KeywordRecord, VerboseError<&[u8]>> {
    map_parser(
        take(80u8),
        map(
            pair(
                keyword,
                alt((
                    all_consuming(pair(character_string, opt(comment))),
                    all_consuming(pair(complex_float, opt(comment))),
                    all_consuming(pair(complex_integer, opt(comment))),
                    all_consuming(pair(continued_string, opt(comment))),
                    all_consuming(pair(date, opt(comment))),
                    all_consuming(pair(integer, opt(comment))),
                    all_consuming(pair(logical, opt(comment))),
                    all_consuming(pair(real, opt(comment))),
                    all_consuming(pair(unknown, success(None))),
                )),
            ),
            |(key, (value, comment))| {
                let record = KeywordRecord::new(key, value, comment);
                trace!("keyword_record: {}", record);
                record
            },
        ),
    )(i)
}

fn comment(i: &[u8]) -> IResult<&[u8], &str, VerboseError<&[u8]>> {
    context(
        "comment",
        map(
            preceded(
                space0,
                preceded(tag("/"), take_while(super::is_allowed_ascii)),
            ),
            |s: &[u8]| std::str::from_utf8(s).unwrap().trim_end(),
        ),
    )(i)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::{keyword::Keyword, value::Value};

    #[test]
    fn test_keyword() {
        assert_eq!(keyword(b"COMMENT "), Ok((&b""[..], Keyword::Comment)));
        assert_eq!(
            keyword(b"COMMENT-"),
            Ok((
                &b""[..],
                Keyword::Unknown("COMMENT-".as_bytes().try_into().unwrap())
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
        assert_eq!(
            keyword_record(
                b"BSCALE  =                1.0E0 / Scale factor for pixel values                  "
            ),
            Ok((
                &b""[..],
                (KeywordRecord::new(
                    Keyword::BScale,
                    Value::Real(1.0),
                    Some(" Scale factor for pixel values")
                ))
            ))
        );
        assert_eq!(
            keyword_record(
                b"NAXIS1  =  512 / length of data axis 1                                          "
            ),
            Ok((
                &b""[..],
                (KeywordRecord::new(
                    Keyword::NAxisn(1),
                    Value::Integer(512),
                    Some(" length of data axis 1")
                ))
            ))
        );
        assert_eq!(
            keyword_record(
                b"END                                                                             "
            ),
            Ok((
                &b""[..],
                (KeywordRecord::new(Keyword::End, Value::CharacterString(String::from("")), None))
            ))
        );
    }

    #[test]
    fn test_keyword_record_display() {
        let record = KeywordRecord::new(
            Keyword::Simple,
            Value::Logical(true),
            Some(" FITS STANDARD"),
        );
        assert_eq!(format!("{}", record), "Simple = true /  FITS STANDARD");
        let record = KeywordRecord::new(
            Keyword::Unknown(*b"CREATOR "),
            Value::Unknown(String::from("'STWFITS '")),
            Some("Fitsio version 11-May-1995 "),
        );
        assert_eq!(
            format!("{}", record),
            "Unknown(CREATOR ) = 'STWFITS ' / Fitsio version 11-May-1995 "
        );
    }
}
