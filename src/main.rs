use regex::Regex;
use reqwest::header;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::fs;

#[derive(Debug, Deserialize, Serialize)]
struct Champion {
    id: i16,
    name: String,
    title: String,
    tags: Vec<String>,
    region: Region,
}

#[derive(Debug, Deserialize, Serialize)]
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

#[tokio::main]
async fn main() {
    let client = reqwest::ClientBuilder::new()
        .user_agent("reqwest/0.10.0")
        .build()
        .unwrap();

    let response = client
        .get("https://universe-meeps.leagueoflegends.com/v1/en_us/champions/ahri/index.json")
        .header(header::CONTENT_TYPE, "application/json")
        .send()
        .await;

    println!("{:?}", response);
}

async fn get_race_and_region() {
    //request a get to the riot api
    let response = reqwest::get("https://api.spotify.com/v1/search").await;

    println!("{:?}", response);
}

fn create_base() {
    //creates starting structure from riot data
    let paths = fs::read_dir("assets/13.5.1/data/en_US/champion").unwrap();
    let re = Regex::new(r"/([a-zA-Z]+).json").unwrap();

    for path in paths {
        let path_str = path.unwrap().path().display().to_string();
        let matches = re.captures(&path_str).unwrap();

        let json_input = fs::read_to_string(&path_str).expect("Unable to read file");
        let v: Value = serde_json::from_str(&json_input).unwrap();
        let champion = Champion {
            id: v["data"][&matches[1]]["key"]
                .as_str()
                .unwrap()
                .parse::<i16>()
                .unwrap(),
            name: v["data"][&matches[1]]["name"].as_str().unwrap().to_string(),
            title: v["data"][&matches[1]]["title"]
                .as_str()
                .unwrap()
                .to_string(),
            tags: v["data"][&matches[1]]["tags"]
                .as_array()
                .unwrap()
                .to_vec()
                .iter()
                .map(|x| x.as_str().unwrap().to_string())
                .collect(),
            region: Region::Runeterra,
        };

        println!("{:?}", champion);

        let out_json = serde_json::to_string(&champion).unwrap();
        //save to file
        let out_path = format!("assets/new_format/{}.json", &matches[1]);
        fs::write(out_path, out_json).expect("Unable to write file");
    }
}

// fn print_type_of<T>(_: &T) {
//     println!("{}", std::any::type_name::<T>())
// }
