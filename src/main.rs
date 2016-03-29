extern crate docopt;
extern crate rustc_serialize;
extern crate passivetotal;

use docopt::Docopt;

use passivetotal::config;
use passivetotal::PTClient;
use passivetotal::constants::Source;

static USAGE: &'static str = "
Usage: passivetotal pdns <query>
       passivetotal --help
       
       Option:
            -h, --help      Show this error message
";

#[derive(Debug, RustcDecodable)]
struct Args {
    cmd_pdns: bool,
    arg_query: String,
}

fn main() {
    let args: Args = Docopt::new(USAGE)
        .and_then(|d| d.decode())
        .unwrap_or_else(|err| err.exit());
    let conf = config::read_config();
    let client = PTClient::from(conf);

    if args.cmd_pdns {
        let response = client.get_pdns(args.arg_query.as_str());
        println!("PDNS results for passivetotal.org from RiskIQ:");
        for result in response.results {
            if result.source.contains(&Source::RISKIQ.string()) {
                println!("{}: {}", result.lastSeen.unwrap(), result.resolve.unwrap());
            }
        }
    }
}
