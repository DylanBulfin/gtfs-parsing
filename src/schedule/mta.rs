// This file is for parsing the data into a format specifically useful for the MTA Subway.
// E.g. If the information is not provided in either the current regular or supplemented
// file it's ignored, and the organization of entities is a bit more basic.
// It's very brittle and could be interrupted by minor changes to their csv so, yk.

use super::{
    Schedule,
    agency::Agency,
    calendar::{Activity, ExceptionType, Service, ServiceException},
    routes::{Route, RouteType},
    shapes::Shape,
    stop_times::StopTime,
    stops::{LocationType, Stop},
    transfers::{Transfer, TransferType},
    trips::{DirectionType, Trip},
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

pub struct SubwayRoute {
    pub route_id: String,
    pub agency_id: String,
    pub route_short_name: String,
    pub route_long_name: String,
    pub route_desc: String,
    pub route_type: RouteType,
    pub route_url: String,
    pub route_color: Option<String>,
    pub route_text_color: Option<String>,
}

impl SubwayRoute {
    fn try_create(value: Route) -> Result<Self, String> {
        Ok(Self {
            route_id: value.route_id,
            agency_id: value.agency_id.ok_or("agency_id cannot be empty")?,
            route_short_name: value
                .route_short_name
                .ok_or("route_short_name cannot be empty")?,
            route_long_name: value
                .route_long_name
                .ok_or("route_long_name cannot be empty")?,
            route_desc: value.route_desc.ok_or("route_desc cannot be empty")?,
            route_type: value.route_type,
            route_url: value.route_url.ok_or("route_url cannot be empty")?,
            route_color: value.route_color,
            route_text_color: value.route_text_color,
        })
    }
}

pub struct SubwayTrip {
    pub trip_id: String,
    pub route_id: String,
    pub service_id: String,
    pub trip_headsign: String,
    pub direction_id: DirectionType,
    pub shape_id: Option<String>,
}

impl SubwayTrip {
    fn try_create(value: Trip) -> Result<Self, String> {
        Ok(Self {
            trip_id: value.trip_id,
            trip_headsign: value.trip_headsign.ok_or("trip_headsign cannot be empty")?,
            direction_id: value.direction_id.ok_or("direction_type cannot be empty")?,
            route_id: value.route_id,
            service_id: value.service_id,
            shape_id: value.shape_id,
        })
    }
}

pub struct SubwayTransferRule {
    pub from_stop_id: String,
    pub to_stop_id: String,
    pub min_transfer_time: u32,
}

impl SubwayTransferRule {
    fn try_create(value: Transfer) -> Result<Self, String> {
        Ok(Self {
            min_transfer_time: value
                .min_transfer_time
                .ok_or("min_transfer_time cannot be empty")?,
            from_stop_id: value.from_stop_id.ok_or("from_stop_id cannot be empty")?,
            to_stop_id: value.to_stop_id.ok_or("to_stop_id cannot be empty")?,
        })
    }
}

pub struct SubwayStation {
    pub station_id: String,
    pub stop_name: String,
    pub stop_lat: String,
    pub stop_lon: String,

    pub uptown_platform_id: String,
    pub downtown_platform_id: String,
}

impl SubwayStation {
    fn try_create(parent: Stop, uptown: Stop, downtown: Stop) -> Result<Self, String> {
        let Stop {
            stop_id,
            stop_name,
            stop_lat,
            stop_lon,
            ..
        } = parent;

        Ok(Self {
            station_id: stop_id,
            stop_name: stop_name.ok_or("stop_name cannot be empty")?,
            stop_lat: stop_lat.ok_or("stop_lat cannot be empty")?,
            stop_lon: stop_lon.ok_or("stop_lon cannot be empty")?,

            uptown_platform_id: uptown.stop_id,
            downtown_platform_id: downtown.stop_id,
        })
    }
}

pub struct SubwayStopTime {
    pub trip_id: String,
    pub arrival_time: String,
    pub departure_time: String,
    pub station_id: String,
    pub stop_sequence: u32,

    pub uptown: bool, // For each station the MTA format defines a parent station, and two
                      // platforms (N and S). This determines which platform this stoptime
                      // uses
}

impl SubwayStopTime {
    fn try_create(value: StopTime, station_id: String, uptown: bool) -> Result<Self, String> {
        Ok(Self {
            trip_id: value.trip_id,
            station_id,
            arrival_time: value.arrival_time.ok_or("arrival_time cannot be empty")?,
            departure_time: value
                .departure_time
                .ok_or("departure_time cannot be empty")?,
            stop_sequence: value.stop_sequence.ok_or("stop_sequence cannot be empty")?,

            uptown,
        })
    }
}

// The others are used as-is
use Service as SubwayService;
use ServiceException as SubwayServiceException;
use Shape as SubwayShape;

pub struct SubwaySchedule {
    agency: MTAgency,
    routes: Vec<SubwayRoute>,
    trips: Vec<SubwayTrip>,
    transfers: Vec<SubwayTransferRule>,
    stations: Vec<SubwayStation>,
    stop_times: Vec<SubwayStopTime>,
    services: Vec<SubwayService>,
    service_exceptions: Vec<SubwayServiceException>,
    shapes: Vec<SubwayShape>,
}

impl TryFrom<Schedule> for SubwaySchedule {
    type Error = String;

    fn try_from(value: Schedule) -> Result<Self, Self::Error> {
        let Schedule {
            mut agencies,
            routes: base_routes,
            trips: mut base_trips,
            transfers: mut base_transfers,
            stops: mut base_stops,
            stop_times: mut base_stop_times,
            shapes,
            services,
            service_exceptions,
        } = value;

        assert_eq!(agencies.len(), 1);
        let agency = MTAgency::try_from(agencies.pop().unwrap())?;

        let mut schedule = Self {
            agency,
            routes: Vec::new(),
            trips: Vec::new(),
            transfers: Vec::new(),
            stations: Vec::new(),
            stop_times: Vec::new(),
            services: Vec::new(),
            service_exceptions: Vec::new(),
            shapes: Vec::new(),
        };

        //let mut routes: Vec<SubwayRoute> = Vec::new();
        for route in base_routes {
            schedule.routes.push(SubwayRoute::try_create(route)?);
        }

        for trip in base_trips {
            schedule.trips.push(SubwayTrip::try_create(trip)?);
        }

        assert_eq!(base_stops.len() % 3, 0);
        while base_stops.len() >= 3 {
            // TODO This relies on a specific order for the stops, probably want to change
            let (parent, uptown, downtown) = (
                base_stops.pop().unwrap(),
                base_stops.pop().unwrap(),
                base_stops.pop().unwrap(),
            );

            schedule
                .stations
                .push(SubwayStation::try_create(parent, uptown, downtown)?);
        }
        assert_eq!(base_stops.len(), 0);

        for stop_time in base_stop_times {
            let platform_id = stop_time.stop_id.clone().ok_or("stop_id cannot be empty")?;
            assert!(platform_id.is_ascii());

            let uptown = match platform_id.as_bytes()[platform_id.len() - 1] {
                b'N' => true,
                b'S' => false,
                c => panic!("Unexpected character at end of stop: {}", c),
            };

            schedule.stop_times.push(SubwayStopTime::try_create(
                stop_time,
                platform_id[0..platform_id.len() - 1].to_owned(),
                uptown,
            )?)
        }

        for transfer in base_transfers {
            schedule
                .transfers
                .push(SubwayTransferRule::try_create(transfer)?);
        }

        Ok(schedule)
    }
}

#[cfg(test)]
mod tests {
    use crate::schedule::Schedule;

    use super::SubwaySchedule;

    #[test]
    fn test_basics() -> Result<(), String> {
        let schedule = Schedule::from_dir("./test_data");

        let mta_schedule = SubwaySchedule::try_from(schedule)?;

        // Verify Agency
        assert_eq!(mta_schedule.agency.agency_id, "MTA NYCT");
        assert_eq!(mta_schedule.agency.agency_name, "MTA New York City Transit");
        assert_eq!(mta_schedule.agency.agency_url, "http://www.mta.info");
        assert_eq!(mta_schedule.agency.agency_timezone, "America/New_York");
        assert_eq!(mta_schedule.agency.agency_lang, "en");
        assert_eq!(mta_schedule.agency.agency_phone, "718-330-1234");

        // Verify collection lengths
        assert_eq!(mta_schedule.routes.len(), 30);
        assert_eq!(mta_schedule.trips.len(), 20298);
        assert_eq!(mta_schedule.stations.len(), 499);
        assert_eq!(mta_schedule.stop_times.len(), 10000);
        assert_eq!(mta_schedule.transfers.len(), 616);

        for route in mta_schedule.routes {
            assert_eq!(route.agency_id, "MTA NYCT");
        }

        Ok(())
    }
}
