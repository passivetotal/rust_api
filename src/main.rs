extern crate hyper;
extern crate rustc_serialize;
use hyper::header::Basic;

mod lib;
mod config;

fn main() {
    let conf = config::read_config();
    let path = String::from("/dns/passive?query=passivetotal.org");
    let username = conf.username;
    let password = Some(conf.api_key);
    let auth = Basic { username: username, password: password };
    let body = lib::pt_get(auth, path);
    println!("{}", body);
}
