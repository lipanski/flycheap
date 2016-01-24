use std::fmt::{Display, Formatter};
use std::fmt::Result as FmtResult;

use time::{strptime, at, Tm, Timespec};
use rustc_serialize::json;
use rusqlite::Connection;

use money;
use Error;

const PRETTY_TIME_FORMAT: &'static str = "%d.%m %H:%I";
const ISO_TIME_FORMAT: &'static str = "%Y-%m-%dT%H:%M%z";

pub struct Offer {
    pub id: Option<i64>,
    pub request_id: i64,
    pub currency: String,
    pub base_price: f64,
    pub sale_price: f64,
    pub tax_price: f64,
    pub total_price: f64,
    pub latest_ticketing_at: Timespec,
    pub refundable: bool,
    pub flights: Vec<Flight>
}

pub struct Flight {
    pub id: Option<i64>,
    pub offer_id: Option<i64>,
    pub origin: String,
    pub destination: String,
    pub departs_at: Timespec,
    pub departs_at_offset: i64,
    pub arrives_at: Timespec,
    pub arrives_at_offset: i64,
    pub duration: i64,
    pub mileage: i64,
    pub seat: String,
    pub aircraft: String,
    pub carrier: String,
    pub number: String
}

#[derive(RustcDecodable)]
struct SearchResponse {
    trips: Trips
}

impl SearchResponse {
    pub fn to_offers(self, request_id: &i64) -> Result<Vec<Offer>, Error> {
        let mut offers = vec!();
        for option in self.trips.tripOption {
            match option.to_offer(request_id) {
                Ok(offer) => offers.push(offer),
                _ => {}
            };
        }

        Ok(offers)
    }
}

#[derive(RustcDecodable)]
#[allow(non_snake_case)]
struct Trips {
    requestId: String,
    data: TripsData,
    tripOption: Vec<TripOption>
}

#[derive(RustcDecodable)]
struct TripsData {
    airport: Vec<Airport>,
    city: Vec<City>,
    aircraft: Vec<Aircraft>,
    tax: Vec<Tax>,
    carrier: Vec<Carrier>
}

#[derive(RustcDecodable)]
struct Airport {
    code: String,
    city: String,
    name: String
}

#[derive(RustcDecodable)]
struct City {
    code: String,
    name: String
}

#[derive(RustcDecodable)]
struct Aircraft {
    code: String,
    name: String
}

#[derive(RustcDecodable)]
struct Tax {
    id: String,
    name: String
}

#[derive(RustcDecodable)]
struct Carrier {
    code: String,
    name: String
}

#[derive(RustcDecodable)]
#[allow(non_snake_case)]
struct TripOption {
    saleTotal: String,
    id: String,
    slice: Vec<Slice>,
    pricing: Vec<Pricing>
}

#[derive(RustcDecodable)]
struct Slice {
    duration: i64,
    segment: Vec<Segment>
}

#[derive(RustcDecodable)]
#[allow(non_snake_case)]
struct Segment {
    duration: i64,
    flight: GoogleFlight,
    id: String,
    cabin: String,
    bookingCode: String,
    bookingCodeCount: i64,
    leg: Vec<Leg>,
    connectionDuration: Option<i64>
}

#[derive(RustcDecodable)]
#[allow(non_snake_case)]
struct Leg {
    id: String,
    aircraft: String,
    arrivalTime: String,
    departureTime: String,
    origin: String,
    destination: String,
    duration: i64,
    mileage: i64,
    meal: Option<String>
}

#[derive(RustcDecodable)]
struct GoogleFlight {
    carrier: String,
    number: String
}

#[derive(RustcDecodable)]
#[allow(non_snake_case)]
struct Pricing {
    baseFareTotal: String,
    saleFareTotal: String,
    saleTaxTotal: String,
    saleTotal: String,
    fareCalculation: String,
    latestTicketingTime: String,
    ptc: String,
    refundable: Option<bool>
}

