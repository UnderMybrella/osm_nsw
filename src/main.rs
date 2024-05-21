#![feature(assert_matches)]

use std::{env, fs};
use std::cmp::PartialEq;
use std::fs::File;
use std::io::{BufWriter, IsTerminal, stderr, stdout};
use std::path::Path;
use dotenvy::dotenv;
use zip::ZipArchive;
use crate::gtfs::gtfs_schedule::{GtfsScheduleStop, GtfsScheduleStopTime};
use crate::transport_nswapi::TransportNswApiClient;

mod transport_nswapi;
mod osm_api_client;
mod gtfs;

#[macro_use]
pub mod macros;
mod errors;
mod configs;
mod rnd;
mod tests;

use std::io::Write;
use std::num::{IntErrorKind, ParseIntError};
use std::str::FromStr;
use anyhow::bail;
use chrono::{NaiveTime, TimeDelta};
use config::builder::DefaultState;
use config::Config;
use indicatif::{ProgressBar, ProgressDrawTarget, ProgressIterator, ProgressStyle};
use log::debug;
use rand::{Rng, thread_rng};
use rand::distributions::uniform::SampleRange;
use rand::rngs::ThreadRng;
use serde::Deserialize;
use crate::configs::{build_config, ConfigBuilderOptions};
use crate::configs::ConfigPath::Optional;
use crate::gtfs::gtfs_chrono::GtfsLexingError::ParseInt;
use crate::gtfs::gtfs_types::GtfsTime;
use crate::rnd::RandomTarget;

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
    // let log4rs_config = build_config(ConfigBuilderOptions {
    //     env_prefix: Some("LOG4RS"),
    //     default_file_path: Optional("log4rs"),
    //     ..Default::default()
    // });
    //
    // log4rs::init_raw_config(log4rs_config.build()?.try_deserialize()?)?;

    debug!("Stdout: {:?}", stdout());
    debug!("Stderr: {:?}", stderr());

    let settings: TransportNswConfig = Config::builder()
        .add_source(config::Environment::with_prefix("TRANSPORT_NSW"))
        .add_source(config::File::with_name("transport_nsw"))
        .build()?
        .try_deserialize()?;

    // let client = TransportNswApiClient::new(env::var("TRANSPORT_NSW_API_KEY")?)?;
    // let get_complete = client.timetables().get_complete_gtfs().await?;
    // println!("{get_complete:?}");

    let mut schedule = ZipArchive::new(File::open("full_greater_sydney_gtfs_static_0.zip")?)?;

    let (suburb_name, suburb_lat, suburb_lon) = match settings.target_suburb {
        TransportNswTargetSuburb::Static { name, min_latitude, max_latitude, min_longitude, max_longitude } => (name, min_latitude..=max_latitude, min_longitude..=max_longitude)
    };

    // let mut output = BufWriter::new(File::create("stops_out.txt")?);
    let mut i = 0;

    eprintln!("Testing!");

    let mut suburb_stops_vec = Vec::new();

    macro_rules! make_csv_reader {
        ($name:tt) => {csv::ReaderBuilder::new().from_reader(schedule.by_name($name)?)};
    }

    {
        let mut stops_reader = make_csv_reader!("stops.txt");
        let mut suburb_stops_out = csv::WriterBuilder::new()
            .double_quote(true)
            .from_writer(BufWriter::new(File::create(format!("stops_{}.csv", suburb_name))?));

        // Loop over each record.
        for result in stops_reader.deserialize() {
            let record: GtfsScheduleStop = result?;
            if let Some(lat) = record.stop_latitude {
                if let Some(lon) = record.stop_longitude {
                    if suburb_lat.contains(&lat) && suburb_lon.contains(&lon) {
                        i += 1;
                        print!("\rRecord {i}");
                        suburb_stops_out.serialize(&record)?;
                        suburb_stops_vec.push(record);
                    }
                }
            }
        }

        suburb_stops_out.flush()?;
    }

    {
        println!("Getting stop times count");
        let stop_times_count = make_csv_reader!("stop_times.txt").records().count();

        let bar = ProgressBar::new(stop_times_count as u64)
            .with_style(ProgressStyle::with_template("[{elapsed_precise}] {bar:40.cyan/blue} {pos:>7}/{len:7} {msg}")?);

        bar.set_draw_target(ProgressDrawTarget::stdout());

        println!("Making reader");
        let mut stop_times_reader = make_csv_reader!("stop_times.txt");

        let mut suburb_stop_times_out = csv::WriterBuilder::new()
            .double_quote(true)
            .from_writer(BufWriter::new(File::create(format!("stop_times_{}.csv", suburb_name))?));
        i = 0;
        let mut s = 0;

        println!("{}", std::io::stdout().is_terminal());

        println!("Running!!");
        for_iter!(result in stop_times_reader.deserialize().progress_with(bar), |iter| {
            let record: GtfsScheduleStopTime = result?;
            if let Some(ref stop_id) = record.stop_id {
                if suburb_stops_vec.iter().any(|stop| &stop.stop_id == stop_id) {
                    i += 1;
                    suburb_stop_times_out.serialize(&record)?;
                    iter.progress.set_message(format!("Found {i} stop times in {suburb_name}"))
                }
            }

            s += 1;

            if s % 100 == 0 {
                suburb_stop_times_out.flush()?
            }
        });

        suburb_stop_times_out.flush()?
    }

    Ok(())
}