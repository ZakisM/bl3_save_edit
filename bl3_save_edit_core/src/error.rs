use std::fmt::Debug;

use thiserror::Error;

#[derive(Debug, Error)]
pub enum BL3ParserError<I: Debug> {
    #[error("failed to read {0:?} as could not read - {1:?}")]
    NomError(I, nom::error::ErrorKind),
    #[error("failed to parse data due to - {0}")]
    Other(anyhow::Error),
}

impl nom::error::ParseError<&[u8]> for BL3ParserError<String> {
    fn from_error_kind(_: &[u8], kind: nom::error::ErrorKind) -> Self {
        BL3ParserError::NomError("Binary Data".to_owned(), kind)
    }

    fn append(_: &[u8], _: nom::error::ErrorKind, other: Self) -> Self {
        other
    }
}

impl<I: Debug> nom::error::ParseError<I> for BL3ParserError<I> {
    fn from_error_kind(input: I, kind: nom::error::ErrorKind) -> Self {
        BL3ParserError::NomError(input, kind)
    }

    fn append(_: I, _: nom::error::ErrorKind, other: Self) -> Self {
        other
    }
}

impl<T> nom::ErrorConvert<BL3ParserError<String>> for nom::error::Error<(&[u8], T)> {
    fn convert(self) -> BL3ParserError<String> {
        BL3ParserError::NomError("Bit Data".to_owned(), self.code)
    }
}

impl<I: Debug> From<BL3ParserError<I>> for nom::Err<BL3ParserError<I>> {
    fn from(e: BL3ParserError<I>) -> Self {
        nom::Err::Error(e)
    }
}

pub trait ErrorExt<T, I: Debug> {
    fn parser_error(self) -> Result<T, BL3ParserError<I>>;
}

impl<T, I: Debug, E> ErrorExt<T, I> for Result<T, E>
where
    E: std::error::Error + Send + Sync + 'static,
{
    fn parser_error(self) -> Result<T, BL3ParserError<I>> {
        self.map_err(|e| BL3ParserError::Other(anyhow::Error::from(e)))
    }
}
