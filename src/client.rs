pub use std::io::Read;
pub use hyper::{Client, Url};
use hyper::header::{Authorization, Basic};
use rustc_serialize::json;
use response::*;
use hyper;

use config;

// This is the client exposed to the user for abstracting the passivetotal API
pub struct PTClient {
    // the hyper::Client
    pub client: Client,
    // stores the http basic auth credentials
    pub auth: Basic,
}

// This macro allows me to define functions that perform a GET on the endpoint specified, and
// return an instance of the type passed into the macro.
// json::decode doesn't like decoding with a function response of a generic type, so a macro seemed
// like the best option to abstract this.
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

macro_rules! define_get_decoder_no_args {
    ($name: ident, $path: expr, $elem_ty: ty) => {
        pub fn $name(&self) -> $elem_ty {
            let url = self.make_url($path);
            let body = self.get_response_body(&url);
            json::decode(body.as_str()).unwrap()
        }
    }
}

impl PTClient {

    pub fn new(conf: config::Config) -> PTClient {
        // Creates a PTClient from a JSON Config from ~/.config/passivetotal/api_config.json
        let username = conf.username;
        let password = Some(conf.api_key);
        PTClient {
            auth: Basic { username: username, password: password },
            client: Client::new(),
        }
    }

    fn get_response_body(&self, url: &Url) -> String {
        // Takes a hyper::Url and returns the text body
        let mut res = self.client.get(url.serialize().as_str()).header(Authorization(self.auth.clone())).send().unwrap();
        let mut body = String::new();
        res.read_to_string(&mut body).unwrap();
        body
    }

    fn make_url(&self, path: &str) -> hyper::Url {
        // Takes the passivetotal url path and builds a hyper::Url
        let url_str = format!("https://api.passivetotal.org/v2{}", path);
        match Url::parse(url_str.as_str()) {
            Ok(u) => u,
            Err(e) => panic!("Failed to build url from {}: {}", path, e),
        }
    }

    // These macros define functions named get_pdns, get_whois, get_sslcert which will return an
    // instance of the response type.
    // I used a macro because generics don't seem to work with json::decode with a generic return
    // type.
    // The definition will be: pub fn get_pdns(&self, query: &str) -> PDNSResponse
    define_get_decoder!(get_pdns, "/dns/passive", PDNSResponse);
    define_get_decoder!(get_whois, "/whois", WhoisResponse);
    define_get_decoder!(get_sslcert, "/ssl-certificate/history", SSLCertResponse);
    define_get_decoder!(get_osint, "/enrichment/osint", OSINTResponse);
    define_get_decoder!(get_malware, "/enrichment/malware", MalwareResponse);
    define_get_decoder!(get_subdomains, "/enrichment/subdomains", SubdomainsResponse);
    define_get_decoder_no_args!(get_account, "/account", AccountResponse);
    define_get_decoder!(get_host_attribute_components, "/host-attributes/components", HostAttributeComponentResponse);
    define_get_decoder!(get_host_attribute_trackers, "/host-attributes/trackers", HostAttributeTrackerResponse);
}
