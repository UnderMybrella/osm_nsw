use std::fmt::{Display, Write};
use std::num::ParseIntError;
use std::str::FromStr;
use num_traits::{FromPrimitive, PrimInt, ToPrimitive};
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use crate::make_from_primitive_try_from;
use crate::try_from_prim;

pub struct ColourCode(pub u32);

impl FromStr for ColourCode {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        u32::from_str_radix(s, 16).map(ColourCode)
    }
}
impl Display for ColourCode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{:X}", self.0))
    }
}

impl Into<u32> for &ColourCode {
    fn into(self) -> u32 {
        self.0
    }
}

impl FromPrimitive for ColourCode {
    fn from_i64(n: i64) -> Option<Self> {
        n.to_u32().map(ColourCode)
    }

    fn from_u64(n: u64) -> Option<Self> {
        n.to_u32().map(ColourCode)
    }
}

impl ToPrimitive for ColourCode {
    fn to_i64(&self) -> Option<i64> {
        self.0.to_i64()
    }

    fn to_u64(&self) -> Option<u64> {
        self.0.to_u64()
    }
}

make_from_primitive_try_from!(to_u32: ColourCode[ColourCode]);