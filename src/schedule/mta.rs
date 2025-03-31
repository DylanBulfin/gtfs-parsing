// This file is for parsing the data into a format specifically useful for the MTA Subway.
// E.g. If the information is not provided in either the current regular or supplemented
// file it's ignored, and the organization of entities is a bit more basic.
// It's very brittle and could be interrupted by minor changes to their csv so, yk.

use std::collections::{HashMap, hash_map::Entry};

use super::{
    Schedule,
    agency::Agency,
    calendar::{Activity, ExceptionType, Service, ServiceException},
    routes::{Route, RouteType},
    shapes::{Shape, ShapePointData},
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
    pub route_id: String,
    pub service_id: String,
    pub trip_headsign: String,
    pub direction_id: DirectionType,
    pub shape_id: Option<String>,
}

impl SubwayTrip {
    fn try_create(value: Trip) -> Result<Self, String> {
        Ok(Self {
            //trip_id: value.trip_id,
            trip_headsign: value.trip_headsign.ok_or("trip_headsign cannot be empty")?,
            direction_id: value.direction_id.ok_or("direction_type cannot be empty")?,
            route_id: value.route_id,
            service_id: value.service_id,
            shape_id: value.shape_id,
        })
    }
}

pub struct SubwayTransferRule {
    pub from_station_id: String,
    pub to_station_id: String,
    pub min_transfer_time: u32,
}

impl SubwayTransferRule {
    fn try_create(value: Transfer) -> Result<Self, String> {
        Ok(Self {
            min_transfer_time: value
                .min_transfer_time
                .ok_or("min_transfer_time cannot be empty")?,
            from_station_id: value.from_stop_id.ok_or("from_stop_id cannot be empty")?,
            to_station_id: value.to_stop_id.ok_or("to_stop_id cannot be empty")?,
        })
    }
}

pub struct SubwayStation {
    //pub station_id: String,
    pub stop_name: String,
    pub stop_lat: String,
    pub stop_lon: String,

    pub uptown_platform_id: String,
    pub downtown_platform_id: String,
}

impl SubwayStation {
    fn try_create(parent: Stop, uptown: Stop, downtown: Stop) -> Result<Self, String> {
        let Stop {
            stop_name,
            stop_lat,
            stop_lon,
            ..
        } = parent;

        Ok(Self {
            //station_id: stop_id,
            stop_name: stop_name.ok_or("stop_name cannot be empty")?,
            stop_lat: stop_lat.ok_or("stop_lat cannot be empty")?,
            stop_lon: stop_lon.ok_or("stop_lon cannot be empty")?,

            uptown_platform_id: uptown.stop_id,
            downtown_platform_id: downtown.stop_id,
        })
    }
}

pub struct SubwayStopTime {
    //pub trip_id: String,
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
            station_id,
            arrival_time: value.arrival_time.ok_or("arrival_time cannot be empty")?,
            departure_time: value
                .departure_time
                .ok_or("departure_time cannot be empty")?,
            stop_sequence: value.stop_sequence,

            uptown,
        })
    }
}

pub struct SubwayService {
    pub sunday: Activity,
    pub monday: Activity,
    pub tuesday: Activity,
    pub wednesday: Activity,
    pub thursday: Activity,
    pub friday: Activity,
    pub saturday: Activity,
    pub start_date: String,
    pub end_date: String,
}

impl From<Service> for SubwayService {
    fn from(value: Service) -> Self {
        Self {
            sunday: value.sunday,
            monday: value.monday,
            tuesday: value.tuesday,
            wednesday: value.wednesday,
            thursday: value.thursday,
            friday: value.friday,
            saturday: value.saturday,
            start_date: value.start_date,
            end_date: value.end_date,
        }
    }
}

pub struct SubwayServiceException {
    pub date: String,
    pub exception_type: ExceptionType,
}

impl From<ServiceException> for SubwayServiceException {
    fn from(value: ServiceException) -> Self {
        Self {
            date: value.date,
            exception_type: value.exception_type,
        }
    }
}

pub struct SubwayShape {
    pub points: Vec<ShapePointData>,
}

