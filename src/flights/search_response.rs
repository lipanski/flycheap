#[derive(RustcDecodable)]
pub struct SearchResponse {
    pub trips: Trips
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
