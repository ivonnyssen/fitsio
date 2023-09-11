use crate::types::Value;

use nom::{
    branch::alt,
    bytes::complete::{tag, take_while},
    character::complete::{i64, space0},
    combinator::map,
    error::context,
    error::VerboseError,
    multi::many0,
    number::complete::double,
    sequence::{preceded, separated_pair, terminated},
    IResult,
};

pub fn character_string(i: &[u8]) -> IResult<&[u8], Value, VerboseError<&[u8]>> {
    context(
        "character_string",
        map(
            preceded(
                alt((tag("= "), tag("  "))),
                preceded(
                    space0,
                    terminated(
                        many0(preceded(tag(b"'"), terminated(no_single_quote, tag(b"'")))),
                        space0,
                    ),
                ),
            ),
            |parts: Vec<&[u8]>| Value::CharacterString(u8vec_to_string(parts)),
        ),
    )(i)
}

fn no_single_quote(i: &[u8]) -> IResult<&[u8], &[u8], VerboseError<&[u8]>> {
    context("no_single_quote", take_while(|c| c != b'\''))(i)
}

fn u8vec_to_string(v: Vec<&[u8]>) -> String {
    let mut it = v.iter().peekable();
    let mut acc = String::new();
    while let Some(part) = it.next() {
        acc.push_str(std::str::from_utf8(part).unwrap_or(""));
        match it.peek().is_some() {
            true => acc.push('\''),
            false => (),
        }
    }
    acc
}

fn u8vec_to_string_eating_last_ampersand(v: Vec<&[u8]>) -> String {
    let mut it = v.iter().peekable();
    let mut acc = String::new();
    while let Some(part) = it.next() {
        acc.push_str(std::str::from_utf8(part).unwrap_or(""));
        match it.peek().is_some() {
            true => acc.push('\''),
            false => match acc.ends_with('&') {
                true => {
                    _ = acc.pop();
                }
                false => (),
            },
        }
    }
    acc
}

pub fn complex_float(i: &[u8]) -> IResult<&[u8], Value, VerboseError<&[u8]>> {
    context(
        "complex integer",
        map(
            preceded(
                tag("= "),
                preceded(
                    space0,
                    terminated(
                        separated_pair(
                            preceded(tag("("), double),
                            tag(","),
                            preceded(space0, terminated(double, tag(")"))),
                        ),
                        space0,
                    ),
                ),
            ),
            Value::ComplexFloat,
        ),
    )(i)
}

pub fn complex_integer(i: &[u8]) -> IResult<&[u8], Value, VerboseError<&[u8]>> {
    context(
        "complex integer",
        map(
            preceded(
                tag("= "),
                preceded(
                    space0,
                    terminated(
                        separated_pair(
                            preceded(tag("("), preceded(space0, i64)),
                            tag(","),
                            preceded(space0, terminated(i64, tag(")"))),
                        ),
                        space0,
                    ),
                ),
            ),
            Value::ComplexInteger,
        ),
    )(i)
}

pub fn continued_string(i: &[u8]) -> IResult<&[u8], Value, VerboseError<&[u8]>> {
    context(
        "continued string",
        map(
            preceded(
                alt((tag("= "), tag("  "))),
                preceded(
                    space0,
                    terminated(
                        many0(preceded(tag(b"'"), terminated(no_single_quote, tag(b"'")))),
                        space0,
                    ),
                ),
            ),
            |parts: Vec<&[u8]>| {
                Value::CharacterString(u8vec_to_string_eating_last_ampersand(parts))
            },
        ),
    )(i)
}

pub fn date(i: &[u8]) -> IResult<&[u8], Value, VerboseError<&[u8]>> {
    //CCYY-MM-DD[Thh:mm:ss[.s...]]
    context(
        "date",
        map(
            preceded(
                tag("= "),
                preceded(space0, take_while(super::is_allowed_ascii)),
            ),
            |s: &[u8]| Value::Date(std::str::from_utf8(s).unwrap_or("").trim_end().to_string()),
        ),
    )(i)
}

pub fn integer(i: &[u8]) -> IResult<&[u8], Value, VerboseError<&[u8]>> {
    context(
        "integer",
        map(
            preceded(tag("= "), preceded(space0, terminated(i64, space0))),
            Value::Integer,
        ),
    )(i)
}

pub fn logical(i: &[u8]) -> IResult<&[u8], Value, VerboseError<&[u8]>> {
    context(
        "logical",
        map(
            preceded(
                space0,
                preceded(
                    tag("= "),
                    preceded(space0, terminated(alt((tag("T"), tag("F"))), space0)),
                ),
            ),
            |s: &[u8]| Value::Logical(s == b"T"),
        ),
    )(i)
}

