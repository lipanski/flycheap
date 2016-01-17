pub mod search_request;
pub mod search_response;

pub type SearchRequest = search_request::SearchRequest;
pub type SearchResponse = search_response::SearchResponse;

#[derive(Debug)]
pub enum Error {
    EncodingJson,
    SendingRequest,
    ReadingResponse,
    ResponseNotOk,
    DecodingJson(String)
}
