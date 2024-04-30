// use std::sync::Arc;
// use reqwest::header::{HeaderMap, HeaderValue};
// use reqwest::Url;
//
// pub struct OsmApiClient {
//     api_base: Url,
//     client: Arc<reqwest::Client>,
// }
//
// impl OsmApiClient {
//     pub fn new(domain: &str, key: &str) -> Result<OsmApiClient, dyn std::error::Error> {
//         let api_base = Url::parse(format!("https://{domain}/api/").as_str())?;
//
//         let mut headers = HeaderMap::new();
//         headers.insert("x-key-header", HeaderValue::from_str(key)?);
//
//         // set default client operation
//         let client = Arc::new(reqwest::Client::builder().default_headers(headers).build()?);
//
//         Ok(OsmApiClient { api_base, client,})
//     }
//
//     pub fn users(&self) -> UsersEndpoint {
//         UsersEndpoint(self)
//     }
// }
//
// pub struct UsersEndpoint<'c>(&'c OsmApiClient);
//
// impl<'c> UsersEndpoint<'c> {
//     fn endpoint(&self) -> Result<Url, dyn std::error::Error> {
//         Ok(self.0.api_base.join("users/")?)
//     }
//
//     pub async fn get_by_id(&self, id: &str) -> Result<String, dyn std::error::Error> {
//         let endpoint = self.endpoint()?.join(id)?;
//
//         Ok(self.0.client.get(endpoint).send().await?.text().await?)
//     }
// }
//
// // Make sure the lifetimes work when using it
// async fn check() {
//     let client = OsmApiClient::new("example.com", "notakey").unwrap();
//
//     client.users().get_by_id("100").await.unwrap();
// }