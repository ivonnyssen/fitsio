pub mod data_array;
pub mod header;
pub mod keyword;
pub mod keyword_record;
pub mod value;

use data_array::DataArray;
use header::Header;

#[derive(PartialEq, Debug)]
pub struct Fits<'a> {
    primary_hdu: HDU<'a>,
    extensions: Vec<HDU<'a>>,
}

impl<'a> Fits<'a> {
    pub fn new() -> Self {
        Self {
            primary_hdu: HDU::new(Header::new(), None),
            extensions: Vec::new(),
        }
    }

    pub fn primary_hdu(&self) -> &HDU {
        &self.primary_hdu
    }

    pub fn extensions(&self) -> &[HDU] {
        &self.extensions
    }
}

impl<'a> Default for Fits<'a> {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(PartialEq, Debug)]
pub struct HDU<'a> {
    header: Header<'a>,
    data: Option<DataArray>,
}

impl<'a> HDU<'a> {
    pub fn header(&self) -> &Header {
        &self.header
    }
    pub fn data_array(&self) -> &Option<DataArray> {
        &self.data
    }

    pub fn new(header: Header<'a>, data: Option<DataArray>) -> Self {
        Self { header, data }
    }
}
