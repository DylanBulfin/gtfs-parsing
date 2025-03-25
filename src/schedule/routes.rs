use serde::Deserialize;

use super::agency::Agency;

#[derive(Debug, Deserialize, PartialEq, Eq)]
#[serde(from = "u32")]
pub enum RouteType {
    LightRail,
    Subway,
    Rail,
    Bus,
    Ferry,
    CableTram,
    AerialLift,
    Funicular,
    TrolleyBus,
    Monorail,
}

impl From<u32> for RouteType {
    fn from(value: u32) -> Self {
        match value {
            0 => Self::LightRail,
            1 => Self::Subway,
            2 => Self::Rail,
            3 => Self::Bus,
            4 => Self::Ferry,
            5 => Self::CableTram,
            6 => Self::AerialLift,
            7 => Self::Funicular,
            11 => Self::TrolleyBus,
            12 => Self::Monorail,
            _ => panic!("Invalid RouteType: {}", value),
        }
    }
}

#[derive(Debug, Deserialize, PartialEq, Eq)]
#[serde(from = "u32")]
pub enum ContinuousType {
    Continuous,
    NoContinuous,
    CallAgency,
    CallDriver,
}

impl From<u32> for ContinuousType {
    fn from(value: u32) -> Self {
        match value {
            0 => Self::Continuous,
            1 => Self::NoContinuous,
            2 => Self::CallAgency,
            3 => Self::CallDriver,
            _ => panic!("Invalid ContinuousType: {}", value),
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct Route {
    route_id: String,
    agency_id: Option<String>,
    route_short_name: Option<String>,
    route_long_name: Option<String>,
    route_desc: Option<String>,
    route_type: RouteType,
    route_url: Option<String>,
    route_color: Option<String>,
    route_text_color: Option<String>,
    route_sort_order: Option<u32>,
    continuous_pickup: Option<ContinuousType>,
    continuous_drop_off: Option<ContinuousType>,
    network_id: Option<String>,
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use super::*;

    #[test]
    fn test_routes() -> Result<(), csv::Error> {
        let path = PathBuf::from("./test_data/routes.txt");
        let mut reader = csv::Reader::from_path(path)?;
        let mut res: Vec<Route> = Vec::new();

        for rec in reader.deserialize() {
            res.push(rec?);
        }

        assert_eq!(res.len(), 30);

        let mta = res.pop().unwrap();

        assert_eq!(mta.route_id, "Z");
        assert_eq!(mta.agency_id, Some("MTA NYCT".to_owned()));
        assert_eq!(mta.route_short_name, Some("Z".to_owned()));
        assert_eq!(mta.route_long_name, Some("Nassau St Express".to_owned()));
        assert!(mta.route_desc.is_some() && mta.route_desc.unwrap().len() > 0);
        assert_eq!(mta.route_type, RouteType::Subway);
        assert_eq!(
            mta.route_url,
            Some("http://web.mta.info/nyct/service/pdf/tjcur.pdf".to_owned())
        );
        assert_eq!(mta.route_color, Some("996633".to_owned()));
        assert_eq!(mta.route_text_color, None);
        assert_eq!(mta.route_sort_order, None);
        assert_eq!(mta.continuous_pickup, None);
        assert_eq!(mta.continuous_drop_off, None);
        assert_eq!(mta.network_id, None);

        Ok(())
    }
}
