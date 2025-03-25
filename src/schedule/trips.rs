use serde::Deserialize;

use super::{Route, Service, ServiceException, ShapePoint};

// Only meaningful to separate routes according to docs
#[derive(Debug, Deserialize, PartialEq, Eq)]
#[serde(from = "u32")]
pub enum DirectionType {
    Uptown,
    Downtown,
}

impl From<u32> for DirectionType {
    fn from(value: u32) -> Self {
        match value {
            0 => Self::Uptown,
            1 => Self::Downtown,
            _ => panic!("Invalid DirectionType: {}", value),
        }
    }
}

#[derive(Debug, Deserialize, PartialEq, Eq)]
#[serde(from = "u32")]
enum WheelchairAccessibility {
    NoInfo,
    SomeSupport,
    NoSupport,
}

impl From<u32> for WheelchairAccessibility {
    fn from(value: u32) -> Self {
        match value {
            0 => Self::NoInfo,
            1 => Self::SomeSupport,
            2 => Self::NoSupport,
            _ => panic!("Invalid WheelchairAccessibility: {}", value),
        }
    }
}

#[derive(Debug, Deserialize, PartialEq, Eq)]
#[serde(from = "u32")]
enum BikeSupport {
    NoInfo,
    SomeSupport,
    NoSupport,
}

impl From<u32> for BikeSupport {
    fn from(value: u32) -> Self {
        match value {
            0 => Self::NoInfo,
            1 => Self::SomeSupport,
            2 => Self::NoSupport,
            _ => panic!("Invalid BikeSupport: {}", value),
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct Trip<'sc> {
    trip_id: String,
    route_id: String,
    service_id: String,
    trip_headsign: Option<String>,
    trip_short_name: Option<String>,
    direction_id: Option<DirectionType>,
    block_id: Option<String>,
    shape_id: Option<String>,
    wheelchair_accessible: Option<WheelchairAccessibility>,
    bikes_allowed: Option<BikeSupport>,

    #[serde(skip)]
    // I may not be able to use the lifetime in both places
    route: Option<&'sc Route<'sc>>,
    #[serde(skip)]
    calender_service: Option<&'sc Service>,
    #[serde(skip)]
    calendar_date_service: Option<&'sc ServiceException<'sc>>, // Only used when calendar.txt is omitted, not the case for MTA
    #[serde(skip)]
    shape: Option<&'sc ShapePoint>,
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use super::*;

    #[test]
    fn test_trips() -> Result<(), csv::Error> {
        let path = PathBuf::from("./test_data/trips.txt");
        let mut reader = csv::Reader::from_path(path)?;
        let mut res: Vec<Trip> = Vec::new();

        for rec in reader.deserialize() {
            res.push(rec?);
        }

        assert_eq!(res.len(), 20298);

        let mta = res.pop().unwrap();

        assert_eq!(mta.route_id, "SI");
        assert_eq!(mta.trip_id, "SIR-FA2017-SI017-Weekday-08_147100_SI..N03R");
        assert_eq!(mta.service_id, "Weekday");
        assert_eq!(mta.trip_headsign, Some("St George".to_owned()));
        assert_eq!(mta.direction_id, Some(DirectionType::Uptown));
        assert_eq!(mta.shape_id, Some("SI..N03R".to_owned()));

        Ok(())
    }
}
