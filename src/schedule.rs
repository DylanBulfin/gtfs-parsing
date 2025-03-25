mod agency;
mod calendar;
mod routes;
mod shapes;
mod stop_times;
mod stops;
mod transfers;
mod trips;

use std::{collections::HashMap, fs, path::Path};

pub use agency::Agency;
pub use calendar::{Service, ServiceException};
pub use routes::Route;
pub use shapes::ShapePoint;
pub use stop_times::StopTime;
pub use stops::Stop;
pub use transfers::Transfer;
pub use trips::Trip;

macro_rules! parse_file_vec {
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

macro_rules! parse_file_map {
    ($tf:literal, $t:ty, $dir:ident, $id:ident) => {{
        let mut res: HashMap<String, $t> = HashMap::new();
        let mut path = $dir.as_ref().to_path_buf();
        path.push($tf);
        if Path::try_exists(&path).expect("Unable to check existence of file") {
            let mut reader = csv::Reader::from_path(path).expect("Unable to read file");

            for rec in reader.deserialize() {
                let rec: $t = rec.expect("Unable to parse file");
                res.insert(rec.$id.clone(), rec);
            }
        }

        res
    }};
}

pub struct Schedule {
    pub stops: HashMap<String, Stop>,
    pub services: HashMap<String, Service>,
    pub shape_points: HashMap<String, ShapePoint>,
    pub transfers: HashMap<String, Transfer>,
    pub routes: HashMap<String, Route>,
    pub trips: HashMap<String, Trip>,

    pub agencies: Vec<Agency>,
    pub service_exceptions: HashMap<String, ServiceException>,
    pub stop_times: HashMap<String, StopTime>,
}

impl Schedule {
    pub fn from_dir<P>(dir: P) -> Self
    where
        P: AsRef<Path>,
    {
        let agencies = parse_file_map!("agency.txt", Agency, dir, agency_id);
        let stops = parse_file_map!("stops.txt", Stop, dir, stop_id);
        let stop_times = parse_file_map!("stop_times.txt", StopTime, dir, stop_id);
        let services = parse_file_map!("services.txt", services, dir, services_id);
        let service_exceptions = parse_file_map!(
            "service_exceptions.txt",
            service_exceptions,
            dir,
            service_exceptions_id
        );
        let shape_points = parse_file_map!("shape_points.txt", shape_points, dir, shape_points_id);
        let transfers = parse_file_map!("transfers.txt", transfers, dir, transfers_id);
        let routes = parse_file_map!("routes.txt", routes, dir, routes_id);
        let trips = parse_file_map!("trips.txt", trips, dir, trips_id);

        //schedule
        //    .agencies
        //    .append(&mut parse_file!("agency.txt", Agency, dir));
        //schedule
        //    .stops
        //    .append(&mut parse_file!("stops.txt", Stop, dir));
        //schedule
        //    .stop_times
        //    .append(&mut parse_file!("stop_times_abbrev.txt", StopTime, dir));
        //schedule
        //    .services
        //    .append(&mut parse_file!("calendar.txt", Service, dir));
        //schedule.service_exceptions.append(&mut parse_file!(
        //    "calendar_dates.txt",
        //    ServiceException,
        //    dir
        //));
        //schedule
        //    .shape_points
        //    .append(&mut parse_file!("shapes.txt", ShapePoint, dir));
        //schedule
        //    .transfers
        //    .append(&mut parse_file!("transfers.txt", Transfer, dir));
        //schedule
        //    .routes
        //    .append(&mut parse_file!("routes.txt", Route, dir));
        //schedule
        //    .trips
        //    .append(&mut parse_file!("trips.txt", Trip, dir));

        unimplemented!()
        //schedule
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_dir() {
        let schedule = Schedule::from_dir("./test_data");

        assert_eq!(schedule.agencies.len(), 1);
        assert_eq!(schedule.stops.len(), 1497);
        assert_eq!(schedule.stop_times.len(), 10000);
        assert_eq!(schedule.services.len(), 3);
        assert_eq!(schedule.service_exceptions.len(), 456);
        assert_eq!(schedule.shape_points.len(), 176482);
        assert_eq!(schedule.transfers.len(), 616);
        assert_eq!(schedule.routes.len(), 30);
        assert_eq!(schedule.trips.len(), 20298);
    }
}
