use json;
use std::{
    fs::{self, File},
    io::BufReader,
};

struct Champion {
    id: i16,
    name: String,
    title: String,
    tags: Vec<String>,
    region: Region,
}

enum Region {
    Ionia,
    Demacia,
    Noxus,
    Freljord,
    Piltover,
    Zaun,
    BandleCity,
    Shurima,
    Targon,
    ShadowIsles,
    Ixtal,
    Bilgewater,
    Runeterra,
}

fn main() {
    // let champion = println!("Hello, world!");
    //open json file and read it
    // let file_str = fs::read_to_string("assets/13.4.1/data/en_GB/champion/Aatrox.json")?.parse()?;
    // let json = json::parse(file_str).unwrap();
    // println!("{}", json)
}
