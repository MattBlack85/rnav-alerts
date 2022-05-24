use serde::Deserialize;

#[derive(Deserialize, Clone, Debug)]
pub struct Airdata {
    pub msg: String,
    pub msg_type: String,
    _dummy_1: String,
    _dummy_2: String,
    pub aircraft_address: String,
    _dummy_3: String,
    pub date_received: String,
    pub hour_received: String,
    pub date_written: String,
    pub hour_written: String,
    pub callsign: String,
    pub altitude: String,
    pub groundspeed: String,
    pub ground_track: String,
    pub latitude: String,
    pub longitude: String,
    pub vertical_rate: String,
    pub squawk: String,
    pub alert_flag: String,
    pub emergency_flag: String,
    pub spi_flag: String,
    pub ground_flag: String,
}
