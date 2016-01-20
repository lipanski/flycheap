extern crate toml;
extern crate rustc_serialize;
extern crate hyper;
extern crate regex;
extern crate rusqlite;
extern crate mockito;

pub mod config;
pub mod flights;
pub mod money;

pub type Config = config::Config;

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
    EncodingJson,
    SendingRequest,
    ReadingResponse,
    ResponseNotOk(String),
    DecodingJson(String),
    NoPricing,
    NoFlights,
    ParsingMoney(String)
}
