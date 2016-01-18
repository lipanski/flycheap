use money;
use flights::{Offer, Error};
use flights::Flight as OfferFlight;

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

impl TripOption {
    pub fn to_offer(self) -> Result<Offer, Error> {
        let mut flights: Vec<OfferFlight> = vec!();

        for slice in self.slice {
            for segment in slice.segment {
                let carrier = &segment.flight.carrier;
                let number = &segment.flight.number;
                let seat = &segment.cabin;

                for leg in segment.leg {
                    let flight = OfferFlight {
                        id: None,
                        offer_id: None,
                        origin: leg.origin,
                        destination: leg.destination,
                        departure_time: leg.departureTime,
                        arrival_time: leg.arrivalTime,
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

        let (base_price, _) = try!(money::parse(&pricing.baseFareTotal).map_err(|_| Error::ParsingPrice));
        let (sale_price, _) = try!(money::parse(&pricing.saleFareTotal).map_err(|_| Error::ParsingPrice));
        let (tax_price, _) = try!(money::parse(&pricing.saleTaxTotal).map_err(|_| Error::ParsingPrice));
        let (total_price, currency) = try!(money::parse(&pricing.saleTotal).map_err(|_| Error::ParsingPrice));

        let offer = Offer {
            id: None,
            currency: currency.to_string(),
            base_price: base_price,
            sale_price: sale_price,
            tax_price: tax_price,
            total_price: total_price,
            latest_ticketing_time: pricing.latestTicketingTime.clone(),
            refundable: pricing.refundable.unwrap_or(false),
            flights: flights
        };

        Ok(offer)
    }
}

#[derive(RustcDecodable)]
pub struct Slice {
    pub duration: u32,
    pub segment: Vec<Segment>
}

#[derive(RustcDecodable)]
#[allow(non_snake_case)]
pub struct Segment {
    pub duration: u32,
    pub flight: Flight,
    pub id: String,
    pub cabin: String,
    pub bookingCode: String,
    pub bookingCodeCount: u32,
    pub leg: Vec<Leg>,
    pub connectionDuration: Option<u32>
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
    pub duration: u32,
    pub mileage: u32,
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
