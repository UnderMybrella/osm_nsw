use std::sync::Arc;
use std::time::Duration;
use futures::TryStreamExt;

use governor::{DefaultDirectRateLimiter, Jitter, Quota, RateLimiter};
use nonzero_ext::nonzero;
use reqwest::{Client, IntoUrl, Response, Url};
use reqwest::header::{ETAG, HeaderMap, HeaderValue, LAST_MODIFIED};
use tokio::io::BufReader;
use tokio_util::io::{StreamReader, SyncIoBridge};
use futures::StreamExt;
use tempfile::NamedTempFile;
use zip::ZipArchive;
use crate::errors::{IntoAnyhowError, IntoStdIOError};
use crate::osm_api_client::{ResourceWithValidity, TryToString};
use crate::osm_api_client::ResourceWithValidity::{ETag, ETagAndModification, LastModified, Missing};


pub struct TransportNswApiClient {
    api_base: Url,
    client: Arc<reqwest::Client>,
    rate_limiter: Arc<DefaultDirectRateLimiter>,
}

impl TransportNswApiClient {
    pub fn new<S: AsRef<str>>(key: S) -> anyhow::Result<TransportNswApiClient> {
        Self::with_api_base("https://api.transport.nsw.gov.au/v1/", key)
    }

    pub fn with_api_base<T: IntoUrl, S: AsRef<str>>(api_base: T, key: S) -> anyhow::Result<TransportNswApiClient> {
        let api_base = api_base.into_url()?;
        let key = key.as_ref();

        let mut headers = HeaderMap::new();
        headers.insert(reqwest::header::AUTHORIZATION, HeaderValue::try_from(format!("apikey {key}"))?);

        // set default client operation
        let client = Arc::new(reqwest::Client::builder().default_headers(headers).build()?);
        let rate_limiter = Arc::new(RateLimiter::direct(
            Quota::per_hour(nonzero!(2500u32))
                .allow_burst(nonzero!(5u32))
        ));

        Ok(TransportNswApiClient { api_base, client, rate_limiter })
    }

    pub fn timetables(&self) -> TransportNswTimetablesEndpoint {
        TransportNswTimetablesEndpoint(self)
    }
}

pub struct TransportNswTimetablesEndpoint<'c>(&'c TransportNswApiClient);

impl<'c> TransportNswTimetablesEndpoint<'c> {
    #[inline]
    fn client(&self) -> &Arc<Client> {
        &self.0.client
    }

    #[inline]
    fn rate_limiter(&self) -> &Arc<DefaultDirectRateLimiter> {
        &self.0.rate_limiter
    }

    fn endpoint(&self) -> anyhow::Result<Url> {
        self.0.api_base.join("publictransport/timetables/").map_err(|e| e.into())
    }

    async fn until_ready(&self) {
        self.rate_limiter().until_ready_with_jitter(Jitter::up_to(Duration::from_secs(1u64))).await;
    }

    fn get_complete_gtfs_endpoint(&self) -> anyhow::Result<Url> {
        self.endpoint()?.join("complete/gtfs").map_anyhow()
    }

    pub async fn get_complete_gtfs_head(&self) -> anyhow::Result<Response> {
        let endpoint = self.get_complete_gtfs_endpoint()?;

        // I'm not sure if HEAD requests count against the limit, but we'll do it just in case
        self.until_ready().await;

        self.client().head(endpoint).send().await.map_anyhow()
    }

    pub async fn get_complete_gtfs(&self) -> anyhow::Result<ResourceWithValidity<ZipArchive<NamedTempFile>>> {
        let endpoint = self.get_complete_gtfs_endpoint()?;

        self.until_ready().await;

        let response = self.client()
            .get(endpoint)
            .send()
            .await
            .and_then(|r| r.error_for_status())
            .map_anyhow()?;

        let headers = response.headers();
        let etag = headers.get(ETAG).and_then(|v| v.try_to_string().ok());
        let last_modified = headers.get(LAST_MODIFIED).and_then(|v| v.try_to_string().ok());

        let read = StreamReader::new(response.bytes_stream().map(|result| result.map_err_std_io()));
        let mut reader = BufReader::new(read);
        let mut file = NamedTempFile::new()?;
        let mut tmp_file = tokio::fs::File::from(file.reopen()?);
        tokio::io::copy(&mut reader, &mut tmp_file).await?;

        let mut zip = ZipArchive::new(file)?;

        for i in 0..zip.len() {
            let mut file = zip.by_index(i)?;
            println!("Filename: {}", file.name());
        }

        if let Some(etag) = etag {
            if let Some(last_modified) = last_modified {
                Ok(ETagAndModification { value: zip, etag, last_modified })
            } else {
                Ok(ETag((zip, etag)))
            }
        } else if let Some(last_modified) = last_modified {
            Ok(LastModified((zip, last_modified)))
        } else {
            Ok(Missing(zip))
        }
    }
}