use std::fmt::{Display, Formatter};
use std::fmt::Result as FmtResult;

use Error;

pub struct Offer {
    pub id: Option<u32>,
    pub currency: String,
    pub base_price: f32,
    pub sale_price: f32,
    pub tax_price: f32,
    pub total_price: f32,
    pub latest_ticketing_time: String,
    pub refundable: bool,
    pub flights: Vec<Flight>
}

pub struct Flight {
    pub id: Option<u32>,
    pub offer_id: Option<u32>,
    pub origin: String,
    pub destination: String,
    pub departure_time: String,
    pub arrival_time: String,
    pub duration: u32,
    pub mileage: u32,
    pub seat: String,
    pub aircraft: String,
    pub carrier: String,
    pub number: String
}

impl Offer {

}

impl Flight {
    pub fn save(&self) -> Result<(), Error> {
        Ok(())
    }
}

impl Display for Offer {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        try!(writeln!(f, "PRICE: {}", self.total_price));
        try!(writeln!(f, "SALE: {} / TAX: {} / REFUNDABLE: {} / LATEST: {}", self.base_price, self.tax_price, self.refundable, self.latest_ticketing_time));

        for flight in &self.flights {
            try!(writeln!(f, "---"));
            try!(writeln!(f, "{}{} / {}", flight.carrier, flight.number, flight.seat));
            try!(writeln!(f, "{} ({}) ---> {} ({})", flight.origin, flight.departure_time, flight.destination, flight.arrival_time));
        }

        Ok(())
    }
}
