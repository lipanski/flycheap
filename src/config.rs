use std::fs::File;
use std::io::Read;

use toml::decode_str;

use Error;

const DEFAULT_FILE_PATH: &'static str = "config.toml";

#[derive(RustcDecodable)]
pub struct Config {
    pub email: Option<String>,
    pub google_api_key: String,
    pub trips: Vec<Trip>
}

#[derive(RustcDecodable)]
pub struct Trip {
    pub from: String,
    pub to: String,
    pub dates: Vec<String>
}

impl Config {
    pub fn load() -> Result<Self, Error> {
        let mut file = try!(File::open(DEFAULT_FILE_PATH).map_err(|_| Error::LoadingConfig));
        let mut buf = String::new();

        try!(file.read_to_string(&mut buf).map_err(|_| Error::ReadingConfig));

        decode_str(&buf).ok_or(Error::ParsingConfig)
    }

    pub fn total_calls(&self) -> usize {
        if self.trips.len() == 0 { return 0 }

        self.trips.iter().fold(1, |acc, trip| acc * trip.dates.len())
    }
}
