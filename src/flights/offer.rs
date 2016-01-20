use std::fmt::{Display, Formatter};
use std::fmt::Result as FmtResult;

use time;
use time::Timespec;
use rusqlite::Connection;

use Error;

const TIME_FORMAT: &'static str = "%d.%m %H:%I";

pub struct Offer {
    pub id: Option<i64>,
    pub currency: String,
    pub base_price: f64,
    pub sale_price: f64,
    pub tax_price: f64,
    pub total_price: f64,
    pub latest_ticketing_time: Timespec,
    pub refundable: bool,
    pub flights: Vec<Flight>
}

pub struct Flight {
    pub id: Option<i64>,
    pub offer_id: Option<i64>,
    pub origin: String,
    pub destination: String,
    pub departure_time: Timespec,
    pub departure_utcoff: i64,
    pub arrival_time: Timespec,
    pub arrival_utcoff: i64,
    pub duration: i64,
    pub mileage: i64,
    pub seat: String,
    pub aircraft: String,
    pub carrier: String,
    pub number: String
}

impl Offer {
    pub fn create(&mut self, conn: &Connection) -> Result<(), Error> {
        let transaction = try!(conn.transaction().map_err(|err| Error::CreatingTransaction(err.to_string())));

        let mut sql = try!(conn.prepare(
            "INSERT INTO offers
                (
                    currency,
                    base_price,
                    sale_price,
                    tax_price,
                    total_price,
                    latest_ticketing_time,
                    refundable
                ) VALUES (?, ?, ?, ?, ?, ?, ?)"
            ).map_err(|err| Error::PreparingDbQuery(err.to_string())));

        try!(sql.execute(
            &[
                &self.currency,
                &self.base_price,
                &self.sale_price,
                &self.tax_price,
                &self.total_price,
                &self.latest_ticketing_time,
                &self.refundable
            ]).map_err(|err| Error::ExecutingDbQuery(err.to_string())));

        self.id = Some(conn.last_insert_rowid());

        for flight in &mut self.flights {
            flight.offer_id = self.id;
            try!(flight.create(conn));
        }

        try!(transaction.commit().map_err(|err| Error::CommitingTransaction(err.to_string())));

        Ok(())
    }
}

impl Flight {
    pub fn create(&mut self, conn: &Connection) -> Result<(), Error> {
        let mut sql = try!(conn.prepare(
            "INSERT INTO flights
                (
                    offer_id,
                    origin,
                    destination,
                    departure_time,
                    departure_utcoff,
                    arrival_time,
                    arrival_utcoff,
                    duration,
                    mileage,
                    seat,
                    aircraft,
                    carrier,
                    number
                ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"
            ).map_err(|err| Error::PreparingDbQuery(err.to_string())));

        try!(sql.execute(
            &[
                &self.offer_id,
                &self.origin,
                &self.destination,
                &self.departure_time,
                &self.departure_utcoff,
                &self.arrival_time,
                &self.arrival_utcoff,
                &self.duration,
                &self.mileage,
                &self.seat,
                &self.aircraft,
                &self.carrier,
                &self.number
            ]).map_err(|err| Error::ExecutingDbQuery(err.to_string())));

        self.id = Some(conn.last_insert_rowid());

        Ok(())
    }
}

impl Display for Offer {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        try!(write!(f, "PRICE: {}{}", self.total_price, self.currency));
        try!(writeln!(f, " ({} + {}) / REFUNDABLE: {} / LATEST: {}", self.base_price, self.tax_price, self.refundable, format_time(self.latest_ticketing_time, None)));

        for flight in &self.flights {
            try!(write!(f, "{}, {} ---> {}, {}", flight.origin, format_time(flight.departure_time, Some(flight.departure_utcoff)), flight.destination, format_time(flight.arrival_time, Some(flight.arrival_utcoff))));
            try!(writeln!(f, " ({}{}, {})", flight.carrier, flight.number, flight.seat));
        }

        Ok(())
    }
}

fn format_time(timespec: Timespec, utc_offset: Option<i64>) -> String {
    let mut time = time::at(timespec);

    if utc_offset.is_some() { time.tm_utcoff = utc_offset.unwrap() as i32; }

    time.strftime(TIME_FORMAT).unwrap().to_string()
}
