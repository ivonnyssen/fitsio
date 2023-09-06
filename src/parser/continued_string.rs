use crate::types::Value;

use nom::{error::VerboseError, IResult};

pub fn continued_string(i: &[u8]) -> IResult<&[u8], Value, VerboseError<&[u8]>> {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;
    fn test_continued_string() {}
}
