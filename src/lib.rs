extern crate toml;
extern crate rustc_serialize;
extern crate hyper;
extern crate mockito;

pub mod config;
pub mod flights;

pub type Config = config::Config;

#[derive(Debug)]
pub enum Error {
    LoadingConfig,
    ReadingConfig,
    ParsingConfig
}
