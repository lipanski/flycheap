extern crate fly_cheap;

use std::env;
use fly_cheap::flights::PriceRequest;

fn main() {
    let mut request = PriceRequest::new("TXL", "OTP", "2016-03-28", 0);
    request.add_rountrip("2016-04-03");

    let api_key = env::var("FLY_CHEAP_GOOGLE_API_KEY").unwrap();
    let _ = request.call(&api_key).unwrap();
}
