rust-passivetotal
=================

Rust API and command-line app for Rust

Installation
------------

From the project root directory::

    $ cargo build

Or link to the project in your Cargo.toml::

    [dependencies]
    passivetotal = { git = "https://bitbucket.org/riskiq/rust-passivetotal.git" }

Configuration
-------------

Create a JSON config in ~/.config/passivetotal/api_config.json with the following structure::

    {
        "username": "",
        "api_key": "<key>",
        "http_proxy": "",
        "https_proxy": "",
        "api_server": "api.passivetotal.org", 
        "api_version": "v2"
    }

Usage
-----

Simply run the command line project with::

    $ ./target/debug/passivetotal (pdns|whois|ssl) <query>

Use --help/-h to view info on the arguments::

    $ ./target/debug/passivetotal --help

API Usage
---------

See main.rs for examples of usage::

    extern crate passivetotal;

    use passivetotal::config;
    use passivetotal::PTClient;

    // automatically parses config at ~/.config/passivetotal/api_config.json
    let conf = config::read_config();
    let client = PTClient::from(conf);

    let response = client.get_pdns("passivetotal.org");
    for result in response.results {
        println!("{}", result.resolve.unwrap());
    }

    let response = client.get_whois("passivetotal.org");
    println!("email: {}", response.contactEmail.unwrap());

    let response = client.get_sslcert("52.8.228.23");
    for result in response.results.unwrap() {
        println!("SHA1: {}", result.sha1.unwrap());
    }


Release Notes
-------------

:0.0.1:
    Project created