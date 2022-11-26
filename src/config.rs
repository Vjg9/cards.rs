use serde::{Deserialize, Serialize};
use std::env;
use std::fs;
use std::io::Write;

const CONFIG_DIR: &str = ".config/cards/";

#[derive(Serialize, Deserialize)]
pub struct Config {
    pub db_file: String,
}

pub fn init() {
    let home_dir: String = env::var("HOME").unwrap();
    if std::path::Path::new(format!("{}/{}", home_dir, CONFIG_DIR).as_str()).exists() {
    } else {
        fs::create_dir(format!("{}/{}", home_dir, CONFIG_DIR)).unwrap();
        let mut file = fs::File::create(format!("{}/{}{}", home_dir, CONFIG_DIR, "config.json")).unwrap();
        file.write(b"{\n \"db_file\": \".config/cards/cards.db\"\n}").unwrap();
    }
}

pub fn get_db_file() -> String {
    let home_dir: String = env::var("HOME").unwrap();
    println!("{}", &format!("{}/{}{}", home_dir, CONFIG_DIR, "config.json"));
    let json: String = fs::read_to_string(format!("{}/{}{}", home_dir, CONFIG_DIR, "config.json")).unwrap();
    let config: Config = serde_json::from_str(json.as_str()).unwrap(); 

    println!("{}/{}", &home_dir, &config.db_file);

    format!("{}/{}", &home_dir, &config.db_file)
}
