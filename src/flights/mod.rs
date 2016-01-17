pub mod search_request;
pub mod search_response;
pub mod offer;

pub type SearchRequest = search_request::SearchRequest;
pub type SearchResponse = search_response::SearchResponse;
pub type Offer = offer::Offer;

#[derive(Debug)]
pub enum Error {
    EncodingJson,
    SendingRequest,
    ReadingResponse,
    ResponseNotOk,
    DecodingJson(String),
    NoPricing,
    NoFlights
}
