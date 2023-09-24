#[cfg(test)]
use mockall::{automock, predicate::*};

use super::{keyword::Keyword, keyword_record::KeywordRecord, value::Value};

#[derive(PartialEq, Debug, Default)]
pub enum HeaderKind {
    #[default]
    Primary,
    Image,
    Table,
    BinaryTable,
    Other(String),
}

#[cfg_attr(test, automock)]
pub trait FitsHeader {
    fn has_data_array(&self) -> bool;
    fn len(&self) -> usize;
    fn is_empty(&self) -> bool;
    fn header_kind(&self) -> &HeaderKind;
    fn naxis(&self) -> u16;
    fn bitpix(&self) -> Option<i8>;
    fn dimensions(&self) -> &Vec<u32>;
    fn bzero(&self) -> Option<f32>;
    fn bscale(&self) -> Option<f32>;
    fn pcount(&self) -> Option<u32>;
    fn gcount(&self) -> Option<u32>;
}

#[derive(PartialEq, Debug, Default)]
pub struct Header<'a> {
    keyword_records: Vec<KeywordRecord<'a>>,
    has_data_array: bool,
    header_kind: HeaderKind,
    naxis: u16,
    bitpix: Option<i8>,
    dimensions: Vec<u32>,
    bzero: Option<f32>,
    bscale: Option<f32>,
    pcount: Option<u32>,
    gcount: Option<u32>,
}

impl<'a> FitsHeader for Header<'a> {
    fn has_data_array(&self) -> bool {
        self.has_data_array
    }

    fn len(&self) -> usize {
        self.keyword_records.len()
    }

    fn is_empty(&self) -> bool {
        self.keyword_records.is_empty()
    }

    fn header_kind(&self) -> &HeaderKind {
        &self.header_kind
    }

    fn naxis(&self) -> u16 {
        self.naxis
    }

    fn bitpix(&self) -> Option<i8> {
        self.bitpix
    }

    fn dimensions(&self) -> &Vec<u32> {
        self.dimensions.as_ref()
    }

    fn bzero(&self) -> Option<f32> {
        self.bzero
    }

    fn bscale(&self) -> Option<f32> {
        self.bscale
    }

    fn pcount(&self) -> Option<u32> {
        self.pcount
    }

    fn gcount(&self) -> Option<u32> {
        self.gcount
    }
}

impl<'a> Header<'a> {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn keyword_records(&self) -> &[KeywordRecord<'a>] {
        &self.keyword_records
    }

    pub fn from(keyword_records: Vec<KeywordRecord<'a>>) -> Self {
        let number_of_axes = match keyword_records
            .iter()
            .find(|keyword_record| *keyword_record.keyword() == Keyword::NAxis)
        {
            Some(record) => {
                if let Value::Integer(n) = record.value() {
                    *n as u16
                } else {
                    0
                }
            }
            None => 0,
        };
        Self {
            has_data_array: {
                keyword_records.iter().any(|keyword_record| {
                    *keyword_record.keyword() == Keyword::NAxis
                        && *keyword_record.value() != Value::Integer(0)
                })
            },
            header_kind: {
                match keyword_records
                    .iter()
                    .find(|keyword_record| *keyword_record.keyword() == Keyword::Simple)
                {
                    Some(_) => HeaderKind::Primary,
                    None => {
                        match keyword_records
                            .iter()
                            .find(|keyword_record| *keyword_record.keyword() == Keyword::Xtension)
                        {
                            Some(record) => {
                                if let Value::CharacterString(s) = record.value() {
                                    match s.as_str() {
                                        "IMAGE   " => HeaderKind::Image,
                                        "TABLE   " => HeaderKind::Table,
                                        "BINTABLE" => HeaderKind::BinaryTable,
                                        _ => HeaderKind::Other(s.clone()),
                                    }
                                } else {
                                    HeaderKind::Other(String::from("Unknown"))
                                }
                            }
                            None => HeaderKind::Other(String::from("Unknown")),
                        }
                    }
                }
            },
            naxis: number_of_axes,
            bitpix: {
                match keyword_records
                    .iter()
                    .find(|keyword_record| *keyword_record.keyword() == Keyword::BitPix)
                {
                    Some(record) => {
                        if let Value::Integer(n) = record.value() {
                            Some(*n as i8)
                        } else {
                            None
                        }
                    }
                    None => None,
                }
            },
            dimensions: {
                let mut dimensions = Vec::new();
                for i in 1..=number_of_axes {
                    let keyword = Keyword::from(format!("NAXIS{}", i).as_bytes());
                    if let Some(record) = keyword_records
                        .iter()
                        .find(|keyword_record| *keyword_record.keyword() == keyword)
                    {
                        if let Value::Integer(n) = record.value() {
                            dimensions.push(*n as u32);
                        }
                    }
                }
                dimensions
            },
            bzero: {
                match keyword_records
                    .iter()
                    .find(|keyword_record| *keyword_record.keyword() == Keyword::BZero)
                {
                    Some(record) => {
                        if let Value::Real(n) = record.value() {
                            Some(*n as f32)
                        } else {
                            None
                        }
                    }
                    None => None,
                }
            },
            bscale: {
                match keyword_records
                    .iter()
                    .find(|keyword_record| *keyword_record.keyword() == Keyword::BScale)
                {
                    Some(record) => {
                        if let Value::Real(n) = record.value() {
                            Some(*n as f32)
                        } else {
                            None
                        }
                    }
                    None => None,
                }
            },
            pcount: {
                match keyword_records
                    .iter()
                    .find(|keyword_record| *keyword_record.keyword() == Keyword::PCount)
                {
                    Some(record) => {
                        if let Value::Integer(n) = record.value() {
                            Some(*n as u32)
                        } else {
                            None
                        }
                    }
                    None => None,
                }
            },
            gcount: {
                match keyword_records
                    .iter()
                    .find(|keyword_record| *keyword_record.keyword() == Keyword::GCount)
                {
                    Some(record) => {
                        if let Value::Integer(n) = record.value() {
                            Some(*n as u32)
                        } else {
                            None
                        }
                    }
                    None => None,
                }
            },
            keyword_records,
        }
    }
}

