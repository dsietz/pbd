//! Data Security Guard specific Errors

use derive_more::Display;
use std::error;

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

impl error::Error for Error {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_bad_keypair() {
        let err = Error::BadKeyPairError;
        assert_eq!(format!("{}", err), "Bad key pair provided.");
    }

    #[test]
    fn test_error_bad_transferset() {
        let err = Error::BadTransferSetError;
        assert_eq!(format!("{}", err), "Bad transfer set provided.");
    }

    #[test]
    fn test_error_decryption() {
        let err = Error::DecryptionError;
        assert_eq!(format!("{}", err), "Unable to decrypt the data.");
    }

    #[test]
    fn test_error_encryption() {
        let err = Error::EncryptionError;
        assert_eq!(format!("{}", err), "Unable to encrypt the data.");
    }

    #[test]
    fn test_error_missing_nonce() {
        let err = Error::MissingNonceError;
        assert_eq!(format!("{}", err), "Missing required nonce (a.k.a. IV).");
    }

    #[test]
    fn test_error_missing_symmetric_key() {
        let err = Error::MissingSymmetricKeyError;
        assert_eq!(format!("{}", err), "Missing required symmetric key.");
    }

    #[test]
    fn test_error_payload_overflow() {
        let err = Error::PayloadOverflowError;
        assert_eq!(format!("{}", err), "Exeeded limit for reading payload.");
    }

    #[test]
    fn test_error_payload_timeout() {
        let err = Error::PayloadTimeoutError;
        assert_eq!(format!("{}", err), "Timed out while reading the payload.");
    }

    #[test]
    fn test_error_payload_unreadbale() {
        let err = Error::PayloadUnreadableError;
        assert_eq!(format!("{}", err), "Cannot read payload.");
    }
}
