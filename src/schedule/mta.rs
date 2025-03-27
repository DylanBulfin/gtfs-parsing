// This file is for parsing the data into a format specifically useful for the MTA Subway.
// E.g. If the information is not provided in either the current regular or supplemented
// file it's ignored, and the organization of entities is a bit more basic.
// It's very brittle and could be interrupted by minor changes to their csv so, yk.

use super::{
    agency::Agency,
    calendar::{Activity, ExceptionType, Service, ServiceException},
    routes::RouteType,
    shapes::Shape,
    stops::LocationType,
    transfers::TransferType,
    trips::DirectionType,
};

pub struct MTAgency {
    pub agency_id: String,
    pub agency_name: String,
    pub agency_url: String,
    pub agency_timezone: String,
    pub agency_lang: String,
    pub agency_phone: String,
}

impl TryFrom<Agency> for MTAgency {
    type Error = String;

    fn try_from(value: Agency) -> Result<Self, Self::Error> {
        Ok(Self {
            agency_id: value.agency_id.ok_or("agency_id cannot be empty")?,
            agency_name: value.agency_name,
            agency_url: value.agency_url,
            agency_timezone: value.agency_timezone,
            agency_lang: value.agency_lang.ok_or("agency_lang cannot be empty")?,
            agency_phone: value.agency_phone.ok_or("agency_phone cannot be empty")?,
        })
    }
}

pub struct SubwayRoute<'sc> {
    pub route_id: String,
    pub agency_id: String,
    pub route_short_name: String,
    pub route_long_name: String,
    pub route_desc: String,
    pub route_type: RouteType,
    pub route_url: String,
    pub route_color: String,
    pub route_text_color: String,

    pub agency: &'sc Agency,
}

pub struct SubwayTrip<'sc> {
    pub trip_id: String,
    //pub route_id: String,
    //pub service_id: String,
    pub trip_headsign: String,
    pub direction_id: DirectionType,
    //pub shape_id: String,
    pub shape: &'sc Shape,
    pub service: &'sc Service,
    pub route: &'sc SubwayRoute<'sc>,
}

pub struct SubwayTransferRule<'sc> {
    //pub from_stop_id: String,
    //pub to_stop_id: String,
    pub min_transfer_time: u32,

    pub from_stop: &'sc SubwayStation<'sc>,
    pub to_stop: &'sc SubwayStation<'sc>,
}

pub struct SubwayStation<'sc> {
    pub station_id: String,
    pub stop_name: String,
    pub stop_lat: String,
    pub stop_lon: String,

    pub uptown_platform: SubwayPlatform<'sc>,
    pub downtown_platform: SubwayPlatform<'sc>,
}

pub struct SubwayPlatform<'sc> {
    pub stop_id: String,
    pub parent: &'sc SubwayStation<'sc>,
}

pub struct SubwayStopTime<'sc> {
    pub trip_id: String,
    pub arrival_time: String,
    pub departure_time: String,
    //pub stop_id: String,
    pub stop_sequence: u32,

    pub stop: &'sc SubwayStation<'sc>,
}

// The others are used as-is
use Service as SubwayService;
use ServiceException as SubwayServiceException;
use Shape as SubwayShape;