pub fn real(i: &[u8]) -> IResult<&[u8], Value, VerboseError<&[u8]>> {
    context(
        "real",
        map(
            preceded(tag("= "), preceded(space0, terminated(double, space0))),
            Value::Real,
        ),
    )(i)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_character_string() {
        assert_eq!(
            character_string(
                b"= 'This file is part of the EUVE Science Archive. It contains'          "
            ),
            Ok((
                &b""[..],
                Value::CharacterString(
                    "This file is part of the EUVE Science Archive. It contains".to_string()
                )
            ))
        );
        assert_eq!(
            character_string(
                b"= 'String with single quote '' 123.45 , _ + - '                         "
            ),
            Ok((
                &b""[..],
                Value::CharacterString("String with single quote ' 123.45 , _ + - ".to_string())
            ))
        );
        assert_eq!(
            character_string(
                b"= 'String with comment' / not returned in the string                    "
            ),
            Ok((
                &b"/ not returned in the string                    "[..],
                Value::CharacterString("String with comment".to_string())
            ))
        );
    }

    #[test]
    fn test_complex_float() {
        assert_eq!(
            complex_float(
                b"= (123.23, -45.7)                                                       "
            ),
            Ok((&b""[..], Value::ComplexFloat((123.23, -45.7))))
        );
        assert_eq!(
            complex_float(
                b"=  (+123.23, 45.7)                                                      "
            ),
            Ok((&b""[..], Value::ComplexFloat((123.23, 45.7))))
        );
        assert_eq!(
            complex_float(
                b"= (-123.23, +45.7)                                                      "
            ),
            Ok((&b""[..], Value::ComplexFloat((-123.23, 45.7))))
        );
        assert_ne!(
            complex_float(
                b"= (500.23, -45.7)                                                       "
            ),
            Ok((&b""[..], Value::ComplexFloat((123.23, -45.7))))
        );
    }

    #[test]
    fn test_complex_integer() {
        assert_eq!(
            complex_integer(
                b"= ( 123, 45)                                                            "
            ),
            Ok((&b""[..], Value::ComplexInteger((123, 45))))
        );
        assert_eq!(
            complex_integer(
                b"=   (123, 45)                                                           "
            ),
            Ok((&b""[..], Value::ComplexInteger((123, 45))))
        );
        assert_eq!(
            complex_integer(
                b"= (-123,-45)                                                            "
            ),
            Ok((&b""[..], Value::ComplexInteger((-123, -45))))
        );
        assert_eq!(
            complex_integer(
                b"= (+123, +45)                                                           "
            ),
            Ok((&b""[..], Value::ComplexInteger((123, 45))))
        );
        assert_ne!(
            complex_integer(
                b"= (-500,-45)                                                            "
            ),
            Ok((&b""[..], Value::ComplexInteger((-123, -45))))
        );
    }

    #[test]
    fn test_continued_string() {
        assert_eq!(
            continued_string(b"  ' over multiple keyword records.&'"),
            Ok((
                &b""[..],
                Value::CharacterString(" over multiple keyword records.".to_string())
            ))
        );
        assert_eq!(
            continued_string(b"  '&' / The comment field for this"),
            Ok((
                &b"/ The comment field for this"[..],
                Value::CharacterString("".to_string())
            ))
        );
    }

    #[test]
    fn test_date() {
        assert_eq!(
            date(b"= 0000-01-01T00:00:00                                                   "),
            Ok((&b""[..], Value::Date("0000-01-01T00:00:00".to_string())))
        );
        assert_eq!(
            date(b"=                                                    9999-12-31T23:59:59"),
            Ok((&b""[..], Value::Date("9999-12-31T23:59:59".to_string())))
        );
        assert_eq!(
            date(b"= 99999-01-01T00:00:00                                                  "),
            Ok((&b""[..], Value::Date("99999-01-01T00:00:00".to_string())))
        );
        assert_eq!(
            date(b"= +99999-12-31T23:59:59                                                 "),
            Ok((&b""[..], Value::Date("+99999-12-31T23:59:59".to_string())))
        );
        assert_eq!(
            date(b"= -04713-11-24T12:00:00                                                 "),
            Ok((&b""[..], Value::Date("-04713-11-24T12:00:00".to_string())))
        );
    }

    #[test]
    fn test_integer() {
        assert_eq!(
            integer(b"= +300                                                                  "),
            Ok((&b""[..], Value::Integer(300)))
        );
        assert_eq!(
            integer(b"=   -300                                                                "),
            Ok((&b""[..], Value::Integer(-300)))
        );
        assert_eq!(
            integer(b"=  300                                                                  "),
            Ok((&b""[..], Value::Integer(300)))
        );
        assert_eq!(
            integer(b"= 300                                                                   "),
            Ok((&b""[..], Value::Integer(300)))
        );
        assert_ne!(
            integer(b"= +500                                                                  "),
            Ok((&b""[..], Value::Integer(300)))
        );
    }

    #[test]
    fn test_logical() {
        assert_eq!(
            logical(b"=                    T                                                  "),
            Ok((&b""[..], Value::Logical(true)))
        );
        assert_eq!(
            logical(b"=                    F                                                  "),
            Ok((&b""[..], Value::Logical(false)))
        );
        assert_eq!(
            logical(b"= T                                                                     "),
            Ok((&b""[..], Value::Logical(true)))
        );
        assert_eq!(
            logical(b"=  F                                                                    "),
            Ok((&b""[..], Value::Logical(false)))
        );
        assert_ne!(
            logical(b"=  T   /Test comment                                                    "),
            Ok((
                &b"/Test comment                                                    "[..],
                Value::Logical(false)
            ))
        );
    }
    #[test]
    fn test_real() {
        assert_eq!(
            real(b"= +300.1                                                                "),
            Ok((&b""[..], Value::Real(300.1)))
        );
        assert_eq!(
            real(b"= -300.1                                                                "),
            Ok((&b""[..], Value::Real(-300.1)))
        );
        assert_eq!(
            real(b"=  300.1                                                                "),
            Ok((&b""[..], Value::Real(300.1)))
        );
        assert_eq!(
            real(b"= 300.1                                                                 "),
            Ok((&b""[..], Value::Real(300.1)))
        );
        assert_ne!(
            real(b"= +500.1                                                                "),
            Ok((&b""[..], Value::Real(300.1)))
        );
    }
}
