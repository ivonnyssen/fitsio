use std::u8;

use nom::{error::VerboseError, IResult};

use crate::types::KeywordRecord;

mod header;
mod keyword_record;
mod value;

fn is_allowed_ascii(c: u8) -> bool {
    (32u8..=126u8).contains(&c)
}

pub fn hdu(i: &[u8]) -> IResult<&[u8], Vec<KeywordRecord>, VerboseError<&[u8]>> {
    let res = header::header(i);
    match res {
        Ok((i, _)) => image_extension(i),
        Err(_) => res,
    }
}

pub fn image_extension(i: &[u8]) -> IResult<&[u8], Vec<KeywordRecord>, VerboseError<&[u8]>> {
    header::header(i)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hdu() {}
}
