use fitsio::parse_headers;
use std::env;
use std::fs::File;
use tracing::{error, instrument, Level};
use tracing_subscriber::FmtSubscriber;

fn main() -> Result<(), std::io::Error> {
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::TRACE)
        .with_test_writer()
        .finish();

    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");

    let args: Vec<String> = env::args().collect();

    let file = open_file(args)?;

    //read the fits file
    let _ = parse_headers(&file);
    //write the fits file
    //fits::write_fits(&file, &fits_file)
    //println!("{res:?}");

    Ok(())
}

#[instrument]
fn open_file(args: Vec<String>) -> std::result::Result<std::fs::File, std::io::Error> {
    let default_filename =
        String::from("/home/parallels/projects/fitsio/FITS-EXAMPLES/EUVEngc4151imgx.fits");
    let file_name = match args.len() > 1 {
        true => &args[1],
        false => {
            error!(
                "No filename provided. Using default filename: {}.",
                default_filename
            );
            println!("Usage: fitstest <filename>");
            &default_filename
        }
    };
    File::open(file_name)
}
