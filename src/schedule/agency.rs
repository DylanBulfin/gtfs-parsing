use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct Agency {
    pub agency_id: Option<String>,
    pub agency_name: String,
    pub agency_url: String,
    pub agency_timezone: String,
    pub agency_lang: Option<String>,
    pub agency_phone: Option<String>,
    pub agency_fare_url: Option<String>,
    pub agency_email: Option<String>,
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use super::*;

    #[test]
    fn test_agency() -> Result<(), csv::Error> {
        let path = PathBuf::from("./test_data/schedule/agency.txt");
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
