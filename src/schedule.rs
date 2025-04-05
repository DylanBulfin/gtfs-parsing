pub mod agency;
pub mod calendar;
pub mod routes;
pub mod shapes;
pub mod stop_times;
pub mod stops;
pub mod transfers;
pub mod trips;

pub mod mta;

use std::{
    collections::{HashMap, hash_map::Entry},
    fs,
    io::Read,
    path::Path,
};

use agency::Agency;
use calendar::{Service, ServiceException};
use routes::Route;
use shapes::{Shape, ShapePoint};
use stop_times::StopTime;
use stops::Stop;
use transfers::Transfer;
use trips::Trip;

//pub use agency::Agency;
//pub use calendar::{Activity, ExceptionType, Service, ServiceException};
//pub use routes::{ContinuousType, Route, RouteType};
//pub use shapes::{Shape, ShapePoint, ShapePointData};
//pub use stop_times::{DropoffType, PickupType, StopTime, Timepoint};
//pub use stops::{LocationType, Stop, WheelchairBoarding};
//pub use transfers::{Transfer, TransferType};
//pub use trips::{BikeSupport, DirectionType, Trip, WheelchairAccessibility};

macro_rules! parse_file {
    ($tf:literal, $t:ty, $dir: ident) => {{
        let mut res: Vec<$t> = Vec::new();
        let mut path = $dir.as_ref().to_path_buf();
        path.push($tf);
        if Path::try_exists(&path).expect("Unable to check existence of file") {
            let mut reader = csv::Reader::from_path(path).expect("Unable to read file");

            for rec in reader.deserialize() {
                res.push(rec.expect("Unable to parse file"));
            }
        }

        res
    }};
}

macro_rules! parse_reader {
    (vec: $r:expr, $t:ty) => {{
        let mut res: Vec<$t> = Vec::new();
        let mut reader = csv::Reader::from_reader($r);

        for rec in reader.deserialize() {
            let rec: $t = if let Ok(x) = rec { x } else { continue };
            res.push(rec);
        }

        res
    }};
    (map: $r:expr, $kt:ty, $vt:ty, $kf: ident) => {{
        let mut res: HashMap<$kt, $vt> = HashMap::new();
        let mut reader = csv::Reader::from_reader($r);

        for rec in reader.deserialize() {
            let rec: $vt = if let Ok(x) = rec { x } else { continue };
            res.insert(rec.$kf.clone(), rec);
        }

        res
    }};
}

#[derive(Debug)]
/// This struct stores things in maps which allows for far more efficient parsing into a natural,
/// hierarchical format
pub struct NewSchedule {
    // Agencies is tiny, no need for map
    pub agencies: Vec<Agency>,
    // Indexed by stop_id
    pub stops: HashMap<String, Stop>,
    // Indexed by stop_id, then stop_sequence
    pub stop_times: HashMap<String, HashMap<u32, StopTime>>,
    // Indexed by service_id
    pub services: HashMap<String, Service>,
    // Indexed by service_id, then date
    pub service_exceptions: HashMap<String, HashMap<String, ServiceException>>,
    // Indexed by shape_id
    pub shapes: HashMap<String, Shape>,
    // Indexed by from_stop_id
    pub transfers: HashMap<String, Vec<Transfer>>,
    // Indexed by route_id
    pub routes: HashMap<String, Route>,
    // Indexed by trip_id
    pub trips: HashMap<String, Trip>,
}