#[cfg(test)]
mod tests {
    use std::vec;

    use super::*;

    #[test]
    fn empty_header() {
        let header = Header::new();
        assert_eq!(header.len(), 0);
        assert!(header.is_empty());
        assert!(!header.has_data_array());
        assert_eq!(header.naxis(), 0);
        assert_eq!(header.bitpix(), None);
        assert_eq!(header.dimensions(), &Vec::new());
        assert_eq!(header.bzero(), None);
        assert_eq!(header.bscale(), None);
    }

    #[test]
    fn header_from_keyword_records() {
        let keyword_records = vec![
            KeywordRecord::new(
                Keyword::Simple,
                Value::Logical(true),
                Some("file does conform to FITS standard"),
            ),
            KeywordRecord::new(
                Keyword::BitPix,
                Value::Integer(8),
                Some("number of bits per data pixel"),
            ),
            KeywordRecord::new(
                Keyword::NAxis,
                Value::Integer(0),
                Some("number of data axes"),
            ),
            KeywordRecord::new(Keyword::BZero, Value::Real(0.0), Some("data range offset")),
            KeywordRecord::new(Keyword::BScale, Value::Real(1.0), Some("data scale factor")),
            KeywordRecord::new(Keyword::End, Value::CharacterString(String::from("")), None),
        ];
        let header = Header::from(keyword_records);
        assert_eq!(header.len(), 6);
        assert!(!header.is_empty());
        assert!(!header.has_data_array());
        assert_eq!(header.header_kind(), &HeaderKind::Primary);
        assert_eq!(header.naxis(), 0);
        assert_eq!(header.bitpix(), Some(8));
        assert_eq!(header.dimensions(), &Vec::new());
        assert_eq!(header.bzero(), Some(0.0));
        assert_eq!(header.bscale(), Some(1.0));
    }
    #[test]
    fn image_extension_from_keyword_records() {
        let keyword_records = vec![
            KeywordRecord::new(
                Keyword::Xtension,
                Value::CharacterString(String::from("IMAGE   ")),
                Some(" image data extension"),
            ),
            KeywordRecord::new(
                Keyword::BitPix,
                Value::Integer(8),
                Some("number of bits per data pixel"),
            ),
            KeywordRecord::new(Keyword::NAxisn(1), Value::Integer(2), Some("axis length")),
            KeywordRecord::new(Keyword::NAxisn(2), Value::Integer(2), Some("axis length")),
            KeywordRecord::new(
                Keyword::NAxis,
                Value::Integer(2),
                Some("number of data axes"),
            ),
            KeywordRecord::new(Keyword::PCount, Value::Integer(0), None),
            KeywordRecord::new(Keyword::GCount, Value::Integer(1), None),
            KeywordRecord::new(Keyword::BZero, Value::Real(0.0), Some("data range offset")),
            KeywordRecord::new(Keyword::BScale, Value::Real(1.0), Some("data scale factor")),
            KeywordRecord::new(Keyword::End, Value::CharacterString(String::from("")), None),
        ];
        let header = Header::from(keyword_records);
        assert_eq!(header.len(), 10);
        assert!(!header.is_empty());
        assert!(header.has_data_array());
        assert_eq!(header.header_kind(), &HeaderKind::Image);
        assert_eq!(header.naxis(), 2);
        assert_eq!(header.bitpix(), Some(8));
        assert_eq!(header.dimensions(), &vec![2, 2]);
        assert_eq!(header.pcount(), Some(0));
        assert_eq!(header.gcount(), Some(1));
        assert_eq!(header.bzero(), Some(0.0));
        assert_eq!(header.bscale(), Some(1.0));
    }
    #[test]
    fn table_extension_from_keyword_records() {
        let keyword_records = vec![
            KeywordRecord::new(
                Keyword::Xtension,
                Value::CharacterString(String::from("TABLE   ")),
                None,
            ),
            KeywordRecord::new(
                Keyword::BitPix,
                Value::Integer(8),
                Some("number of bits per data pixel"),
            ),
            KeywordRecord::new(Keyword::NAxisn(1), Value::Integer(2), Some("axis length")),
            KeywordRecord::new(Keyword::NAxisn(2), Value::Integer(2), Some("axis length")),
            KeywordRecord::new(
                Keyword::NAxis,
                Value::Integer(2),
                Some("number of data axes"),
            ),
            KeywordRecord::new(Keyword::PCount, Value::Integer(0), None),
            KeywordRecord::new(Keyword::GCount, Value::Integer(1), None),
            KeywordRecord::new(Keyword::End, Value::CharacterString(String::from("")), None),
        ];
        let header = Header::from(keyword_records);
        assert_eq!(header.len(), 8);
        assert!(!header.is_empty());
        assert!(header.has_data_array());
        assert_eq!(header.header_kind(), &HeaderKind::Table);
        assert_eq!(header.naxis(), 2);
        assert_eq!(header.bitpix(), Some(8));
        assert_eq!(header.dimensions(), &vec![2, 2]);
        assert_eq!(header.pcount(), Some(0));
        assert_eq!(header.gcount(), Some(1));
        assert_eq!(header.bzero(), None);
        assert_eq!(header.bscale(), None);
    }
}
