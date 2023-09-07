use std::env;
use std::fs::File;

use fitsio::parse_headers;

fn main() -> Result<(), std::io::Error> {
    let args: Vec<String> = env::args().collect();
    let mut file_name =
        &String::from("/home/parallels/projects/fitsio/FITS-EXAMPLES/EUVEngc4151imgx.fits");
    if args.len() < 2 {
        println!("Usage: fitstest <filename>");
    } else {
        file_name = &args[1];
    }

    let file = File::open(file_name)?;
    //read the fits file
    let res = parse_headers(&file);
    //write the fits file
    //fits::write_fits(&file, &fits_file)
    println!("{res:?}");
    Ok(())
}
