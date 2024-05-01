use std::sync::Arc;
use std::time::Duration;

use governor::{DefaultDirectRateLimiter, Jitter, Quota, RateLimiter};
use nonzero_ext::nonzero;
use reqwest::{Client, IntoUrl, Response, Url};
use reqwest::header::{HeaderMap, HeaderValue};

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

        println!("{api_base:?}");

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

    fn get_complete_gtfs_endpoint(&self) -> anyhow::Result<Url> {
        let endpoint = self.endpoint()?;
        println!("{endpoint:?}");
        endpoint.join("complete/gtfs").map_err(|e| e.into())
    }

    pub async fn get_complete_gtfs_head(&self) -> anyhow::Result<Response> {
        let endpoint = self.get_complete_gtfs_endpoint()?;

        // I'm not sure if HEAD requests count against the limit, but we'll do it just in case
        self.rate_limiter()
            .until_ready_with_jitter(Jitter::up_to(Duration::from_secs(1u64)))
            .await;

        self.client().head(endpoint).send().await.map_err(|e| e.into())
    }

    // pub async fn get_complete_gtfs(&self) -> anyhow::Result<ZipFile> {
    //     let endpoint = self.endpoint()?.join("/complete/gtfs")?;
    //
    //     let response = self.0.client.get()
    // }
}