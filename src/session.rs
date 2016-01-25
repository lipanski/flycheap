use std::fs::{File, remove_file};
use std::io::Read;
use std::convert::From;

use std::time::Duration as StdDuration;
use time::{now, Duration, Tm, Timespec};
use toml::decode_str;
use rusqlite::Connection;

use flights::Request;
use Error;

const DEFAULT_CONFIG_PATH: &'static str = "config.toml";
const DEFAULT_DB_PATH: &'static str = "data.sqlite";

#[derive(RustcDecodable)]
pub struct Session {
    pub email: Option<String>,
    pub google_api_key: String,
    pub requests_per_day: usize,
    pub sale_country: String,
    pub request_name: String,
    pub trips: Vec<Trip>
}

#[derive(RustcDecodable)]
pub struct Trip {
    pub from: String,
    pub to: String,
    pub dates: Vec<String>
}

impl Session {
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

    pub fn request_sets_per_day(&self) -> usize {
        if self.total_calls() == 0 { return 0 }

        self.requests_per_day / self.total_calls()
    }

    pub fn db_connection() -> Result<Connection, Error> {
        Connection::open(DEFAULT_DB_PATH).map_err(|_| Error::EstablishingDbConnection)
    }

    pub fn db_setup(&self, conn: &Connection) {
        let create_requests = conn.execute(
            "CREATE TABLE IF NOT EXISTS requests
            (
                id INTEGER PRIMARY KEY,
                name TEXT NOT NULL,
                created_at INTEGER NOT NULL
            )", &[]);

        create_requests.unwrap();

        let create_offers = conn.execute(
            "CREATE TABLE IF NOT EXISTS offers
            (
                id INTEGER PRIMARY KEY,
                request_id INTEGER NOT NULL,
                currency TEXT NOT NULL,
                base_price REAL NOT NULL,
                sale_price REAL NOT NULL,
                tax_price REAL NOT NULL,
                total_price REAL NOT NULL,
                latest_ticketing_at INTEGER NOT NULL,
                refundable INTEGER NOT NULL
            )", &[]);

        create_offers.unwrap();

        let create_flights = conn.execute(
            "CREATE TABLE IF NOT EXISTS flights
            (
                id INTEGER PRIMARY KEY,
                offer_id INTEGER NOT NULL,
                origin TEXT NOT NULL,
                destination TEXT NOT NULL,
                departs_at INTEGER NOT NULL,
                departs_at_offset INTEGER NOT NULL,
                arrives_at INTEGER NOT NULL,
                arrives_at_offset INTEGER NOT NULL,
                duration INTEGER NOT NULL,
                mileage INTEGER NOT NULL,
                seat TEXT NOT NULL,
                aircraft TEXT NOT NULL,
                carrier TEXT NOT NULL,
                number TEXT NOT NULL
            )", &[]);

        create_flights.unwrap();
    }

    pub fn db_reset(&self) {
        remove_file(DEFAULT_DB_PATH).unwrap_or(());
    }

    pub fn requests(&self) -> Vec<Request> {
        let mut requests: Vec<Request> = (0..self.total_calls()).map(|_| {
            Request::new(&self.request_name, &self.sale_country)
        }).collect();

        for trip in &self.trips {
            let mut dates_iterator = trip.dates.iter().cycle();
            for request in &mut requests {
                request.add_trip(&trip.to, &trip.from, dates_iterator.next().unwrap(), 0);
            }
        }

        requests
    }

    pub fn duration_per_request(&self) -> Result<Duration, Error> {
        if self.request_sets_per_day() == 0 { return Err(Error::NoTripsOrDates) }

        Ok(Duration::hours(24) / (self.request_sets_per_day() as i32))
    }

    pub fn next_run_at(&self) -> Result<Tm, Error> {
        let duration_per_request = try!(self.duration_per_request());
        let now = now();
        let mut next_run_at = midnight();

        while next_run_at < now {
            next_run_at = next_run_at + duration_per_request;
        }

        Ok(next_run_at)
    }

    pub fn next_run_seconds(&self) -> Result<u64, Error> {
        let next_run_at = try!(self.next_run_at());
        let next_run_seconds = next_run_at.to_timespec().sec - now().to_timespec().sec;

        match next_run_seconds.is_positive() {
            true => Ok(next_run_seconds as u64),
            false => Ok(0)
        }
    }

    pub fn next_run_duration(&self) -> Result<StdDuration, Error> {
        let next_run_seconds = try!(self.next_run_seconds());

        Ok(StdDuration::from_secs(next_run_seconds))
    }
}

pub fn midnight() -> Tm {
    let mut midnight = now();
    midnight.tm_hour = 0;
    midnight.tm_min = 0;
    midnight.tm_sec = 0;

    midnight
}
