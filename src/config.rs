use serde::{Deserialize, Serialize};
use std::env;
use std::fs;
use std::io::Write;

const CONFIG_DIR: &str = ".config/cards/";

#[derive(Serialize, Deserialize)]
pub struct Config {
    pub db_file: String,
    pub highlight_color: u8,
}

pub fn init() {
    let home_dir: String = env::var("HOME").unwrap();
    if std::path::Path::new(format!("{}/{}", home_dir, CONFIG_DIR).as_str()).exists() {
    } else {
        fs::create_dir(format!("{}/{}", home_dir, CONFIG_DIR)).unwrap();
        let mut file = fs::File::create(format!("{}/{}{}", home_dir, CONFIG_DIR, "config.json")).unwrap();
        file.write(b"{\n \"db_file\": \".config/cards/cards.db\",\n \"highlight_color\": 4\n}").unwrap();
    }
}

pub fn get_db_file() -> String {
    let home_dir: String = env::var("HOME").unwrap();
    let json: String = fs::read_to_string(format!("{}/{}{}", home_dir, CONFIG_DIR, "config.json")).unwrap();
    let config: Config = serde_json::from_str(json.as_str()).unwrap_or(
        Config {
            db_file: format!("{}/{}{}", home_dir, CONFIG_DIR, "cards.db"),
            highlight_color: 7
        }
    ); 

    if std::path::Path::new(format!("{}/{}", &home_dir, &config.db_file).as_str()).exists() {
        format!("{}/{}", &home_dir, &config.db_file)
    } else {
        format!("{}/{}{}", &home_dir, CONFIG_DIR, "cards.db")
    }
}

pub fn get_db_file_raw() -> String {
    let home_dir: String = env::var("HOME").unwrap();
    let json: String = fs::read_to_string(format!("{}/{}{}", home_dir, CONFIG_DIR, "config.json")).unwrap();
    let config: Config = serde_json::from_str(json.as_str()).unwrap(); 
    
    config.db_file
}

pub fn get_highlight_color() -> u8 {
    let home_dir: String = env::var("HOME").unwrap();
    let json: String = fs::read_to_string(format!("{}/{}{}", home_dir, CONFIG_DIR, "config.json")).unwrap();
    let config: Config = serde_json::from_str(json.as_str()).unwrap_or(Config {
        db_file: "".to_string(),
        highlight_color: 7
    }); 

    config.highlight_color
}

pub fn set_config(db_file: String, highlight_color: u8) {
   let config = Config {
       db_file,
       highlight_color,
   }; 

   let home_dir: String = env::var("HOME").unwrap();
   let json = serde_json::to_string_pretty(&config).unwrap();
   fs::remove_file(format!("{}/{}{}", home_dir, CONFIG_DIR, "config.json")).unwrap();
   let mut file = fs::File::create(format!("{}/{}{}", home_dir, CONFIG_DIR, "config.json")).unwrap();
   file.write(json.as_bytes()).unwrap();
}
