use std::u8;

use nom::{
    error::{context, VerboseError},
    multi::many0,
    IResult,
};
use tracing::trace;

use crate::types::{header::FitsHeader, Fits, HDU};

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

pub fn fits(i: &[u8]) -> IResult<&[u8], Fits, VerboseError<&[u8]>> {
    match context("fits", many0(hdu))(i) {
        Ok((i, hdus)) => Ok((i, Fits::from(hdus))),
        Err(e) => Err(e),
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn hdu() {}
}
