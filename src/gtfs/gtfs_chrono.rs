use std::fmt::{Display as StdDisplay, Formatter};
use std::num::{IntErrorKind, ParseIntError};
use std::str::FromStr;
use anyhow::bail;
use chrono::TimeDelta;
use logos::Logos;
use thiserror::Error;
use crate::gtfs::gtfs_chrono::GtfsLexingError::{MissingHours, MissingMinutes, MissingSeconds, UnknownToken};
use crate::gtfs::gtfs_types::GtfsTime;

const GTFS_TIME_FORMAT: &'static str = "%Y-%m-%d %H:%M:%S";

#[derive(Default, Debug, Clone, PartialEq, Error)]
pub enum GtfsLexingError {
    #[error("Failed to parse int")]
    ParseInt(#[from] ParseIntError),
    #[error("Missing number of hours in GtfsTime")]
    MissingHours,
    #[error("Missing number of minutes in GtfsTime")]
    MissingMinutes,
    #[error("Missing number of seconds in GtfsTime")]
    MissingSeconds,
    #[default]
    #[error("Unknown token")]
    UnknownToken,
}

#[derive(Logos, Debug, PartialEq)]
#[logos(error = GtfsLexingError)]
#[logos(skip ":")]
enum GtfsTimeToken {
    #[regex(r"-?[0-9]+", | lex | lex.slice().parse())]
    Integer(i32)
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

impl StdDisplay for GtfsTime {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut seconds = self.0.num_seconds();
        let hours = seconds / (60 * 60);
        seconds %= (60 * 60);

        let minutes = seconds / 60;
        seconds %= 60;

        f.write_fmt(format_args!("{:02}:{:02}:{:02}", hours, minutes, seconds))
    }
}

#[cfg(test)]
mod tests {
    #![feature(assert_matches)]
    use std::assert_matches::assert_matches;
    use std::num::IntErrorKind::PosOverflow;
    use std::num::{IntErrorKind, ParseIntError};
    use logos::Logos;
    use crate::gtfs::gtfs_chrono::GtfsLexingError::{ParseInt, UnknownToken};
    use crate::gtfs::gtfs_chrono::GtfsTimeToken;

    macro_rules! assert_lex {
        ($lex:ident, |$next:ident| $($on_next:block)?$($on_next_stmt:stmt)?$(, |$span:ident| $($on_span:block)?$($on_span_stmt:stmt)?$(, |$slice:ident| $($on_slice:block)?$($on_slice_stmt:stmt)?)?)?) => {
            let $next = $lex.next();
            $($on_next)?;
            $($on_next_stmt)?;

            $(
                let $span = $lex.span();
                $($on_span)?;
                $($on_span_stmt)?;

                $(
                    let $slice = $lex.slice();
                    $($on_slice)?;
                    $($on_slice_stmt)?;
                )?
            )?
        };
    }

    #[test]
    fn test_gtfs_time_lexer_unknown_token() {
        let mut lex = GtfsTimeToken::lexer("48:60:FF");

        assert_lex!(lex,
            |next| assert_eq!(next, Some(Ok(GtfsTimeToken::Integer(48)))),
            |span| assert_eq!(span, 0..2),
            |slice| assert_eq!(slice, "48")
        );

        assert_lex!(lex,
            |next| assert_eq!(next, Some(Ok(GtfsTimeToken::Integer(60)))),
            |span| assert_eq!(span, 3..5),
            |slice| assert_eq!(slice, "60")
        );

        for i in 0..2 {
            assert_lex!(lex,
                |next| assert_eq!(next, Some(Err(UnknownToken))),
                |span| assert_eq!(span, (6 + i) .. (7 + i)),
                |slice| assert_eq!(slice, "F")
            );
        }

        assert_eq!(lex.next(), None);
    }



    #[test]
    fn test_gtfs_time_lexer_overflow() {
        let mut lex = GtfsTimeToken::lexer("2147483648");

        assert_lex!(lex,
            |next| assert_matches!(next, Some(Err(ParseInt(err))) if err.kind() == &PosOverflow),
            |span| assert_eq!(span, 0..10),
            |slice| assert_eq!(slice, "2147483648")
        );

        assert_eq!(lex.next(), None);
    }
}