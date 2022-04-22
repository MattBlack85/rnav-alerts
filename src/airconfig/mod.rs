use std::collections::HashMap;
use std::io::Write;
use std::path::Path;

use configparser::ini::Ini;
use dirs;

static CONFIG_PATH: &'static str = ".config/rnav_alerts/";
static CONFIG_FILENAME: &'static str = "alerts.cfg";

pub fn check_config_exists() -> bool {
    let full_path = format!(
        "{}/{}/{}",
        dirs::home_dir().unwrap().as_path().display().to_string(),
        CONFIG_PATH,
        CONFIG_FILENAME
    );
    let full_config_path = format!(
        "{}/{}",
        dirs::home_dir().unwrap().as_path().display().to_string(),
        CONFIG_PATH
    );

    if !Path::new(&full_config_path).exists() {
        std::fs::create_dir(&full_config_path).unwrap();
    }

    match Path::new(&full_path).exists() {
        true => return true,
        false => {
            std::fs::File::create(&full_path).unwrap();
            return false;
        }
    };
}

pub fn read_config() -> HashMap<String, HashMap<String, Option<String>>> {
    let mut config = Ini::new();
    let map = config
        .load(format!(
            "{}/{}/{}",
            dirs::home_dir().unwrap().as_path().display().to_string(),
            CONFIG_PATH,
            CONFIG_FILENAME
        ))
        .unwrap();
    return map;
}

pub fn store_config(content: String) {
    let full_path = format!(
        "{}/{}/{}",
        dirs::home_dir().unwrap().as_path().display().to_string(),
        CONFIG_PATH,
        CONFIG_FILENAME
    );

    match std::fs::write(&full_path, content.as_bytes()) {
        Ok(_) => (),
        Err(e) => panic!("{}", e),
    }
}
