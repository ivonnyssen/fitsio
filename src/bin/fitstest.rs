use fitsio::parse_fits;
use fitsio::types::data_array::DataArray;
use fitsio::types::FitsError;
use std::fs::File;
use std::io::BufWriter;
use std::{env, io::Read};
use tracing::{error, instrument, Level};
use tracing_subscriber::FmtSubscriber;

fn main() -> Result<(), FitsError> {
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::TRACE)
        .with_test_writer()
        .finish();

    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");

    let args: Vec<String> = env::args().collect();

    let (in_file, out_file) = open_files(args)?;

    //read the fits file
    let mut reader = std::io::BufReader::new(in_file);
    let mut buffer = [0u8; 4000];
    let mut bytes = Vec::new();
    loop {
        let bytes_read = reader.read(&mut buffer)?;
        if bytes_read == 0 {
            break;
        }
        bytes.extend_from_slice(&buffer[..bytes_read]);
    }

    let res = parse_fits(&bytes);
    match res {
        Ok(fits) => match fits.primary_hdu() {
            Some(hdu) => match hdu.data_array() {
                Some(data_array) => match data_array {
                    DataArray::U8(data, dimensions, _, _) => {
                        let bit_depth = png::BitDepth::Eight;
                        write_png(data, dimensions[0], dimensions[1], bit_depth, out_file);
                    }
                    DataArray::I16(data, dimensions, _, _) => {
                        let bit_depth = png::BitDepth::Eight;
                        let data = data.iter().map(|&x| (x * 50) as u8).collect::<Vec<u8>>();
                        write_png(&data, dimensions[0], dimensions[1], bit_depth, out_file);
                    }
                    DataArray::I32(_data, _, _, _) => todo!(),
                    DataArray::I64(_data, _, _, _) => todo!(),
                    DataArray::F32(data, dimensions, _, _) => {
                        let bit_depth = png::BitDepth::Sixteen;
                        let data = data
                            .iter()
                            .flat_map(|&x| ((x * 1000.0) as u16).to_be_bytes())
                            .collect::<Vec<u8>>();
                        write_png(&data, dimensions[0], dimensions[1], bit_depth, out_file);
                    }
                    DataArray::F64(_data, _, _, _) => todo!(),
                },
                None => {
                    println!("No data array found.");
                }
            },
            None => panic!("No primary hdu found"),
        },
        Err(e) => return Err(e),
    }

    //write the fits file
    //fits::write_fits(&file, &fits_file)
    //println!("{res:?}");

    Ok(())
}

#[instrument]
fn write_png(data: &[u8], width: u32, height: u32, bit_depth: png::BitDepth, file: File) -> () {
    let mut w = &mut BufWriter::new(file);
    //    let mut image_buffer = Vec::new();

    let mut encoder = png::Encoder::new(&mut w, width, height); // Width is 2 pixels and height is 1.
    encoder.set_color(png::ColorType::Grayscale);
    encoder.set_depth(bit_depth);
    let mut writer = encoder.write_header().unwrap(); // We must call this method first.

    // An array containing a RGBA sequence. First pixel is red and second pixel is black.
    writer.write_image_data(data).unwrap(); // Save

    writer.finish().unwrap();
}

#[instrument]
fn open_files(
    args: Vec<String>,
) -> std::result::Result<(std::fs::File, std::fs::File), std::io::Error> {
    let (in_file_path, out_file_path) = match args.len() > 2 {
        true => (&args[1], &args[2]),
        false => {
            error!("no filename provided",);
            println!("Usage: fitstest <file_to_read> <png_to_write>");
            std::process::exit(1);
        }
    };
    Ok((File::open(in_file_path)?, File::create(out_file_path)?))
}
