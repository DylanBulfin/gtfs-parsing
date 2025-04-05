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
    (map: $r:expr, $kt:ty, $vt:ty, $kf:ident) => {{
        let mut res: HashMap<$kt, $vt> = HashMap::new();
        let mut reader = csv::Reader::from_reader($r);

        for rec in reader.deserialize() {
            let rec: $vt = if let Ok(x) = rec { x } else { continue };
            res.insert(rec.$kf.clone(), rec);
        }

        res
    }};
    (cmap: $r:expr, $kt:ty, $vt:ty, $kf:ident, $rec:ident, $cond:expr) => {{
        let mut res: HashMap<$kt, $vt> = HashMap::new();
        let mut reader = csv::Reader::from_reader($r);

        for rec in reader.deserialize() {
            let $rec: $vt = if let Ok(x) = rec { x } else { continue };
            if $cond {
                res.insert($rec.$kf.clone(), $rec);
            }
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
        date_bounds: Option<(&String, &String)>,
    ) -> Option<Self>
    where
        R: Read,
    {
        let agencies = parse_reader!(vec: agency_reader, Agency);
        let stops = parse_reader!(map: stop_reader, String, Stop, stop_id);
        let services = match date_bounds {
            Some((start, end)) => {
                parse_reader!(cmap: service_reader, String, Service, service_id, service, &service.start_date <= end && &service.end_date >= start)
            }
            None => parse_reader!(map: service_reader, String, Service, service_id),
        };

        let mut service_exceptions: HashMap<String, HashMap<String, ServiceException>> =
            HashMap::new();
        let mut csv_reader = csv::Reader::from_reader(service_exception_reader);
        for rec in csv_reader.deserialize() {
            let rec: ServiceException = if let Ok(x) = rec { x } else { continue };

            if let Some((start, end)) = date_bounds {
                if &rec.date < start || &rec.date > end {
                    continue;
                }
            }

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

        let routes = parse_reader!(map: route_reader, String, Route, route_id);
        let trips = parse_reader!(cmap: trip_reader, String, Trip, trip_id, trip, services.contains_key(&trip.service_id) || service_exceptions.contains_key(&trip.service_id));

        let shape_points: Vec<ShapePoint> = parse_reader!(vec: shape_reader, ShapePoint);
        let shapes = Shape::process_points(&shape_points);

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
            if !trips.contains_key(&rec.trip_id) {
                continue;
            }
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
#[cfg(test)]
mod tests {
    use std::fs::File;

    use super::*;

    macro_rules! setup_new_schedule {
        ($bounds:expr) => {{
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
                $bounds,
            )
        }};
    }

    #[test]
    #[ignore]
    fn test_from_readers_full() {
        let schedule = setup_new_schedule!(None).unwrap();

        assert_eq!(schedule.agencies.len(), 1);
        assert_eq!(schedule.services.len(), 71);
        assert_eq!(schedule.stops.len(), 1497);
        assert_eq!(
            schedule
                .stop_times
                .values()
                .flat_map(HashMap::values)
                .count(),
            2_339_542
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

    #[test]
    fn test_from_readers_abbrev() {
        let (start, end) = ("20250301".to_owned(), "20250401".to_owned());
        let schedule = setup_new_schedule!(Some((&start, &end))).unwrap();

        assert_eq!(schedule.agencies.len(), 1);
        assert_eq!(schedule.services.len(), 71);
        assert_eq!(schedule.stops.len(), 1497);
        assert_eq!(
            schedule
                .stop_times
                .values()
                .flat_map(HashMap::values)
                .count(),
            1_914_369
        );
        assert_eq!(
            schedule
                .service_exceptions
                .values()
                .flat_map(HashMap::values)
                .count(),
            82
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
        assert_eq!(schedule.trips.len(), 65871);

        for (service_id, service) in schedule.services.iter() {
            assert!(service.start_date <= end && service.end_date >= start);
        }
        for (service_id, except_map) in schedule.service_exceptions.iter() {
            for (date, service) in except_map {
                assert!(date >= &start && date <= &end);
            }
        }
        for (trip_id, trip) in schedule.trips.iter() {
            assert!(
                schedule.services.contains_key(&trip.service_id)
                    || schedule.service_exceptions.contains_key(&trip.service_id)
            );
        }
        for (trip_id, stop_time_map) in schedule.stop_times {
            for (stop_seq, stop_time) in stop_time_map {
                assert!(schedule.trips.contains_key(&trip_id));
            }
        }
    }
}
