use std::collections::HashMap;
use std::path::Path;

use configparser::ini::Ini;
use dialoguer::console::Term;
use dialoguer::{Input, Select};
use dirs;

static CONFIG_PATH: &'static str = ".config/rnav_alerts/";
static CONFIG_FILENAME: &'static str = "alerts.cfg";

/// Checks if the configuration file exists, if not create it.
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

/// Convenient entry point to read the configuration file, the configuration
/// is parsed and dumped into a HashMap which is then returned to the user so values can be fetched.
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

/// Creates the configuration file and stores it on the disk.
///
/// This function is meant to be used only once, when the configuration file
/// is not found on the disk. It will drive the user through some questions
/// building an ini compatible file that will be stored at $HOME/.config/rnav-alerts/alerts.cfg
pub fn setup_config() {
    let mut config = String::new();

    println!("First, I will ask you about the place that I should monitor\n");

    // Ask the user for the current 1090dump installation latitude
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

    let units = vec!["km", "mi"];
    let unit_chosen = Select::new()
        .with_prompt("Select how units should be displayed")
        .items(&units)
        .default(0)
        .interact_on_opt(&Term::stderr())
        .unwrap();

    let alerting_distance: String = Input::new()
        .with_prompt("Provide the radius of the area we want to monitor")
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
        "[generic]\n1090dump_host = {}\n1090dump_port = {}\nunits = {}\n\n",
        host,
        port,
        units[unit_chosen.unwrap()]
    );

    config = config + &geo_conf + &limit_config + &generic_config;

    let full_path = format!(
        "{}/{}/{}",
        dirs::home_dir().unwrap().as_path().display().to_string(),
        CONFIG_PATH,
        CONFIG_FILENAME
    );

    match std::fs::write(&full_path, config.as_bytes()) {
        Ok(_) => (),
        Err(e) => panic!("{}", e),
    }
}
