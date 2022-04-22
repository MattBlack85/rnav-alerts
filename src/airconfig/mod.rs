use dialoguer::Input;

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

fn store_config(content: String) {
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

pub fn setup_config() {
    let mut config = String::new();

    println!("First, I will ask you about the place that I should monitor\n");

    let latitude: String = Input::new()
        .with_prompt("Provide the latitude")
        .interact_text()
        .unwrap();

    let longitude: String = Input::new()
        .with_prompt("Provide the longitude")
        .interact_text()
        .unwrap();

    let geo_conf = format!(
        "[geo]\nlatitude = {}\nlongitude = {}\n\n",
        latitude, longitude
    );

    let alerting_distance: String = Input::new()
        .with_prompt("Provide the radius of the are we want to monitor")
        .interact_text()
        .unwrap();

    let limit_config = format!("[limits]\nalerting_distance = {}\n\n", alerting_distance);

    let host: String = Input::new()
        .with_prompt("IP of the host running 1090dump (leave blank for default 127.0.0.1)")
        .default("127.0.0.1".into())
        .interact_text()
        .unwrap();

    let port: String = Input::new()
        .with_prompt("port for the CSV output run by 1090dump (leave blank for default 30003)")
        .default("30003".into())
        .interact_text()
        .unwrap();

    let generic_config = format!(
        "[generic]\n1090dump_host = {}\n1090dump_port = {}\n\n",
        host, port
    );

    config = config + &geo_conf + &limit_config + &generic_config;
    store_config(config);
}
