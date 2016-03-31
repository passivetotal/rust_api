//! It is necessary to create the correct configuration before using the API.
//! 
//! You must put a JSON document at ~/.config/passivetotal/api_config.json, that looks like:
//!
//! ```json
//! {
//!     "username": "your_email@example.com",
//!     "api_key": "The API key associated with your account",
//!     "http_proxy": "<unused, keep empty>",
//!     "https_proxy": "<unused, keep empty>",
//!     "api_server": "api.passivetotal.org",
//! }
//! ```
//!
//! The config generator isn't implemented in this rust implementation yet so it is necessary to
//! create it by hand or generate it with the [Python API][1], installable via:
//! `pip install passivetotal`
//!
//! # Examples
//! ```
//! use passivetotal::config::read_config;
//! use passivetotal::client::PTClient;
//!
//! let conf = try!(read_config());
//! let client = PTClient::new(conf);
//! ```
//!
//! [1]: https://pypi.python.org/pypi/passivetotal

use std::io;
use std::env;
use std::io::prelude::*;
use std::fs::File;
use rustc_serialize::json;

#[derive(Debug)]
pub enum ConfigError {
    Io(io::Error),
    Json(json::DecoderError),
    Path(String),
}

map_to_error!(ConfigError, io::Error, ConfigError::Io);
map_to_error!(ConfigError, json::DecoderError, ConfigError::Json);

#[derive(RustcDecodable, RustcEncodable)]
pub struct Config {
    pub username: String,
    pub api_server: String,
    pub http_proxy: String,
    pub https_proxy: String,
    pub api_key: String,
    pub api_version: String,
}

pub fn read_config() -> Result<Config, ConfigError> {
    let mut path = try!(env::home_dir().ok_or(ConfigError::Path("cant get homedir".to_string())));
    path.push(".config/passivetotal/api_config.json");
    let mut file = try!(File::open(path));
    let mut contents = String::new();
    try!(file.read_to_string(&mut contents));
    Ok(try!(json::decode(contents.as_str())))
}
