use std::fs::File;
use std::collections::HashMap;
use std::io::BufReader;
use serde::Deserialize;
use serde_yaml;
use crate::areas::Room;


#[derive(Deserialize)]
pub struct Zone {
    rooms: Vec<Room>
}

#[derive(Deserialize)]
pub struct World {
    zones: Vec<Zone>
}

fn load_from_yaml(path: &str) {
    let f = File::open(path).expect("could not open file");
    let r = BufReader::new(f);

    let world: Zone = serde_yaml::from_reader(r).expect("could not deserialize");
}
