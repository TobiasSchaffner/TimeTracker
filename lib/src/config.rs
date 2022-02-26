use chrono::{Date, Utc};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use std::fs;

use crate::time;

const BASE_DIR: &'static str = "time-tracker";

#[derive(Serialize, Deserialize)]
pub struct Config {
    pub activities_timestamp: Vec<i64>,
    pub activities_duration: Vec<i64>,
}

fn get_config_file(date: Date<Utc>) -> PathBuf {
    match dirs::config_dir() {
        Some(mut conf) => {
            conf.push(format!("{}/{}.json", BASE_DIR, date));
            conf
        }
        None => panic!("Config path does not exist!"),
    }
}

pub fn config_exists(date: Date<Utc>) -> bool {
    get_config_file(date).exists()
}

pub fn load_config(date: Date<Utc>) -> Config {
    match fs::read_to_string(get_config_file(date)) {
        Ok(config) => serde_json::from_str(&config).expect("Malformed Json!"),
        Err(_) => Config {
            activities_timestamp: vec![time::current_time()],
            activities_duration: vec![0],
        },
    }
}

pub fn save_config(config: &Config) {
    fs::write(
        get_config_file(time::current_date()),
        serde_json::to_string(config).expect("Could not serialize config."),
    )
    .expect("Could not save config file!");
}