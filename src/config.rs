use std::fs::{File, remove_file};
use std::io::Read;

use toml::decode_str;
use rusqlite::Connection;

use Error;

const DEFAULT_CONFIG_PATH: &'static str = "config.toml";
const DEFAULT_DB_PATH: &'static str = "data.sqlite";

#[derive(RustcDecodable)]
pub struct Config {
    pub email: Option<String>,
    pub google_api_key: String,
    pub requests_per_day: u8,
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
        let mut file = try!(File::open(DEFAULT_CONFIG_PATH).map_err(|_| Error::LoadingConfig));
        let mut buf = String::new();

        try!(file.read_to_string(&mut buf).map_err(|_| Error::ReadingConfig));

        decode_str(&buf).ok_or(Error::ParsingConfig)
    }

    pub fn total_calls(&self) -> usize {
        if self.trips.len() == 0 { return 0 }

        self.trips.iter().fold(1, |acc, trip| acc * trip.dates.len())
    }

    pub fn db_connection(&self) -> Result<Connection, Error> {
        Connection::open(DEFAULT_DB_PATH).map_err(|_| Error::EstablishingDbConnection)
    }

    pub fn db_setup(&self) {
        let conn = self.db_connection().unwrap();

        let create_offers = conn.execute("CREATE TABLE IF NOT EXISTS offers (id INTEGER PRIMARY KEY, currency TEXT NOT NULL, base_price REAL NOT NULL, sale_price REAL NOT NULL, tax_price REAL NOT NULL, total_price REAL NOT NULL, latest_ticketing_time TEXT NOT NULL, refundable INTEGER NOT NULL)", &[]);
        create_offers.unwrap();

        let create_flights = conn.execute("CREATE TABLE IF NOT EXISTS flights (id INTEGER PRIMARY KEY, offer_id INTEGER NOT NULL, origin TEXT NOT NULL, destination TEXT NOT NULL, departure_time TEXT NOT NULL, arrival_time TEXT NOT NULL, duration INTEGER NOT NULL, mileage INTEGER NOT NULL, seat TEXT NOT NULL, aircraft TEXT NOT NULL, carrier TEXT NOT NULL)", &[]);
        create_flights.unwrap();
    }

    pub fn db_reset(&self) {
        remove_file(DEFAULT_DB_PATH).unwrap_or(());
    }
}
