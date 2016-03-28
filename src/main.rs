extern crate passivetotal;

use passivetotal::config;
use passivetotal::PTClient;

fn main() {
    let conf = config::read_config();
    let client = PTClient::from(conf);
    let path = String::from("/dns/passive?query=passivetotal.org");
    let body = client.get(path);
    println!("{}", body);
}
