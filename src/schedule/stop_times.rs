use std::marker::PhantomData;

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
pub struct StopTime {
    pub trip_id: String,
    pub arrival_time: Option<String>,
    pub departure_time: Option<String>,
    pub stop_id: Option<String>,
    pub location_group_id: Option<String>,
    pub location_id: Option<String>,
    pub stop_sequence: Option<u32>,
    pub stop_headsign: Option<String>,
    pub start_pickup_drop_off_window: Option<String>,
    pub end_pickup_drop_off_window: Option<String>,
    pub pickup_type: Option<PickupType>,
    pub drop_off_type: Option<DropoffType>,
    pub continuous_pickup: Option<PickupType>,
    pub continuous_drop_off: Option<DropoffType>,
    pub shape_dist_traveled: Option<f64>,
    pub timepoint: Option<Timepoint>,
    pub pickup_booking_rule_id: Option<String>,
    pub drop_off_booking_rule_id: Option<String>,
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use super::*;

    #[test]
    fn test_stop_times() -> Result<(), csv::Error> {
        let path = PathBuf::from("./test_data/schedule/stop_times_abbrev.txt");
        let mut reader = csv::Reader::from_path(path)?;
        let mut res: Vec<StopTime> = Vec::new();

        for rec in reader.deserialize() {
            res.push(rec?);
        }

        assert_eq!(res.len(), 10000);

        let mta = res.pop().unwrap();

        assert_eq!(mta.trip_id, "AFA24GEN-1038-Sunday-00_123700_1..N03R");
        assert_eq!(mta.arrival_time, Some("20:44:00".to_owned()));
        assert_eq!(mta.departure_time, Some("20:44:00".to_owned()));
        assert_eq!(mta.stop_id, Some("135N".to_owned()));
        assert_eq!(mta.location_group_id, None);
        assert_eq!(mta.location_id, None);
        assert_eq!(mta.stop_sequence, Some(6));
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
