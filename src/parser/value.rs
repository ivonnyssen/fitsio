use crate::types::Value;

use nom::{
    branch::alt,
    bytes::complete::{tag, take, take_while},
    character::complete::{digit1, i64, one_of, space0},
    combinator::{map, opt, recognize},
    error::context,
    error::VerboseError,
    multi::many0,
    number::complete::double,
    sequence::{pair, preceded, separated_pair, terminated, tuple},
    IResult,
};
use time::macros::format_description;
use time::PrimitiveDateTime;
use tracing::{error, trace};

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

const DATE_FORMAT: &[time::format_description::FormatItem<'_>] = format_description!(
    version = 2,
    "[year]-[month]-[day][ optional [T[hour]:[minute]:[second][ optional [.[subsecond]]]]]"
);
pub fn date(i: &[u8]) -> IResult<&[u8], Value, VerboseError<&[u8]>> {
    //CCYY-MM-DD[Thh:mm:ss[.s...]]
    context(
        "date",
        map(
            preceded(
                tag("= "),
                preceded(
                    space0,
                    terminated(
                        recognize(tuple((
                            opt(one_of("+-")),
                            digit1,
                            tag("-"),
                            digit1,
                            tag("-"),
                            digit1,
                            opt(tuple((
                                tag("T"),
                                digit1,
                                tag(":"),
                                digit1,
                                tag(":"),
                                digit1,
                                opt(pair(tag("."), digit1)),
                            ))),
                        ))),
                        space0,
                    ),
                ),
            ),
            |date| {
                let date = std::str::from_utf8(date).unwrap_or("");
                let date = PrimitiveDateTime::parse(date, &DATE_FORMAT)
                    .unwrap_or(time::PrimitiveDateTime::MIN);
                Value::Date(date)
            },
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

pub fn unknown(i: &[u8]) -> IResult<&[u8], Value, VerboseError<&[u8]>> {
    context(
        "unknown",
        map(take(72u8), |value: &[u8]| {
            trace!("value: {:?}", value);
            match std::str::from_utf8(value) {
                Ok(res) => Value::Unknown(res.to_string()),
                Err(err) => {
                    error!("error parsing value: {:?}", err);
                    Value::Unknown(String::from(""))
                }
            }
        }),
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
            Ok((
                &b""[..],
                Value::Date(PrimitiveDateTime::parse("0000-01-01T00:00:00", &DATE_FORMAT).unwrap())
            ))
        );
        assert_eq!(
            date(b"=                                                    9999-12-31T23:59:59"),
            Ok((
                &b""[..],
                Value::Date(PrimitiveDateTime::parse("9999-12-31T23:59:59", &DATE_FORMAT).unwrap())
            ))
        );
        assert_eq!(
            date(b"= +99999-01-01T00:00:00                                                 "),
            Ok((
                &b""[..],
                Value::Date(
                    PrimitiveDateTime::parse("+99999-01-01T00:00:00", &DATE_FORMAT).unwrap()
                )
            ))
        );
        assert_eq!(
            date(b"= +99999-12-31T23:59:59                                                 "),
            Ok((
                &b""[..],
                Value::Date(
                    PrimitiveDateTime::parse("+99999-12-31T23:59:59", &DATE_FORMAT).unwrap()
                )
            ))
        );
        assert_eq!(
            date(b"= -04713-11-24T12:00:00                                                 "),
            Ok((
                &b""[..],
                Value::Date(
                    PrimitiveDateTime::parse("-04713-11-24T12:00:00", &DATE_FORMAT).unwrap()
                )
            ))
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
            real(b"=                1.0E0 / Scale factor for pixel values                  "),
            Ok((
                &b"/ Scale factor for pixel values                  "[..],
                Value::Real(1.0)
            ))
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

    #[test]
    fn test_unknown() {
        assert_eq!(
            unknown(b"XXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX"),
            Ok((
                &b""[..],
                Value::Unknown(
                    "XXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX"
                        .to_string()
                )
            ))
        );
        assert!(unknown(b"").is_err());
    }
}
