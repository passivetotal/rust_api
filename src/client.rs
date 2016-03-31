//! The client module exposes the `PTClient` struct which is the main interface to use, and
//! contains all methods necessary to make calls to the API.
//!
//! Please see the [`passivetotal::client::PTClient`][1] documentation
//!
//! [1]: ./struct.PTClient.html

use std::io::Error as IoError;
use std::io::Read;
use hyper::{Client, Url};
use hyper::header::{Authorization, Basic};
use rustc_serialize::json;
use response::*;
use hyper;

use config;

/// This is the client exposed to the user for abstracting the passivetotal API.
pub struct PTClient {
    /// the hyper::Client
    pub client: Client,
    /// stores the http basic auth credentials
    pub auth: Basic,
}

#[derive(Debug)]
pub enum ResponseError {
    Json(json::DecoderError),
    Http(IoError),
}

map_to_error!(ResponseError, json::DecoderError, ResponseError::Json);
map_to_error!(ResponseError, IoError, ResponseError::Http);

/// This macro allows me to define functions that perform a GET on the endpoint specified, and
/// return an instance of the type passed into the macro.
/// json::decode doesn't like decoding with a function response of a generic type, so a macro seemed
/// like the best option to abstract this.
macro_rules! define_get_decoder {
    ($name: ident, $path: expr, $elem_ty: ty) => {
        pub fn $name(&self, query: &str) -> Result<$elem_ty, ResponseError> {
            let mut url = self.make_url($path);
            url.set_query_from_pairs(&[("query", query)]);
            let body = try!(self.get_response_body(&url));
            Ok(try!(json::decode(body.as_str())))
        }
    }
}

/// This is used to create a GET function without arguments (just one endpoint /account)
macro_rules! define_get_decoder_no_args {
    ($name: ident, $path: expr, $elem_ty: ty) => {
        pub fn $name(&self) -> Result<$elem_ty, ResponseError> {
            let url = self.make_url($path);
            let body = try!(self.get_response_body(&url));
            Ok(try!(json::decode(body.as_str())))
        }
    }
}

/// The `PTClient` is the main interface into making calls to the PassiveTotal API.
/// You instanciate a `PTClient` with the `new` method which takes a `config::Config` as its
/// argument.
///
/// All the GET functions are defined by the macro `define_get_decoder!`
///
/// # Examples
/// ```
/// use passivetotal::config::read_config;
/// use passivetotal::client::PTClient;
///
/// let conf = match read_config() {
///     Ok(conf) => conf,
///     _ => panic!("Please create your config at ~/.config/api_config.json"),
/// }
/// let client = PTClient::new(conf);
/// let response = match client.get_pdns("passivetotal.org") {
///     Ok(response) => response,
///     _ => panic!("Something bad happened in the JSON response"),
/// };
/// ```
///
/// See main.rs for full usage examples.
/// See the [response module][2] documentation for details on the sorts of responses returned.
/// See the [API documentation][1] for more resources.
///
/// [1]: https://api.passivetotal.org/api/docs/
/// [2]: ../response/index.html
impl PTClient {

    /// Creates a PTClient from a JSON Config from ~/.config/passivetotal/api_config.json
    pub fn new(conf: config::Config) -> PTClient {
        let username = conf.username;
        let password = Some(conf.api_key);
        PTClient {
            auth: Basic { username: username, password: password },
            client: Client::new(),
        }
    }

    fn get_response_body(&self, url: &Url) -> Result<String, ResponseError> {
        // Takes a hyper::Url and returns the text body
        let mut res = self.client.get(url.serialize().as_str()).header(Authorization(self.auth.clone())).send().unwrap();
        let mut body = String::new();
        try!(res.read_to_string(&mut body));
        Ok(body)
    }

    fn make_url(&self, path: &str) -> hyper::Url {
        // Takes the passivetotal url path and builds a hyper::Url
        let url_str = format!("https://api.passivetotal.org/v2{}", path);
        match Url::parse(url_str.as_str()) {
            Ok(u) => u,
            Err(e) => panic!("Failed to build url from {}: {}", path, e),
        }
    }

    /// These macros define functions named get_pdns, get_whois, get_sslcert which will return an
    /// instance of the response type.
    /// I used a macro because generics don't seem to work with json::decode with a generic return
    /// type.
    /// The definition will be: pub fn get_pdns(&self, query: &str) -> PDNSResponse
    define_get_decoder!(get_pdns, "/dns/passive", PDNSResponse);
    define_get_decoder!(get_pdns_unique, "/dns/passive/unique", PDNSUniqueResponse);
    define_get_decoder!(get_whois, "/whois", WhoisResponse);
    define_get_decoder!(get_sslcert, "/ssl-certificate", SSLCertResponse);
    define_get_decoder!(get_sslcert_history, "/ssl-certificate/history", SSLCertHistoryResponse);
    define_get_decoder!(get_osint, "/enrichment/osint", OSINTResponse);
    define_get_decoder!(get_malware, "/enrichment/malware", MalwareResponse);
    define_get_decoder!(get_subdomains, "/enrichment/subdomains", SubdomainsResponse);
    define_get_decoder!(get_host_attribute_components, "/host-attributes/components", HostAttributeComponentResponse);
    define_get_decoder!(get_host_attribute_trackers, "/host-attributes/trackers", HostAttributeTrackerResponse);
    define_get_decoder!(get_classification, "/actions/classification", ActionClassificationResponse);
    define_get_decoder!(get_ever_compromised, "/actions/ever-compromised", ActionEverCompromisedResponse);
    define_get_decoder!(get_ddns, "/actions/dynamic-dns", ActionDDNSResponse);
    define_get_decoder!(get_monitor, "/actions/monitor", ActionMonitorResponse);
    define_get_decoder!(get_sinkhole, "/actions/sinkhole", ActionSinkholeResponse);
    define_get_decoder!(get_tags, "/actions/tags", ActionTagResponse);
    
    /// This doesn't take a query, just responds with information about your account.
    define_get_decoder_no_args!(get_account, "/account", AccountResponse);
}
