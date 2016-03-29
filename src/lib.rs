pub mod config;
pub mod constants;

extern crate rustc_serialize;
extern crate hyper;

use std::io::Read;
use hyper::{Client, Url};
use hyper::header::{Authorization, Basic};
use rustc_serialize::json;

pub struct PTClient {
    pub client: Client,
    pub auth: Basic,
}

#[derive(RustcDecodable, Debug)]
pub struct Pager {
    pub next: Option<String>,
    pub previous: Option<String>,
    pub page_size: Option<u32>,
}

#[derive(RustcDecodable, Debug)]
pub struct PDNSResult {
    pub recordHash: Option<String>,
    pub resolve: Option<String>,
    pub value: Option<String>,
    pub source: Option<Vec<String>>,
    pub lastSeen: Option<String>,
    pub firstSeen: Option<String>,
    pub collected: Option<String>,
}

#[derive(RustcDecodable, Debug)]
pub struct PDNSResponse {
    pub totalRecords: u32,
    pub queryValue: Option<String>,
    pub queryType: Option<String>,
    pub firstSeen: Option<String>,
    pub lastSeen: Option<String>,
    pub results: Option<Vec<PDNSResult>>,
    pub pager: Option<Pager>,
}

#[derive(RustcDecodable, Debug)]
pub struct WhoisResponse {
    pub contactEmail: Option<String>,
    pub domain: Option<String>,
    pub billing: Option<Registrant>,
    pub zone: Option<Registrant>,
    pub nameServers: Option<Vec<String>>,
    pub registered: Option<String>,
    pub lastLoadedAt: Option<String>,
    pub whoisServer: Option<String>,
    pub registryUpdatedAt: Option<String>,
    pub admin: Option<Registrant>,
    pub expiresAt: Option<String>,
    pub registrar: Option<String>,
    pub tech: Option<Registrant>,
    pub registrant: Option<Registrant>,
}

#[derive(RustcDecodable, Debug)]
pub struct Registrant {
    pub city: Option<String>,
    pub name: Option<String>,
    pub country: Option<String>,
    pub telephone: Option<String>,
    pub state: Option<String>,
    pub street: Option<String>,
    pub postalCode: Option<String>,
    pub organization: Option<String>,
    pub email: Option<String>,
}

#[derive(RustcDecodable, Debug)]
pub struct SSLCertResponse {
    pub results: Option<Vec<SSLCertResults>>,
}

#[derive(RustcDecodable, Debug)]
pub struct SSLCertResults {
    pub sha1: Option<String>,
    pub ipAddresses: Option<Vec<String>>,
    pub firstSeen: Option<String>,
    pub lastSeen: Option<String>,
}

macro_rules! define_get_decoder {
    ($name: ident, $path: expr, $elem_ty: ty) => {
        pub fn $name(&self, query: &str) -> $elem_ty {
            let mut url = self.make_url($path);
            url.set_query_from_pairs(&[("query", query)]);
            let body = self.get_response_body(&url);
            json::decode(body.as_str()).unwrap()
        }
    }
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

    define_get_decoder!(get_pdns, "/dns/passive", PDNSResponse);
    define_get_decoder!(get_whois, "/whois", WhoisResponse);
    define_get_decoder!(get_sslcert, "/ssl-certificate/history", SSLCertResponse);
}
