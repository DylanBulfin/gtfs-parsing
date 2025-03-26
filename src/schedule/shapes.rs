use std::collections::{
    HashMap, VecDeque,
    hash_map::{Entry, OccupiedEntry},
};

use serde::Deserialize;

#[derive(Debug, Deserialize, Clone, Copy)]
// Holds the base data for a point on a shape. Allows us to avoid storing the shape ID multiple times
pub struct ShapePointData {
    pub shape_pt_lat: f64,
    pub shape_pt_lon: f64,
    pub shape_dist_traveled: Option<f64>,
}

impl From<&ShapePoint> for ShapePointData {
    fn from(value: &ShapePoint) -> Self {
        Self {
            shape_dist_traveled: value.shape_dist_traveled,
            shape_pt_lon: value.shape_pt_lon,
            shape_pt_lat: value.shape_pt_lat,
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct ShapePoint {
    pub shape_id: String,
    pub shape_pt_sequence: u32,
    pub shape_pt_lat: f64,
    pub shape_pt_lon: f64,
    pub shape_dist_traveled: Option<f64>,
}

#[derive(Debug, Deserialize)]
pub struct Shape {
    pub shape_id: String,
    pub points: Vec<ShapePointData>,
}

impl Shape {
    pub fn process_points(points: &Vec<ShapePoint>) -> Vec<Self> {
        let mut map = HashMap::new();

        for point in points {
            let id = &point.shape_id;

            match map.entry(id) {
                Entry::Occupied(mut e) => {
                    let shape: &mut Shape = (e.get_mut());

                    shape.points.insert(shape.points.len(), point.into());
                }
                Entry::Vacant(mut e) => {
                    e.insert(Shape {
                        shape_id: id.to_string(),
                        points: Vec::new(),
                    });
                }
            }
        }

        map.into_values().collect()
    }
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

    #[test]
    fn test_process_points() -> Result<(), csv::Error> {
        let path = PathBuf::from("./test_data/shapes.txt");
        let mut reader = csv::Reader::from_path(path)?;
        let mut res: Vec<ShapePoint> = Vec::new();

        for rec in reader.deserialize() {
            res.push(rec?);
        }

        assert_eq!(res.len(), 176482);

        let mut shapes = Shape::process_points(&res);
        shapes.sort_by_key(|s| s.shape_id.to_owned());

        assert_eq!(shapes.len(), 311);

        let mut mta = shapes.pop().unwrap();

        assert_eq!(mta.points.len(), 689);
        assert_eq!(mta.shape_id, "SI.S31R");

        let point = mta.points.pop().unwrap();

        assert_eq!(point.shape_pt_lat, 40.512764);
        assert_eq!(point.shape_pt_lon, -74.251961);
        assert_eq!(point.shape_dist_traveled, None);

        Ok(())
    }
}
