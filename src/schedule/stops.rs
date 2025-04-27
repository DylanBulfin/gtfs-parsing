use serde::Deserialize;

#[derive(Debug, Deserialize, PartialEq, Eq, Clone)]
#[serde(from = "u32")]
pub enum LocationType {
    StopPlatform,
    Station,
    EntranceExit,
    GenericNode,
    BoardingArea,
}

impl From<u32> for LocationType {
    fn from(value: u32) -> Self {
        match value {
            0 => Self::StopPlatform,
            1 => Self::Station,
            2 => Self::EntranceExit,
            3 => Self::GenericNode,
            4 => Self::BoardingArea,
            _ => panic!("Invalid LocationType: {}", value),
        }
    }
}

#[derive(Debug, Deserialize, PartialEq, Eq, Clone)]
#[serde(from = "u32")]
pub enum WheelchairBoarding {
    NoInfo,
    SomeSupport,
    NoSupport,
}

impl From<u32> for WheelchairBoarding {
    fn from(value: u32) -> Self {
        match value {
            0 => Self::NoInfo,
            1 => Self::SomeSupport,
            2 => Self::NoSupport,
            _ => panic!("Invalid WheelchairBoarding: {}", value),
        }
    }
}

#[derive(Debug, Deserialize, PartialEq, Eq, Clone)]
pub struct Stop {
    pub stop_id: String,
    pub stop_code: Option<String>,
    pub stop_name: Option<String>,
    pub tts_stop_name: Option<String>,
    pub stop_desc: Option<String>,
    pub stop_lat: Option<String>,
    pub stop_lon: Option<String>,
    pub zone_id: Option<String>,
    pub stop_url: Option<String>,
    pub location_type: Option<LocationType>,
    pub parent_station: Option<String>,
    pub stop_timezone: Option<String>,
    pub wheelchair_boarding: Option<WheelchairBoarding>,
    pub level_id: Option<String>,
    pub platform_code: Option<String>,
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use super::*;

    #[test]
    fn test_stops() -> Result<(), csv::Error> {
        let path = PathBuf::from("./test_data/schedule/stops.txt");
        let mut reader = csv::Reader::from_path(path)?;
        let mut res: Vec<Stop> = Vec::new();

        for rec in reader.deserialize() {
            res.push(rec?);
        }

        assert_eq!(res.len(), 1497);

        let mta = res.pop().unwrap();

        assert_eq!(mta.stop_id, "S31S");
        assert_eq!(mta.stop_code, None);
        assert_eq!(mta.stop_name, Some("St George".to_owned()));
        assert_eq!(mta.tts_stop_name, None);
        assert_eq!(mta.stop_desc, None);
        assert_eq!(mta.stop_lat, Some("40.643748".to_owned()));
        assert_eq!(mta.stop_lon, Some("-74.073643".to_owned()));
        assert_eq!(mta.zone_id, None);
        assert_eq!(mta.stop_url, None);
        assert_eq!(mta.location_type, None);
        assert_eq!(mta.parent_station, Some("S31".to_owned()));
        assert_eq!(mta.stop_timezone, None);
        assert_eq!(mta.wheelchair_boarding, None);
        assert_eq!(mta.level_id, None);
        assert_eq!(mta.platform_code, None);

        Ok(())
    }
}
