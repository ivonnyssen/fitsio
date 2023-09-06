use crate::types::Value;

use nom::{
    bytes::complete::{tag, take_while},
    character::complete::space0,
    combinator::map,
    error::context,
    error::VerboseError,
    multi::many0,
    sequence::{preceded, terminated},
    IResult,
};

pub fn character_string(i: &[u8]) -> IResult<&[u8], Value, VerboseError<&[u8]>> {
    context(
        "character_string",
        map(
            preceded(
                space0,
                terminated(
                    many0(preceded(tag(b"'"), terminated(no_single_quote, tag(b"'")))),
                    space0,
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
        acc.push_str(std::str::from_utf8(part).unwrap());
        match it.peek().is_some() {
            true => acc.push('\''),
            false => (),
        }
    }
    acc
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_character_string() {
        assert_eq!(
            character_string(
                b"'This file is part of the EUVE Science Archive. It contains'          "
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
                b"'String with single quote '' 123.45 , _ + - '                         "
            ),
            Ok((
                &b""[..],
                Value::CharacterString("String with single quote ' 123.45 , _ + - ".to_string())
            ))
        );
    }
}
