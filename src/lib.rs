#![allow(non_snake_case)]
// Disable warnings that JSON struct fields are camelCase.
// This is just to reflect the true field names of the JSON document.
//! This is the Rust implementation of a client for the PassiveTotal API.
//! Usage is provided through `passivetotal::client::PTClient`
//!
//! # Examples
//! ```
//! use passivetotal::config::read_config;
//! use passivetotal::client::PTClient;
//!
//! let conf = try!(read_config());
//! let client = PTClient::new(conf);
//! let response = client.get_pdns("passivetotal.org");
//! ```

extern crate rustc_serialize;
extern crate hyper;

mod macros;
pub mod client;
pub mod constants;
pub mod config;
pub mod response;
