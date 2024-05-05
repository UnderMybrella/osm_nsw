use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};
use url::Url;
use crate::gtfs::gtfs_schedule::GtfsStopLocationType::Stop;
use crate::gtfs::gtfs_schedule::GtfsWheelchairBoarding::Unknown;
use crate::gtfs::gtfs_types::GtfsID;

#[derive(Serialize, Deserialize, Debug)]
pub struct GTFSAgency {
    agency_id: String
}
#[derive(Serialize, Deserialize, Debug)]
pub struct GTFSStop {
    pub stop_id: GtfsID,
    pub stop_code: Option<String>,
    pub stop_name: Option<String>,
    pub tts_stop_name: Option<String>,
    pub stop_desc: Option<String>,
    #[serde(rename = "stop_lat")]
    pub stop_latitude: Option<f64>,
    #[serde(rename = "stop_lon")]
    pub stop_longitude: Option<f64>,
    pub zone_id: Option<GtfsID>,
    pub stop_url: Option<Url>,
    pub location_type: Option<GtfsStopLocationType>,
    pub parent_station: Option<GtfsID>,
    pub stop_timezone: Option<String>,
    pub wheelchair_boarding: Option<GtfsWheelchairBoarding>,
    pub level_id: Option<GtfsID>,
    pub platform_code: Option<String>
}

// #[derive(Serialize, Deserialize, Debug)]
// pub struct GtfsStopTime {
//     pub trip_id: GtfsID,
//     pub arrival_time:
// }

#[repr(u8)]
#[derive(Serialize_repr, Deserialize_repr, PartialEq, Debug)]
pub enum GtfsStopLocationType {
    Stop = 0,
    Station = 1,
    Entrance = 2,
    GenericNode = 3,
    BoardingArea = 4
}

#[repr(u8)]
#[derive(Serialize_repr, Deserialize_repr, PartialEq, Debug)]
pub enum GtfsWheelchairBoarding {
    Unknown = 0,
    Some = 1,
    None = 2
}