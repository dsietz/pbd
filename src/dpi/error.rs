//! Data Privacy Inspector specific Errors

use derive_more::Display;
use std::error;

#[derive(Debug, Clone, Display)]
pub enum Error {
    /// Bad format of Data Tracker Chain
    #[display(fmt = "Unknown Score")]
    UnknownScore,
}

impl error::Error for Error {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_unknown_score() {
        let err = Error::UnknownScore;
        assert_eq!(format!("{}", err), "Unknown Score");
    }
}