impl NewSchedule {
    pub fn from_readers<R>(
        agency_reader: R,
        stop_reader: R,
        stop_time_reader: R,
        service_reader: R,
        service_exception_reader: R,
        shape_reader: R,
        transfer_reader: R,
        route_reader: R,
        trip_reader: R,
    ) -> Option<Self>
    where
        R: Read,
    {
        let agencies = parse_reader!(vec: agency_reader, Agency);
        let stops = parse_reader!(map: stop_reader, String, Stop, stop_id);
        let services = parse_reader!(map: service_reader, String, Service, service_id);
        let routes = parse_reader!(map: route_reader, String, Route, route_id);
        let trips = parse_reader!(map: trip_reader, String, Trip, trip_id);

        let shape_points: Vec<ShapePoint> = parse_reader!(vec: shape_reader, ShapePoint);
        let shapes = Shape::process_points(&shape_points);

        let mut service_exceptions: HashMap<String, HashMap<String, ServiceException>> =
            HashMap::new();
        let mut csv_reader = csv::Reader::from_reader(service_exception_reader);
        for rec in csv_reader.deserialize() {
            let rec: ServiceException = rec.expect("Unable to parse service_exception reader");
            match service_exceptions.entry(rec.service_id.clone()) {
                Entry::Occupied(mut e) => {
                    e.get_mut().insert(rec.date.clone(), rec);
                }
                Entry::Vacant(e) => {
                    let mut new_entry: HashMap<String, ServiceException> = HashMap::new();
                    new_entry.insert(rec.date.clone(), rec);
                    e.insert(HashMap::new());
                }
            }
        }

        let mut transfers: HashMap<String, Vec<Transfer>> = HashMap::new();
        csv_reader = csv::Reader::from_reader(transfer_reader);
        for rec in csv_reader.deserialize() {
            let rec: Transfer = if let Ok(x) = rec { x } else { continue };
            let from_stop_id: String = if let Some(x) = rec.from_stop_id.clone() {
                x
            } else {
                continue;
            };

            match transfers.entry(from_stop_id) {
                Entry::Occupied(mut e) => {
                    e.get_mut().push(rec);
                }
                Entry::Vacant(mut e) => {
                    e.insert(vec![rec]);
                }
            }
        }

        let mut stop_times: HashMap<String, HashMap<u32, StopTime>> = HashMap::new();
        csv_reader = csv::Reader::from_reader(stop_time_reader);
        for rec in csv_reader.deserialize() {
            let rec: StopTime = if let Ok(x) = rec { x } else { continue };
            match stop_times.entry(rec.trip_id.clone()) {
                Entry::Occupied(mut e) => {
                    e.get_mut().insert(rec.stop_sequence, rec);
                }
                Entry::Vacant(e) => {
                    let mut new_entry: HashMap<u32, StopTime> = HashMap::new();
                    new_entry.insert(rec.stop_sequence, rec);
                    e.insert(new_entry);
                }
            }
        }

        Some(Self {
            agencies,
            routes,
            transfers,
            trips,
            stop_times,
            service_exceptions,
            shapes,
            stops,
            services,
        })
    }
}

pub struct Schedule {
    pub agencies: Vec<Agency>,
    pub stops: Vec<Stop>,
    pub stop_times: Vec<StopTime>,
    pub services: Vec<Service>,
    pub service_exceptions: Vec<ServiceException>,
    pub shapes: Vec<Shape>,
    pub transfers: Vec<Transfer>,
    pub routes: Vec<Route>,
    pub trips: Vec<Trip>,
}

impl Schedule {
    pub fn from_dir_full<P>(dir: P) -> Self
    where
        P: AsRef<Path>,
    {
        Self::from_dir(dir, false)
    }

    pub fn from_dir_abbrev<P>(dir: P) -> Self
    where
        P: AsRef<Path>,
    {
        Self::from_dir(dir, true)
    }

