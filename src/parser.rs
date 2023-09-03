use nom::{
    branch::alt,
    bytes::complete::tag,
    bytes::complete::take,
    character::complete::{alpha0, space0},
    combinator::map,
    error::context,
    error::VerboseError,
    sequence::{pair, tuple},
    IResult,
};

use crate::keywords::Keyword;

fn comment_keyword(i: &[u8]) -> IResult<&[u8], Keyword, VerboseError<&[u8]>> {
    context("comment_keyword", tag("COMMENT "))(i).map(|(i, res)| (i, res.into()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_comment_keyword() {
        assert_eq!(
            comment_keyword(b"COMMENT "),
            Ok((&b""[..], Keyword::Comment))
        );
        assert_ne!(
            comment_keyword(b"COMMENT"),
            Ok((&b""[..], Keyword::Comment))
        );
    }
}
