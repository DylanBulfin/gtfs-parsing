use serde::Deserialize;

#[derive(Debug, Deserialize, PartialEq, Eq, Clone, Copy)]
#[serde(from = "u32")]
pub enum Activity {
    Inactive,
    Active,
}

impl From<u32> for Activity {
    fn from(value: u32) -> Self {
        match value {
            0 => Self::Inactive,
            1 => Self::Active,
            _ => panic!("Invalid Availability: {}", value),
        }
    }
}

impl From<Activity> for bool {
    fn from(value: Activity) -> Self {
        value == Activity::Active
    }
}

#[derive(Debug, Deserialize)]
pub struct Service {
    pub service_id: String,
    pub sunday: Activity,
    pub monday: Activity,
    pub tuesday: Activity,
    pub wednesday: Activity,
    pub thursday: Activity,
    pub friday: Activity,
    pub saturday: Activity,
    pub start_date: String,
    pub end_date: String,
}

#[derive(Debug, Deserialize, PartialEq, Eq)]
#[serde(from = "u32")]
pub enum ExceptionType {
    Added,
    Removed,
}

impl From<u32> for ExceptionType {
    fn from(value: u32) -> Self {
        match value {
            1 => Self::Added,
            2 => Self::Removed,
            _ => panic!("Invalid ExceptionType: {}", value),
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct ServiceException {
    pub service_id: String,
    pub date: String,
    pub exception_type: ExceptionType,
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use super::*;

    #[test]
    fn test_calendar() -> Result<(), csv::Error> {
        let path = PathBuf::from("./test_data/schedule/calendar.txt");
        let mut reader = csv::Reader::from_path(path)?;
        let mut res: Vec<Service> = Vec::new();

        for rec in reader.deserialize() {
            res.push(rec?);
        }

        assert_eq!(res.len(), 71);

        let mta = res.pop().unwrap();

        assert_eq!(mta.service_id, "SIR-FA2017-SI017-Weekday-08_C17");
        assert_eq!(mta.sunday, Activity::Inactive);
        assert_eq!(mta.monday, Activity::Active);
        assert_eq!(mta.tuesday, Activity::Active);
        assert_eq!(mta.wednesday, Activity::Active);
        assert_eq!(mta.thursday, Activity::Active);
        assert_eq!(mta.friday, Activity::Active);
        assert_eq!(mta.saturday, Activity::Inactive);
        assert_eq!(mta.start_date, "20241216");
        assert_eq!(mta.end_date, "20250606");

        Ok(())
    }

    #[test]
    fn test_calendar_dates() -> Result<(), csv::Error> {
        let path = PathBuf::from("./test_data/schedule/calendar_dates.txt");
        let mut reader = csv::Reader::from_path(path)?;
        let mut res: Vec<ServiceException> = Vec::new();

        for rec in reader.deserialize() {
            res.push(rec?);
        }

        assert_eq!(res.len(), 406);

        let mta = res.pop().unwrap();

        assert_eq!(mta.service_id, "SIR-FA2017-SI017-Weekday-08_C17");
        assert_eq!(mta.date, "20250526");
        assert_eq!(mta.exception_type, ExceptionType::Removed);

        Ok(())
    }
}
