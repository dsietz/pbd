//! An Extractor that parses the HTTP request and pulls out the TransferSet

use super::*;
extern crate actix_web;
extern crate actix_service;
extern crate futures;

pub mod actix;