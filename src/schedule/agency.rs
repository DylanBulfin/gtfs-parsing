use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Agency {
    agency_id: Option<String>,
    agency_name: String,
    agency_url: String,
    agency_timezone: String,
    agency_lang: Option<String>,
    agency_phone: Option<String>,
    agency_fare_url: Option<String>,
    agency_email: Option<String>,
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use super::*;

    #[test]
    fn test_agency() -> Result<(), csv::Error> {
        let path = PathBuf::from("./test_data/agency.txt");
        let mut reader = csv::Reader::from_path(path)?;
        let mut res: Vec<Agency> = Vec::new();

        for rec in reader.deserialize() {
            res.push(rec?);
        }

        assert_eq!(res.len(), 1);

        let mta = res.pop().unwrap();

        assert_eq!(mta.agency_id, Some("MTA NYCT".to_owned()));
        assert_eq!(mta.agency_name, "MTA New York City Transit".to_owned());
        assert_eq!(mta.agency_url, "http://www.mta.info");
        assert_eq!(mta.agency_timezone, "America/New_York");
        assert_eq!(mta.agency_lang, Some("en".to_owned()));
        assert_eq!(mta.agency_phone, Some("718-330-1234".to_owned()));
        assert_eq!(mta.agency_fare_url, None);
        assert_eq!(mta.agency_email, None);

        Ok(())
    }
}
