//! The DSG Extractor is a simple way to pull the TransferSet from the HTTP Request. 
//! 
//! ---
//! 
//! Example 
//! ```
//! extern crate pbd;
//! extern crate actix_web;
//! 
//! use pbd::dsg::TransferSet;
//! use pbd::dsg::{DSG_NONCE_HEADER, DSG_PADDING_HEADER, DSG_SYTMMETRIC_KEY_HEADER};
//! use pbd::dsg::extractor::actix::*;
//! use actix_web::{web, http, test, App, HttpRequest, HttpResponse};
//! use actix_web::http::{StatusCode};
//! use actix_web::dev::Service;
//!
//! fn index_extract_transferset(transferset: TransferSet, _req: HttpRequest) -> HttpResponse {
//!     HttpResponse::Ok()
//!         .header(http::header::CONTENT_TYPE, "application/json")
//!         .body(format!("{}", transferset.serialize()))
//! }
//! 
//! fn main () {
//!     let encrypted_symmetric_key = "[83,205,166,96,120,119,1,178,36,144,152,51,106,17,220,9,165,240,236,25,228,164,97,192,194,9,117,249,52,77,14,194,181,37,19,202,104,89,50,2,223,181,173,6,226,32,85,148,103,96,186,188,217,169,112,109,73,184,39,196,95,161,18,180,239,74,0,112,175,26,116,21,31,88,125,157,54,39,147,242,28,202,179,132,157,40,163,159,194,74,9,241,108,16,40,81,67,165,57,46,146,195,37,89,173,124,167,103,30,148,7,4,75,19,73,71,132,142,45,229,150,188,96,56,150,106,125,12,56,251,8,89,51,5,195,235,234,91,169,36,32,134,183,127,231,159,61,55,221,98,71,217,228,49,52,12,47,186,14,86,143,247,54,228,184,75,78,3,160,96,214,118,182,133,61,209,129,68,231,121,178,111,217,99,238,213,101,29,83,11,223,243,239,166,67,180,78,60,1,0,177,74,65,8,5,222,168,170,230,92,193,31,45,14,111,96,7,232,6,6,26,44,192,197,71,115,204,134,191,0,147,128,244,198,189,201,24,85,16,170,21,235,143,158,146,206,28,10,200,51,171,135,139,27,120,44]";
//!     let mut app = test::init_service(App::new().route("/", web::get().to(index_extract_transferset)));
//!     let req = test::TestRequest::get().uri("/")
//!         .header("content-type", "application/json")
//!         .header(DSG_NONCE_HEADER, "[83,114,81,112,67,85,116,114,83,86,49,49,89,75,65,49]")
//!         .header(DSG_PADDING_HEADER, "1")
//!         .header(DSG_SYTMMETRIC_KEY_HEADER, encrypted_symmetric_key)
//!         .set_payload(String::from("my private data").as_bytes())
//!         .to_request();
//!     let resp = test::block_on(app.call(req)).unwrap();
//!     
//!     assert_eq!(resp.status(), StatusCode::OK);
//! }
//! ```

use super::*;
use std::fmt;
use actix_web::{FromRequest, HttpRequest};
use futures::{Stream};
use futures::prelude::Async;
use std::str::FromStr;

// 
// The TransfereSet Extractor
// 

pub type LocalError = super::error::Error;

impl fmt::Display for TransferSet {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", serde_json::to_string(&self).unwrap())
    }
}

pub trait TransferSetRequest {
    fn transferset_from_request(req: &HttpRequest, payload: &mut actix_web::dev::Payload) -> TransferSet;
}

//const MAX_SIZE: usize = 262_144; // max payload size is 256k

