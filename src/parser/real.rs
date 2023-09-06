use crate::types::Value;

use nom::{
    character::complete::space0,
    combinator::map,
    error::context,
    error::VerboseError,
    number::complete::double,
    sequence::{preceded, terminated},
    IResult,
};

pub fn real(i: &[u8]) -> IResult<&[u8], Value, VerboseError<&[u8]>> {
    context(
        "real",
        map(preceded(space0, terminated(double, space0)), Value::Real),
    )(i)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_real() {
        assert_eq!(
            real(b"+300.1                                                                "),
            Ok((&b""[..], Value::Real(300.1)))
        );
        assert_eq!(
            real(b"-300.1                                                                "),
            Ok((&b""[..], Value::Real(-300.1)))
        );
        assert_eq!(
            real(b" 300.1                                                                "),
            Ok((&b""[..], Value::Real(300.1)))
        );
        assert_eq!(
            real(b"300.1                                                                 "),
            Ok((&b""[..], Value::Real(300.1)))
        );
        assert_ne!(
            real(b"+500.1                                                                "),
            Ok((&b""[..], Value::Real(300.1)))
        );
    }
}
