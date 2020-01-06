//! Data Tracker Chain specific Errors

use std::error;
use derive_more::Display;
use actix_web::ResponseError;

#[derive(Debug, Clone, Display)]
pub enum Error {
    /// Bad Data Uasage Agreement
    #[display(fmt = "Invalid or Currupt Marker")]
    BadMarker,
    /// Bad format of Data Uasage Agreement
    #[display(fmt = "Invalid Marker Chain")]
    BadChain,
} 

impl error::Error for Error{}

impl ResponseError for Error{}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_marker_bad() {
        let err = Error::BadMarker;
        assert_eq!(format!("{}", err), "Invalid or Currupt Marker");
    }

    #[test]
    fn test_error_chain_bad() {
        let err = Error::BadChain;
        assert_eq!(format!("{}", err), "Invalid Marker Chain");
    }
}