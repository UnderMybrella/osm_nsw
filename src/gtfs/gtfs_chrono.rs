//HH:MM:SS format (H:MM:SS is also accepted)

use std::num::{IntErrorKind, ParseIntError};
use logos::Logos;
use derive_more::{Display, Error};

const GTFS_TIME_FORMAT: &'static str = "%Y-%m-%d %H:%M:%S";

#[derive(Default, Debug, Clone, PartialEq, Display, Error)]
enum GtfsLexingError {
    ParseIntError(IntErrorKind),
    #[default]
    UnknownToken,
}

#[automatically_derived]
impl ::core::fmt::Display for GtfsLexingError
    where
        IntErrorKind: ::core::fmt::Display,
{
    fn fmt(
        &self,
        __derive_more_f: &mut ::core::fmt::Formatter<'_>,
    ) -> ::core::fmt::Result {
        match self {
            Self::ParseIntError(_0) => ::core::fmt::Display::fmt(_0, __derive_more_f),
            Self::UnknownToken => {
                __derive_more_f.write_fmt(format_args!("UnknownToken"))
            }
        }
    }
}

/// Error type returned by calling `lex.slice().parse()` to u8.
impl From<ParseIntError> for GtfsLexingError {
    fn from(err: ParseIntError) -> Self {
        GtfsLexingError::ParseIntError(err.kind().to_owned())
    }
}

#[derive(Logos, Debug, PartialEq)]
#[logos(error = GtfsLexingError)]
enum GtfsTimeToken {
    #[regex(r"[0-9]+", |lex| lex.slice().parse())]
    Integer(u16),
    #[token(":")]
    Separator
}

pub fn test_chrono() {
    let mut lexer = GtfsTimeToken::lexer("01:12:32");

    assert_eq!(lexer.next(), Some(Ok(GtfsTimeToken::Integer(1))));
    assert_eq!(lexer.slice(), "01");

    assert_eq!(lexer.next(), Some(Ok(GtfsTimeToken::Integer(12))));
    assert_eq!(lexer.slice(), "12");

    assert_eq!(lexer.next(), Some(Ok(GtfsTimeToken::Integer(32))));
    assert_eq!(lexer.slice(), "32");
}