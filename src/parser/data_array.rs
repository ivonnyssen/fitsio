use nom::{
    error::{ParseError, VerboseError},
    IResult,
};

use crate::types::{header::Header, DataArray};

pub fn data_array<'a>(
    i: &'a [u8],
    header: &Header,
) -> IResult<&'a [u8], DataArray, VerboseError<&'a [u8]>> {
    match header.has_data_array() {
        true => {
            //let row =
            Ok((i, DataArray::new()))
        }
        false => Err(nom::Err::Error(ParseError::from_error_kind(
            i,
            nom::error::ErrorKind::Tag,
        ))),
    }
}
