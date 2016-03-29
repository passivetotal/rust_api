extern crate passivetotal;

use passivetotal::config;
use passivetotal::PTClient;
use passivetotal::constants::Source;

fn main() {
    let conf = config::read_config();
    let client = PTClient::from(conf);
    let response = client.get_pdns("passivetotal.org");
    println!("PDNS results for passivetotal.org from RiskIQ:");

    for result in response.results {
        if result.source.contains(&Source::RISKIQ.string()) {
            println!("{}: {}", result.lastSeen.unwrap(), result.resolve.unwrap());
        }
    }

}
