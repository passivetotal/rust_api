#![allow(non_snake_case)]
#![doc(html_logo_url = "https://s3-us-west-1.amazonaws.com/passivetotal-website/public/core-pt-logo-sm.png",
       html_favicon_url = "https://passivetotal.org/static/img/favicon/png",
       html_root_url = "https://passivetotal.org/")]
// Disable warnings that JSON struct fields are camelCase.
// This is just to reflect the true field names of the JSON document.
//! This is the Rust implementation of a client for the PassiveTotal API.
//! Usage is provided through `passivetotal::client::PTClient`
//!
//! Before running anything, you will need to create a JSON configuration. See the [config
//! module][1]
//! documentations for details.
//!
//! Please see the [`passivetotal::client::PTClient`][2] documentation for the available methods.
//!
//! See the [response module documentation][3] for the specification for the response structs.
//!
//! See the [API documentation][4] for more resources.
//!
//! # Examples
//! ```
//! use passivetotal::config::read_config;
//! use passivetotal::client::PTClient;
//!
//! let conf = try!(read_config());
//! let client = PTClient::new(conf);
//! let response = match client.get_pdns("passivetotal.org") {
//!     Ok(response) => response,
//!     _ => panic!("Something bad happened in the JSON response"),
//! };
//! ```
//!
//! [1]: ./config/index.html
//! [2]: ./client/struct.PTClient.html
//! [3]: ./response/index.html
//! [4]: https://api.passivetotal.org/api/docs/

extern crate rustc_serialize;
extern crate hyper;

mod macros;
pub mod client;
pub mod constants;
pub mod config;
pub mod response;
