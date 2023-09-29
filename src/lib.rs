#![forbid(unsafe_code)]
pub mod parser;
pub mod types;

use std::io::Write;

use types::{Fits, FitsError};
/// Reads a fits structure from a byte stream.
/// returns the fits structure or an error.
///
/// # Examples
///
/// ```
/// use fitsio::parse_fits;
/// use fitsio::types::Fits;
/// let mut bytes = b"SIMPLE  =                    T / file does conform to FITS standard".to_vec();
///
/// let res = parse_fits(&bytes);
/// assert!(res.is_ok());
///
/// ```
pub fn parse_fits(bytes: &[u8]) -> Result<Fits, FitsError> {
    // Todo: check if perhaps using Arc instead of the slice is better?
    // Todo: check that we are not copying shit around too much, and perhaps use Arc<str> in a few places
    // Todo: inspect all the structs so they appropriately hold references instead of copies
    // Toto: write documentation and design the library innterface, which types to re-export
    match parser::fits(bytes) {
        Ok((_, fits)) => Ok(fits),
        Err(e) => Err(FitsError::ParseError(e.to_string())),
    }
}

/// Writes a fits structure to a file.
/// returns the number of bytes written.
///
pub fn write_fits(_w: impl Write, _fits: &Fits) -> Result<u64, FitsError> {
    todo!()
}

//todo: add functions to create a fits structure from scratch and save it to a file
//todo: complete parsing of tables and bintables, so it does not crash
//todo: complete parsing of random groups
//todo: complete parsing of other, conforming extensions

#[cfg(test)]
mod tests {
    #[test]
    fn lib() {}
}
