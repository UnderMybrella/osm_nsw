use std::{env, fs};
use std::fs::File;
use std::io::BufWriter;
use std::path::Path;
use dotenvy::dotenv;
use zip::ZipArchive;
use crate::gtfs::gtfs_schedule::GTFSStop;
use crate::transport_nswapi::TransportNswApiClient;

mod transport_nswapi;
mod osm_api_client;
mod gtfs;

#[macro_use]
pub mod macros;
mod errors;

use std::io::Write;
use chrono::{NaiveTime, TimeDelta};
use config::Config;
use serde::Deserialize;
use crate::gtfs::gtfs_types::GtfsTime;

#[derive(Debug, Deserialize)]
struct TransportNswConfig {
    api_key: String,
    target_suburb: TransportNswTargetSuburb,
}

#[derive(Debug, Deserialize)]
enum TransportNswTargetSuburb {
    Static {
        name: String,
        min_latitude: f64,
        max_latitude: f64,
        min_longitude: f64,
        max_longitude: f64,
    }
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let settings: TransportNswConfig = Config::builder()
        .add_source(config::Environment::with_prefix("TRANSPORT_NSW"))
        .add_source(config::File::with_name("transport_nsw"))
        .build()?
        .try_deserialize()?;

    // let client = TransportNswApiClient::new(env::var("TRANSPORT_NSW_API_KEY")?)?;
    // let get_complete = client.timetables().get_complete_gtfs().await?;
    // println!("{get_complete:?}");

    let mut schedule = ZipArchive::new(File::open("full_greater_sydney_gtfs_static_0.zip")?)?;
    let mut stops_reader = csv::ReaderBuilder::new()
        .from_reader(schedule.by_name("stops.txt")?);
    let mut stop_times_reader = csv::ReaderBuilder::new()
        .from_reader(schedule.by_name("stop_times.txt")?);

    let (suburb_name, suburb_lat, suburb_lon) = match settings.target_suburb {
        TransportNswTargetSuburb::Static { name, min_latitude, max_latitude, min_longitude, max_longitude } => (name, min_latitude..=max_latitude, min_longitude..=max_longitude)
    };

    // let mut output = BufWriter::new(File::create("stops_out.txt")?);
    let mut suburb_stops = csv::WriterBuilder::new()
        .double_quote(true)
        .from_writer(File::create(format!("stops_{}.csv", suburb_name))?);
    let mut i = 0;

    // Loop over each record.
    for result in stops_reader.deserialize() {
        let record: GTFSStop = result?;
        if let Some(lat) = record.stop_latitude {
            if let Some(lon) = record.stop_longitude {
                if suburb_lat.contains(&lat) && suburb_lon.contains(&lon) {
                    i += 1;
                    print!("\rRecord {i}");
                    suburb_stops.serialize(record)?;
                }
            }
        }
    }

    Ok(())
}