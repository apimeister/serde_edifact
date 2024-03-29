use std::fmt::{self, Display};

use serde::{de, ser};

#[derive(Debug)]
pub enum Error {
    Message(String),
    // Eof,
    // Syntax,
    // ExpectedBoolean,
    // ExpectedInteger,
    // ExpectedString,
    // ExpectedNull,
    // ExpectedArray,
    // ExpectedArrayComma,
    // ExpectedArrayEnd,
    // ExpectedMap,
    // ExpectedMapColon,
    // ExpectedMapComma,
    // ExpectedMapEnd,
    // ExpectedEnum,
    // TrailingCharacters,
}

impl ser::Error for Error {
    fn custom<T: Display>(msg: T) -> Self {
        Error::Message(msg.to_string())
    }
}

impl de::Error for Error {
    fn custom<T: Display>(msg: T) -> Self {
        Error::Message(msg.to_string())
    }
}

impl Display for Error {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::Message(msg) => formatter.write_str(msg),
            // Error::Eof => formatter.write_str("unexpected end of input"),
            /* and so forth */
        }
    }
}

impl std::error::Error for Error {}
