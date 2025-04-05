#![cfg(feature = "zip")]

use std::io::{Cursor, Read, Seek};

use zip::ZipArchive;

use crate::schedule::{
    parse_agencies, parse_routes, parse_service_exceptions, parse_services, parse_shapes,
    parse_stop_times, parse_stops, parse_transfers, parse_trips,
};

use super::Schedule;

impl Schedule {
    pub fn from_zip<R>(
        mut zip: ZipArchive<R>,
        date_bounds: Option<(&String, &String)>,
    ) -> Option<Self>
    where
        R: Read + Seek,
    {
        let agencies = parse_agencies(zip.by_name("agency.txt").ok()?);
        let stops = parse_stops(zip.by_name("stops.txt").ok()?);
        let services = parse_services(zip.by_name("calendar.txt").ok()?, date_bounds);
        let service_exceptions =
            parse_service_exceptions(zip.by_name("calendar_dates.txt").ok()?, date_bounds);
        let routes = parse_routes(zip.by_name("routes.txt").ok()?);
        let trips = parse_trips(
            zip.by_name("trips.txt").ok()?,
            &services,
            &service_exceptions,
        );
        let shapes = parse_shapes(zip.by_name("shapes.txt").ok()?);
        let transfers = parse_transfers(zip.by_name("transfers.txt").ok()?);
        let stop_times = parse_stop_times(zip.by_name("stop_times.txt").ok()?, &trips);

        Some(Self {
            agencies,
            stops,
            services,
            service_exceptions,
            routes,
            trips,
            shapes,
            transfers,
            stop_times,
        })
    }

    pub fn all_from_zip<R>(mut zip: ZipArchive<R>) -> Option<Self>
    where
        R: Read + Seek,
    {
        Self::from_zip(zip, None)
    }

    pub fn one_day_from_zip<R>(mut zip: ZipArchive<R>, date: String) -> Option<Self>
    where
        R: Read + Seek,
    {
        Self::from_zip(zip, Some((&date, &date)))
    }
}

#[cfg(test)]
mod tests {
    use std::{fs::File, io::Cursor};
    use zip::ZipArchive;

    use crate::schedule::Schedule;

    #[test]
    fn test_zip() {
        let schedule = Schedule::all_from_zip(
            ZipArchive::new(File::open("./test_data/schedule/gtfs_supplemented.zip").unwrap())
                .unwrap(),
        );
    }
}

