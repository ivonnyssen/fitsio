use super::{keyword::Keyword, keyword_record::KeywordRecord, value::Value};

/// The primary header of a FITS file.
#[derive(PartialEq, Debug, Default)]
pub struct Header<'a> {
    keyword_records: Vec<KeywordRecord<'a>>,
    has_data_array: bool,
    naxis: u16,
    bitpix: Option<i8>,
    dimensions: Vec<u32>,
    bzero: Option<f32>,
    bscale: Option<f32>,
}

impl<'a> Header<'a> {
    pub fn keyword_records(&self) -> &[KeywordRecord] {
        &self.keyword_records
    }

    pub fn new() -> Self {
        Self::default()
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
            keyword_records,
        }
    }

    pub fn has_data_array(&self) -> bool {
        self.has_data_array
    }

    pub fn len(&self) -> usize {
        self.keyword_records.len()
    }

    pub fn is_empty(&self) -> bool {
        self.keyword_records.is_empty()
    }

    pub fn naxis(&self) -> u16 {
        self.naxis
    }

    pub fn bitpix(&self) -> Option<i8> {
        self.bitpix
    }

    pub fn dimensions(&self) -> &Vec<u32> {
        self.dimensions.as_ref()
    }

    pub fn bzero(&self) -> Option<f32> {
        self.bzero
    }

    pub fn bscale(&self) -> Option<f32> {
        self.bscale
    }
}

#[cfg(test)]
mod tests {
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
}
