use std::env;
use std::io::prelude::*;
use std::fs::File;
use rustc_serialize::json;

#[derive(RustcDecodable, RustcEncodable)]
pub struct Config {
    pub username: String,
    pub api_server: String,
    pub http_proxy: String,
    pub https_proxy: String,
    pub api_key: String,
    pub api_version: String,
}

pub fn read_config() -> Config {
    let mut path = match env::home_dir() {
        Some(p) => p,
        None => panic!("Unable to get home dir"),
    };
    path.push(".config/passivetotal/api_config.json");
    let mut file = match File::open(path) {
        Ok(file) => file,
        _ => panic!("Could not read config file"),
    };
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    match json::decode(contents.as_str()) {
        Ok(conf) => conf,
        _ => panic!("Could not read config file"),
    }
}
