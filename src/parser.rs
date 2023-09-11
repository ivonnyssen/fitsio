use std::u8;

use nom::{error::context, error::VerboseError, multi::many0, IResult};

use crate::types::KeywordRecord;

mod keyword_record;
mod value;

fn is_allowed_ascii(c: u8) -> bool {
    (32u8..=126u8).contains(&c)
}

pub fn header(i: &[u8]) -> IResult<&[u8], Vec<KeywordRecord>, VerboseError<&[u8]>> {
    context("hdu", many0(keyword_record::keyword_record))(i)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::{keyword::Keyword, KeywordRecord, Value};

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
