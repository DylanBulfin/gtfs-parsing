use serde::Deserialize;

use super::{stops::Stop, trips::Trip};

#[derive(Debug, Deserialize, PartialEq, Eq)]
#[serde(from = "u32")]
pub enum DropoffType {
    Dropoff,
    NoDropoff,
    CallAgency,
    CallDriver,
}

impl From<u32> for DropoffType {
    fn from(value: u32) -> Self {
        match value {
            0 => Self::Dropoff,
            1 => Self::NoDropoff,
            2 => Self::CallAgency,
            3 => Self::CallDriver,
            _ => panic!("Invalid DropoffType: {}", value),
        }
    }
}

#[derive(Debug, Deserialize, PartialEq, Eq)]
#[serde(from = "u32")]
pub enum PickupType {
    Pickup,
    NoPickup,
    CallAgency,
    CallDriver,
}

impl From<u32> for PickupType {
    fn from(value: u32) -> Self {
        match value {
            0 => Self::Pickup,
            1 => Self::NoPickup,
            2 => Self::CallAgency,
            3 => Self::CallDriver,
            _ => panic!("Invalid PickupType: {}", value),
        }
    }
}

#[derive(Debug, Deserialize, PartialEq, Eq)]
#[serde(from = "u32")]
pub enum Timepoint {
    Approximate,
    Precise,
}

impl From<u32> for Timepoint {
    fn from(value: u32) -> Self {
        match value {
            0 => Self::Approximate,
            1 => Self::Precise,
            _ => panic!("Invalid Timepoint: {}", value),
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct StopTime<'sc> {
    trip_id: String,
    arrival_time: Option<String>,
    departure_time: Option<String>,
    stop_id: Option<String>,
    location_group_id: Option<String>,
    location_id: Option<String>,
    stop_sequence: Option<u32>,
    stop_headsign: Option<String>,
    start_pickup_drop_off_window: Option<String>,
    end_pickup_drop_off_window: Option<String>,
    pickup_type: Option<PickupType>,
    drop_off_type: Option<DropoffType>,
    continuous_pickup: Option<PickupType>,
    continuous_drop_off: Option<DropoffType>,
    shape_dist_traveled: Option<f64>,
    timepoint: Option<Timepoint>,
    pickup_booking_rule_id: Option<String>,
    drop_off_booking_rule_id: Option<String>,

    #[serde(skip)]
    trip: Option<&'sc Trip<'sc>>,
    #[serde(skip)]
    stop: Option<&'sc Stop>,
    //location: Option<&'sc GeoJsonLocation>
    //pickup_booking_rule: Option<&'sc BookingRule>
    //dropoff_booking_rule: Option<&'sc BookingRule>
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use super::*;

    #[test]
    fn test_stop_times() -> Result<(), csv::Error> {
        let path = PathBuf::from("./test_data/stop_times_abbrev.txt");
        let mut reader = csv::Reader::from_path(path)?;
        let mut res: Vec<StopTime> = Vec::new();

        for rec in reader.deserialize() {
            res.push(rec?);
        }

        assert_eq!(res.len(), 10000);

        let mta = res.pop().unwrap();

        assert_eq!(mta.trip_id, "SIR-FA2017-SI017-Weekday-08_147100_SI..N03R");
        assert_eq!(mta.arrival_time, Some("25:13:00".to_owned()));
        assert_eq!(mta.departure_time, Some("25:13:00".to_owned()));
        assert_eq!(mta.stop_id, Some("S31N".to_owned()));
        assert_eq!(mta.location_group_id, None);
        assert_eq!(mta.location_id, None);
        assert_eq!(mta.stop_sequence, Some(21));
        assert_eq!(mta.stop_headsign, None);
        assert_eq!(mta.start_pickup_drop_off_window, None);
        assert_eq!(mta.end_pickup_drop_off_window, None);
        assert_eq!(mta.pickup_type, None);
        assert_eq!(mta.drop_off_type, None);
        assert_eq!(mta.continuous_pickup, None);
        assert_eq!(mta.continuous_drop_off, None);
        assert_eq!(mta.shape_dist_traveled, None);
        assert_eq!(mta.timepoint, None);
        assert_eq!(mta.pickup_booking_rule_id, None);
        assert_eq!(mta.drop_off_booking_rule_id, None);

        Ok(())
    }
}
