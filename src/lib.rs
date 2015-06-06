#![feature(plugin,collections)]
#![plugin(regex_macros)]
extern crate rustc_serialize;
extern crate regex;
extern crate hyper;

use std::io::Read;
use hyper::Client;
use regex::Regex;

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

#[derive(Debug,RustcDecodable, RustcEncodable)]
pub struct Wind {
    direction: u32,
    speed: u32,
    altitude: u32,
}

#[derive(Debug,RustcDecodable, RustcEncodable)]
pub struct WindsAloft {
    station: String,
    winds: Vec<Wind>,
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

impl WindsAloft {
    pub fn new(station: &str, winds: Vec<Wind>) -> WindsAloft {
        WindsAloft { station: station.to_string(), winds: winds }
    }

    pub fn wind_at_altitude(&self, altitude: u32) -> Option<&Wind> {
        for wind in self.winds.iter() {
            if wind.altitude == altitude {
                return Some(wind);
            }
        }
        None
    }
}

pub fn winds_aloft_for_station(station: &str) -> Option<WindsAloft> {
    let upper_station = station.to_uppercase();
    match all_winds_aloft() {
        Some(winds) => {
            for wind in winds {
                if wind.station == upper_station {
                    return Some(wind);
                }
            }
        },
        None => return None
    }
    None
}

pub fn all_winds_aloft() -> Option<Vec<WindsAloft>> {
    let mut client = Client::new();

    let mut res = match client.get(URL).send() {
        Ok(x) => x,
        Err(_) => return None,
    };

    let mut body = String::new();
    res.read_to_string(&mut body).unwrap();

    Some(winds_from_body(body))
}

fn winds_from_body(body: String) -> Vec<WindsAloft> {
    let mut wind_aloft: Vec<WindsAloft> = vec![];

    for group in PATTERN.captures_iter(&body) {
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

        wind_aloft.push(WindsAloft::new(station, winds));
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

#[cfg(test)]
mod test {
    #[test]
    fn parse_basic_wind() {
        let wind = ::parse_wind("3127", 3000u32).unwrap();

        assert_eq!(wind.speed, 27u32);
        assert_eq!(wind.direction, 310u32);
    }

    #[test]
    fn parse_bad_wind() {
        let wind = ::parse_wind("invalid", 3000u32);
        let parsed = match wind {
            Some(_) => true,
            None => false,
        };

        assert_eq!(parsed, false);
    }

    #[test]
    fn parse_wind_with_negative_temp() {
        let wind = ::parse_wind("3127-32", 3000u32).unwrap();

        assert_eq!(wind.speed, 27u32);
        assert_eq!(wind.direction, 310u32);
    }

    #[test]
    fn parse_wind_with_positive_temp() {
        let wind = ::parse_wind("3127+32", 3000u32).unwrap();

        assert_eq!(wind.speed, 27u32);
        assert_eq!(wind.direction, 310u32);
    }
}