impl TransferSetRequest for TransferSet {
    // Constructor
    fn transferset_from_request(req: &HttpRequest, payload: &mut actix_web::dev::Payload) -> TransferSet {
        let serialized_transset = match payload.poll() {
            Ok(Async::Ready(t)) => {
                match t {
                    Some(b) => b.to_vec(),
                    None => {
                        debug!("{}", crate::dsg::error::Error::PayloadUnreadableError);
                        panic!("{}", crate::dsg::error::Error::PayloadUnreadableError);
                    },
                }
            },
            Ok(Async::NotReady) => {
                debug!("{}", crate::dsg::error::Error::PayloadTimeoutError);
                panic!("{}", crate::dsg::error::Error::PayloadTimeoutError);
            },
            Err(_e) => {
                debug!("{}", crate::dsg::error::Error::PayloadUnreadableError);
                panic!("{}", crate:: dsg::error::Error::PayloadUnreadableError);
            },
        };

        match TransfereSet::from_serialized(serialized_transset) {
            Ok(ts) => {
                return ts;
            },
            Err(err) => {
                error!("{}",err);
                return Err(err);
            },
        }
/*
        let encrypted_symmetric_key = match req.headers().get(DSG_SYTMMETRIC_KEY_HEADER) {
            Some(val) => {
                val.as_bytes()
            },
            None => {
                error!("{}", super::error::Error::MissingSymmetricKeyError);
                panic!("{}", super::error::Error::MissingSymmetricKeyError);
            },
        };
        
        let nonce = match req.headers().get(DSG_NONCE_HEADER) {
            Some(val) => {
                val.as_bytes()
            },
            None => {
                error!("{}", super::error::Error::MissingNonceError);
                panic!("{}", super::error::Error::MissingNonceError);
            },
        };

        let padding: i32 = match req.headers().get(DSG_PADDING_HEADER) {
            Some(val) => {
                FromStr::from_str(val.to_str().unwrap()).unwrap()
            },
            None => {
                error!("{}", super::error::Error::MissingNonceError);
                panic!("{}", super::error::Error::MissingNonceError);
            },
        };

        // temporary return
        TransferSet {
            //encrypted_data: String::from("my private data").as_bytes().to_vec(),
            encrypted_data: encrypted_data,
            encrypted_symmetric_key: encrypted_symmetric_key.to_vec(),
            nonce: nonce.to_vec(),
            padding: padding
        }
*/

        
    }
}

impl FromRequest for TransferSet {
    type Config = ();
    type Future = Result<Self, Self::Error>;
    type Error = LocalError;
    // convert request to future self
    fn from_request(req: &HttpRequest, payload: &mut actix_web::dev::Payload) -> Self::Future {
        Ok(TransferSet::transferset_from_request(req, payload))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::{test, web, http, App, HttpRequest, HttpResponse};
    use actix_web::dev::Service;
    use actix_web::http::{StatusCode};
    use std::io::prelude::*;
    use std::fs::File;
    use std::convert::TryInto;

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

    // supporting functions
    fn index_extract_transferset(transset: TransferSet, _req: HttpRequest) -> HttpResponse {
        let guard = PrivacyGuard {};
        let priv_key = get_priv_pem();

        match guard.data_from_tranfer(priv_key, transset) {
            Ok(_msg) => {
                return HttpResponse::Ok()
                    .header(http::header::CONTENT_TYPE, "plain/tesxt")
                    .body(r#"ok"#)
            },
            Err(err) => {
                println!("{}", err);
                return HttpResponse::BadRequest()
                    .header(http::header::CONTENT_TYPE, "plain/tesxt")
                    .body(format!("{}", err))
            }
        }
    }

    // tests
    #[test]
    fn test_transferset_extractor_good() {
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

        //let encrypted_symmetric_key = "[83,205,166,96,120,119,1,178,36,144,152,51,106,17,220,9,165,240,236,25,228,164,97,192,194,9,117,249,52,77,14,194,181,37,19,202,104,89,50,2,223,181,173,6,226,32,85,148,103,96,186,188,217,169,112,109,73,184,39,196,95,161,18,180,239,74,0,112,175,26,116,21,31,88,125,157,54,39,147,242,28,202,179,132,157,40,163,159,194,74,9,241,108,16,40,81,67,165,57,46,146,195,37,89,173,124,167,103,30,148,7,4,75,19,73,71,132,142,45,229,150,188,96,56,150,106,125,12,56,251,8,89,51,5,195,235,234,91,169,36,32,134,183,127,231,159,61,55,221,98,71,217,228,49,52,12,47,186,14,86,143,247,54,228,184,75,78,3,160,96,214,118,182,133,61,209,129,68,231,121,178,111,217,99,238,213,101,29,83,11,223,243,239,166,67,180,78,60,1,0,177,74,65,8,5,222,168,170,230,92,193,31,45,14,111,96,7,232,6,6,26,44,192,197,71,115,204,134,191,0,147,128,244,198,189,201,24,85,16,170,21,235,143,158,146,206,28,10,200,51,171,135,139,27,120,44]";
        let mut app = test::init_service(App::new().route("/", web::get().to(index_extract_transferset)));      
 
        let req = test::TestRequest::get().uri("/")
            .header("content-type", "plain/text")
            /*
            .header::<&str, Vec<u8>>(DSG_NONCE_HEADER, HeaderValue::from_bytes(&trans.nonce).set_sensitive(true))
            .header::<&str, usize>(DSG_PADDING_HEADER, trans.padding.try_into().unwrap())
            .header::<&str, Vec<u8>>(DSG_SYTMMETRIC_KEY_HEADER, trans.encrypted_symmetric_key)
            .set_payload(trans.encrypted_data)
            */
            .set_payload(trans.serialize())
            .to_request();
        let resp = test::block_on(app.call(req)).unwrap();

        //assert!(false);
        assert_eq!(resp.status(), StatusCode::OK);

    }
}