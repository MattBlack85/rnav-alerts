use geo::point;
use geo::prelude::GeodesicDistance;
use dialoguer::Input;

fn main() {
    let other = point!(x: 20.055488618197895f64, y: 50.0013109665335f64);

    // check if the config file is present, make one if not (stored under ~/.config/rnav-alert.cfg

    // read the config and check if we have the coordinates of the place we want to monitor

    // if not ask for the coordinates of the place, first latitude then longitude, invert them before storing them
    println!("First, I will ask you about the place that I should monitor\n");
    let latitude: String = Input::new()
	.with_prompt("Provide the latitude")
	.interact_text().unwrap();

    let longitude: String = Input::new()
	.with_prompt("Provide the longitude")
	.interact_text().unwrap();

    let place = point!(x: longitude.parse::<f64>().unwrap(), y: latitude.parse::<f64>().unwrap());
    let distance = place.geodesic_distance(&other);
    println!("The distance between point 1 and 2 is {:.3} Km", distance / 1000f64);

    
	
}
