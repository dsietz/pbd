
//! The `dsg` module provides functionality and structures that the Data Security Guard utilizes to enforce the Privacy by Design `Separate` and `Enforce` strategies.
//! 
//! These security features can be implemented in two manners:
//! 
//! 1. Instantiating a `PrivacyGuard` object and calling it's methods
//! 2. Implementing the `PrivacySecurityGuard` traits for your own defined structure
//!
//! # Examples
//!
//! Utilizing the PrivacyGuard structure to generate a RSA keypair
//!
//! ```
//! extern crate pbd;
//!
//! use pbd::dsg::{PrivacyGuard, PrivacySecurityGuard, TransferSet};
//!
//! fn main() {
//!     let guard = PrivacyGuard {};
//!     let keypair = guard.generate_keypair();
//!     assert!(keypair.is_ok());    
//! }
//! ```
//! 
//! Implementing the PrivacySecurityGuard trait to generate a RSA keypair
//!
//! ```
//! extern crate pbd;
//!
//! use pbd::dsg::{PrivacySecurityGuard};
//!
//! fn main() {
//!     struct MyStruct {}
//!     impl MyStruct {
//!         fn hello(&self) -> String {
//!             "Hello World!".to_string()
//!         }
//!     }
//!     impl PrivacySecurityGuard for MyStruct {}
//! 
//!     let my_obj = MyStruct {};
//!     let keypair = my_obj.generate_keypair();
//! 
//!     println!("{}", my_obj.hello());
//!     assert!(keypair.is_ok());    
//! }
//! ```
//! 
//! Use the `secure_for_tranfer()` and `data_from_tranfer()` methods, we can safely trasnfer the private data.
//! 
//! ```
//! extern crate pbd;
//! extern crate openssl;
//!
//! use pbd::dsg::{PrivacyGuard, PrivacySecurityGuard, TransferSet};
//! use openssl::rsa::Padding;
//!
//! fn main() {
//!     // Obtain your public key, We will generate one for this example instead of reading a predefined public key.
//!     let guard = PrivacyGuard {};
//!     let keypair = guard.generate_keypair().unwrap();
//!     let priv_key = keypair.0;
//!     let pub_key = keypair.1;
//!     let padding = Padding::PKCS1;
//!     let original_message = String::from("my private data").as_bytes().to_vec(); 
//! 
//!     // prepare the data for transfer
//!     let transset = guard.secure_for_tranfer(pub_key, original_message.clone(), padding).unwrap();
//! 
//!     // The TransferSet returned has all the information the source will need to securely transfer the data
//!     // Once the transfer has completed, the target can extract the decrytped data form teh TranferSet
//!     let message_received = guard.data_from_tranfer(priv_key, transset).unwrap();
//!     
//!     assert_eq!(original_message, message_received);
//! }
//! ```

use crate::dsg::error::*;
use rand::Rng; 
use rand::distributions::Alphanumeric;
use openssl::rsa::{Rsa, Padding};
use openssl::symm::{decrypt, encrypt, Cipher};

/// The HTTP header that holds the Nonce (a.k.a. IV) for the RSA encrypted sytemmetirc key
pub static DSG_NONCE_HEADER: &str = "Data-Security-Guard-Nonce";
/// The HTTP header that holds the Padding for the RSA encrypted sytemmetirc key
pub static DSG_PADDING_HEADER: &str = "Data-Security-Guard-Padding";
/// The HTTP header that holds the RSA encrypted sytemmetirc key
pub static DSG_SYTMMETRIC_KEY_HEADER: &str = "Data-Security-Guard-Key";

/// Represents the Security Gaurd
pub struct PrivacyGuard {}

/// Represents the set of attributes your will need to transfer the data safely
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TransferSet {
    pub encrypted_data: Vec<u8>,
    pub encrypted_symmetric_key: Vec<u8>,
    pub nonce: Vec<u8>,
    pub padding: i32,
}

