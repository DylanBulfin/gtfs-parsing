pub mod agency;
pub mod calendar;
pub mod routes;
pub mod shapes;
pub mod stop_times;
pub mod stops;
pub mod transfers;
pub mod trips;

pub mod mta;

use std::{fs, path::Path};

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

#[derive(Debug)]
pub struct Schedule {
    pub agencies: Vec<Agency>,
    pub stops: Vec<Stop>,
    pub stop_times: Vec<StopTime>,
    pub services: Vec<Service>,
    pub service_exceptions: Vec<ServiceException>,
    //pub shape_points: Vec<ShapePoint>,
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
        schedule
            .shapes
            .append(&mut Shape::process_points(&parse_file!(
                "shapes.txt",
                ShapePoint,
                dir
            )));
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
        let schedule = Schedule::from_dir_abbrev("./test_data");

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
}
