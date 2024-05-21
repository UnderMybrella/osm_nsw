use std::fmt::Debug;
use std::ops::{Deref, Range};
use std::str::FromStr;
use rand::distributions::uniform::{SampleBorrow, SampleRange, SampleUniform, UniformSampler};
use rand::{Rng, RngCore};
use serde::{Deserialize, Serialize};

type StdRange<T> = std::ops::Range<T>;
type StdRangeInclusive<T> = std::ops::RangeInclusive<T>;

#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MauSampleRange<T: SampleUniform + PartialOrd> {
    SampleRange { start: T, end: T },
    SampleRangeInclusive { start: T, end: T },
}

impl<T: SampleUniform + PartialOrd> From<StdRange<T>> for MauSampleRange<T> {
    fn from(value: StdRange<T>) -> Self {
        MauSampleRange::SampleRange { start: value.start, end: value.end }
    }
}

impl<T: SampleUniform + PartialOrd + Copy> From<StdRangeInclusive<T>> for MauSampleRange<T> {
    fn from(value: StdRangeInclusive<T>) -> Self {
        let start = value.start();
        let end = value.end();
        MauSampleRange::SampleRangeInclusive { start: *start, end: *end }
    }
}

impl<T> SampleRange<T> for MauSampleRange<T> where T: SampleUniform + PartialOrd + Sized {
    #[inline]
    fn sample_single<R: RngCore + ?Sized>(self, rng: &mut R) -> T {
        match self {
            MauSampleRange::SampleRange { start, end } => T::Sampler::sample_single(start, end, rng),
            MauSampleRange::SampleRangeInclusive { start, end } => T::Sampler::sample_single(start, end, rng),
        }
    }

    fn is_empty(&self) -> bool {
        match self {
            MauSampleRange::SampleRange { start, end } => !(start < end),
            MauSampleRange::SampleRangeInclusive { start, end } => !(start <= end),
        }
    }
}

pub enum RandomTarget<T: SampleUniform + PartialOrd> {
    Single(T),
    Range(MauSampleRange<T>),
    None,
}

impl<T: SampleUniform + PartialOrd> RandomTarget<T> {
    pub fn single(value: T) -> RandomTarget<T> {
        RandomTarget::Single(value)
    }

    pub fn range<R>(range: R) -> RandomTarget<T> where R: SampleRange<T>, MauSampleRange<T>: From<R> {
        RandomTarget::Range(MauSampleRange::from(range))
    }

    pub fn none() -> RandomTarget<T> {
        RandomTarget::None
    }
}