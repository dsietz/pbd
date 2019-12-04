//! Middleware for ensuring Data Usage Agreements are present and valid
use super::*;
extern crate actix_web;
extern crate actix_service;
extern crate futures;
extern crate reqwest;
extern crate rayon;

/// Turns off validation so that only the the Data-Usage-Agreement header doesn't need to be set
pub const VALIDATION_NONE: u8 = 0;
/// Default validation level is VALIDATION_LOW
pub const VALIDATION_DEFAULT: u8 = 1;
/// Check to see if the Data-Usage-Agreement header is set and has a valid format, but doesn't check to see if the location of the agreements are valid. 
pub const VALIDATION_LOW: u8 = 1;
/// Check to see if the Data-Usage-Agreement header is set, has a valid format, andthat the location of the agreements are valid. 
pub const VALIDATION_HIGH: u8 = 2;

pub mod actix;