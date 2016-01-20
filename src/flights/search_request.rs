use std::io::Read;

use rustc_serialize::json;
use hyper::Client;
use hyper::header::{Connection, ContentType};
use hyper::status::StatusCode;
use mockito::url::Url;

use flights::{SearchResponse, Offer};
use Error;

const SEARCH_URL: &'static str = "https://www.googleapis.com/qpxExpress/v1/trips/search";
const PASSENGER_COUNT_KIND: &'static str = "qpxexpress#passengerCounts";
const SLICE_KIND: &'static str = "qpxexpress#sliceInput";

#[derive(RustcEncodable)]
pub struct SearchRequest {
    request: Request
}

#[derive(RustcEncodable)]
#[allow(non_snake_case)]
struct Request {
    passengers: Passengers,
    slice: Vec<Slice>,
    maxPrice: Option<String>,
    saleCountry: Option<String>,
    refundable: Option<bool>,
    solutions: Option<u8>
}

#[derive(RustcEncodable)]
#[allow(non_snake_case)]
struct Passengers {
    kind: &'static str,
    adultCount: u8,
    childCount: u8,
    infantInLapCount: u8,
    seniorCount: u8
}

#[derive(RustcEncodable, Clone)]
#[allow(non_snake_case)]
struct Slice {
    kind: &'static str,
    origin: String,
    destination: String,
    date: String,
    maxStops: u8,
    maxConnectionDuration: Option<u32>,
    preferredCabin: Option<String>
}

impl SearchRequest {
    pub fn new() -> Self {
        let passengers = Passengers {
            kind: PASSENGER_COUNT_KIND,
            adultCount: 1,
            childCount: 0,
            infantInLapCount: 0,
            seniorCount: 0
        };

        let request = Request {
            passengers: passengers,
            slice: vec!(),
            maxPrice: None,
            saleCountry: None,
            refundable: None,
            solutions: None
        };

        SearchRequest {
            request: request
        }
    }

    pub fn add_trip(&mut self, origin: &str, destination: &str, date: &str, max_stops: u8) -> &mut Self {
        let slice = Slice {
            kind: SLICE_KIND,
            origin: origin.to_string(),
            destination: destination.to_string(),
            date: date.to_string(),
            maxStops: max_stops,
            maxConnectionDuration: None,
            preferredCabin: None
        };

        self.request.slice.push(slice);

        self
    }

    pub fn to_json(&self) -> Result<String, Error> {
        json::encode(self).map_err(|_| Error::EncodingJson )
    }

    pub fn call(&self, api_key: &str) -> Result<Vec<Offer>, Error> {
        let url = SEARCH_URL.to_string() + "?key=" + api_key;
        let request_body = try!(self.to_json());

        let client  = Client::new();
        let request = client.post(Url(&url))
            .header(Connection::close())
            .header(ContentType::json())
            .body(&request_body);

        let mut response = try!(request.send().map_err(|_| Error::SendingRequest));

        let mut body = String::new();
        try!(response.read_to_string(&mut body).map_err(|_| Error::ReadingResponse));

        match response.status {
            StatusCode::Ok => {
                let price_response: SearchResponse = try!(json::decode(&body).map_err(|_| Error::DecodingJson(body)));
                price_response.to_offers()
            },
            _ => Err(Error::ResponseNotOk(response.status.to_string()))
        }
    }
}
