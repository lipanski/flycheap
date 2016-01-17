use flights::search_response::TripOption;
use flights::Error;

pub struct Offer {
    pub base_price: String,
    pub sale_price: String,
    pub tax_price: String,
    pub total_price: String,
    pub latest_ticketing_time: String,
    pub refundable: bool,
    pub flights: Vec<Flight>
}

pub struct Flight {
    pub from: String,
    pub to: String,
    pub departure_time: String,
    pub arrival_time: String,
    pub duration: u32,
    pub mileage: u32,
    pub seat: String,
    pub aircraft: String,
    pub carrier: String,
    pub number: String
}

impl Offer {
    pub fn from(option: TripOption) -> Result<Self, Error> {
        let pricing = try!(option.pricing.get(0).ok_or(Error::NoPricing));

        let offer = Offer {
            base_price: pricing.baseFareTotal.clone(),
            sale_price: pricing.saleFareTotal.clone(),
            tax_price: pricing.saleTaxTotal.clone(),
            total_price: pricing.saleTotal.clone(),
            latest_ticketing_time: pricing.latestTicketingTime.clone(),
            refundable: pricing.refundable.unwrap_or(false),
            flights: vec!()
        };

        Ok(offer)
    }
}