impl TransferSet {
    /// Constructs a TransferSet from a serialized string
    /// 
    /// # Arguments
    /// 
    /// * serialized: &str - The string that represents the serialized object.</br>
    /// 
    /// #Example
    ///
    /// ```
    /// extern crate pbd; 
    ///
    /// use pbd::dsg::TransferSet;
    ///
    /// fn main() {
    ///     let serialized = r#"{
    ///         "encrypted_data":[82,240,199,226,197,63,161,115,68,5,177,72,246,109,171,165],
    ///         "encrypted_symmetric_key":[83,205,166,96,120,119,1,178,36,144,152,51,106,17,220,9,165,240,236,25,228,164,97,192,194,
    ///                                    9,117,249,52,77,14,194,181,37,19,202,104,89,50,2,223,181,173,6,226,32,85,148,103,96,186,
    ///                                    188,217,169,112,109,73,184,39,196,95,161,18,180,239,74,0,112,175,26,116,21,31,88,125,157,
    ///                                    54,39,147,242,28,202,179,132,157,40,163,159,194,74,9,241,108,16,40,81,67,165,57,46,146,195,
    ///                                    37,89,173,124,167,103,30,148,7,4,75,19,73,71,132,142,45,229,150,188,96,56,150,106,125,12,56,
    ///                                    251,8,89,51,5,195,235,234,91,169,36,32,134,183,127,231,159,61,55,221,98,71,217,228,49,52,12,
    ///                                    47,186,14,86,143,247,54,228,184,75,78,3,160,96,214,118,182,133,61,209,129,68,231,121,178,111,
    ///                                    217,99,238,213,101,29,83,11,223,243,239,166,67,180,78,60,1,0,177,74,65,8,5,222,168,170,230,92,
    ///                                    193,31,45,14,111,96,7,232,6,6,26,44,192,197,71,115,204,134,191,0,147,128,244,198,189,201,24,85,
    ///                                    16,170,21,235,143,158,146,206,28,10,200,51,171,135,139,27,120,44],
    ///         "nonce":[83,114,81,112,67,85,116,114,83,86,49,49,89,75,65,49],
    ///         "padding":1
    ///         }"#;
    ///
    ///     let transset = match TransferSet::from_serialized(serialized) {
    ///         Ok(ts) => ts,
    ///         Err(err) => {
    ///             panic!("{}", err);
    ///         },
    ///     };
    /// }
    /// ```
    pub fn from_serialized(serialized: &str) -> Result<TransferSet, Error> {
		match serde_json::from_str(&serialized) {
            Ok(ts) => Ok(ts),
            Err(_err) => Err(Error::BadTransferSetError)
        }
    }

    /// Serializes the TransferSet
    /// 
    /// #Example
    ///
    /// ```
    /// extern crate pbd; 
    ///
    /// use pbd::dsg::TransferSet;
    ///
    /// fn main() {
    ///     let transset = TransferSet {
    ///         encrypted_data: [82,240,199,226,197,63,161,115,68,5,177,72,246,109,171,165].to_vec(),
    ///         encrypted_symmetric_key: [83,205,166,96,120,119,1,178,36,144,152,51,106,17,220,9,165,240,236,25,228,164,97,192,194,9,117,249,52,77,14,194,181,37,19,202,104,89,50,2,223,181,173,6,226,32,85,148,103,96,186,188,217,169,112,109,73,184,39,196,95,161,18,180,239,74,0,112,175,26,116,21,31,88,125,157,54,39,147,242,28,202,179,132,157,40,163,159,194,74,9,241,108,16,40,81,67,165,57,46,146,195,37,89,173,124,167,103,30,148,7,4,75,19,73,71,132,142,45,229,150,188,96,56,150,106,125,12,56,251,8,89,51,5,195,235,234,91,169,36,32,134,183,127,231,159,61,55,221,98,71,217,228,49,52,12,47,186,14,86,143,247,54,228,184,75,78,3,160,96,214,118,182,133,61,209,129,68,231,121,178,111,217,99,238,213,101,29,83,11,223,243,239,166,67,180,78,60,1,0,177,74,65,8,5,222,168,170,230,92,193,31,45,14,111,96,7,232,6,6,26,44,192,197,71,115,204,134,191,0,147,128,244,198,189,201,24,85,16,170,21,235,143,158,146,206,28,10,200,51,171,135,139,27,120,44].to_vec(),
    ///         nonce: [83,114,81,112,67,85,116,114,83,86,49,49,89,75,65,49].to_vec(),
    ///         padding:1
    ///     };
    ///   
    ///     println!("{}", transset.serialize());
    /// }
    /// ```
    pub fn serialize(&self) -> String {
        serde_json::to_string(&self).unwrap()
    }
}

