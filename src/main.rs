use geo::point;
use geo::prelude::GeodesicDistance;

use std::io::prelude::*;
use std::net::TcpStream;

use std::error::Error;
use std::str;

use log::{debug, error, info};

mod aero;
mod airconfig;

use aero::db::Planes;
use aero::structs::Airdata;

fn main() -> Result<(), Box<dyn Error>> {
    env_logger::init();
    if !airconfig::check_config_exists() {
        airconfig::setup_config();
    }

    let conf = airconfig::read_config();

    let latitude = conf["geo"]["latitude"].clone().unwrap();
    let longitude = conf["geo"]["longitude"].clone().unwrap();
    let host = conf["generic"]["1090dump_host"].clone().unwrap();
    let port = conf["generic"]["1090dump_port"].clone().unwrap();
    let units = conf["generic"]["units"].clone().unwrap();
    let alerting_distance = conf["limits"]["alerting_distance"]
        .clone()
        .unwrap()
        .parse::<f64>()
        .unwrap();

    // This is the place of the listener, from here we base our assumptions about distance
    let place = point!(x: longitude.parse::<f64>().unwrap(), y: latitude.parse::<f64>().unwrap());

    // Open a TCP stream connection to the socket where CSV data are published by the 1090 program
    let mut stream = TcpStream::connect(format!("{}:{}", host, port))?;
    let mut planes = Planes::new(latitude, longitude);

    loop {
        let mut buf = [0; 16384];
        let bytes = stream.read(&mut buf)?;
        let s = str::from_utf8(&buf[0..bytes]).unwrap();

        let mut rdr = csv::Reader::from_reader(s.as_bytes());
        for result in rdr.records() {
            // The iterator yields Result<StringRecord, Error>, so we check the
            // error here.
            // let record = result?;
            let airdata: Airdata = result?.deserialize(None)?;

            if airdata.latitude != "" && airdata.longitude != "" {
                let address = airdata.aircraft_address.clone();
                if planes.already_seen(&address) {
                    debug!("plane {} already seen, checking if approaching", &address);
                    debug!("checking if it is too early to append data");

                    if !planes.check_old_data_too_fresh(airdata.clone()) {
                        debug!("appending data plane {}", &address);
                        planes.add_data_for_aircraft(airdata.clone());
                    }

                    match planes.is_plane_approaching(&address) {
                        0 => {
                            let plane = point!(x: airdata.longitude.parse::<f64>().unwrap(), y: airdata.latitude.parse::<f64>().unwrap());
                            let mut distance = place.geodesic_distance(&plane);

                            if units == "mi" {
                                distance = distance * 0.0006213712f64;
                            } else {
                                distance = distance / 1000f64;
                            }

                            if distance < alerting_distance {
                                info!(
                                    "plane {} is at alerting distance, {:.3} {}",
                                    airdata.aircraft_address, distance, units
                                );
                            } else {
                                info!(
                                    "plane {} is approaching but is not under alerting distance",
                                    &address
                                );
                            }
                        }
                        1 => debug!("plane {} is getting away from us", &address),
                        2 => debug!("Not enough data yet to check {}", &address),
                        _ => error!("Unexpected value"),
                    }
                } else {
                    // First time we meet an aircraft
                    planes.add_plane(airdata.clone());
                }
            }
        }
    }
}
