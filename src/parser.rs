use nom::{error::VerboseError, Finish, IResult};

use crate::errors::Error;

pub type Parser<T, U> = IResult<T, U, VerboseError<T>>;

// Parse the input, returning a complete data packet, and return how much was consumed
// (so the cursor can be incremented)
pub fn parse<T>(
    parser: impl Fn(&str) -> Parser<&str, T>,
    input: &str,
) -> Result<(usize, T), Error> {
    parser(input)
        .finish()
        .map(|(remaining, result)| (input.len() - remaining.len(), result))
        .map_err(|e| Error::ParsingError(e.to_string()))
}
