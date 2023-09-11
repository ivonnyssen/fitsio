use nom::{
    bytes::complete::{tag, take, take_while},
    character::complete::space0,
    combinator::{complete, map, map_parser, opt},
    error::{context, VerboseError},
    sequence::{pair, preceded},
    IResult,
};

use crate::parser::value::{
    character_string, complex_float, complex_integer, continued_string, date, integer, logical,
    real,
};

use crate::types::keyword::Keyword;
use crate::types::KeywordRecord;
use crate::types::Value;

fn keyword(i: &[u8]) -> IResult<&[u8], Keyword, VerboseError<&[u8]>> {
    context("keyword", map(complete(take(8u8)), Keyword::from))(i)
}

pub fn keyword_record(i: &[u8]) -> IResult<&[u8], KeywordRecord, VerboseError<&[u8]>> {
    let (i, key) = keyword(i)?;
    match key {
        Keyword::Author => value_and_comment(i, key, character_string),
        Keyword::BScale => value_and_comment(i, key, real),
        Keyword::BUnit => value_and_comment(i, key, character_string),
        Keyword::BZero => value_and_comment(i, key, real),
        Keyword::BitPix => value_and_comment(i, key, integer),
        Keyword::Blank => value_and_comment(i, key, integer),
        Keyword::CheckSum => value_and_comment(i, key, character_string),
        Keyword::Comment => value_and_comment(i, key, character_string),
        Keyword::Continue => value_and_comment(i, key, continued_string),
        Keyword::DataMax => value_and_comment(i, key, real),
        Keyword::DataMin => value_and_comment(i, key, real),
        Keyword::DataSum => value_and_comment(i, key, character_string),
        Keyword::Date => value_and_comment(i, key, date),
        Keyword::DateObs => value_and_comment(i, key, date),

        /*
        Blocked, // deprecated
        Empty,
        End,
        Epoch,
        Equinox,
        ExtLevel,
        ExtName,
        ExtVer,
        Extend,
        FZALGn(u16),
        FZAlgor,
        FZTileLn,
        GCount,
        Groups,
        History,
        Inherit,
        Instrume,
        NAxis,
        NAxisn(u16),
        Object,
        Obs,
        Observer,
        Origin,
        PCount,
        PScaln(u16),
        PTypen(u16),
        PZeron(u16),
        Referenc,
        Simple,
        TBcoln(u16),
        TDMaxn(u16),
        TDMinn(u16),
        TDimn(u16),
        TDispn(u16),
        TFormn(u16),
        THeap,
        TLMaxn(u16),
        TLMinn(u16),
        TNulln(u16),
        TScaln(u16),
        TTypen(u16),
        TUnitn(u16),
        TZeron(u16),
        Telescop,
        Tfields,
        Unknown([u8; 8]),
        Xtension,
        ZBitPix,
        ZBlocked,
        ZCTypn(u16),
        ZCmpType,
        ZDataSum,
        ZDither0,
        ZExtend,
        ZFormn(u16),
        ZGCount,
        ZImage,
        ZMaskCmp,
        ZNAMEi(u16),
        ZNaxis,
        ZNaxis1,
        ZNaxis2,
        ZPCount,
        ZQuantiz,
        ZSimple,
        ZTHeap,
        ZTable,
        ZTension,
        ZTileLen,
        ZTilen(u16),
        ZVALi(u16),
        ZheckSum,
             */
        Keyword::Simple => value_and_comment(i, key, logical),
        Keyword::NAxis => value_and_comment(i, key, integer),
        Keyword::Extend => value_and_comment(i, key, logical),
        Keyword::Origin => value_and_comment(i, key, character_string),
        Keyword::Telescop => value_and_comment(i, key, character_string),
        _ => map(take(72u8), |value: &[u8]| {
            KeywordRecord::new(
                key,
                Value::Unknown(std::str::from_utf8(value).unwrap_or("").to_string()),
                None,
            )
        })(i),
    }
}

fn comment(i: &[u8]) -> IResult<&[u8], &str, VerboseError<&[u8]>> {
    context(
        "value_comment",
        map(
            preceded(
                space0,
                preceded(tag("/"), take_while(super::is_allowed_ascii)),
            ),
            |s: &[u8]| std::str::from_utf8(s).unwrap().trim_end(),
        ),
    )(i)
}

type ValueParser = fn(i: &[u8]) -> IResult<&[u8], Value, VerboseError<&[u8]>>;

fn value_and_comment(
    i: &[u8],
    key: Keyword,
    parser: ValueParser,
) -> IResult<&[u8], KeywordRecord, VerboseError<&[u8]>> {
    map_parser(
        take(72u8),
        map(pair(parser, opt(comment)), |(value, comment)| {
            KeywordRecord::new(key, value, comment)
        }),
    )(i)
}

#[cfg(test)]
mod tests {
    use super::*;

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
    }

    #[test]
    fn test_value_and_comment() {
        assert_eq!(
            value_and_comment(
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
            value_and_comment(
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
}
