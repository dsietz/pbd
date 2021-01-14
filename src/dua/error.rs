//! Data Usage Agreement specific Errors

use actix_web::ResponseError;
use derive_more::Display;
use std::error;

#[derive(Debug, Clone, Display)]
pub enum Error {
    /// Bad Data Uasage Agreement
    #[display(fmt = "Malformed or missing one or more Data Usage Agreements")]
    BadDUA,
    /// Bad format of Data Uasage Agreement
    #[display(fmt = "Invalid format for Data Usage Agreement")]
    BadDUAFormat,
    /// Missing Data Uasage Agreement
    #[display(fmt = "Missing one or more Data Usage Agreements")]
    MissingDUA,
}

impl error::Error for Error {}

impl ResponseError for Error {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_dua_bad() {
        let err = Error::BadDUA;
        assert_eq!(
            format!("{}", err),
            "Malformed or missing one or more Data Usage Agreements"
        );
    }

    #[test]
    fn test_error_dua_missing() {
        let err = Error::MissingDUA;
        assert_eq!(
            format!("{}", err),
            "Missing one or more Data Usage Agreements"
        );
    }

    #[test]
    fn test_error_dua_bad_format() {
        let err = Error::BadDUAFormat;
        assert_eq!(
            format!("{}", err),
            "Invalid format for Data Usage Agreement"
        );
    }
}
