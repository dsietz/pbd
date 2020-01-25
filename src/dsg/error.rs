//! Data Security Guard specific Errors

use derive_more::Display;

#[derive(Debug, Clone, Display)]
pub enum Error {
    /// Bad RSA Keypair
    #[display(fmt = "Bad key pair provided.")]
    BadKeyPairError,
    /// Decryption issue
    #[display(fmt = "Unable to decrypt the data.")]
    DecryptionError,
    /// Encryption issue
    #[display(fmt = "Unable to encrypt the data.")]
    EncryptionError,
} 