use protobuf::Message;
use protos::gtfs_realtime::FeedMessage;

pub mod protos;

pub fn try_parse_bytes(bytes: &[u8]) -> Option<FeedMessage> {
    FeedMessage::parse_from_bytes(bytes).ok()
}

#[cfg(test)]
mod tests {
    use std::{fs::File, io::Read};

    use protobuf::{Message, MessageField};

    use super::{
        protos::gtfs_realtime::{FeedEntity, FeedMessage},
        *,
    };

    macro_rules! test_feed {
        (
            // $(name = )? $name:ident,
            $(path = )? $path:literal,
            $(num_total = )? $ntot:literal,
            $(num_trip_updates = )? $ntus:literal,
            $(num_trip_modifications = )? $ntms:literal,
            $(num_vehicles = )? $nvs:literal,
            $(num_alerts = )? $nas:literal,
            $(num_shapes = )? $nshs:literal,
            $(num_stops = )? $nsts:literal
        ) => {
            let mut bytes: Vec<u8> = Vec::new();
            let mut file = File::open($path).expect("Unable to open file");
            file.read_to_end(&mut bytes).expect("Unable to read file");
            let feed =
                FeedMessage::parse_from_bytes(bytes.as_slice()).expect("Unable to parse file");

            assert_eq!(feed.entity.len(), $ntot);

            let ntrip_updates = feed
                .entity
                .iter()
                .filter(|e| e.trip_update.is_some())
                .count();
            let ntrip_mods = feed
                .entity
                .iter()
                .filter(|e| e.trip_modifications.is_some())
                .count();
            let nvehicles = feed.entity.iter().filter(|e| e.vehicle.is_some()).count();
            let nalerts = feed.entity.iter().filter(|e| e.alert.is_some()).count();
            let nshapes = feed.entity.iter().filter(|e| e.shape.is_some()).count();
            let nstops = feed.entity.iter().filter(|e| e.stop.is_some()).count();

            assert_eq!(
                ntrip_updates, $ntus,
                "num_trip_updates: {} vs {}",
                ntrip_updates, $ntus
            );
            assert_eq!(
                ntrip_mods, $ntms,
                "num_trip_modificationss: {} {}",
                ntrip_mods, $ntms
            );
            assert_eq!(nvehicles, $nvs, "num_vehicles: {} {}", nvehicles, $nvs);
            assert_eq!(nalerts, $nas, "num_alerts: {} {}", nalerts, $nas);
            assert_eq!(nshapes, $nshs, "num_shapes: {} {}", nshapes, $nshs);
            assert_eq!(nstops, $nsts, "num_stops: {} {}", nstops, $nsts);
        };
    }

    #[test]
    fn test_nqrw() {
        test_feed!(
            "./test_data/realtime/nqrw",
            num_total = 268,
            num_trip_updates = 134,
            num_trip_modifications = 0,
            num_vehicles = 134,
            num_alerts = 0,
            num_shapes = 0,
            num_stops = 0
        );
    }

    #[test]
    fn test_1234567s() {
        test_feed!(
            "./test_data/realtime/1234567S",
            num_total = 247,
            num_trip_updates = 155,
            num_trip_modifications = 0,
            num_vehicles = 91,
            num_alerts = 1,
            num_shapes = 0,
            num_stops = 0
        );
    }

    #[test]
    fn test_ace() {
        test_feed!(
            "./test_data/realtime/ace",
            num_total = 396,
            num_trip_updates = 198,
            num_trip_modifications = 0,
            num_vehicles = 198,
            num_alerts = 0,
            num_shapes = 0,
            num_stops = 0
        );
    }

    #[test]
    fn test_bdfm() {
        test_feed!(
            "./test_data/realtime/bdfm",
            num_total = 256,
            num_trip_updates = 128,
            num_trip_modifications = 0,
            num_vehicles = 128,
            num_alerts = 0,
            num_shapes = 0,
            num_stops = 0
        );
    }

    #[test]
    fn test_g() {
        test_feed!(
            "./test_data/realtime/g",
            num_total = 90,
            num_trip_updates = 45,
            num_trip_modifications = 0,
            num_vehicles = 45,
            num_alerts = 0,
            num_shapes = 0,
            num_stops = 0
        );
    }

    #[test]
    fn test_jz() {
        test_feed!(
            "./test_data/realtime/jz",
            num_total = 92,
            num_trip_updates = 46,
            num_trip_modifications = 0,
            num_vehicles = 46,
            num_alerts = 0,
            num_shapes = 0,
            num_stops = 0
        );
    }

    #[test]
    fn test_l() {
        test_feed!(
            "./test_data/realtime/l",
            num_total = 41,
            num_trip_updates = 25,
            num_trip_modifications = 0,
            num_vehicles = 16,
            num_alerts = 0,
            num_shapes = 0,
            num_stops = 0
        );
    }
}
