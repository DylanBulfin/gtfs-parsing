use serde::Deserialize;

#[derive(Debug, Deserialize, PartialEq, Eq)]
#[serde(from = "u32")]
pub enum TransferType {
    Recommended,
    Timed,       // The "to" vehicle will wait, this transfer is specifically timed
    MinimumTime, // This transfer requires a minimum amount of time
    Impossible,
    InSeat,
    NoInSeat,
}

impl From<u32> for TransferType {
    fn from(value: u32) -> Self {
        match value {
            0 => Self::Recommended,
            1 => Self::Timed,
            2 => Self::MinimumTime,
            3 => Self::Impossible,
            4 => Self::InSeat,
            5 => Self::NoInSeat,
            _ => panic!("Invalid TransferType: {}", value),
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct Transfer {
    from_stop_id: Option<String>,
    to_stop_id: Option<String>,
    from_route_id: Option<String>,
    to_route_id: Option<String>,
    from_trip_id: Option<String>,
    to_trip_id: Option<String>,
    transfer_type: TransferType,
    min_transfer_time: Option<u32>,
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use super::*;

    #[test]
    fn test_transfers() -> Result<(), csv::Error> {
        let path = PathBuf::from("./test_data/transfers.txt");
        let mut reader = csv::Reader::from_path(path)?;
        let mut res: Vec<Transfer> = Vec::new();

        for rec in reader.deserialize() {
            res.push(rec?);
        }

        assert_eq!(res.len(), 616);

        let mta = res.pop().unwrap();

        assert_eq!(mta.from_stop_id, Some("S04".to_owned()));
        assert_eq!(mta.to_stop_id, Some("S04".to_owned()));
        assert_eq!(mta.from_route_id, None);
        assert_eq!(mta.to_route_id, None);
        assert_eq!(mta.from_trip_id, None);
        assert_eq!(mta.to_trip_id, None);
        assert_eq!(mta.transfer_type, TransferType::MinimumTime);
        assert_eq!(mta.min_transfer_time, Some(180));

        Ok(())
    }
}
