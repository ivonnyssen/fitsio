#![forbid(unsafe_code)]
pub mod parser;
pub mod types;

use std::io::{Read, Write};

use types::{Fits, FitsError};

pub fn parse_fits(bytes: &[u8]) -> Result<Fits, FitsError> {
    match parser::fits(bytes) {
        Ok((_, fits)) => Ok(fits),
        Err(e) => Err(FitsError::ParseError(e.to_string())),
    }
}

/// Writes a fits structure to a file.
/// returns the number of bytes written.
/// # Examples
///
/// ```
/// use fitsio::write_fits;
///
/// assert_eq!(write_fits(w, fits), );
/// ```
///
pub fn write_fits(_w: impl Write, _fits: &Fits) -> Result<u64, FitsError> {
    todo!()
}

//todo: add functions to create a fits structure from scratch and save it to a file
//todo: complete parsing of tables and bintables, so it does not crash
//todo: complete parsing of random groups
//todo: complete parsing of other, conforming extensions

pub fn parse_headers(file: &std::fs::File) -> Result<Vec<u8>, std::io::Error> {
    let mut reader = std::io::BufReader::new(file);
    let mut buffer = [0u8; 4000];
    let mut header = Vec::new();
    loop {
        let bytes_read = reader.read(&mut buffer)?;
        if bytes_read == 0 {
            break;
        }
        header.extend_from_slice(&buffer[..bytes_read]);
    }
    let (i, hdu) = parser::hdu(&header).unwrap();
    let (_, _) = parser::hdu(i).unwrap();
    match hdu.data_array() {
        Some(data_array) => match data_array {
            types::data_array::DataArray::U8(data, dimensions, _, _) => {
                let bit_depth = png::BitDepth::Eight;
                write_png(data, dimensions[0], dimensions[1], bit_depth)
            }
            types::data_array::DataArray::I16(data, dimensions, _, _) => {
                let bit_depth = png::BitDepth::Eight;
                let data = data.iter().map(|&x| (x * 50) as u8).collect::<Vec<u8>>();
                write_png(&data, dimensions[0], dimensions[1], bit_depth)
            }
            types::data_array::DataArray::I32(_data, _, _, _) => todo!(),
            types::data_array::DataArray::I64(_data, _, _, _) => todo!(),
            types::data_array::DataArray::F32(data, dimensions, _, _) => {
                let bit_depth = png::BitDepth::Sixteen;
                let data = data
                    .iter()
                    .flat_map(|&x| ((x * 1000.0) as u16).to_be_bytes())
                    .collect::<Vec<u8>>();
                write_png(&data, dimensions[0], dimensions[1], bit_depth)
            }
            types::data_array::DataArray::F64(_data, _, _, _) => todo!(),
        },
        None => {
            println!("No data array found.");
            Ok(Vec::new())
        }
    }
}

fn write_png(
    data: &[u8],
    width: u32,
    height: u32,
    bit_depth: png::BitDepth,
) -> Result<Vec<u8>, std::io::Error> {
    //let file = File::create("/home/parallels/projects/fitsio/FITS-EXAMPLES/test.png").unwrap();
    let mut image_buffer = Vec::new();
    //let w = &mut BufWriter::new(image_buffer);

    let mut encoder = png::Encoder::new(&mut image_buffer, width, height); // Width is 2 pixels and height is 1.
    encoder.set_color(png::ColorType::Grayscale);
    encoder.set_depth(bit_depth);
    let mut writer = encoder.write_header().unwrap(); // We must call this method first.

    // An array containing a RGBA sequence. First pixel is red and second pixel is black.
    writer.write_image_data(data).unwrap(); // Save

    writer.finish().unwrap();

    Ok(image_buffer)
}

#[cfg(test)]
mod tests {
    #[test]
    fn lib() {}
}
