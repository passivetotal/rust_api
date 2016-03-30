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


impl From<io::Error> for ConfigError {
    fn from(err: io::Error) -> ConfigError {
        ConfigError::Io(err)
    }
}

impl From<json::DecoderError> for ConfigError {
    fn from(err: json::DecoderError) -> ConfigError {
        ConfigError::Json(err)
    }
}

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
