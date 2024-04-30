use reqwest::{Client, IntoUrl, RequestBuilder, Response};

pub trait TransportNSWApi {
    fn tfnswGetCompleteGTFSTimetables(self) -> reqwest::RequestBuilder;
}

impl TransportNSWApi for &reqwest::Client {
    fn tfnswGetCompleteGTFSTimetables(self) -> RequestBuilder {
        self.get("https://api.transport.nsw.gov.au/v1/publictransport/timetables/complete")
    }
}