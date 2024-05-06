//HH:MM:SS format (H:MM:SS is also accepted)

use std::fmt::Formatter;
use std::num::{IntErrorKind, ParseIntError};
use std::str::FromStr;
use anyhow::bail;
use chrono::TimeDelta;
use derive_more::{Display, Error};
use logos::Logos;
use crate::gtfs::gtfs_chrono::GtfsLexingError::{MissingHours, MissingMinutes, MissingSeconds, UnknownToken};
use crate::gtfs::gtfs_types::GtfsTime;

const GTFS_TIME_FORMAT: &'static str = "%Y-%m-%d %H:%M:%S";

#[derive(Default, Debug, Clone, PartialEq, Display, Error)]
pub enum GtfsLexingError {
    ParseIntError(ParseIntError),
    MissingHours,
    MissingMinutes,
    MissingSeconds,
    #[default]
    UnknownToken,
}

/// Error type returned by calling `lex.slice().parse()` to u8.
impl From<ParseIntError> for GtfsLexingError {
    fn from(err: ParseIntError) -> Self {
        GtfsLexingError::ParseIntError(err.to_owned())
    }
}

#[derive(Logos, Debug, PartialEq)]
#[logos(error = GtfsLexingError)]
#[logos(skip ":")]
enum GtfsTimeToken {
    #[regex(r"[0-9]+", | lex | lex.slice().parse())]
    Integer(u16)
}

macro_rules! logos_next {
    ($lexer:ident, $bail:expr $(, $m:pat => $body:block)+) => {
        match $lexer.next() {
            Some(Err(e)) => bail!(e),
            None => bail!($bail),
            $(Some(Ok($m)) => $body)+
        }
    };

    ($lexer:ident, Err: $bail:expr $(, $m:pat => $body:block)+) => {
        match $lexer.next() {
            Some(Err(e)) => return std::result::Result::Err(e),
            None => return std::result::Result::Err($bail),
            $(Some(Ok($m)) => $body)+
        }
    };
}

macro_rules! logos_end {
    ($lexer:ident, $bail:expr, $body:block) => {
        match $lexer.next() {
            Some(Err(e)) => bail!(e),
            Some(..) => bail!($bail),
            None => $body
        }
    };

    ($lexer:ident, Err: $bail:expr, $body:block) => {
        match $lexer.next() {
            Some(Err(e)) => return std::result::Result::Err(e),
            Some(..) => return std::result::Result::Err($bail),
            None => $body
        }
    };
}

impl FromStr for GtfsTime {
    type Err = GtfsLexingError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut lexer = GtfsTimeToken::lexer(s);
        logos_next!(lexer, Err: MissingHours, GtfsTimeToken::Integer(hours) => {
            logos_next!(lexer, Err: MissingMinutes, GtfsTimeToken::Integer(minutes) => {
                logos_next!(lexer, Err: MissingSeconds, GtfsTimeToken::Integer(seconds) => {
                    logos_end!(lexer, Err: UnknownToken, {
                        return Ok(GtfsTime(TimeDelta::hours(i64::from(hours)) + TimeDelta::minutes(i64::from(minutes)) + TimeDelta::seconds(i64::from(seconds))))
                    });
                })
            })
        });
    }
}

impl Display for GtfsTime {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut seconds = self.0.num_seconds();
        let hours = seconds / (60 * 60);
        seconds %= (60 * 60);

        let minutes = seconds / 60;
        seconds %= 60;

        f.write_fmt(format_args!("{:02}:{:02}:{:02}", hours, minutes, seconds))
    }
}