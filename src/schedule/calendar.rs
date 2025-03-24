use serde::Deserialize;

#[derive(Debug, Deserialize, PartialEq, Eq)]
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

#[derive(Debug, Deserialize)]
pub struct Calendar {
    service_id: String,
    sunday: Activity,
    monday: Activity,
    tuesday: Activity,
    wednesday: Activity,
    thursday: Activity,
    friday: Activity,
    saturday: Activity,
    start_date: String,
    end_date: String,
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
pub struct CalendarDate {
    service_id: String,
    date: String,
    exception_type: ExceptionType,
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use super::*;

    #[test]
    fn test_calendar() -> Result<(), csv::Error> {
        let path = PathBuf::from("./test_data/calendar.txt");
        let mut reader = csv::Reader::from_path(path)?;
        let mut res: Vec<Calendar> = Vec::new();

        for rec in reader.deserialize() {
            res.push(rec?);
        }

        assert_eq!(res.len(), 3);

        let mta = res.pop().unwrap();

        assert_eq!(mta.service_id, "Weekday");
        assert_eq!(mta.sunday, Activity::Inactive);
        assert_eq!(mta.monday, Activity::Active);
        assert_eq!(mta.tuesday, Activity::Active);
        assert_eq!(mta.wednesday, Activity::Active);
        assert_eq!(mta.thursday, Activity::Active);
        assert_eq!(mta.friday, Activity::Active);
        assert_eq!(mta.saturday, Activity::Inactive);
        assert_eq!(mta.start_date, "20250323");
        assert_eq!(mta.end_date, "20250518");

        Ok(())
    }

    #[test]
    fn test_calendar_dates() -> Result<(), csv::Error> {
        let path = PathBuf::from("./test_data/calendar_dates.txt");
        let mut reader = csv::Reader::from_path(path)?;
        let mut res: Vec<CalendarDate> = Vec::new();

        for rec in reader.deserialize() {
            res.push(rec?);
        }

        assert_eq!(res.len(), 456);

        let mta = res.pop().unwrap();

        assert_eq!(mta.service_id, "SIR-FA2017-SI017-Weekday-08_C20");
        assert_eq!(mta.date, "20250526");
        assert_eq!(mta.exception_type, ExceptionType::Removed);

        Ok(())
    }
}
