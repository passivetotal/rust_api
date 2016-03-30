#![allow(non_snake_case)]
// Disable warnings that JSON struct fields are camelCase.
// This is just to reflect the true field names of the JSON document.

extern crate rustc_serialize;
extern crate hyper;

mod macros;
pub mod constants;
pub mod config;
pub mod response;
pub mod client;

#[test]
fn test_pdns_riskiq() {
    let conf = config::read_config().unwrap();
    let client = client::PTClient::from(conf);
    let response = client.get_pdns("sf.riskiq.net");
    let results = response.results.unwrap();
    assert_eq!(results.len(), 2);
    assert_eq!(results[0].resolve, Some("192.65.247.107".to_string()));
}
