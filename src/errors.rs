use std::fmt::Display;
use std::io::Error;
use anyhow::anyhow;
use either::{Either, for_both};

pub trait IntoAnyhowError<T> {
    fn map_anyhow(self) -> anyhow::Result<T>;
}

impl<T, E: Into<anyhow::Error>> IntoAnyhowError<T> for core::result::Result<T, E> {
    fn map_anyhow(self) -> anyhow::Result<T> {
        self.map_err(|e| e.into())
    }
}

pub trait IntoStdIOError<T> {
    fn map_err_std_io(self) -> core::result::Result<T, std::io::Error>;
}

impl<T, E: Into<Box<dyn std::error::Error + Send + Sync>>> IntoStdIOError<T> for core::result::Result<T, E> {
    fn map_err_std_io(self) -> Result<T, Error> {
        self.map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))
    }
}