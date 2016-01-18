use std::fmt::{Display, Formatter, Result};

pub struct Offer {
    pub base_price: String,
    pub sale_price: String,
    pub tax_price: String,
    pub total_price: String,
    pub latest_ticketing_time: String,
    pub refundable: bool,
    pub flights: Vec<Flight>
}

pub struct Flight {
    pub from: String,
    pub to: String,
    pub departure_time: String,
    pub arrival_time: String,
    pub duration: u32,
    pub mileage: u32,
    pub seat: String,
    pub aircraft: String,
    pub carrier: String,
    pub number: String
}

impl Display for Offer {
    fn fmt(&self, f: &mut Formatter) -> Result {
        try!(writeln!(f, "PRICE: {}", self.total_price));
        try!(writeln!(f, "SALE: {} / TAX: {} / REFUNDABLE: {} / LATEST: {}", self.base_price, self.tax_price, self.refundable, self.latest_ticketing_time));

        for flight in &self.flights {
            try!(writeln!(f, "---"));
            try!(writeln!(f, "{}{} / {}", flight.carrier, flight.number, flight.seat));
            try!(writeln!(f, "{} ({}) ---> {} ({})", flight.from, flight.departure_time, flight.to, flight.arrival_time));
        }

        Ok(())
    }
}
