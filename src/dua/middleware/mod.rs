//! Middleware for ensuring Data Usage Agreements are present and valid
use super::*;
extern crate actix_web;
extern crate actix_service;
extern crate futures;
extern crate reqwest;
extern crate rayon;

pub mod actix;