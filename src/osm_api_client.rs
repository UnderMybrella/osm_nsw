use std::net::SocketAddr;
use reqwest::header::{HeaderMap, HeaderValue, ToStrError};
use reqwest::StatusCode;
use reqwest::tls::Version;
use tempfile::NamedTempFile;
use url::Url;
use zip::ZipArchive;
use crate::transport_nswapi::TransportNswApiClient;

pub struct OsmRouter {
    transport_nsw: TransportNswApiClient,
}

#[derive(Debug)]
pub enum ResourceWithValidity<T> {
    ETagAndModification {
        value: T,
        etag: String,
        last_modified: String
    },
    ETag((T, String)),
    LastModified((T, String)),
    Missing(T),
}

pub trait TryToString {
    type Error;

    /// Performs the conversion.
    fn try_to_string(self) -> Result<String, Self::Error>;
}

impl TryToString for &HeaderValue {
    type Error = ToStrError;

    fn try_to_string(self) -> Result<String, Self::Error> {
        self.to_str().map(|v| v.to_owned())
    }
}