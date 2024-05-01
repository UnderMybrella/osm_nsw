use std::env;
use dotenvy::dotenv;
use crate::transport_nswapi::TransportNswApiClient;

mod transport_nswapi;
mod osm_api_client;
mod gtfs;

#[macro_use]
pub mod macros;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // load environment variables from .env file
    dotenv().expect(".env file not found");

    let client = TransportNswApiClient::new(env::var("TRANSPORT_NSW_API_KEY")?)?;
    let get_complete = client.timetables().get_complete_gtfs_head().await?;
    println!("{get_complete:?}");

    Ok(())
}