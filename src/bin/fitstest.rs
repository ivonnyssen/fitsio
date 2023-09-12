use fitsio::parse_headers;
use std::env;
use std::fs::File;
use tracing::{error, Level};
use tracing_subscriber::FmtSubscriber;

fn main() -> Result<(), std::io::Error> {
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::TRACE)
        .with_test_writer()
        .finish();

    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");

    // Traced app logic here...
    let default_filename =
        String::from("/home/parallels/projects/fitsio/FITS-EXAMPLES/EUVEngc4151imgx.fits");
    let args: Vec<String> = env::args().collect();
    let file_name = match args.len() > 1 {
        true => &args[1],
        false => {
            error!("Usage: fitstest <filename>");
            println!("Usage: fitstest <filename>");
            &default_filename
        }
    };

    let file = File::open(file_name)?;

    //read the fits file
    let _ = parse_headers(&file);
    //write the fits file
    //fits::write_fits(&file, &fits_file)
    //println!("{res:?}");

    Ok(())
}
