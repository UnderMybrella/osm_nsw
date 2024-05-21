use std::fmt::{Display, Formatter};
use std::num::ParseIntError;
use std::str::FromStr;
use chrono::{NaiveTime, TimeDelta};

use num_traits::{FromPrimitive, ToPrimitive};
use serde::{Deserialize, Serialize};

use crate::make_from_primitive_try_from;
use crate::try_from_prim;

#[derive(Debug, Serialize, Deserialize)]
pub struct GtfsColourCode(pub u32);
#[derive(Debug)]
pub struct GtfsCurrencyCode(pub String);
#[derive(Debug)]
pub struct GtfsCurrencyAmount(pub f64);
#[derive(Debug)]
pub struct GtfsEmail(pub String);
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct GtfsID(pub String);
#[derive(Debug, Serialize, Deserialize, Ord, PartialOrd, Eq, PartialEq)]
pub struct GtfsLanguageCode(pub String);
#[derive(Debug)]
pub struct GtfsTime(pub TimeDelta);

impl FromStr for GtfsColourCode {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        u32::from_str_radix(s, 16).map(GtfsColourCode)
    }
}
impl Display for GtfsColourCode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{:X}", self.0))
    }
}
impl Display for GtfsID {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.0.as_str())
    }
}

impl AsRef<str> for GtfsID {
    fn as_ref(&self) -> &str {
        self.0.as_str()
    }
}

impl Into<u32> for &GtfsColourCode {
    fn into(self) -> u32 {
        self.0
    }
}

impl FromPrimitive for GtfsColourCode {
    fn from_i64(n: i64) -> Option<Self> {
        n.to_u32().map(GtfsColourCode)
    }

    fn from_u64(n: u64) -> Option<Self> {
        n.to_u32().map(GtfsColourCode)
    }
}

impl ToPrimitive for GtfsColourCode {
    fn to_i64(&self) -> Option<i64> {
        self.0.to_i64()
    }

    fn to_u64(&self) -> Option<u64> {
        self.0.to_u64()
    }
}

make_from_primitive_try_from!(to_u32: GtfsColourCode[GtfsColourCode]);