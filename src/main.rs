extern crate docopt;
extern crate rustc_serialize;
extern crate passivetotal;

use docopt::Docopt;

use passivetotal::config;
use passivetotal::client::PTClient;
// use passivetotal::constants::Source;

static USAGE: &'static str = "
Usage: passivetotal pdns <query> [--source=<source>]
       passivetotal pdnsuniq <query>
       passivetotal whois <query>
       passivetotal ssl <query>
       passivetotal sslhistory <query>
       passivetotal osint <query>
       passivetotal malware <query>
       passivetotal subdomains <query>
       passivetotal hostattr <query>
       passivetotal hosttracker <query>
       passivetotal classification <query>
       passivetotal evercomp <query>
       passivetotal ddns <query>
       passivetotal monitor <query>
       passivetotal sinkhole <query>
       passivetotal tag <query>
       passivetotal account
       passivetotal --help
       
       Options:
            -h, --help          Show this error message
            --source=<source>   For pdns command, the source to filter [default:none]
";

#[derive(Debug, RustcDecodable)]
struct Args {
    cmd_pdns: bool,
    cmd_pdnsuniq: bool,
    cmd_whois: bool,
    cmd_ssl: bool,
    cmd_sslhistory: bool,
    cmd_osint: bool,
    cmd_malware: bool,
    cmd_subdomains: bool,
    cmd_account: bool,
    cmd_hostattr: bool,
    cmd_hosttracker: bool,
    cmd_classification: bool,
    cmd_evercomp: bool,
    cmd_ddns: bool,
    cmd_monitor: bool,
    cmd_sinkhole: bool,
    cmd_tag: bool,
    arg_query: String,
    flag_source: String,
}

fn main() {
    let args: Args = Docopt::new(USAGE)
        .and_then(|d| d.decode())
        .unwrap_or_else(|err| err.exit());
    let conf = config::read_config().unwrap();
    let client = PTClient::new(conf);

    if args.cmd_pdns {
        let response = client.get_pdns(args.arg_query.as_str()).unwrap();
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
    } else if args.cmd_pdnsuniq {
        let response = client.get_pdns_unique(args.arg_query.as_str()).unwrap();
        println!("UniquePDNS results for {}:", args.arg_query);
        for (res, ct) in response.frequency.unwrap() {
            println!("{}: {}", res, ct);
        }
        for uniq in response.results.unwrap() {
            println!("{}", uniq);
        }
        match response.pager {
            Some(pager) => {
                println!("previous/next/page_size: {}/{}/{}", pager.previous.unwrap(), pager.next.unwrap(), pager.page_size.unwrap());
            },
            _ => { println!("End of Results"); },
        };
    } else if args.cmd_whois {
        let response = client.get_whois(args.arg_query.as_str()).unwrap();
        println!("Whois results for {}:", args.arg_query);
        println!("email: {}", response.contactEmail.unwrap());
        println!("domain: {}", response.domain.unwrap());
        println!("name servers:");
        for ns in response.nameServers.unwrap() {
            println!("{}", ns);
        }
    } else if args.cmd_ssl {
        let response = client.get_sslcert(args.arg_query.as_str()).unwrap();
        println!("{:?}", response);
    } else if args.cmd_sslhistory {
        let response = client.get_sslcert_history(args.arg_query.as_str()).unwrap();
        println!("SSL Certificate history for {}:", args.arg_query);
        for result in response.results.unwrap() {
            println!("SHA1: {}", result.sha1.unwrap());
            println!("First Seen: {}", result.firstSeen.unwrap());
            for ip in result.ipAddresses.unwrap() {
                println!("  IPv4: {}", ip);
            }
        }
    } else if args.cmd_osint {
        let response = client.get_osint(args.arg_query.as_str()).unwrap();
        println!("OSINT resources:");
        for result in response.results.unwrap() {
            println!("Source URL: {}", result.sourceUrl.unwrap());
            for reported in result.inReport.unwrap() {
                println!("  - {}", reported);
            }
        }
    } else if args.cmd_malware {
        let response = client.get_malware(args.arg_query.as_str()).unwrap();
        for result in response.results.unwrap() {
            println!("Source URL: {}", result.sourceUrl.unwrap());
            println!("Sample:     {}", result.sample.unwrap());
        }
    } else if args.cmd_subdomains {
        let response = client.get_subdomains(args.arg_query.as_str()).unwrap();
        for sub in response.subdomains.unwrap() {
            println!("{}", sub);
        }
    } else if args.cmd_account {
        let response = client.get_account().unwrap();
        println!("Username: {}", response.username.unwrap());
        println!("Name: {} {}", response.firstName.unwrap(), response.lastName.unwrap());
        println!("Org: {}", response.organization.unwrap());
    } else if args.cmd_hostattr {
        let response = client.get_host_attribute_components(args.arg_query.as_str()).unwrap();
        for result in response.results.unwrap() {
            println!("Hostname: {}  Category: {}  Label: {}", result.hostname.unwrap(), result.category.unwrap(), result.label.unwrap());
        }
    } else if args.cmd_hosttracker {
        let response = client.get_host_attribute_trackers(args.arg_query.as_str()).unwrap();
        for result in response.results.unwrap() {
            println!("Hostname: {}  {}: {}", result.hostname.unwrap(), result.attributeType.unwrap(), result.attributeValue.unwrap());
        }
    } else if args.cmd_classification {
        let response = client.get_classification(args.arg_query.as_str()).unwrap();
        println!("{}", response.classification.unwrap());
    } else if args.cmd_evercomp {
        let response = client.get_ever_compromised(args.arg_query.as_str()).unwrap();
        println!("{}", response.everCompromised.unwrap());
    } else if args.cmd_ddns {
        let response = client.get_ddns(args.arg_query.as_str()).unwrap();
        println!("{}", response.dynamicDns.unwrap());
    } else if args.cmd_monitor {
        let response = client.get_monitor(args.arg_query.as_str()).unwrap();
        println!("{}", response.monitor.unwrap());
    } else if args.cmd_sinkhole {
        let response = client.get_sinkhole(args.arg_query.as_str()).unwrap();
        println!("{}", response.sinkhole.unwrap());
    } else if args.cmd_tag {
        let response = client.get_tags(args.arg_query.as_str()).unwrap();
        for tag in response.tags.unwrap() {
            println!("{}", tag);
        }
    }
}
