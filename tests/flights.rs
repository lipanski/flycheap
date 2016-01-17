extern crate fly_cheap;
extern crate mockito;

use fly_cheap::flights::{PriceRequest, PriceResponse};

fn mocked_roundtrip() -> PriceResponse {
    mockito::mock("POST", "/qpxExpress/v1/trips/search?key=api_key").respond_with_file("tests/mocks/roundtrip.http");

    let mut request = PriceRequest::new();
    request.add_trip("TXL", "OTP", "2016-03-28", 0);
    request.add_trip("OTP", "TXL", "2016-04-03", 0);

    request.call("api_key").unwrap()
}

#[test]
fn test_call_with_roundtrip_basic() {
    let response = mocked_roundtrip();

    assert_eq!("bwrfa5nntkafbK6iE0NcY4", response.trips.requestId);
}

#[test]
fn test_call_with_roundtrip_airports() {
    let response = mocked_roundtrip();
    let airports = &response.trips.data.airport;

    assert_eq!(2, airports.len());
    assert_eq!("Bucharest Henri Coanda", airports.get(0).unwrap().name);
    assert_eq!("Berlin Tegel", airports.get(1).unwrap().name);
}

#[test]
fn test_call_with_roundtrip_aircraft() {
    let response = mocked_roundtrip();
    let aircrafts = &response.trips.data.aircraft;

    assert_eq!(1, aircrafts.len());
    assert_eq!("Airbus A319", aircrafts.get(0).unwrap().name);
}

#[test]
fn test_call_with_roundtrip_carrier() {
    let response = mocked_roundtrip();
    let carriers = &response.trips.data.carrier;

    assert_eq!(1, carriers.len());
    assert_eq!("AB", carriers.get(0).unwrap().code);
}

#[test]
fn test_call_with_roundtrip_first_option_sale_total() {
    let response = mocked_roundtrip();
    let first_option = &response.trips.tripOption;

    assert_eq!(4, first_option.len());
    assert_eq!("EUR194.56", first_option.get(0).unwrap().saleTotal);
}

#[test]
fn test_call_with_roundtrip_first_option_slice() {
    let response = mocked_roundtrip();
    let slice = &response.trips.tripOption.get(0).unwrap().slice;

    assert_eq!(2, slice.len());
    assert_eq!(130, slice.get(0).unwrap().duration);
    assert_eq!(1, slice.get(0).unwrap().segment.len());
    assert_eq!("8272", slice.get(0).unwrap().segment.get(0).unwrap().flight.number);
}

#[test]
fn test_call_with_rountrip_first_option_first_leg() {
    let response = mocked_roundtrip();
    let leg = &response.trips.tripOption.get(0).unwrap().slice.get(0).unwrap().segment.get(0).unwrap().leg;

    assert_eq!(1, leg.len());
    assert_eq!("2016-03-29T00:45+03:00", leg.get(0).unwrap().arrivalTime);
    assert_eq!("2016-03-28T21:35+02:00", leg.get(0).unwrap().departureTime);
    assert_eq!("TXL", leg.get(0).unwrap().origin);
    assert_eq!("OTP", leg.get(0).unwrap().destination);
}

#[test]
fn test_call_with_rountrip_first_option_second_leg() {
    let response = mocked_roundtrip();
    let leg = &response.trips.tripOption.get(0).unwrap().slice.get(1).unwrap().segment.get(0).unwrap().leg;

    assert_eq!(1, leg.len());
    assert_eq!("2016-04-03T07:35+02:00", leg.get(0).unwrap().arrivalTime);
    assert_eq!("2016-04-03T06:30+03:00", leg.get(0).unwrap().departureTime);
    assert_eq!("OTP", leg.get(0).unwrap().origin);
    assert_eq!("TXL", leg.get(0).unwrap().destination);
}

#[test]
fn test_call_with_roundtrip_first_option_pricing() {
    let response = mocked_roundtrip();
    let pricing = &response.trips.tripOption.get(0).unwrap().pricing;

    assert_eq!(1, pricing.len());
    assert_eq!("EUR72.00", pricing.get(0).unwrap().baseFareTotal);
    assert_eq!("EUR72.00", pricing.get(0).unwrap().saleFareTotal);
    assert_eq!("EUR122.56", pricing.get(0).unwrap().saleTaxTotal);
    assert_eq!("EUR194.56", pricing.get(0).unwrap().saleTotal);
    assert_eq!("2016-01-20T17:48-05:00", pricing.get(0).unwrap().latestTicketingTime);
}