/// Trait that provides the DaaS security functionality 
pub trait PrivacySecurityGuard{
    /// Removes the control NUL characters form the decrypted message
    fn clean_decrypted(&self, message: Vec<u8>) -> Vec<u8> {
        //remove the control NUL characters
        let zero: u8 = 0;
        let mut c: usize = 0;
        let mut message_trimmed: Vec<u8> = Vec::new();

        for chr in message {
            if chr.is_ascii_control() && chr == zero {
                c = c + 1;
            } else {
                message_trimmed.push(chr);
            }
        }

        debug!("There are {} zero control characters.", c);
        message_trimmed
    }

    fn data_from_tranfer(&self, priv_key: Vec<u8>, transfer_set: TransferSet) -> Result<Vec<u8>, Error> {
        // 1. Decrypt the symmetric key
        let decrypted_key = match self.decrypt_symmetric_key(priv_key, transfer_set.encrypted_symmetric_key, Padding::from_raw(transfer_set.padding)) {
            Ok(e_key) => {
                e_key
            },
            Err(_err) => {
                return Err(Error::DecryptionError);
            },
        };

        // 2. Decrypt the data using the symmetric key
        let decrypted_data = match self.decrypt_data(decrypted_key, Some(&transfer_set.nonce), transfer_set.encrypted_data) {
            Ok(msg) => {
                msg                
            },
            Err(_err) => {
                return Err(Error::DecryptionError);
            },
        }; 

        Ok(decrypted_data)
    }

    /// Decrypts the data (small or large) using the symmetric key, IV and AES encryption algorithm
    fn decrypt_data(&self, key: Vec<u8>, nonce: Option<&[u8]>, data_to_decrypt: Vec<u8>) -> Result<Vec<u8>, Error> {
        match decrypt(Cipher::aes_128_cbc(), &key, nonce, &data_to_decrypt) {
            Ok(data) => {
                Ok(data)
            },
            Err(err) => {
                error!("{}", err);
                Err(Error::DecryptionError)
            },
        }
    }

    /// Decrypts the symmetric key using RSA algorithm for the specified padding
    fn decrypt_symmetric_key(&self, priv_key: Vec<u8>, encrypted_key: Vec<u8>, padding: Padding) -> Result<Vec<u8>, Error> {
        let receiver = match Rsa::private_key_from_pem(&priv_key) {
            Ok(rsa) => rsa,
            Err(err) => {
                debug!("{}", err);
                return Err(Error::BadKeyPairError);
            },
        };
        //let sz = std::cmp::max(encrypted_data.len() as usize, priv_key.len() as usize);
        let mut message: Vec<u8> = vec![0; encrypted_key.len()];
        
        match receiver.private_decrypt(&encrypted_key, message.as_mut_slice(), padding){
            Ok(_sz) => {
                Ok(self.clean_decrypted(message))
            },
            Err(err) => {
                debug!("{}", err);
                return Err(Error::DecryptionError);
            },
        }
    }

    /// Encrypts the data (small or large) using the symmetric key, IV and AES encryption algorithm
    fn encrypt_data(&self, key: Vec<u8>, nonce: Option<&[u8]>, data_to_encrypt: Vec<u8>) -> Result<Vec<u8>, Error> {
        match encrypt(Cipher::aes_128_cbc(), &key, nonce, &data_to_encrypt) {
            Ok(cipherdata) => {
                Ok(cipherdata)
            },
            Err(err) => {
                error!("{}", err);
                Err(Error::EncryptionError)
            },
        }
    }

    /// Encrypts the symmetric key using RSA algorithm for the specified padding
    fn encrypt_symmetric_key(&self, pub_key: Vec<u8>, key_to_encrypt: Vec<u8>, padding: Padding) -> Result<Vec<u8>, Error> {
        let sender = match Rsa::public_key_from_pem(&pub_key){
            Ok(rsa) => rsa,
            Err(err) => {
                error!("{}", err);
                return Err(Error::BadKeyPairError);
            },
        };
        let mut encrypted_data: Vec<u8> = vec![0; sender.size() as usize];
        sender.public_encrypt(&key_to_encrypt, encrypted_data.as_mut_slice(), padding).unwrap(); 

        Ok(encrypted_data)
    }