impl TripOption {
    pub fn to_offer(self, request_id: &i64) -> Result<Offer, Error> {
        let mut flights: Vec<Flight> = vec!();

        for slice in self.slice {
            for segment in slice.segment {
                let carrier = &segment.flight.carrier;
                let number = &segment.flight.number;
                let seat = &segment.cabin;

                for leg in segment.leg {
                    let departs_at = try!(parse_time(leg.departureTime));
                    let arrives_at = try!(parse_time(leg.arrivalTime));

                    let flight = Flight {
                        id: None,
                        offer_id: None,
                        origin: leg.origin,
                        destination: leg.destination,
                        departs_at: departs_at.to_timespec(),
                        departs_at_offset: departs_at.tm_utcoff as i64,
                        arrives_at: arrives_at.to_timespec(),
                        arrives_at_offset: arrives_at.tm_utcoff as i64,
                        duration: leg.duration,
                        mileage: leg.mileage,
                        seat: seat.to_string(),
                        aircraft: leg.aircraft,
                        carrier: carrier.to_string(),
                        number: number.to_string()
                    };

                    flights.push(flight);
                }
            }
        }

        let pricing = try!(self.pricing.get(0).ok_or(Error::NoPricing));

        let (base_price, _) = try!(money::parse(&pricing.baseFareTotal));
        let (sale_price, _) = try!(money::parse(&pricing.saleFareTotal));
        let (tax_price, _) = try!(money::parse(&pricing.saleTaxTotal));
        let (total_price, currency) = try!(money::parse(&pricing.saleTotal));

        let offer = Offer {
            id: None,
            request_id: request_id.clone(),
            currency: currency.to_string(),
            base_price: base_price,
            sale_price: sale_price,
            tax_price: tax_price,
            total_price: total_price,
            latest_ticketing_at: try!(parse_time(pricing.latestTicketingTime.clone())).to_timespec(),
            refundable: pricing.refundable.unwrap_or(false),
            flights: flights
        };

        Ok(offer)
    }
}

impl Offer {
    pub fn from_json(json: String, request_id: i64) -> Result<Vec<Self>, Error> {
        let price_response: SearchResponse = try!(json::decode(&json).map_err(|_| Error::DecodingJson(json)));

        price_response.to_offers(&request_id)
    }

    pub fn create(&mut self, conn: &Connection) -> Result<(), Error> {
        let transaction = try!(conn.transaction().map_err(|err| Error::CreatingTransaction(err.to_string())));

        let mut sql = try!(conn.prepare(
            "INSERT INTO offers
                (
                    request_id,
                    currency,
                    base_price,
                    sale_price,
                    tax_price,
                    total_price,
                    latest_ticketing_at,
                    refundable
                ) VALUES (?, ?, ?, ?, ?, ?, ?, ?)"
            ).map_err(|err| Error::PreparingDbQuery(err.to_string())));

        try!(sql.execute(
            &[
                &self.request_id,
                &self.currency,
                &self.base_price,
                &self.sale_price,
                &self.tax_price,
                &self.total_price,
                &self.latest_ticketing_at,
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
                    departs_at,
                    departs_at_offset,
                    arrives_at,
                    arrives_at_offset,
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
                &self.departs_at,
                &self.departs_at_offset,
                &self.arrives_at,
                &self.arrives_at_offset,
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
        try!(writeln!(f, " ({} + {}) / REFUNDABLE: {} / LATEST: {}", self.base_price, self.tax_price, self.refundable, format_time(self.latest_ticketing_at, None)));

        for flight in &self.flights {
            try!(write!(f, "{}, {} ---> {}, {}", flight.origin, format_time(flight.departs_at, Some(flight.departs_at_offset)), flight.destination, format_time(flight.arrives_at, Some(flight.arrives_at_offset))));
            try!(writeln!(f, " ({}{}, {})", flight.carrier, flight.number, flight.seat));
        }

        Ok(())
    }
}

fn format_time(timespec: Timespec, utc_offset: Option<i64>) -> String {
    let mut time = at(timespec);

    if utc_offset.is_some() { time.tm_utcoff = utc_offset.unwrap() as i32; }

    time.strftime(PRETTY_TIME_FORMAT).unwrap().to_string()
}

fn parse_time(time: String) -> Result<Tm, Error> {
    strptime(&time, ISO_TIME_FORMAT).map_err(|_| Error::ParsingTime(time))
}
