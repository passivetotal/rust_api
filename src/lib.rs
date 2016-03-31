#![allow(non_snake_case)]
// Disable warnings that JSON struct fields are camelCase.
// This is just to reflect the true field names of the JSON document.

extern crate rustc_serialize;
extern crate hyper;

mod macros;
pub mod client;
pub mod constants;
pub mod config;
pub mod response;
