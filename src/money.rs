use std::str::FromStr;

use regex::Regex;

use Error;

pub fn parse(input: &str) -> Result<(f32, &str), Error> {
    let regexp = try!(Regex::new(r"(?P<currency>\w{3})(?P<amount>-?\d+(.\d+)?)").map_err(|_| Error::ParsingMoney));
    let captures = try!(regexp.captures(input).ok_or(Error::ParsingMoney));

    match (captures.name("amount"), captures.name("currency")) {
        (Some(amount), Some(currency)) => {
            let parsed_amount = try!(f32::from_str(&amount).map_err(|_| Error::ParsingMoney));

            Ok((parsed_amount, currency))
        },
        _ => Err(Error::ParsingMoney)
    }
}
