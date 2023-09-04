use super::keywords;

#[derive(PartialEq, Debug)]
pub struct Fits<'a> {
    primary_hdu: HDU<'a>,
    extensions: Vec<HDU<'a>>,
}

impl<'a> Fits<'a> {
    pub fn new() -> Self {
        Self {
            primary_hdu: HDU {
                header: Header {
                    keyword_records: Vec::new(),
                },
                data_array: None,
            },
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
    data_array: Option<DataArray>,
}

impl<'a> HDU<'a> {
    pub fn header(&self) -> &Header {
        &self.header
    }
    pub fn data_array(&self) -> &Option<DataArray> {
        &self.data_array
    }
}

#[derive(PartialEq, Debug)]
pub struct DataArray;

/// The primary header of a FITS file.
#[derive(PartialEq, Debug)]
pub struct Header<'a> {
    keyword_records: Vec<KeywordRecord<'a>>,
}

impl<'a> Header<'a> {
    pub fn keyword_records(&self) -> &[KeywordRecord] {
        &self.keyword_records
    }
}

/// A keyword record contains information about a FITS header. It consists of a
/// keyword, the corresponding value and an optional comment.
#[derive(PartialEq, Debug)]
pub struct KeywordRecord<'a> {
    keyword: keywords::Keyword,
    value: Value<'a>,
    comment: Option<&'a str>,
}

impl<'a> KeywordRecord<'a> {
    pub fn new(keyword: keywords::Keyword, value: Value<'a>, comment: Option<&'a str>) -> Self {
        Self {
            keyword,
            value,
            comment,
        }
    }

    pub fn keyword(&self) -> &keywords::Keyword {
        &self.keyword
    }

    pub fn value(&self) -> &Value {
        &self.value
    }

    pub fn comment(&self) -> &Option<&str> {
        &self.comment
    }
}

/// The possible values of a KeywordRecord.
#[derive(PartialEq, Debug)]
pub enum Value<'a> {
    /// An ASCII string 0x20 - 0x7E. FITS standard section 4.2.1.1
    CharacterString(&'a str),
    /// A multi-part string. FITS standard section 4.2.1.2
    ContinuedString(Vec<&'a str>),
    /// A logical constant or value `F` or `T`. FITS standard section 4.2.2
    Logical(bool),
    /// An signed integer. FITS standard section 4.2.3
    Integer(i64),
    /// Fixed format real floating point number. FITS standard section 4.2.4
    Real(f64),
    /// Complex integer with real and imaginary parts. FITS standard section 4.2.5
    ComplexInteger((i64, i64)),
    /// Complex floating point number. FITS standard section 4.2.6
    ComplexFloat((f64, f64)),
    /// Date. ISO-8601 string. FITS standard section 4.2.7
    Date(&'a str),
}
