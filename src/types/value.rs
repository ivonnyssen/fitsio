use std::fmt;

use time::PrimitiveDateTime;

/// The possible values of a KeywordRecord.
#[derive(PartialEq, Debug)]
pub enum Value<'a> {
    /// An ASCII string 0x20 - 0x7E. FITS standard section 4.2.1.1
    CharacterString(String),
    /// Complex floating point number. FITS standard section 4.2.6
    ComplexFloat((f64, f64)),
    /// Complex integer with real and imaginary parts. FITS standard section 4.2.5
    ComplexInteger((i64, i64)),
    /// A multi-part string. FITS standard section 4.2.1.2
    ContinuedString(Vec<&'a str>),
    /// Date. ISO-8601 string. FITS standard section 4.2.7
    Date(PrimitiveDateTime),
    /// An signed integer. FITS standard section 4.2.3
    Integer(i64),
    /// A logical constant or value `F` or `T`. FITS standard section 4.2.2
    Logical(bool),
    /// Fixed format real floating point number. FITS standard section 4.2.4
    Real(f64),
    /// Unknown value - presented as 72 ASCII characters
    Unknown(String),
}

impl fmt::Display for Value<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Value::CharacterString(s) => write!(f, "'{}'", s),
            Value::ComplexFloat((r, i)) => write!(f, "{} + {}i", r, i),
            Value::ComplexInteger((r, i)) => write!(f, "{} + {}i", r, i),
            Value::ContinuedString(s) => write!(f, "{:#?}", s),
            Value::Date(d) => write!(f, "{}", d),
            Value::Integer(i) => write!(f, "{}", i),
            Value::Logical(b) => write!(f, "{}", b),
            Value::Real(r) => write!(f, "{}", r),
            Value::Unknown(s) => write!(f, "{}", s),
        }
    }
}

#[cfg(test)]
mod tests {
    use time::macros::datetime;

    use super::*;

    #[test]
    fn test_display_for_value() {
        assert_eq!(
            format!("{}", Value::CharacterString("hello".to_string())),
            "'hello'"
        );
        assert_eq!(format!("{}", Value::ComplexFloat((1.0, 2.0))), "1 + 2i");
        assert_eq!(format!("{}", Value::ComplexInteger((1, 2))), "1 + 2i");
        assert_eq!(
            format!("{}", Value::ContinuedString(vec!["hello", "world"])),
            "[\n    \"hello\",\n    \"world\",\n]"
        );
        assert_eq!(
            format!("{}", Value::Date(datetime!(2019-01-01 0:00))),
            "2019-01-01 0:00:00.0"
        );
        assert_eq!(format!("{}", Value::Integer(1)), "1");
        assert_eq!(format!("{}", Value::Logical(true)), "true");
        assert_eq!(format!("{}", Value::Real(1.0)), "1");
        assert_eq!(format!("{}", Value::Unknown("hello".to_string())), "hello");
    }
}
