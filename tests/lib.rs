extern crate passivetotal;

use passivetotal::{client,config};

#[test]
fn test_pdns_riskiq() {
    let conf = config::read_config().unwrap();
    let client = client::PTClient::from(conf);
    let response = client.get_pdns("sf.riskiq.net");
    let results = response.results.unwrap();
    assert_eq!(results.len(), 2);
    assert_eq!(results[0].resolve, Some("192.65.247.107".to_string()));
}
