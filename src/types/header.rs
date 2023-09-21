use super::{keyword::Keyword, keyword_record::KeywordRecord, value::Value};

/// The primary header of a FITS file.
#[derive(PartialEq, Debug)]
pub struct Header<'a> {
    keyword_records: Vec<KeywordRecord<'a>>,
}

impl<'a> Header<'a> {
    pub fn keyword_records(&self) -> &[KeywordRecord] {
        &self.keyword_records
    }

    pub fn new(keyword_records: Vec<KeywordRecord<'a>>) -> Self {
        Self { keyword_records }
    }

    pub fn has_data_array(&self) -> bool {
        self.keyword_records.iter().any(|keyword_record| {
            *keyword_record.keyword() == Keyword::NAxis
                && *keyword_record.value() != Value::Integer(0)
        })
    }

    pub fn len(&self) -> usize {
        self.keyword_records.len()
    }

    pub fn is_empty(&self) -> bool {
        self.keyword_records.is_empty()
    }

    pub fn naxis(&self) -> u16 {
        match self
            .keyword_records
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
        }
    }

    pub fn bitpix(&self) -> Option<i8> {
        match self
            .keyword_records
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
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_header() {
        let header = Header::new(Vec::new());
        assert_eq!(header.len(), 0);
        assert!(header.is_empty());
        assert_eq!(header.naxis(), 0);
        assert_eq!(header.bitpix(), None);
    }
}
