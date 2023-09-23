use std::u8;

use nom::{
    error::{context, VerboseError},
    IResult,
};
use tracing::trace;

use crate::types::{header::FitsHeader, HDU};

mod data_array;
mod header;
mod keyword_record;
mod value;

fn is_allowed_ascii(c: u8) -> bool {
    (32u8..=126u8).contains(&c)
}

pub fn hdu(i: &[u8]) -> IResult<&[u8], HDU, VerboseError<&[u8]>> {
    match context("header", header::header)(i) {
        Ok((i, header)) => match header.has_data_array() {
            true => {
                let (i, data_array) = data_array::data_array(i, &header)?;
                let hdu = HDU::new(header, Some(data_array));
                trace!("{:?}", hdu);
                Ok((i, hdu))
            }
            false => {
                let hdu = HDU::new(header, None);
                trace!("{:?}", hdu);
                Ok((i, hdu))
            }
        },
        Err(e) => Err(e),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hdu() {}
}
