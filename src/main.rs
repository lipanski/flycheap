extern crate flycheap;

use std::thread::sleep;

use flycheap::Session;

fn main() {
    let session = Session::load().unwrap();
    let conn = Session::db_connection().unwrap();
    session.db_setup(&conn);

    loop {
        let next_run = session.next_run_duration().unwrap();
        println!("next run in {} seconds", next_run.as_secs());

        sleep(next_run);

        println!("requesting flights...");
        for mut request in session.requests() {
            let mut offers = request.call(&session.google_api_key).unwrap();
            for offer in &mut offers {
                println!("{}", offer);
                offer.create(&conn).unwrap();
            }
        }
    }

    // TODO: if any price < total average => deliver report (mailgun?)

    // TODO: count dates, not trips

    // TODO: save carrier in offers table (if always the same)

    // TODO: daily / weekly report

    // TODO: save config (?) & db to home folder

    // TODO: remove all unwrap calls + handle offer errros gracefully (?)

    // TODO: extract time functions to their own module
}
