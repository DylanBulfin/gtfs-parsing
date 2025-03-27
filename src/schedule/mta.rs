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

impl MTAgency {
    fn try_create(value: Agency) -> Result<Self, String> {
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

    pub agency: &'sc MTAgency,
}

impl<'sc> SubwayRoute<'sc> {
    fn try_create(value: Route, agency: &'sc MTAgency) -> Result<Self, String> {
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
            route_color: value.route_color.ok_or("route_color cannot be empty")?,
            route_text_color: value
                .route_text_color
                .ok_or("route_text_color cannot be empty")?,

            agency,
        })
    }
}

pub struct SubwayTrip<'sc> {
    pub trip_id: String,
    //pub route_id: String,
    //pub service_id: String,
    pub trip_headsign: String,
    pub direction_id: DirectionType,
    //pub shape_id: String,
    pub shape: &'sc SubwayShape,
    pub service: &'sc SubwayService,
    pub route: &'sc SubwayRoute<'sc>,
}

impl<'sc> SubwayTrip<'sc> {
    fn try_create(
        value: Trip,
        shape: &'sc SubwayShape,
        service: &'sc SubwayService,
        route: &'sc SubwayRoute,
    ) -> Result<Self, String> {
        Ok(Self {
            trip_id: value.trip_id,
            trip_headsign: value.trip_headsign.ok_or("trip_headsign cannot be empty")?,
            direction_id: value.direction_id.ok_or("direction_type cannot be empty")?,

            shape,
            service,
            route,
        })
    }
}

pub struct SubwayTransferRule<'sc> {
    //pub from_stop_id: String,
    //pub to_stop_id: String,
    pub min_transfer_time: u32,

    pub from_station: &'sc SubwayStation,
    pub to_station: &'sc SubwayStation,
}

impl<'sc> SubwayTransferRule<'sc> {
    fn try_create(
        value: Transfer,
        from_station: &'sc SubwayStation,
        to_station: &'sc SubwayStation,
    ) -> Result<Self, String> {
        Ok(Self {
            min_transfer_time: value
                .min_transfer_time
                .ok_or("min_transfer_time cannot be empty")?,
            from_station,
            to_station,
        })
    }
}

pub struct SubwayStation {
    pub station_id: String,
    pub stop_name: String,
    pub stop_lat: String,
    pub stop_lon: String,

    pub uptown_platform_id: String,
    pub downtown_platform: String,
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
            downtown_platform: downtown.stop_id,
        })
    }
}

pub struct SubwayStopTime<'sc> {
    //pub trip_id: String,
    pub arrival_time: String,
    pub departure_time: String,
    //pub stop_id: String,
    pub stop_sequence: u32,

    pub trip: &'sc SubwayTrip<'sc>,
    pub station: &'sc SubwayStation,
    pub uptown: bool, // For each station the MTA format defines a parent station, and two
                      // platforms (N and S). This determines which platform this stoptime
                      // uses
}

impl<'sc> SubwayStopTime<'sc> {
    fn try_create(
        value: StopTime,
        trip: &'sc SubwayTrip<'sc>,
        station: &'sc SubwayStation,
        uptown: bool,
    ) -> Result<Self, String> {
        Ok(Self {
            arrival_time: value.arrival_time.ok_or("arrival_time cannot be empty")?,
            departure_time: value
                .departure_time
                .ok_or("departure_time cannot be empty")?,
            stop_sequence: value.stop_sequence.ok_or("stop_sequence cannot be empty")?,

            trip,
            station,
            uptown,
        })
    }
}

// The others are used as-is
use Service as SubwayService;
use ServiceException as SubwayServiceException;
use Shape as SubwayShape;

pub struct SubwaySchedule<'sc> {
    agency: MTAgency,
    routes: Vec<SubwayRoute<'sc>>,
    trips: Vec<SubwayTrip<'sc>>,
    transfers: Vec<SubwayTransferRule<'sc>>,
    stations: Vec<SubwayStation>,
    stop_times: Vec<SubwayStopTime<'sc>>,
    services: Vec<SubwayService>,
    service_exceptions: Vec<SubwayServiceException>,
    shapes: Vec<SubwayShape>,
}

impl<'sc> TryFrom<Schedule> for SubwaySchedule<'sc> {
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
        let agency = MTAgency::try_create(agencies.pop().unwrap())?;

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
            schedule
                .routes
                .push(SubwayRoute::try_create(route, &schedule.agency)?);
        }

        for trip in base_trips {
            let shape_id = trip.shape_id.as_ref().ok_or("shape_id cannot be empty")?;
            let shape = shapes
                .iter()
                .find(|s| &s.shape_id == shape_id)
                .ok_or("Invalid shape_id")?;

            let service = services
                .iter()
                .find(|s| s.service_id == trip.service_id)
                .ok_or("Invalid service_id")?;

            let route = schedule
                .routes
                .iter()
                .find(|r| r.route_id == trip.route_id)
                .ok_or("Invalid route_id")?;

            schedule
                .trips
                .push(SubwayTrip::try_create(trip, shape, service, route)?);
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
            let platform_id = stop_time
                .stop_id
                .as_ref()
                .ok_or("stop_id cannot be empty")?;
            assert!(platform_id.is_ascii());

            let uptown = match platform_id.as_bytes()[platform_id.len() - 1] {
                b'N' => true,
                b'S' => false,
                c => panic!("Unexpected character at end of stop: {}", c),
            };

            let station = schedule
                .stations
                .iter()
                .find(|s| s.station_id == platform_id[0..platform_id.len() - 1])
                .ok_or("Invalid stop_id")?;

            let trip = schedule
                .trips
                .iter()
                .find(|t| t.trip_id == stop_time.trip_id)
                .ok_or("Invalid trip_id in StopTime")?;

            schedule.stop_times.push(SubwayStopTime::try_create(
                stop_time, trip, station, uptown,
            )?)
        }

        for transfer in base_transfers {
            let from_station_id = transfer
                .from_stop_id
                .as_ref()
                .ok_or("from_stop_id cannot be empty")?;
            let to_station_id = transfer
                .to_stop_id
                .as_ref()
                .ok_or("to_stop_id cannot be empty")?;

            let from_station = schedule
                .stations
                .iter()
                .find(|s| &s.station_id == from_station_id)
                .ok_or("Invalid from_station_id")?;
            let to_station = schedule
                .stations
                .iter()
                .find(|s| &s.station_id == to_station_id)
                .ok_or("Invalid to_station_id")?;

            schedule.transfers.push(SubwayTransferRule::try_create(
                transfer,
                from_station,
                to_station,
            )?);
        }

        Ok(schedule)
    }
}
