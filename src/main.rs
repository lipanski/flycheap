extern crate fly_cheap;

use fly_cheap::Config;
use fly_cheap::flights::SearchRequest;

fn main() {
    let config = Config::load().unwrap();
    println!("email: {}", config.email.as_ref().unwrap());
    println!("to: {}", config.trips.get(0).unwrap().to);
    println!("total_calls: {}", config.total_calls());

    let mut request = SearchRequest::new();
    request.add_trip("TXL", "OTP", "2016-03-28", 0);
    request.add_trip("OTP", "TXL", "2016-04-03", 0);

    // let response = request.call(&config.google_api_key).unwrap();
    // println!("price: {}", response.trips.tripOption.first().unwrap().saleTotal);
}
