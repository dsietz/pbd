//! Data Privacy Inspector specific Errors

use std::error;
use derive_more::Display;
use actix_web::ResponseError;

#[derive(Debug, Clone, Display)]
pub enum Error {
    /// Bad format of Data Tracker Chain
    #[display(fmt = "Unknown Score")]
    UnknownScore,
} 

impl error::Error for Error{}

impl ResponseError for Error{}


#[cfg(test)]
mod tests {
}