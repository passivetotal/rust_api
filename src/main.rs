extern crate docopt;
extern crate rustc_serialize;
extern crate passivetotal;

use docopt::Docopt;

use passivetotal::config;
use passivetotal::client::PTClient;
// use passivetotal::constants::Source;

static USAGE: &'static str = "
Usage: passivetotal pdns <query> [--source=<source>]
       passivetotal whois <query>
       passivetotal ssl <query>
       passivetotal --help
       
       Options:
            -h, --help          Show this error message
            --source=<source>   Source to filter [default:none]
";

#[derive(Debug, RustcDecodable)]
struct Args {
    cmd_pdns: bool,
    cmd_whois: bool,
    cmd_ssl: bool,
    arg_query: String,
    flag_source: String,
}

fn main() {
    let args: Args = Docopt::new(USAGE)
        .and_then(|d| d.decode())
        .unwrap_or_else(|err| err.exit());
    let conf = config::read_config().unwrap();
    let client = PTClient::from(conf);

    if args.cmd_pdns {
        let response = client.get_pdns(args.arg_query.as_str());
        println!("PDNS results for {}:", args.arg_query);
        for result in response.results.unwrap() {
            if args.flag_source == "none" || result.source.unwrap().contains(&args.flag_source) {
                println!("{}: {}", result.lastSeen.unwrap(), result.resolve.unwrap());
            }
        }
        match response.pager {
            Some(pager) => {
                println!("previous/next/page_size: {}/{}/{}", pager.previous.unwrap(), pager.next.unwrap(), pager.page_size.unwrap());
            },
            _ => { println!("End of Results"); },
        };
    } else if args.cmd_whois {
        let response = client.get_whois(args.arg_query.as_str());
        println!("Whois results for {}:", args.arg_query);
        println!("email: {}", response.contactEmail.unwrap());
        println!("domain: {}", response.domain.unwrap());
        println!("name servers:");
        for ns in response.nameServers.unwrap() {
            println!("{}", ns);
        }
    } else if args.cmd_ssl {
        let response = client.get_sslcert(args.arg_query.as_str());
        println!("SSL Certificate history for {}:", args.arg_query);
        for result in response.results.unwrap() {
            println!("SHA1: {}", result.sha1.unwrap());
            println!("First Seen: {}", result.firstSeen.unwrap());
            for ip in result.ipAddresses.unwrap() {
                println!("  IPv4: {}", ip);
            }
        }
    }
}
