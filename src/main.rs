mod transport_nswapi;
mod osm_api_client;
mod gtfs;

#[macro_use]
pub mod macros;

use std::collections::HashMap;
use std::f64::consts::PI;
use proj::Proj;
use reqwest::Client;
use crate::gtfs::gtfs_types::ColourCode;



pub fn convert_epsg3857to_epsg4326((lat, lng): (f64, f64)) -> (f64, f64) {
    const X: f64 = 20037508.34;

    let lng3857 = (lng * X) / 180.0;

    let mut lat3857 = lat + 90.0;
    lat3857 = lat3857 * (PI / 360.0);
    lat3857 = lat3857.tan();
    lat3857 = lat3857.ln();
    lat3857 = lat3857 / (PI / 180.0);

    lat3857 = (lat3857 * X) / 180.0;

    return (lat3857, lng3857);
}

// fn convertEPSG4326ToEPSG3857((lat, long): (f64, f64)) -> (f64, f64) {
//     const X: f64 = 20037508.34;
//     //converting the logitute from epsg 3857 to 4326
//     const lng = (lat3857*180)/X;
//
// //converting the latitude from epsg 3857 to 4326 split in multiple lines for readability
//
//     let lat = lat/(X / 180)
//     const exponent = (Math.PI / 180) * lat4326
//
//     lat4326 = Math.atan(e ** exponent)
//     lat4326 = lat4326 / (Math.PI / 360)
//     lat4326 = lat4326 - 90
// }


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // let client = Client::builder()
    //     .build()?;
    //
    // let _ = client.get("")
    //     .send()
    //     .await?
    //
    // let resp = reqwest::get("https://httpbin.org/ip")
    //     .await?
    //     .json::<HashMap<String, String>>()
    //     .await?;
    // println!("{resp:#?}");
    //
    //
    // let lon_in_epsg4326 = 150.7459255;
    // let lat_in_epsg4326 = -33.7468897;
    //
    // let (lat, lng) = convert_epsg3857to_epsg4326((lat_in_epsg4326, lon_in_epsg4326));
    //
    // println!("{0},{1}", lat, lng);

    // let from = "EPSG:4326";
    // let to = "EPSG:3857";
    // let ft_to_m = Proj::new_known_crs(&from, &to, None).unwrap();
    // let result = ft_to_m
    //     .convert((150.7459255, -33.7468897))
    //     .unwrap();

    Ok(())
}