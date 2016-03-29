pub mod config;

extern crate rustc_serialize;
extern crate hyper;

use std::io::Read;
use hyper::{Client, Url};
use hyper::header::{Authorization, Basic};

pub struct PTClient {
    pub client: Client,
    pub auth: Basic,
}

impl PTClient {

    pub fn from(conf: config::Config) -> PTClient {
        // Creates a PTClient from a JSON Config from ~/.config/passivetotal/api_config.json
        let username = conf.username;
        let password = Some(conf.api_key);
        PTClient {
            auth: Basic { username: username, password: password },
            client: Client::new(),
        }
    }

    pub fn get_response_body(&self, url: &Url) -> String {
        // Takes a hyper::Url and returns the text body
        let mut res = self.client.get(url.serialize().as_str()).header(Authorization(self.auth.clone())).send().unwrap();
        let mut body = String::new();
        res.read_to_string(&mut body).unwrap();
        body
    }

    fn make_url(&self, path: &str) -> hyper::Url {
        // Takes the passivetotal url path and builds a hyper::Url
        let mut url_str = String::from("https://api.passivetotal.org/v2");
        url_str.push_str(path);
        let url = match Url::parse(url_str.as_str()) {
            Ok(u) => u,
            _ => panic!("Failed to build url from {}", path),
        };
        url
    }

    pub fn get_pdns(&self, query: &str) -> String {
        // Returns the String body of a pdns query
        let mut url = self.make_url("/dns/passive");
        url.set_query_from_pairs(&[("query", query)]);
        self.get_response_body(&url)
    }
}
