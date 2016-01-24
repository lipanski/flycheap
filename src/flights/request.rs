use std::io::Read;

use rustc_serialize::json;
use hyper::Client;
use hyper::header::{Connection, ContentType};
use hyper::status::StatusCode;
use mockito::url::Url;
use time::{now_utc, Timespec};
use rusqlite::Connection as DbConnection;

use flights::Offer;
use Session;
use Error;

const SEARCH_URL: &'static str = "https://www.googleapis.com/qpxExpress/v1/trips/search";
const PASSENGER_COUNT_KIND: &'static str = "qpxexpress#passengerCounts";
const SLICE_KIND: &'static str = "qpxexpress#sliceInput";

pub struct Request {
    pub id: Option<i64>,
    pub name: String,
    pub created_at: Timespec,
    google_search_request: GoogleSearchRequest
}

#[derive(RustcEncodable)]
struct GoogleSearchRequest {
    request: GoogleRequest
}

#[derive(RustcEncodable)]
#[allow(non_snake_case)]
struct GoogleRequest {
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

impl Request {
    pub fn new(name: &str, sale_country: &str) -> Self {
        let passengers = Passengers {
            kind: PASSENGER_COUNT_KIND,
            adultCount: 1,
            childCount: 0,
            infantInLapCount: 0,
            seniorCount: 0
        };

        let request = GoogleRequest {
            passengers: passengers,
            slice: vec!(),
            maxPrice: None,
            saleCountry: Some(sale_country.to_string()),
            refundable: None,
            solutions: None
        };

        let google_search_request = GoogleSearchRequest {
            request: request
        };

        Request {
            id: None,
            name: name.to_string(),
            created_at: now_utc().to_timespec(),
            google_search_request: google_search_request
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

        self.google_search_request.request.slice.push(slice);

        self
    }

    pub fn to_json(&self) -> Result<String, Error> {
        json::encode(&self.google_search_request).map_err(|_| Error::EncodingJson )
    }

    pub fn call(&mut self, api_key: &str) -> Result<Vec<Offer>, Error> {
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
                let conn = try!(Session::db_connection());
                try!(self.create(&conn));

                let request_id = try!(self.id.ok_or(Error::NoIdAssigned));

                Offer::from_json(body, request_id)
            },
            _ => Err(Error::ResponseNotOk(response.status.to_string()))
        }
    }

    pub fn create(&mut self, conn: &DbConnection) -> Result<(), Error> {
        let mut sql = try!(conn.prepare(
            "INSERT INTO requests
                (
                    name,
                    created_at
                ) VALUES (?, ?)"
            ).map_err(|err| Error::PreparingDbQuery(err.to_string())));

        try!(sql.execute(
            &[
                &self.name,
                &self.created_at
            ]).map_err(|err| Error::ExecutingDbQuery(err.to_string())));

        self.id = Some(conn.last_insert_rowid());

        Ok(())
    }

    pub fn requests_in_the_past_24_hours() -> Result<Vec<Self>, Error> {

        Ok(vec!())
    }
}
