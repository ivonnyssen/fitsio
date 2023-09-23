use nom::{
    error::{ParseError, VerboseError},
    multi::count,
    number, IResult,
};

use crate::types::{header::FitsHeader, DataArray};

pub fn data_array<'a>(
    i: &'a [u8],
    header: &impl FitsHeader,
) -> IResult<&'a [u8], DataArray, VerboseError<&'a [u8]>> {
    if !header.has_data_array() | header.bitpix().is_none() {
        return Err(nom::Err::Error(ParseError::from_error_kind(
            i,
            nom::error::ErrorKind::Tag,
        )));
    }
    let number_of_elements = header.dimensions().iter().product::<u32>() as usize;
    match header.bitpix().unwrap() {
        8 => {
            let (i, data_array) = count(number::complete::u8, number_of_elements)(i)?;
            Ok((i, DataArray::from_u8(data_array)))
        }
        16 => {
            let (i, data_array) = count(number::complete::be_i16, number_of_elements)(i)?;
            Ok((i, DataArray::from_i16(data_array)))
        }
        32 => {
            let (i, data_array) = count(number::complete::be_i32, number_of_elements)(i)?;
            Ok((i, DataArray::from_i32(data_array)))
        }
        64 => {
            let (i, data_array) = count(number::complete::be_i64, number_of_elements)(i)?;
            Ok((i, DataArray::from_i64(data_array)))
        }
        -32 => {
            let (i, data_array) = count(number::complete::be_f32, number_of_elements)(i)?;
            Ok((i, DataArray::from_f32(data_array)))
        }
        -64 => {
            let (i, data_array) = count(number::complete::be_f64, number_of_elements)(i)?;
            Ok((i, DataArray::from_f64(data_array)))
        }
        _ => Err(nom::Err::Error(ParseError::from_error_kind(
            i,
            nom::error::ErrorKind::Tag,
        ))),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::header::MockFitsHeader;

    #[test]
    fn data_array() {
        let mut mock_header = MockFitsHeader::new();
        mock_header
            .expect_has_data_array()
            .times(1)
            .return_const(true);
        mock_header.expect_bitpix().times(2).return_const(Some(8));
        mock_header
            .expect_dimensions()
            .times(1)
            .return_const(vec![2, 2]);
        let data = vec![1u8, 2u8, 3u8, 4u8];
        let array = DataArray::from_u8(data.clone());
        let (i, result) = super::data_array(&data, &mock_header).unwrap();
        assert_eq!(i, &[]);
        assert_eq!(result, array);
    }
}
