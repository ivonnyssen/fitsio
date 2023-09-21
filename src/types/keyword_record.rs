use super::{keyword::Keyword, value::Value};
use std::fmt;
#[derive(PartialEq, Debug)]
pub struct KeywordRecord<'a> {
    keyword: Keyword,
    value: Value<'a>,
    comment: Option<&'a str>,
}

impl fmt::Display for KeywordRecord<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} = {}", self.keyword, self.value)?;
        if let Some(comment) = self.comment {
            write!(f, " / {}", comment)?;
        }
        Ok(())
    }
}

impl<'a> KeywordRecord<'a> {
    pub fn new(keyword: Keyword, value: Value<'a>, comment: Option<&'a str>) -> Self {
        Self {
            keyword,
            value,
            comment,
        }
    }

    pub fn keyword(&self) -> &Keyword {
        &self.keyword
    }

    pub fn value(&self) -> &Value {
        &self.value
    }

    pub fn comment(&self) -> &Option<&str> {
        &self.comment
    }
}