impl From<Shape> for SubwayShape {
    fn from(value: Shape) -> Self {
        Self {
            points: value.points,
        }
    }
}

pub struct SubwaySchedule {
    agency: MTAgency,
    routes: HashMap<String, SubwayRoute>,
    trips: HashMap<String, SubwayTrip>,
    stations: HashMap<String, SubwayStation>,
    services: HashMap<String, SubwayService>,
    shapes: HashMap<String, SubwayShape>,

    stop_times: HashMap<String, Vec<SubwayStopTime>>, // key = trip_id
    transfers: HashMap<String, Vec<SubwayTransferRule>>, // key = from_station_id
    service_exceptions: HashMap<String, Vec<SubwayServiceException>>, // key = service_id
}

impl TryFrom<Schedule> for SubwaySchedule {
    type Error = String;

    fn try_from(value: Schedule) -> Result<Self, Self::Error> {
        let Schedule {
            mut agencies,
            routes: base_routes,
            trips: base_trips,
            transfers: base_transfers,
            stops: mut base_stops,
            stop_times: base_stop_times,
            shapes: base_shapes,
            services: base_services,
            service_exceptions: base_service_exceptions,
        } = value;

        assert_eq!(agencies.len(), 1);
        let agency = MTAgency::try_from(agencies.pop().unwrap())?;

        let mut routes = HashMap::new();
        for route in base_routes {
            routes.insert(route.route_id.to_owned(), SubwayRoute::try_create(route)?);
        }

        let mut trips = HashMap::new();
        for trip in base_trips {
            trips.insert(trip.trip_id.to_owned(), SubwayTrip::try_create(trip)?);
        }

        // Stops are stored in reverse order due to the base vector creation code, should probably
        // stop doing it like this but it's much easier
        base_stops.reverse();
        let mut stations = HashMap::new();
        assert_eq!(base_stops.len() % 3, 0);
        while base_stops.len() >= 3 {
            // TODO This relies on a specific order for the stops, probably want to change
            let (parent, uptown, downtown) = (
                base_stops.pop().unwrap(),
                base_stops.pop().unwrap(),
                base_stops.pop().unwrap(),
            );

            stations.insert(
                parent.stop_id.to_owned(),
                SubwayStation::try_create(parent, uptown, downtown)?,
            );
        }
        assert_eq!(base_stops.len(), 0);

        let mut stop_times: HashMap<String, Vec<SubwayStopTime>> = HashMap::new();
        for stop_time in base_stop_times {
            let platform_id = stop_time
                .stop_id
                .to_owned()
                .ok_or("stop_id cannot be empty")?;
            assert!(platform_id.is_ascii());
            let trip_id = stop_time.trip_id.to_owned();

            let uptown = match platform_id.as_bytes()[platform_id.len() - 1] {
                b'N' => true,
                b'S' => false,
                c => panic!("Unexpected character at end of stop: {}", c),
            };

            let elem = SubwayStopTime::try_create(
                stop_time,
                platform_id[0..platform_id.len() - 1].to_owned(),
                uptown,
            )?;

            match stop_times.entry(trip_id) {
                Entry::Occupied(mut e) => {
                    e.get_mut().push(elem);
                }
                Entry::Vacant(e) => {
                    e.insert(vec![elem]);
                }
            }
        }

        let mut shapes = HashMap::new();
        for shape in base_shapes {
            shapes.insert(shape.shape_id.to_owned(), shape.into());
        }

        let mut transfers: HashMap<String, Vec<SubwayTransferRule>> = HashMap::new();
        for transfer in base_transfers {
            let from_station_id = transfer
                .from_stop_id
                .to_owned()
                .ok_or("from_station_id cannot be empty")?;

            let elem = SubwayTransferRule::try_create(transfer)?;

            match transfers.entry(from_station_id) {
                Entry::Occupied(mut e) => e.get_mut().push(elem),
                Entry::Vacant(e) => {
                    e.insert(vec![elem]);
                }
            }
        }

        let mut services = HashMap::new();
        for service in base_services {
            services.insert(service.service_id.to_owned(), service.into());
        }

        let mut service_exceptions: HashMap<String, Vec<SubwayServiceException>> = HashMap::new();
        for service_exception in base_service_exceptions {
            let service_id = service_exception.service_id.to_owned();
            let elem = SubwayServiceException::from(service_exception);

            match service_exceptions.entry(service_id) {
                Entry::Occupied(mut e) => e.get_mut().push(elem),
                Entry::Vacant(e) => {
                    e.insert(vec![elem]);
                }
            }
        }

        Ok(Self {
            agency,
            routes,
            trips,
            stations,
            stop_times,
            service_exceptions,
            services,
            shapes,
            transfers,
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::schedule::Schedule;

    use super::SubwaySchedule;

    #[test]
    fn test_basics() -> Result<(), String> {
        let schedule = Schedule::from_dir_abbrev("./test_data/schedule");

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
        assert_eq!(mta_schedule.trips.len(), 79970);
        assert_eq!(mta_schedule.stations.len(), 499);

        // Indexed by trip_id, but from abbreviated file so count is meaningless
        assert_eq!(mta_schedule.stop_times.len(), 264);

        assert_eq!(mta_schedule.services.len(), 71);

        // Since it uses service_id as primary key, and there are 36 exceptions with no
        // corresponding schedule, we get 107 total
        assert_eq!(mta_schedule.service_exceptions.len(), 107);

        assert_eq!(mta_schedule.transfers.len(), 465);

        assert_eq!(mta_schedule.shapes.len(), 311);

        // Verify relations exist for trips
        for trip_id in mta_schedule.trips.keys() {
            let service_id = mta_schedule
                .trips
                .get(trip_id)
                .unwrap()
                .service_id
                .to_owned();
            let shape_id = mta_schedule.trips.get(trip_id).unwrap().shape_id.to_owned();

            // If we can't find ID in calendar.txt (services) we look in calendar_date.txt
            // (service_exceptions)
            if mta_schedule.services.get(&service_id).is_none() {
                assert!(mta_schedule.service_exceptions.get(&service_id).is_some());
            }

            if let Some(shape_id) = shape_id {
                assert!(mta_schedule.shapes.get(&shape_id).is_some());
            }
        }

        // Verify that stops and stations are grouped correctly
        for station_id in mta_schedule.stations.keys() {
            let station = mta_schedule.stations.get(station_id).unwrap();
            assert_eq!(format!("{}N", station_id), station.uptown_platform_id);
            assert_eq!(format!("{}S", station_id), station.downtown_platform_id);
        }

        // Verify relations exist for transfers
        for from_station_id in mta_schedule.transfers.keys() {
            if mta_schedule.stations.get(from_station_id).is_none() {
                panic!("{:?} {:?}", from_station_id, mta_schedule.stations.keys());
            }

            assert!(mta_schedule.stations.get(from_station_id).is_some());

            for transfer_rule in mta_schedule.transfers.get(from_station_id).unwrap() {
                assert!(
                    mta_schedule
                        .stations
                        .get(&transfer_rule.to_station_id)
                        .is_some()
                );
            }
        }

        // Verify relations exist for stop_times
        for trip_id in mta_schedule.stop_times.keys() {
            for stop_time in mta_schedule.stop_times.get(trip_id).unwrap() {
                let station_id = stop_time.station_id.clone();

                assert!(mta_schedule.stations.get(&station_id).is_some())
            }
        }

        Ok(())
    }

    #[test]
    #[ignore]
    fn test_full() -> Result<(), String> {
        let schedule = Schedule::from_dir_full("./test_data/schedule");

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
        assert_eq!(mta_schedule.trips.len(), 79970);
        assert_eq!(mta_schedule.stations.len(), 499);

        // Indexed by trip_id, so has same size
        assert_eq!(mta_schedule.stop_times.len(), 79970);

        assert_eq!(mta_schedule.services.len(), 71);

        // Since it uses service_id as primary key, and there are 36 exceptions with no
        // corresponding schedule, we get 107 total
        assert_eq!(mta_schedule.service_exceptions.len(), 107);

        assert_eq!(mta_schedule.transfers.len(), 465);

        assert_eq!(mta_schedule.shapes.len(), 311);

        Ok(())
    }
}
