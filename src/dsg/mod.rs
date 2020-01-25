
use crate::dsg::error::*;
use rand::Rng; 
use rand::distributions::Alphanumeric;
use openssl::rsa::{Rsa, Padding};
use openssl::symm::{decrypt, encrypt, Cipher};

/// The HTTP header that holds the RSA encrypted sytemmetirc key
pub static DSG_SYTMMETRIC_KEY_HEADER: &str = "Data-Security-Guard-Key";

/// Represents the Security Gaurd
pub struct PrivacyGuard {}

/// Represents the set of attributes your will need to transfer the data safely
pub struct TransferSet {
    pub encrypted_data: Vec<u8>,
    pub encrypted_symmetric_key: Vec<u8>,
    pub nonce: Option<Vec<u8>>,
    pub padding: i32,
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
                panic!("{:?}", err);
            },
        };  

        // 2. Encrypt the symmetric key
        let encrypted_key = match self.encrypt_symmetric_key(pub_key, key.clone(), padding) {
            Ok(e_key) => {
                e_key
            },
            Err(err) => {
                error!("{:?}", err);
                panic!("{:?}", err);
            },
        };

        // 3. Return the set of attributes that will be needed for a secure data transfer
        Ok(TransferSet {
                encrypted_data: secured_data,
                encrypted_symmetric_key: encrypted_key,
                nonce: Some(nonce),
                padding: padding.as_raw(),
            })
    }
}

/// Implementaitons of the PrivacySecurityGuard
impl PrivacySecurityGuard for PrivacyGuard{}

pub mod error;
//pub mod extractor;
//pub mod middleware;

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
        assert_eq!(trans.nonce.unwrap().len(), 16);
        assert_eq!(Padding::from_raw(trans.padding), padding);
    }
}