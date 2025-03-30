use std::{fs::File, io::Read};

use gtfs_parsing::realtime::protos::gtfs_realtime::{FeedMessage, VehiclePosition};
use protobuf::{Message, MessageField};

fn main() {
    let mut bytes: Vec<u8> = Vec::new();
    let mut file = File::open("./test_data/realtime/1234567S").expect("Unable to open file");
    file.read_to_end(&mut bytes).expect("Unable to read file");
    let feed = FeedMessage::parse_from_bytes(bytes.as_slice()).expect("Unable to parse file");

    for entity in feed.entity {
        if let MessageField(Some(pos)) = entity.vehicle {
            // if let Some(stat) = pos.current_status {
            //     print!("{:?}", stat)
            // }
            if let Some(seq) = pos.current_stop_sequence {
                if let Some(id) = pos.stop_id {
                    println!("{:?} {:?} {:?} {:?}", id, seq, pos.current_status, "");
                } else {
                    println!("{:?}", seq)
                }
            } else {
                if let Some(id) = pos.stop_id {
                    println!("{:?}", id)
                } else {
                    println!("NOOOOO")
                }
            }
        }
    }
}