    /// Generates a RSA (private/public) keypair
    fn generate_keypair(&self) -> Result<(Vec<u8>,Vec<u8>,usize), Error>{
        let rsa = Rsa::generate(2048).unwrap();
        let priv_key: Vec<u8> = match rsa.private_key_to_pem() {
            Ok(key) => key,
            Err(_err) => {
                error!("Unable to generate a RSA private key.");
                return Err(Error::BadKeyPairError)
            }
        };
        let pub_key: Vec<u8> = match rsa.public_key_to_pem() {
            Ok(key) => key,
            Err(_err) => {
                error!("Unable to generate a RSA public key.");
                return Err(Error::BadKeyPairError)
            }
        };
    
        Ok((priv_key, pub_key, rsa.size() as usize))
    }
    
    /// Generates a random alphanumeric key with a length of 16 characters
    fn generate_symmetric_key(&self) -> Vec<u8>{
        rand::thread_rng()
            .sample_iter(&Alphanumeric)
            .take(16)
            .collect::<String>()
            .as_bytes()
            .to_vec()
    }

    /// Generates a random alphanumeric nonce (a.k.a. IV) with a length of 16 characters
    fn generate_nonce(&self) -> Vec<u8>{
        rand::thread_rng()
            .sample_iter(&Alphanumeric)
            .take(16)
            .collect::<String>()
            .as_bytes()
            .to_vec()
    }

    fn secure_for_tranfer(&self, pub_key: Vec<u8>, data_to_encrypt: Vec<u8>, padding: Padding) ->  Result<TransferSet, Error> {
        // These are unique attributes for the data being secured which ensures that no 2 data transfers 
        // can be decrypted using the private key without these unique attributes.
        let key = self.generate_symmetric_key();
        let nonce = self.generate_nonce();

        // 1. encrypt the data using the symmetric key
        let secured_data = match self.encrypt_data(key.clone(), Some(&nonce.clone()), data_to_encrypt.clone()) {
            Ok(msg) => {
                msg                
            },
            Err(err) => {
                error!("{:?}", err);
                return Err(err);
            },
        };  

        // 2. Encrypt the symmetric key
        let encrypted_key = match self.encrypt_symmetric_key(pub_key, key.clone(), padding) {
            Ok(e_key) => {
                e_key
            },
            Err(err) => {
                error!("{:?}", err);
                return Err(err);
            },
        };

        // 3. Return the set of attributes that will be needed for a secure data transfer
        Ok(TransferSet {
                encrypted_data: secured_data,
                encrypted_symmetric_key: encrypted_key,
                nonce: nonce,
                padding: padding.as_raw(),
            })
    }
}

/// Implementaitons of the PrivacySecurityGuard
impl PrivacySecurityGuard for PrivacyGuard{}

