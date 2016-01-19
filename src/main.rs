extern crate fly_cheap;

use fly_cheap::Config;
use fly_cheap::flights::SearchRequest;

fn main() {
    let config = Config::load().unwrap();

    config.db_setup();

    let mut request = SearchRequest::new();
    request.add_trip("TXL", "OTP", "2016-03-28", 0);
    request.add_trip("OTP", "TXL", "2016-04-03", 0);

    let mut offers = request.call(&config.google_api_key).unwrap();
    for offer in &mut offers {
        println!("{}", offer);
        offer.create().unwrap();
    }

    // TODO: request all possible trip combinations

    // TODO: store all returned options

    // TODO: if any price < total average => deliver report (mailgun?)

    // TODO: determine when the next set of calls should be placed

    // TODO: sleep til the next moment

    // TODO: daily / weekly report
}
