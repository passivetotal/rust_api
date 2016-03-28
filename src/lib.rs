pub mod config;

extern crate rustc_serialize;
extern crate hyper;

use std::io::Read;
use hyper::Client;
use hyper::header::{Authorization, Basic};

pub struct PTClient {
    pub client: Client,
    pub auth: Basic,
}

impl PTClient {

    pub fn from(conf: config::Config) -> PTClient {
        let username = conf.username;
        let password = Some(conf.api_key);
        PTClient {
            auth: Basic { username: username, password: password },
            client: Client::new(),
        }
    }

    pub fn get(self, path: String) -> String {
        let mut url = String::from("https://api.passivetotal.org/v2");
        url.push_str(path.as_str());
        let mut res = self.client.get(url.as_str()).header(Authorization(self.auth)).send().unwrap();
        let mut body = String::new();
        res.read_to_string(&mut body).unwrap();
        body
    }

}
