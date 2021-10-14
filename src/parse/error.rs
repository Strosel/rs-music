use nom::error::{ErrorKind, FromExternalError, ParseError};

pub type IResult<I, O, E = ParserError<I>> = nom::IResult<I, O, E>;

#[derive(Debug)]
pub enum ParserError<I> {
    AccidentalError(&'static str),
    DurationError(&'static str),
    Other(&'static str),
    Nom(I, ErrorKind),
}

impl<I> ParseError<I> for ParserError<I> {
    fn from_error_kind(input: I, kind: ErrorKind) -> Self {
        Self::Nom(input, kind)
    }

    fn append(_: I, _: ErrorKind, other: Self) -> Self {
        other
    }
}

impl<I> FromExternalError<I, &'static str> for ParserError<I> {
    fn from_external_error(_: I, _: ErrorKind, e: &'static str) -> Self {
        Self::Other(e)
    }
}
