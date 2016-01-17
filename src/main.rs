extern crate fly_cheap;

use fly_cheap::Config;
use fly_cheap::flights::PriceRequest;

fn main() {
    let mut request = PriceRequest::new();
    request.add_trip("TXL", "OTP", "2016-03-28", 0);
    request.add_trip("OTP", "TXL", "2016-04-03", 0);

    // let api_key = env::var("FLY_CHEAP_GOOGLE_API_KEY").unwrap();
    // let response = request.call(&api_key).unwrap();
    // println!("price: {}", response.trips.tripOption.first().unwrap().saleTotal);

    let config = Config::load().unwrap();
    println!("email: {}", config.email.as_ref().unwrap());
    println!("to: {}", config.trips.get(0).unwrap().to);
    println!("total_calls: {}", config.total_calls());
}
