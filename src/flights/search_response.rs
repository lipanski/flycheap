use time::{strptime, Tm};

use money;
use Error;
use flights::Offer;
use flights::Flight as OfferFlight;

const TIME_FORMAT: &'static str = "%Y-%m-%dT%H:%M%z";

#[derive(RustcDecodable)]
pub struct SearchResponse {
    pub trips: Trips
}

impl SearchResponse {
    pub fn to_offers(self) -> Result<Vec<Offer>, Error> {
        let mut offers = vec!();
        for option in self.trips.tripOption {
            match option.to_offer() {
                Ok(offer) => offers.push(offer),
                _ => {}
            };
        }

        Ok(offers)
    }
}

#[derive(RustcDecodable)]
#[allow(non_snake_case)]
pub struct Trips {
    pub requestId: String,
    pub data: TripsData,
    pub tripOption: Vec<TripOption>
}

#[derive(RustcDecodable)]
pub struct TripsData {
    pub airport: Vec<Airport>,
    pub city: Vec<City>,
    pub aircraft: Vec<Aircraft>,
    pub tax: Vec<Tax>,
    pub carrier: Vec<Carrier>
}

#[derive(RustcDecodable)]
pub struct Airport {
    pub code: String,
    pub city: String,
    pub name: String
}

#[derive(RustcDecodable)]
pub struct City {
    pub code: String,
    pub name: String
}

#[derive(RustcDecodable)]
pub struct Aircraft {
    pub code: String,
    pub name: String
}

#[derive(RustcDecodable)]
pub struct Tax {
    pub id: String,
    pub name: String
}

#[derive(RustcDecodable)]
pub struct Carrier {
    pub code: String,
    pub name: String
}

#[derive(RustcDecodable)]
#[allow(non_snake_case)]
pub struct TripOption {
    pub saleTotal: String,
    pub id: String,
    pub slice: Vec<Slice>,
    pub pricing: Vec<Pricing>
}

#[derive(RustcDecodable)]
pub struct Slice {
    pub duration: i64,
    pub segment: Vec<Segment>
}

#[derive(RustcDecodable)]
#[allow(non_snake_case)]
pub struct Segment {
    pub duration: i64,
    pub flight: Flight,
    pub id: String,
    pub cabin: String,
    pub bookingCode: String,
    pub bookingCodeCount: i64,
    pub leg: Vec<Leg>,
    pub connectionDuration: Option<i64>
}

#[derive(RustcDecodable)]
#[allow(non_snake_case)]
pub struct Leg {
    pub id: String,
    pub aircraft: String,
    pub arrivalTime: String,
    pub departureTime: String,
    pub origin: String,
    pub destination: String,
    pub duration: i64,
    pub mileage: i64,
    pub meal: Option<String>
}

#[derive(RustcDecodable)]
pub struct Flight {
    pub carrier: String,
    pub number: String
}

#[derive(RustcDecodable)]
#[allow(non_snake_case)]
pub struct Pricing {
    pub baseFareTotal: String,
    pub saleFareTotal: String,
    pub saleTaxTotal: String,
    pub saleTotal: String,
    pub fareCalculation: String,
    pub latestTicketingTime: String,
    pub ptc: String,
    pub refundable: Option<bool>
}

impl TripOption {
    pub fn to_offer(self) -> Result<Offer, Error> {
        let mut flights: Vec<OfferFlight> = vec!();

        for slice in self.slice {
            for segment in slice.segment {
                let carrier = &segment.flight.carrier;
                let number = &segment.flight.number;
                let seat = &segment.cabin;

                for leg in segment.leg {
                    let departure_time = try!(parse_time(leg.departureTime));
                    let arrival_time = try!(parse_time(leg.arrivalTime));

                    let flight = OfferFlight {
                        id: None,
                        offer_id: None,
                        origin: leg.origin,
                        destination: leg.destination,
                        departure_time: departure_time.to_timespec(),
                        departure_utcoff: departure_time.tm_utcoff as i64,
                        arrival_time: arrival_time.to_timespec(),
                        arrival_utcoff: arrival_time.tm_utcoff as i64,
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
            currency: currency.to_string(),
            base_price: base_price,
            sale_price: sale_price,
            tax_price: tax_price,
            total_price: total_price,
            latest_ticketing_time: try!(parse_time(pricing.latestTicketingTime.clone())).to_timespec(),
            refundable: pricing.refundable.unwrap_or(false),
            flights: flights
        };

        Ok(offer)
    }
}

fn parse_time(time: String) -> Result<Tm, Error> {
    strptime(&time, TIME_FORMAT).map_err(|_| Error::ParsingTime(time))
}
