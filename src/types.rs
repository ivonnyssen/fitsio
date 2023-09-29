pub mod data_array;
pub mod header;
pub mod keyword;
pub mod keyword_record;
pub mod value;

use data_array::DataArray;
use header::Header;
use nom::error;
use thiserror::Error;

use self::header::{FitsHeader, HeaderKind};

#[derive(PartialEq, Debug)]
pub struct Fits<'a> {
    hdus: Vec<HDU<'a>>,
}

impl<'a> Fits<'a> {
    pub fn new() -> Self {
        Self { hdus: Vec::new() }
    }

    pub fn primary_hdu(&self) -> Option<&HDU> {
        self.hdus
            .iter()
            .find(|hdu| *hdu.header().header_kind() == HeaderKind::Primary)
    }

    pub fn extensions(&self) -> Vec<&HDU> {
        self.hdus
            .iter()
            .filter(|hdu| *hdu.header().header_kind() != HeaderKind::Primary)
            .collect()
    }

    pub fn from(hdus: Vec<HDU<'a>>) -> Self {
        Self { hdus }
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
#[derive(Error, Debug, PartialEq)]
#[error("could not pars fits file")]
pub enum FitsError {
    ParseError(String),
}

impl From<error::VerboseError<&[u8]>> for FitsError {
    fn from(_: error::VerboseError<&[u8]>) -> Self {
        Self::ParseError("".to_string())
    }
}

impl From<std::io::Error> for FitsError {
    fn from(e: std::io::Error) -> Self {
        Self::ParseError(e.to_string())
    }
}
