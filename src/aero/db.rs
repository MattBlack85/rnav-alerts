use crate::Airdata;
use geo::point;
use geo_types::Point;
use log::debug;
use std::collections::hash_map::Entry;
use std::collections::HashMap;

use chrono::NaiveDateTime;
use geo::prelude::GeodesicDistance;

/// The "database" where data are stored and checked, the DB is a hashmap
/// where the key is the plane number and the value is a vector contaiing
/// Airdata struct containing all plane information.
pub struct Planes {
    pub db: HashMap<String, Vec<Airdata>>,
    point: Point<f64>,
}

impl Planes {
    pub fn new(latitude: String, longitude: String) -> Self {
        Self {
            db: HashMap::new(),
            point: point!(x: longitude.parse::<f64>().unwrap(), y: latitude.parse::<f64>().unwrap()),
        }
    }

    pub fn is_plane_approaching(&mut self, aircraft_address: &String) -> u8 {
        // retrieve the data we already have about an aircraft
        let aircraft_data: &Vec<Airdata> = self.db.get(aircraft_address).unwrap();
        let data_length = aircraft_data.len();

        // If we have less than 2 entries we can't yet calculate if the plane is
        // getting away/closer from/to us
        if data_length < 2 {
            return 2;
        }

        // Build a point in space of the last know coordinates of this aircraft
        let airplane = &aircraft_data[data_length - 2];
        let old_plane_location = point!(x: airplane.longitude.parse::<f64>().unwrap(), y: airplane.latitude.parse::<f64>().unwrap());

        // Build a point in space where the actual aircraft is
        let last_data: &Airdata = aircraft_data.last().unwrap();
        let actual_plane_location = point!(x: last_data.longitude.parse::<f64>().unwrap(), y: last_data.latitude.parse::<f64>().unwrap());

        if self.point.geodesic_distance(&actual_plane_location)
            < self.point.geodesic_distance(&old_plane_location)
        {
            return 0;
        } else {
            self.db.remove(aircraft_address);
            return 1;
        }
    }

    /// Returns true/false if the aircraft id already exists inside the database
    pub fn already_seen(&self, aircraft_id: &String) -> bool {
        return self.db.contains_key(aircraft_id);
    }

    pub fn add_plane(&mut self, airplane: Airdata) {
        self.db
            .insert(airplane.aircraft_address.clone(), vec![airplane]);
    }

    /// If the aircraft is already in the database new data is appended
    /// in the database
    pub fn add_data_for_aircraft(&mut self, airplane: Airdata) {
        if let Entry::Occupied(mut entry) = self.db.entry(airplane.aircraft_address.to_owned()) {
            debug!("Pushing data for {}", &airplane.aircraft_address);
            let data = entry.get_mut();
            data.push(airplane);
        }
    }

    pub fn check_old_data_too_fresh(&self, airplane: Airdata) -> bool {
        let aircraft_data: &Vec<Airdata> = self.db.get(&airplane.aircraft_address).unwrap();
        let last_data: &Airdata = aircraft_data.last().unwrap();

        let last_data_received = NaiveDateTime::parse_from_str(
            &format!("{} {}", &last_data.date_received, &last_data.hour_received),
            "%Y/%m/%d %H:%M:%S%.3f",
        )
        .unwrap();

        let new_data_received = NaiveDateTime::parse_from_str(
            &format!("{} {}", &airplane.date_received, &airplane.hour_received),
            "%Y/%m/%d %H:%M:%S%.3f",
        )
        .unwrap();
        let duration = new_data_received - last_data_received;
        debug!(
            "Elapsed {} seconds form last time we saw {}",
            duration.num_seconds(),
            &last_data.aircraft_address,
        );
        if duration.num_seconds() > 5 {
            false
        } else {
            true
        }
    }
}
