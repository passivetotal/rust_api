extern crate passivetotal;

use passivetotal::config;
use passivetotal::PTClient;

fn main() {
    let conf = config::read_config();
    let client = PTClient::from(conf);
    let body = client.get_pdns("passivetotal.org");
    println!("{}", body);
}
