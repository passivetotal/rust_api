extern crate hyper;

use std::io::Read;

use hyper::Client;
use hyper::header::{Authorization, Basic};

pub fn pt_get(auth: Basic, path: String) -> String {
    let client = Client::new();
    let mut url = String::from("https://api.passivetotal.org/v2");
    url.push_str(path.as_str());
    let mut res = client.get(url.as_str()).header(Authorization(auth)).send().unwrap();
    let mut body = String::new();
    res.read_to_string(&mut body).unwrap();
    body
}
