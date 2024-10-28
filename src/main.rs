use futures::lock::Mutex;
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
    race: String,
    year: String,
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
    Void,
    Runeterra,
    Unaffiliated,
    Unknown,
    NotSet,
}

#[tokio::main]
async fn main() {
    let mut champion_list: Vec<Champion> = Vec::new();
    create_base(&mut champion_list);
    get_race_and_region(&mut champion_list).await;
    save_champ_to_file(&champion_list);
}

fn save_champ_to_file(champion_list: &Vec<Champion>) {
    for champion in champion_list {
        let json = serde_json::to_string(&champion).unwrap();
        fs::write(format!("assets/champions/{}.json", champion.name), json)
            .expect("Unable to write file");
    }
}

// gets race and region from riot api
async fn get_race_and_region(champion_list: &mut Vec<Champion>) {
    let client = reqwest::ClientBuilder::new()
        .user_agent("reqwest/0.10.0")
        .build()
        .unwrap();

    for champ in champion_list {
        let url = format!(
            "https://universe-meeps.leagueoflegends.com/v1/en_us/champions/{}/index.json",
            match champ.name.as_str() {
                "Nunu & Willump" => "nunu".to_string(),
                "Wukong" => "monkeyking".to_string(),
                _ => champ
                    .name
                    .to_ascii_lowercase()
                    .replace(" ", "")
                    .replace(".", "")
                    .replace("'", ""),
            }
        );

        let response = client
            .get(url)
            .header(header::CONTENT_TYPE, "application/json")
            .send()
            .await
            .unwrap();

        //todo: handle errors

        //get and set region
        let mut re = Regex::new(r#""associated-faction-slug": "(.+)","#).unwrap();
        let json = response.text().await.unwrap();
        let region = match re.captures(&json) {
            Some(x) => match x.get(1).unwrap().as_str() {
                "ionia" => Region::Ionia,
                "demacia" => Region::Demacia,
                "noxus" => Region::Noxus,
                "freljord" => Region::Freljord,
                "piltover" => Region::Piltover,
                "zaun" => Region::Zaun,
                "bandle-city" => Region::BandleCity,
                "shurima" => Region::Shurima,
                "mount-targon" => Region::Targon,
                "shadow-isles" => Region::ShadowIsles,
                "ixtal" => Region::Ixtal,
                "bilgewater" => Region::Bilgewater,
                "void" => Region::Void,
                "runeterra" => Region::Runeterra,
                "unaffiliated" => Region::Unaffiliated,
                _ => Region::Unknown,
            },
            None => Region::NotSet,
        };
        champ.region = region;

        //get and set race
        re = Regex::new(r#""races": \[\s+\{\s+"name": "(.+)","#).unwrap();
        let race = match re.captures(&json) {
            Some(x) => x.get(1).unwrap().as_str(),
            None => "Unknown",
        };
        champ.race = race.to_string();

        //get and set year
        re = Regex::new(r#""release\-date": "(\d{4})"#).unwrap();
        let year = match re.captures(&json) {
            Some(x) => x.get(1).unwrap().as_str(),
            _ => "XXXX",
        };
        champ.year = year.to_string();
    }
}

// creates base structure from riot data
fn create_base(champion_list: &mut Vec<Champion>) {
    //creates starting structure from riot data
    let paths = fs::read_dir("assets/13.4.1/data/en_US/champion").unwrap();
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
            region: Region::NotSet,
            race: "Unknown".to_string(),
            year: "XXXX".to_string(),
        };

        champion_list.push(champion);
    }
}
