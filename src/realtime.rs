mod protos;

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

            assert_eq!(ntrip_updates, $ntus);
            assert_eq!(ntrip_mods, $ntms);
            assert_eq!(nvehicles, $nvs);
            assert_eq!(nalerts, $nas);
            assert_eq!(nshapes, $nshs);
            assert_eq!(nstops, $nsts);
        };
    }

    #[test]
    fn test_n() {
        test_feed!("./test_data/realtime/nqrw", 268, 134, 0, 134, 0, 0, 0);
    }
}
