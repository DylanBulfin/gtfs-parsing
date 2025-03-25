mod agency;
mod calendar;
mod routes;
mod shapes;
mod stop_times;
mod stops;
mod transfers;
mod trips;

use std::{fs, path::Path};

pub use agency::Agency;
pub use calendar::{Service, ServiceException};
pub use routes::Route;
pub use shapes::ShapePoint;
pub use stop_times::StopTime;
pub use stops::Stop;
pub use transfers::Transfer;
pub use trips::Trip;

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

pub struct Schedule {
    agencies: Vec<Agency>,
    stops: Vec<Stop>,
    stop_times: Vec<StopTime>,
    services: Vec<Service>,
    service_exceptions: Vec<ServiceException>,
    shape_points: Vec<ShapePoint>,
    transfers: Vec<Transfer>,
    routes: Vec<Route>,
    trips: Vec<Trip>,
}

impl Schedule {
    pub fn from_dir<P>(dir: P) -> Self
    where
        P: AsRef<Path>,
    {
        let mut schedule = Schedule {
            agencies: Vec::new(),
            stops: Vec::new(),
            stop_times: Vec::new(),
            services: Vec::new(),
            service_exceptions: Vec::new(),
            shape_points: Vec::new(),
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
        schedule
            .stop_times
            .append(&mut parse_file!("stop_times_abbrev.txt", StopTime, dir));
        schedule
            .services
            .append(&mut parse_file!("calendar.txt", Service, dir));
        schedule.service_exceptions.append(&mut parse_file!(
            "calendar_dates.txt",
            ServiceException,
            dir
        ));
        schedule
            .shape_points
            .append(&mut parse_file!("shapes.txt", ShapePoint, dir));
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
