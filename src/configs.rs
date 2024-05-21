use std::env;
use std::error::Error;
use std::str::FromStr;
use anyhow::anyhow;
use config::builder::DefaultState;
use either::{Left, Right};
use serde_test::Configure;
use strum::AsRefStr;
use crate::errors::{IntoAnyhowError};

#[derive(Debug, Default)]
pub enum ConfigPath<'l> {
    #[default]
    Ignore,
    Optional(&'l str),
    Required(&'l str),
}

#[derive(Debug, Default)]
pub struct ConfigBuilderOptions<'l> {
    pub env_prefix: Option<&'l str>,
    pub default_file_path: ConfigPath<'l>,
    pub run_mode_path_format: ConfigPath<'l>,
    pub local_file_path: ConfigPath<'l>,
}

pub trait IntoRunMode: Default + FromStr + AsRef<str> {
    type Err: Into<anyhow::Error>;

    fn run_mode(s: &str) -> Result<Self, <Self as IntoRunMode>::Err>;
    fn try_run_mode<T: Into<anyhow::Error>>(res: Result<String, T>) -> anyhow::Result<Self> {
        res.map_anyhow().and_then(|s| Self::run_mode(&s).map_anyhow())
    }
}

impl<T> IntoRunMode for T where T: Default, T: FromStr, T: AsRef<str>, T::Err: Into<anyhow::Error> {
    type Err = T::Err;

    fn run_mode(s: &str) -> Result<Self, T::Err> {
        T::from_str(s)
    }
}


pub fn run_mode<T: IntoRunMode>() -> anyhow::Result<T> {
    T::try_run_mode(env::var("RUN_MODE")).map_anyhow()
}

#[derive(Debug, PartialEq, AsRefStr, strum::Display, strum::EnumString)]
#[strum(ascii_case_insensitive, serialize_all = "lowercase")]
pub enum BasicRunMode {
    Development,
    Prodution
}

pub fn build_config<T : IntoRunMode>(ConfigBuilderOptions { env_prefix, default_file_path, run_mode_path_format, local_file_path }: ConfigBuilderOptions) -> config::ConfigBuilder<DefaultState> {
    let mut builder = config::Config::builder();
    match default_file_path {
        ConfigPath::Ignore => {}
        ConfigPath::Optional(path) => builder = builder.add_source(config::File::with_name(path).required(false)),
        ConfigPath::Required(path) => builder = builder.add_source(config::File::with_name(path).required(true)),
    }

    // match run_mode_path_format {
    //     ConfigPath::Ignore => {}
    //     ConfigPath::Optional(path) => builder = builder.add_source(config::File::with_name(format!("{}{}", path, run_mode::<T>().map(|v| v).unwrap_or("development"))).required(false)),
    //     ConfigPath::Required(path) => builder = builder.add_source(config::File::with_name(format!("{}{}", path, run_mode::<T>().unwrap_or("development"))).required(true)),
    // }

    match local_file_path {
        ConfigPath::Ignore => {}
        ConfigPath::Optional(path) => builder = builder.add_source(config::File::with_name(path).required(false)),
        ConfigPath::Required(path) => builder = builder.add_source(config::File::with_name(path).required(true)),
    }

    if let Some(env_prefix) = env_prefix {
        builder = builder.add_source(config::Environment::with_prefix(env_prefix))
    }

    return builder;
}