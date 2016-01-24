extern crate toml;
extern crate rustc_serialize;
extern crate hyper;
extern crate regex;
extern crate rusqlite;
extern crate time;
extern crate mockito;

pub mod session;
pub mod flights;
pub mod money;

pub type Session = session::Session;

#[derive(Debug)]
pub enum Error {
    LoadingConfig,
    ReadingConfig,
    ParsingConfig,
    EstablishingDbConnection,
    CreatingTransaction(String),
    CommitingTransaction(String),
    PreparingDbQuery(String),
    ExecutingDbQuery(String),
    NoIdAssigned,
    EncodingJson,
    SendingRequest,
    ReadingResponse,
    ResponseNotOk(String),
    DecodingJson(String),
    NoPricing,
    NoFlights,
    ParsingMoney(String),
    ParsingTime(String),
    FormattingTime
}
