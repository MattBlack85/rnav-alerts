use dialoguer::Input;
use geo::point;
use geo::prelude::GeodesicDistance;

use std::io::prelude::*;
use std::net::TcpStream;

use std::error::Error;
use std::str;

use serde::Deserialize;

#[derive(Deserialize)]
struct Airdata {
    msg: String,
    msg_type: String,
    dummy_1: String,
    dummy_2: String,
    aircraft_address: String,
    dummy_3: String,
    date_received: String,
    hour_received: String,
    date_written: String,
    hour_written: String,
    callsign: String,
    altitude: String,
    groundspeed: String,
    ground_track: String,
    latitude: String,
    longitude: String,
    vertical_rate: String,
    squawk: String,
    alert_flag: String,
    emergency_flag: String,
    spi_flag: String,
    ground_flag: String,
}

fn main() -> Result<(), Box<dyn Error>> {
    // check if the config file is present, make one if not (stored under ~/.config/rnav-alert.cfg

    // read the config and check if we have the coordinates of the place we want to monitor

    // if not ask for the coordinates of the place, first latitude then longitude, invert them before storing them
    println!("First, I will ask you about the place that I should monitor\n");
    let latitude: String = Input::new()
        .with_prompt("Provide the latitude")
        .interact_text()
        .unwrap();

    let longitude: String = Input::new()
        .with_prompt("Provide the longitude")
        .interact_text()
        .unwrap();

    let place = point!(x: longitude.parse::<f64>().unwrap(), y: latitude.parse::<f64>().unwrap());
    let mut stream = TcpStream::connect("10.0.1.37:30003")?;

    loop {
        let mut buf = [0; 8192];
        let bytes = stream.read(&mut buf)?;
        let s = str::from_utf8(&buf[0..bytes]).unwrap();

        let mut rdr = csv::Reader::from_reader(s.as_bytes());
        for result in rdr.records() {
            // The iterator yields Result<StringRecord, Error>, so we check the
            // error here.
            // let record = result?;
	    let airdata: Airdata = result?.deserialize(None)?;

            let lat = airdata.latitude;
            let lon = airdata.longitude;

            if lat != "" && lon != "" {
                let plane = point!(x: lon.parse::<f64>().unwrap(), y: lat.parse::<f64>().unwrap());
                let distance = place.geodesic_distance(&plane);
                println!(
                    "The distance between you and the plane {} is {:.3} Km",
                    airdata.aircraft_address,
                    distance / 1000f64
                );
            }
        }
    }
}