    pub fn from_dir<P>(dir: P, use_abbrev: bool) -> Self
    where
        P: AsRef<Path>,
    {
        let mut schedule = Schedule {
            agencies: Vec::new(),
            stops: Vec::new(),
            stop_times: Vec::new(),
            services: Vec::new(),
            service_exceptions: Vec::new(),
            shapes: Vec::new(),
            transfers: Vec::new(),
            routes: Vec::new(),
            trips: Vec::new(),
        };

        let mut agency_path = dir.as_ref().to_path_buf();
        agency_path.push("agency.txt");
        if Path::try_exists(&agency_path).expect("Unable to check existence of file") {
            let mut reader = csv::Reader::from_path(agency_path).expect("Unable to read file");
            let mut res: Vec<Agency> = Vec::new();

            for rec in reader.deserialize() {
                res.push(rec.expect("Unable to parse file"));
            }
        }

        schedule
            .agencies
            .append(&mut parse_file!("agency.txt", Agency, dir));
        schedule
            .stops
            .append(&mut parse_file!("stops.txt", Stop, dir));

        if use_abbrev {
            schedule
                .stop_times
                .append(&mut parse_file!("stop_times_abbrev.txt", StopTime, dir));
        } else {
            schedule
                .stop_times
                .append(&mut parse_file!("stop_times.txt", StopTime, dir));
        }
        schedule
            .services
            .append(&mut parse_file!("calendar.txt", Service, dir));
        schedule.service_exceptions.append(&mut parse_file!(
            "calendar_dates.txt",
            ServiceException,
            dir
        ));
        schedule.shapes.append(
            &mut Shape::process_points(&parse_file!("shapes.txt", ShapePoint, dir))
                .into_values()
                .collect(),
        );
        schedule
            .transfers
            .append(&mut parse_file!("transfers.txt", Transfer, dir));
        schedule
            .routes
            .append(&mut parse_file!("routes.txt", Route, dir));
        schedule
            .trips
            .append(&mut parse_file!("trips.txt", Trip, dir));

        // TODO initialize pointers

        schedule
    }
}

#[cfg(test)]
mod tests {
    use std::fs::File;

    use super::*;

    macro_rules! setup_new_schedule {
        () => {{
            let agency_reader = File::open("./test_data/schedule/agency.txt").unwrap();
            let stop_reader = File::open("./test_data/schedule/stops.txt").unwrap();
            let stop_time_reader = File::open("./test_data/schedule/stop_times.txt").unwrap();
            let service_reader = File::open("./test_data/schedule/calendar.txt").unwrap();
            let service_exception_reader =
                File::open("./test_data/schedule/calendar_dates.txt").unwrap();
            let shape_reader = File::open("./test_data/schedule/shapes.txt").unwrap();
            let transfer_reader = File::open("./test_data/schedule/transfers.txt").unwrap();
            let route_reader = File::open("./test_data/schedule/routes.txt").unwrap();
            let trip_reader = File::open("./test_data/schedule/trips.txt").unwrap();

            NewSchedule::from_readers(
                agency_reader,
                stop_reader,
                stop_time_reader,
                service_reader,
                service_exception_reader,
                shape_reader,
                transfer_reader,
                route_reader,
                trip_reader,
            )
        }};
    }

    #[test]
    fn test_from_dir() {
        let schedule = Schedule::from_dir_abbrev("./test_data/schedule");

        assert_eq!(schedule.agencies.len(), 1);
        assert_eq!(schedule.stops.len(), 1497);
        assert_eq!(schedule.stop_times.len(), 10000);
        assert_eq!(schedule.services.len(), 71);
        assert_eq!(schedule.service_exceptions.len(), 406);
        assert_eq!(schedule.shapes.len(), 311);
        assert_eq!(schedule.transfers.len(), 616);
        assert_eq!(schedule.routes.len(), 30);
        assert_eq!(schedule.trips.len(), 79970);
    }

    #[test]
    fn test_from_readers() {
        let schedule = setup_new_schedule!().unwrap();

        assert_eq!(schedule.agencies.len(), 1);
        assert_eq!(schedule.services.len(), 71);
        assert_eq!(schedule.stops.len(), 1497);
        assert_eq!(
            schedule
                .stop_times
                .values()
                .flat_map(HashMap::values)
                .count(),
            2339542
        );
        assert_eq!(
            schedule
                .service_exceptions
                .values()
                .flat_map(HashMap::values)
                .count(),
            299
        );
        assert_eq!(schedule.shapes.len(), 311);
        assert_eq!(
            schedule
                .transfers
                .values()
                .map(Vec::len)
                .fold(0, |a, b| a + b),
            616
        );
        assert_eq!(schedule.routes.len(), 30);
        assert_eq!(schedule.trips.len(), 79970);
    }
}
