#![feature(plugin,collections,core)]
#![plugin(regex_macros)]
extern crate rustc_serialize;
extern crate regex;
extern crate hyper;
extern crate chrono;
extern crate core;

use std::io::Read;
use hyper::Client;
use regex::Regex;
use chrono::offset::utc::UTC;
use chrono::Timelike;

#[cfg(test)]
mod test;

/// Interact with winds aloft data from NOAA

const URL: &'static str = "http://aviationweather.gov/products/nws/all";
static PATTERN: Regex = regex!(r"(?x)
    (?P<station>\w+)\s          # Airport code
    (?P<three_K>.{4})\s         # Winds aloft at 3,000'
    (?P<six_K>.{7})\s           # Winds aloft at 6,000'
    (?P<nine_K>.{7})\s          # Winds aloft at 9,000'
    (?P<twelve_K>.{7})\s        # Winds aloft at 12,000'
    (?P<eighteen_K>.{7})\s      # Winds aloft at 18,000'
    (?P<twenty_four_K>.{7})\s   # Winds aloft at 24,000'
    (?P<thirty_K>.{6})\s        # Winds aloft at 30,000'
    (?P<thirty_four_K>.{6})\s   # Winds aloft at 34,000'
    (?P<thirty_nine_K>.{6})\s   # Winds aloft at 39,000'
");
static WIND_PATTERN: Regex = regex!(r"(?x)
    \w+
");
static TIME_PATTERN: Regex = regex!(r"(?x)
    DATA\sBASED\sON\s(?P<date>\d{2})(?P<time>\d{4})Z
");

/// The wind data from one weather station at one altitude
#[derive(Debug,RustcEncodable,RustcDecodable)]
pub struct Wind {
    direction: u32,
    speed: u32,
    altitude: u32,
}

/// Weather data from one weather station at all altitudes
#[derive(Debug,RustcEncodable,RustcDecodable)]
pub struct StationForecast {
    station: String,
    winds: Vec<Wind>,
}

/// All information associated with winds aloft forecast
#[derive(Debug,RustcEncodable,RustcDecodable)]
pub struct WindsAloftForecast {
    /// The zulu time that this forecast was received as an unsigned 32-bit number. For example, if
    /// the report was downloaded at 2:45 PM (UTC + 1), this field will be 1345u32.
    time_retrieved: u32,

    /// The zulu time that this report was generated.
    forecast_time: u32,

    /// The list of `StationForecast` objects that are held in this report.
    forecasts: Vec<StationForecast>,
}


impl WindsAloftForecast {
    pub fn new() -> WindsAloftForecast {
        let mut forecast = WindsAloftForecast {
            time_retrieved: 0u32,
            forecast_time: 0u32,
            forecasts: vec![],
        };
        forecast.refresh();

        forecast
    }

    /// Re-downloads the forecast from NOAA. Returns None if there is a problem with the download,
    /// or `Some(())` if the operation completes successfully.
    pub fn refresh(&mut self) -> Option<()> {
        let mut client = Client::new();

        let mut res = match client.get(URL).send() {
            Ok(x) => x,
            Err(_) => return None,
        };

        let mut body = String::new();
        res.read_to_string(&mut body).unwrap();

        self.forecasts = winds_from_body(&body);
        self.forecast_time = WindsAloftForecast::parse_time_from_body(&body);

        let now = UTC::now();
        let hour = now.hour();
        let minute = now.minute();

        self.time_retrieved = hour * 100 + minute;

        Some(())
    }

    /// Returns true if the data stored in this forecast is out of date, false otherwise. The
    /// winds aloft forecast is published at 00Z, 06Z, 12Z and 18Z
    pub fn needs_refresh(&self) -> bool {
        let now = UTC::now();
        let hour = now.hour();
        let minute = now.minute();

        self.needs_refresh_time_given(hour * 100 + minute)
    }

    fn needs_refresh_time_given(&self, time: u32) -> bool {
        let hour = time / 100u32;
        let forecast_hour = self.forecast_time / 100u32;

        if hour > forecast_hour + 6 {
            true
        } else if hour < forecast_hour {
            true
        } else {
            false
        }
    }

    /// Get the forecast for a specific station out of this complete forecast.
    pub fn get_station_forecast(&self, station: &str) -> Option<&StationForecast> {
        let upper_station = station.to_uppercase();
        for wind in self.forecasts.iter() {
            if wind.station == upper_station {
                return Some(wind);
            }
        }
        None
    }

    fn parse_time_from_body(body: &str) -> u32 {
        let mut time = 0u32;
        for group in TIME_PATTERN.captures_iter(body) {
            time = match group.name("time") {
                Some(x) => x.parse().unwrap(),
                None => continue,
            }
        }

        time
    }
}

impl Wind {
    pub fn new(direction: u32, speed: u32, altitude: u32) -> Wind {
        Wind {
            direction: direction,
            speed: speed,
            altitude: altitude,
        }
    }
}

impl StationForecast {
    pub fn new(station: &str, winds: Vec<Wind>) -> StationForecast {
        StationForecast { station: station.to_string(), winds: winds }
    }

    /// Search through the wind data from this station, return the wind data from the given
    /// altitude if it exists, otherwise return `None`.
    pub fn wind_at_altitude(&self, altitude: u32) -> Option<&Wind> {
        for wind in self.winds.iter() {
            if wind.altitude == altitude {
                return Some(wind);
            }
        }
        None
    }
}

fn winds_from_body(body: &str) -> Vec<StationForecast> {
    let mut wind_aloft: Vec<StationForecast> = vec![];

    for group in PATTERN.captures_iter(body) {
        let station = match group.name("station") {
            Some(x) => x,
            None => continue,
        };

        let mut winds: Vec<Wind> = vec![];

        match group.name("three_K") {
            Some(i) => {
                match parse_wind(i, 3000u32) {
                    Some(x) => winds.push(x),
                    None => { }
                }
            }
            None => { }
        }

        match group.name("six_K") {
            Some(i) => {
                match parse_wind(i, 6000u32) {
                    Some(x) => winds.push(x),
                    None => { }
                }
            }
            None => { }
        }

        match group.name("nine_K") {
            Some(i) => {
                match parse_wind(i, 9000u32) {
                    Some(x) => winds.push(x),
                    None => { }
                }
            }
            None => { }
        }

        match group.name("twelve_K") {
            Some(i) => {
                match parse_wind(i, 12000u32) {
                    Some(x) => winds.push(x),
                    None => { }
                }
            }
            None => { }
        }

        match group.name("eighteen_K") {
            Some(i) => {
                match parse_wind(i, 18000u32) {
                    Some(x) => winds.push(x),
                    None => { }
                }
            }
            None => { }
        }

        match group.name("twenty_four_K") {
            Some(i) => {
                match parse_wind(i, 24000u32) {
                    Some(x) => winds.push(x),
                    None => { }
                }
            }
            None => { }
        }

        match group.name("thirty_K") {
            Some(i) => {
                match parse_wind(i, 30000u32) {
                    Some(x) => winds.push(x),
                    None => { }
                }
            }
            None => { }
        }

        match group.name("thirty_four_K") {
            Some(i) => {
                match parse_wind(i, 34000u32) {
                    Some(x) => winds.push(x),
                    None => { }
                }
            }
            None => { }
        }

        match group.name("thirty_nine_K") {
            Some(i) => {
                match parse_wind(i, 39000u32) {
                    Some(x) => winds.push(x),
                    None => { }
                }
            }
            None => { }
        }

        wind_aloft.push(StationForecast::new(station, winds));
    }

    wind_aloft
}

fn parse_wind(wind: &str, altitude: u32) -> Option<Wind> {
    if WIND_PATTERN.is_match(wind) {
        let dir_str = wind.slice_chars(0, 2);
        let spd_str = wind.slice_chars(2, 4);

        let direction = match dir_str.parse::<u32>() {
            Ok(x) => x * 10,
            Err(_) => return None,
        };

        let speed = match spd_str.parse::<u32>() {
            Ok(x) => x,
            Err(_) => return None,
        };

        Some(Wind::new(direction, speed, altitude))
    } else {
        None
    }
}
