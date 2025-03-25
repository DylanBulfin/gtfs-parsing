use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct ShapePoint {
    shape_id: String,
    shape_pt_lat: f64,
    shape_pt_lon: f64,
    shape_pt_sequence: u32,
    shape_dist_traveled: Option<f64>,
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use super::*;

    #[test]
    fn test_shapes() -> Result<(), csv::Error> {
        let path = PathBuf::from("./test_data/shapes.txt");
        let mut reader = csv::Reader::from_path(path)?;
        let mut res: Vec<ShapePoint> = Vec::new();

        for rec in reader.deserialize() {
            res.push(rec?);
        }

        assert_eq!(res.len(), 176482);

        let mta = res.pop().unwrap();

        assert_eq!(mta.shape_id, "SI.S31R");
        assert_eq!(mta.shape_pt_lat, 40.512764);
        assert_eq!(mta.shape_pt_lon, -74.251961);
        assert_eq!(mta.shape_pt_sequence, 689);
        assert_eq!(mta.shape_dist_traveled, None);

        Ok(())
    }
}
