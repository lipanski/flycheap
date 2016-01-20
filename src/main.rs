extern crate fly_cheap;

use fly_cheap::Config;

fn main() {
    let config = Config::load().unwrap();

    let conn = Config::db_connection().unwrap();
    config.db_setup(&conn);

    for request in config.search_requests() {
        let mut offers = request.call(&config.google_api_key).unwrap();
        for offer in &mut offers {
            println!("{}", offer);
            offer.create(&conn).unwrap();
        }
    }

    // TODO: store dates as unix timestamps

    // TODO: chose currency via config file (?)

    // TODO: store request dates

    // TODO: store requests & add a name

    // TODO: rename config to session

    // TODO: if any price < total average => deliver report (mailgun?)

    // TODO: determine when the next set of calls should be placed

    // TODO: sleep til the next moment

    // TODO: daily / weekly report

    // TODO: remove all unwrap calls + handle offer errros gracefully (?)
}
