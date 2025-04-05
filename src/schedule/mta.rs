#![cfg(feature = "zip")]

use std::{collections::HashMap, io::Read};

use zip::ZipArchive;

/// This module has a stripped down, type-enforced version of the spec that's even more catered to
/// the current MTA format, as of April 2025
const SUNDAY: u32 = 0;

pub struct SubwaySchedule {
    routes: HashMap<String, SubwayRoute>,
    shapes: HashMap<String, SubwayShape>,
    stops: HashMap<String, SubwayStop>,
    services: HashMap<String, SubwayService>,
}

impl<R> TryFrom<ZipArchive<R>> for SubwaySchedule
where
    R: Read,
{
    type Error = String;

    fn try_from(value: ZipArchive<R>) -> Result<Self, Self::Error> {
        // Parse stops
        unimplemented!()
    }
}

pub enum SubwayService {
    ServiceNormal(SubwayServiceNormal),
    ServiceException(SubwayServiceException),
}

pub struct SubwayServiceNormal {
    start_date: String,
    end_date: String,
    days_active: [bool; 8],
    exceptions: Vec<SubwayServiceException>,
}

pub struct SubwayServiceException {
    date: String,
    is_added: bool,
}

pub struct SubwayRoute {
    trips: HashMap<String, SubwayTrip>,
}

pub struct SubwayTrip {
    headsign: Option<String>,
    shape_id: Option<String>,
    stop_times: HashMap<u32, SubwayStopTime>,
}

pub struct SubwayStopTime {
    stop_id: String,
    arrival_time: String,
    departure_time: String,
    stop_seqence: u32,
}

pub struct SubwayShape {
    shape_id: String,
    latlons: Vec<(String, String)>,
}

pub struct SubwayStop {
    stop_id: String,
    name: String,
    lat: String,
    lon: String,
    parent_stop_id: Option<String>,
    transfers_from: Vec<SubwayTransfer>,
}

pub struct SubwayTransfer {
    to_stop_id: String,
    min_transfer_time: u32,
}
