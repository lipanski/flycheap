pub mod price_request;
pub mod price_response;

pub type PriceRequest = price_request::PriceRequest;
pub type PriceResponse = price_response::PriceResponse;

#[derive(Debug)]
pub enum Error {
    EncodingJson,
    SendingRequest,
    ReadingResponse,
    ResponseNotOk,
    DecodingJson(String)
}
