pub mod parser;
pub mod types;

use std::io::Read;

pub fn parse_headers(file: &std::fs::File) -> Result<(), std::io::Error> {
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
    let (_, _) = parser::header(&header).unwrap();
    Ok(())
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_lib() {}
}