pub mod error;
pub mod extractor;
pub mod middleware;

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::prelude::*;
    use std::fs::File;

    fn get_priv_pem() -> Vec<u8> {
        let mut f = File::open("./tests/keys/priv-key.pem").unwrap();
        let mut priv_pem = Vec::new();
        f.read_to_end(&mut priv_pem).unwrap();
        
        priv_pem
    }

    fn get_pub_pem() -> Vec<u8> {
        let mut f = File::open("./tests/keys/pub-key.pem").unwrap();
        let mut pub_pem = Vec::new();
        f.read_to_end(&mut pub_pem).unwrap();
        
        pub_pem
    }

    #[test]
    fn test_generate_nonce() {
        let guard = PrivacyGuard {};
        let nonce = guard.generate_nonce();
        println!("{:?}", nonce);
        assert_eq!(nonce.len(),16);        
    }

    #[test]
    fn test_generate_symmetric_key() {
        let guard = PrivacyGuard {};
        let key = guard.generate_symmetric_key();
        println!("{:?}", key);
        assert_eq!(key.len(),16);        
    }

    #[test]
    fn test_generate_keypair() {
        let guard = PrivacyGuard {};
        let keypair = guard.generate_keypair();
        assert!(keypair.is_ok());        
    }

    #[test]
    fn test_decrypt_data() {
        let guard = PrivacyGuard {};
        let key: &[u8] = &[120, 70, 69, 82, 79, 54, 69, 104, 122, 119, 49, 97, 73, 120, 120, 80];
        let nonce: &[u8] = &[116, 85, 83, 118, 121, 112, 103, 50, 99, 101, 54, 105, 67, 54, 51, 88];
        let message_received: &[u8] = &[89, 60, 190, 161, 62, 26, 88, 4, 100, 161, 230, 105, 14, 4, 162, 163];

        match guard.decrypt_data(key.to_vec(), Some(&nonce), message_received.to_vec()) {
            Ok(msg) => {
                assert_eq!("_test123!# ".to_string(), String::from_utf8(msg).unwrap());
            },
            Err(_err) => {
                assert!(false);
            },
        }
    }

    #[test]
    fn test_encrypt_data() {
        let guard = PrivacyGuard {};
        let key = guard.generate_symmetric_key();
        let nonce = guard.generate_nonce();
        let message_sent: Vec<u8> = String::from("_test123!# ").into_bytes();

        match guard.encrypt_data(key, Some(&nonce), message_sent) {
            Ok(_msg) => {
                assert!(true);
            },
            Err(_err) => {
                assert!(false);
            },
        }
    }

    #[test]
    fn test_happy_path_mp3() {
        let priv_key = get_priv_pem();
        let pub_key = get_pub_pem();
        let guard = PrivacyGuard {};
        let key = guard.generate_symmetric_key();
        let nonce = guard.generate_nonce();
        let padding = Padding::PKCS1;
        let mut f = File::open("./tests/example_audio_clip.mp3").unwrap();
        let mut mp3 = Vec::new();
        f.read_to_end(&mut mp3).unwrap();

        // 1. encrypt the mps data using the symmetric key
        let encrypted_data = match guard.encrypt_data(key.clone(), Some(&nonce.clone()), mp3.clone()) {
            Ok(msg) => {
                assert!(true);
                msg                
            },
            Err(err) => {
                assert!(false);
                panic!("{:?}", err);
            },
        };    

        // 2. Encrypt the symmetric key
        let encrypted_key = match guard.encrypt_symmetric_key(pub_key, key.clone(), padding) {
            Ok(e_key) => {
                assert_eq!(e_key.len(), 256);
                e_key
            },
            Err(err) => {
                assert!(false);
                panic!("{:?}", err);
            },
        };

        // The data source sends the following items to the recipient: 
        // + padding
        // + nonce
        // + encrypted symmetric key
        // + encrypted mps data

        // 3. Decrypt the symmetric key
        let decrypted_key = match guard.decrypt_symmetric_key(priv_key, encrypted_key, padding) {
            Ok(e_key) => {
                assert_eq!(e_key.len(), 16);
                e_key
            },
            Err(err) => {
                assert!(false);
                panic!("{:?}", err);
            },
        };

        // 4. Decrypt the data using the symmetric key
        let decrypted_data = match guard.decrypt_data(decrypted_key, Some(&nonce), encrypted_data) {
            Ok(msg) => {
                assert!(true);
                msg                
            },
            Err(err) => {
                assert!(false);
                panic!("{:?}", err);
            },
        }; 

        assert_eq!(mp3, decrypted_data);
    }

    #[test]
    fn test_encrypt_decrypt_mp3() {
        let guard = PrivacyGuard {};
        let key = guard.generate_symmetric_key();
        let nonce = guard.generate_nonce();

        let mut f = File::open("./tests/example_audio_clip.mp3").unwrap();
        let mut mp3 = Vec::new();
        f.read_to_end(&mut mp3).unwrap();

        let encrypted_data = match guard.encrypt_data(key.clone(), Some(&nonce.clone()), mp3.clone()) {
            Ok(msg) => {
                assert!(true);
                msg                
            },
            Err(err) => {
                assert!(false);
                panic!("{:?}", err);
            },
        };        

        let decrypted_data = match guard.decrypt_data(key, Some(&nonce), encrypted_data) {
            Ok(msg) => {
                assert!(true);
                msg                
            },
            Err(err) => {
                assert!(false);
                panic!("{:?}", err);
            },
        }; 

        assert_eq!(mp3, decrypted_data);
    }  
    
    #[test]
    fn test_encrypt_decrypt_symmetric_key() {
        let guard = PrivacyGuard {};
        let keypair = guard.generate_keypair().unwrap();
        let priv_key = keypair.0;
        let pub_key = keypair.1;
        let padding = Padding::PKCS1;
        let key = guard.generate_symmetric_key();

        let encrypted_key = match guard.encrypt_symmetric_key(pub_key, key.clone(), padding) {
            Ok(e_key) => {
                assert_eq!(e_key.len(), 256);
                e_key
            },
            Err(err) => {
                assert!(false);
                panic!("{:?}", err);
            },
        };

        let decrypted_key = match guard.decrypt_symmetric_key(priv_key, encrypted_key, padding) {
            Ok(e_key) => {
                assert_eq!(e_key.len(), 16);
                e_key
            },
            Err(err) => {
                assert!(false);
                panic!("{:?}", err);
            },
        };

        assert_eq!(key, decrypted_key);
    }

    #[test]
    fn test_secure_for_tranfer() {
        let guard = PrivacyGuard {};
        let padding = Padding::PKCS1;
        let pub_key = get_pub_pem();
        let message: Vec<u8> = String::from("_test123!# ").into_bytes();
        let trans = match guard.secure_for_tranfer(pub_key, message.clone(), padding.clone()) {
            Ok(ts) => ts,
            Err(_err) => {
                assert!(false);
                panic!("Cannot secure data for transfer!")
            }
        };

        assert_ne!(trans.encrypted_data, message);
        assert_eq!(trans.encrypted_symmetric_key.len(), 256);
        assert_eq!(trans.nonce.len(), 16);
        assert_eq!(Padding::from_raw(trans.padding), padding);
    }

    #[test]
    fn test_data_from_tranfer() {
        let guard = PrivacyGuard {};
        let priv_key = get_priv_pem();
        let message: Vec<u8> = String::from("_test123!# ").into_bytes();
        let transset = TransferSet {
            encrypted_data: [82,240,199,226,197,63,161,115,68,5,177,72,246,109,171,165].to_vec(),
            encrypted_symmetric_key: [83,205,166,96,120,119,1,178,36,144,152,51,106,17,220,9,165,240,236,25,228,164,97,192,194,9,117,249,52,77,14,194,181,37,19,202,104,89,50,2,223,181,173,6,226,32,85,148,103,96,186,188,217,169,112,109,73,184,39,196,95,161,18,180,239,74,0,112,175,26,116,21,31,88,125,157,54,39,147,242,28,202,179,132,157,40,163,159,194,74,9,241,108,16,40,81,67,165,57,46,146,195,37,89,173,124,167,103,30,148,7,4,75,19,73,71,132,142,45,229,150,188,96,56,150,106,125,12,56,251,8,89,51,5,195,235,234,91,169,36,32,134,183,127,231,159,61,55,221,98,71,217,228,49,52,12,47,186,14,86,143,247,54,228,184,75,78,3,160,96,214,118,182,133,61,209,129,68,231,121,178,111,217,99,238,213,101,29,83,11,223,243,239,166,67,180,78,60,1,0,177,74,65,8,5,222,168,170,230,92,193,31,45,14,111,96,7,232,6,6,26,44,192,197,71,115,204,134,191,0,147,128,244,198,189,201,24,85,16,170,21,235,143,158,146,206,28,10,200,51,171,135,139,27,120,44].to_vec(),
            nonce: [83,114,81,112,67,85,116,114,83,86,49,49,89,75,65,49].to_vec(),
            padding:1
        };

        let data = match guard.data_from_tranfer(priv_key, transset) {
            Ok(msg) => msg,
            Err(_err) => {
                assert!(false);
                panic!("Cannot retrieve data from transfer set!")
            }
        };

        assert_eq!(message, data);
    }

    #[test]
    fn test_data_from_tranfer_bad_key() {
        let guard = PrivacyGuard {};
        let priv_key = guard.generate_keypair().unwrap().0;
        let message: Vec<u8> = String::from("_test123!# ").into_bytes();
        let transset = TransferSet {
            encrypted_data: [82,240,199,226,197,63,161,115,68,5,177,72,246,109,171,165].to_vec(),
            encrypted_symmetric_key: [83,205,166,96,120,119,1,178,36,144,152,51,106,17,220,9,165,240,236,25,228,164,97,192,194,9,117,249,52,77,14,194,181,37,19,202,104,89,50,2,223,181,173,6,226,32,85,148,103,96,186,188,217,169,112,109,73,184,39,196,95,161,18,180,239,74,0,112,175,26,116,21,31,88,125,157,54,39,147,242,28,202,179,132,157,40,163,159,194,74,9,241,108,16,40,81,67,165,57,46,146,195,37,89,173,124,167,103,30,148,7,4,75,19,73,71,132,142,45,229,150,188,96,56,150,106,125,12,56,251,8,89,51,5,195,235,234,91,169,36,32,134,183,127,231,159,61,55,221,98,71,217,228,49,52,12,47,186,14,86,143,247,54,228,184,75,78,3,160,96,214,118,182,133,61,209,129,68,231,121,178,111,217,99,238,213,101,29,83,11,223,243,239,166,67,180,78,60,1,0,177,74,65,8,5,222,168,170,230,92,193,31,45,14,111,96,7,232,6,6,26,44,192,197,71,115,204,134,191,0,147,128,244,198,189,201,24,85,16,170,21,235,143,158,146,206,28,10,200,51,171,135,139,27,120,44].to_vec(),
            nonce: [83,114,81,112,67,85,116,114,83,86,49,49,89,75,65,49].to_vec(),
            padding:1
        };

        match guard.data_from_tranfer(priv_key, transset) {
            Ok(msg) => {
                assert_ne!(message, msg);
            },
            Err(_err) => {
                assert!(true);
            }
        };
    }

    #[test]
    fn test_transferset_from_serialize() {
        let transset = TransferSet {
            encrypted_data: [82,240,199,226,197,63,161,115,68,5,177,72,246,109,171,165].to_vec(),
            encrypted_symmetric_key: [83,205,166,96,120,119,1,178,36,144,152,51,106,17,220,9,165,240,236,25,228,164,97,192,194,9,117,249,52,77,14,194,181,37,19,202,104,89,50,2,223,181,173,6,226,32,85,148,103,96,186,188,217,169,112,109,73,184,39,196,95,161,18,180,239,74,0,112,175,26,116,21,31,88,125,157,54,39,147,242,28,202,179,132,157,40,163,159,194,74,9,241,108,16,40,81,67,165,57,46,146,195,37,89,173,124,167,103,30,148,7,4,75,19,73,71,132,142,45,229,150,188,96,56,150,106,125,12,56,251,8,89,51,5,195,235,234,91,169,36,32,134,183,127,231,159,61,55,221,98,71,217,228,49,52,12,47,186,14,86,143,247,54,228,184,75,78,3,160,96,214,118,182,133,61,209,129,68,231,121,178,111,217,99,238,213,101,29,83,11,223,243,239,166,67,180,78,60,1,0,177,74,65,8,5,222,168,170,230,92,193,31,45,14,111,96,7,232,6,6,26,44,192,197,71,115,204,134,191,0,147,128,244,198,189,201,24,85,16,170,21,235,143,158,146,206,28,10,200,51,171,135,139,27,120,44].to_vec(),
            nonce: [83,114,81,112,67,85,116,114,83,86,49,49,89,75,65,49].to_vec(),
            padding:1
        };
        let serialized = r#"{
            "encrypted_data":[82,240,199,226,197,63,161,115,68,5,177,72,246,109,171,165],
            "encrypted_symmetric_key":[83,205,166,96,120,119,1,178,36,144,152,51,106,17,220,9,165,240,236,25,228,164,97,192,194,
                                       9,117,249,52,77,14,194,181,37,19,202,104,89,50,2,223,181,173,6,226,32,85,148,103,96,186,
                                       188,217,169,112,109,73,184,39,196,95,161,18,180,239,74,0,112,175,26,116,21,31,88,125,157,
                                       54,39,147,242,28,202,179,132,157,40,163,159,194,74,9,241,108,16,40,81,67,165,57,46,146,195,
                                       37,89,173,124,167,103,30,148,7,4,75,19,73,71,132,142,45,229,150,188,96,56,150,106,125,12,56,
                                       251,8,89,51,5,195,235,234,91,169,36,32,134,183,127,231,159,61,55,221,98,71,217,228,49,52,12,
                                       47,186,14,86,143,247,54,228,184,75,78,3,160,96,214,118,182,133,61,209,129,68,231,121,178,111,
                                       217,99,238,213,101,29,83,11,223,243,239,166,67,180,78,60,1,0,177,74,65,8,5,222,168,170,230,92,
                                       193,31,45,14,111,96,7,232,6,6,26,44,192,197,71,115,204,134,191,0,147,128,244,198,189,201,24,85,
                                       16,170,21,235,143,158,146,206,28,10,200,51,171,135,139,27,120,44],
            "nonce":[83,114,81,112,67,85,116,114,83,86,49,49,89,75,65,49],
            "padding":1
            }"#;
        let from_transset = match TransferSet::from_serialized(serialized) {
            Ok(ts) => ts,
            Err(err) => {
                assert!(false);
                panic!("{}", err);
            },
        };

        assert_eq!(transset.encrypted_data, from_transset.encrypted_data);
        assert_eq!(transset.encrypted_symmetric_key, from_transset.encrypted_symmetric_key);
        assert_eq!(transset.nonce, from_transset.nonce);
        assert_eq!(transset.padding, from_transset.padding);
    }

    #[test]
    fn test_transferset_serialize() {
        let serialized = "{\"encrypted_data\":[82,240,199,226,197,63,161,115,68,5,177,72,246,109,171,165],\"encrypted_symmetric_key\":[83,205,166,96,120,119,1,178,36,144,152,51,106,17,220,9,165,240,236,25,228,164,97,192,194,9,117,249,52,77,14,194,181,37,19,202,104,89,50,2,223,181,173,6,226,32,85,148,103,96,186,188,217,169,112,109,73,184,39,196,95,161,18,180,239,74,0,112,175,26,116,21,31,88,125,157,54,39,147,242,28,202,179,132,157,40,163,159,194,74,9,241,108,16,40,81,67,165,57,46,146,195,37,89,173,124,167,103,30,148,7,4,75,19,73,71,132,142,45,229,150,188,96,56,150,106,125,12,56,251,8,89,51,5,195,235,234,91,169,36,32,134,183,127,231,159,61,55,221,98,71,217,228,49,52,12,47,186,14,86,143,247,54,228,184,75,78,3,160,96,214,118,182,133,61,209,129,68,231,121,178,111,217,99,238,213,101,29,83,11,223,243,239,166,67,180,78,60,1,0,177,74,65,8,5,222,168,170,230,92,193,31,45,14,111,96,7,232,6,6,26,44,192,197,71,115,204,134,191,0,147,128,244,198,189,201,24,85,16,170,21,235,143,158,146,206,28,10,200,51,171,135,139,27,120,44],\"nonce\":[83,114,81,112,67,85,116,114,83,86,49,49,89,75,65,49],\"padding\":1}";
        let transset = TransferSet {
            encrypted_data: [82,240,199,226,197,63,161,115,68,5,177,72,246,109,171,165].to_vec(),
            encrypted_symmetric_key: [83,205,166,96,120,119,1,178,36,144,152,51,106,17,220,9,165,240,236,25,228,164,97,192,194,9,117,249,52,77,14,194,181,37,19,202,104,89,50,2,223,181,173,6,226,32,85,148,103,96,186,188,217,169,112,109,73,184,39,196,95,161,18,180,239,74,0,112,175,26,116,21,31,88,125,157,54,39,147,242,28,202,179,132,157,40,163,159,194,74,9,241,108,16,40,81,67,165,57,46,146,195,37,89,173,124,167,103,30,148,7,4,75,19,73,71,132,142,45,229,150,188,96,56,150,106,125,12,56,251,8,89,51,5,195,235,234,91,169,36,32,134,183,127,231,159,61,55,221,98,71,217,228,49,52,12,47,186,14,86,143,247,54,228,184,75,78,3,160,96,214,118,182,133,61,209,129,68,231,121,178,111,217,99,238,213,101,29,83,11,223,243,239,166,67,180,78,60,1,0,177,74,65,8,5,222,168,170,230,92,193,31,45,14,111,96,7,232,6,6,26,44,192,197,71,115,204,134,191,0,147,128,244,198,189,201,24,85,16,170,21,235,143,158,146,206,28,10,200,51,171,135,139,27,120,44].to_vec(),
            nonce: [83,114,81,112,67,85,116,114,83,86,49,49,89,75,65,49].to_vec(),
            padding:1
        };

        assert_eq!(serialized, transset.serialize());
    }
}