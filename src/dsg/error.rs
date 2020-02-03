//! Data Security Guard specific Errors

use std::error;
use derive_more::Display;
use actix_web::ResponseError;

#[derive(Debug, Clone, Display)]
pub enum Error {
    /// Bad RSA Keypair
    #[display(fmt = "Bad key pair provided.")]
    BadKeyPairError,
    /// Bad TransferSet
    #[display(fmt = "Bad transfer set provided.")]
    BadTransferSetError,    
    /// Decryption issue
    #[display(fmt = "Unable to decrypt the data.")]
    DecryptionError,
    /// Encryption issue
    #[display(fmt = "Unable to encrypt the data.")]
    EncryptionError,
    /// Missing Nonce
    #[display(fmt = "Missing required nonce (a.k.a. IV).")]
    MissingNonceError, 
    /// Missing symmetric key
    #[display(fmt = "Missing required symmetric key.")]
    MissingSymmetricKeyError,
    /// Overflow for reading payload
    #[display(fmt = "Exeeded limit for reading payload.")]
    PayloadOverflowError,
    /// Timed out while reading the payload
    #[display(fmt = "Timed out while reading the payload.")]
    PayloadTimeoutError,
    /// Cannot read payload
    #[display(fmt = "Cannot read payload.")]
    PayloadUnreadableError,
} 

impl error::Error for Error{}

impl ResponseError for Error{}