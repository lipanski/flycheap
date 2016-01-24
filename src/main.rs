extern crate flycheap;

use flycheap::Session;

fn main() {
    let session = Session::load().unwrap();

    let conn = Session::db_connection().unwrap();
    session.db_setup(&conn);

    for request in session.search_requests() {
        let mut offers = request.call(&session.google_api_key).unwrap();
        for offer in &mut offers {
            println!("{}", offer);
            offer.create(&conn).unwrap();
        }
    }

    // TODO: store request dates

    // TODO: store requests & add a name

    // TODO: if any price < total average => deliver report (mailgun?)

    // TODO: determine when the next set of calls should be placed

    // TODO: sleep til the next moment

    // TODO: daily / weekly report

    // TODO: remove all unwrap calls + handle offer errros gracefully (?)
}
