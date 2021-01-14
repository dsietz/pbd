//! Middleware for ensuring a Data Tracker Chain is present and valid
use super::*;
extern crate actix_service;
extern crate actix_web;
extern crate futures;
extern crate rayon;

/// Turns off validation so that only the the Data-Tracker-Chain header doesn't need to be set
pub const VALIDATION_NONE: u8 = 0;
/// Default validation level is VALIDATION_LOW
pub const VALIDATION_DEFAULT: u8 = 1;
/// Check to see if the Data-Tracker-Chain header is set, but doesn't check if the chain is valid.
pub const VALIDATION_LOW: u8 = 1;
/// Check to see if the Data-Tracker-Chain header is set and that the chain is valid..
pub const VALIDATION_HIGH: u8 = 2;

pub mod actix;
