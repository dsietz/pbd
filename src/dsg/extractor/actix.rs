//! The DSG Extractor is a simple way to pull the TransferSet from the HTTP Request. 
//! 
//! ---
//! 
//! Example 
//! ```
//! extern crate pbd;
//! extern crate actix_web;
//! 
//! use actix_web::{test, web, http, App, HttpRequest, HttpResponse};
//! use pbd::dsg::{PrivacyGuard,PrivacySecurityGuard, TransferSet};
//! use openssl::rsa::{Rsa, Padding};
//! use std::io::prelude::*;
//! use std::fs::File;
//!
//! fn get_priv_pem() -> Vec<u8> {
//!     let mut f = File::open("./tests/keys/priv-key.pem").unwrap();
//!     let mut priv_pem = Vec::new();
//!     f.read_to_end(&mut priv_pem).unwrap();
//!     
//!     priv_pem
//! }
//! 
//! fn get_pub_pem() -> Vec<u8> {
//!     let mut f = File::open("./tests/keys/pub-key.pem").unwrap();
//!     let mut pub_pem = Vec::new();
//!     f.read_to_end(&mut pub_pem).unwrap();
//!     
//!     pub_pem
//! }
//!
//! // On the server side, the TransferSet is extracted from the payload.  
//! // In this example., the server sends the message back for us to verify in our assertion. 
//! fn index_extract_transferset(transset: TransferSet, _req: HttpRequest) -> HttpResponse {
//!     let guard = PrivacyGuard {};
//!     let priv_key = get_priv_pem();
//! 
//!     // extract the data form the TransferSet
//!     match guard.data_from_tranfer(priv_key, transset) {
//!         Ok(msg) => {
//!             // send the message back to the client for our example
//!             return HttpResponse::Ok()
//!                 .header(http::header::CONTENT_TYPE, "plain/text")
//!                 .body(String::from_utf8(msg).unwrap())
//!         },
//!         Err(err) => {
//!             println!("{}", err);
//!             return HttpResponse::BadRequest()
//!                 .header(http::header::CONTENT_TYPE, "plain/text")
//!                 .body(format!("{}", err))
//!         }
//!     }
//! }
//! 
//! fn main () {
//!    // The data soruce prepares the data for transport
//!    let guard = PrivacyGuard {};
//!    let padding = Padding::PKCS1;
//!    let pub_key = get_pub_pem();
//!    let message: Vec<u8> = String::from("_test123!# ").into_bytes();
//!    let trans = guard.secure_for_tranfer(pub_key, message, padding).unwrap();
//!
//!    // The data source sends the TransferSet in the payload
//!    let mut app = test::init_service(App::new().route("/", web::get().to(index_extract_transferset)));      
//!    let req = test::TestRequest::get().uri("/")
//!        .header("content-type", "plain/text")
//!        .set_payload(trans.serialize())
//!        .to_request();    
//!
//!    // In this example, the server extracted our message and sent it back for us to verify in our assert
//!    let msg = test::read_response(&mut app, req);
//!    assert_eq!(String::from("_test123!# "), msg);
//! }
//! ```

use super::*;
use std::fmt;
use actix_web::{FromRequest, HttpRequest};
use futures::{Stream};
use futures::prelude::Async;

// 
// The TransferSet Extractor
// 

pub type LocalError = super::error::Error;

impl fmt::Display for TransferSet {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", serde_json::to_string(&self).unwrap())
    }
}

pub trait TransferSetRequest {
    fn serialized_transset_from_payload(payload: &mut actix_web::dev::Payload) -> String {
        match payload.poll() {
            Ok(Async::Ready(t)) => {
                match t {
                    Some(b) => {
                        match String::from_utf8(b.to_vec()) {
                            Ok(serialized) => serialized,
                            Err(_err) => {
                                debug!("{}", crate::dsg::error::Error::PayloadUnreadableError);
                                panic!("{}", crate::dsg::error::Error::PayloadUnreadableError);
                            },
                        }
                    },
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
        }
    }    
    fn transferset_from_request(req: &HttpRequest, payload: &mut actix_web::dev::Payload) -> TransferSet;
}

impl TransferSetRequest for TransferSet {
    // Constructor
    fn transferset_from_request(req: &HttpRequest, payload: &mut actix_web::dev::Payload) -> TransferSet {
        match TransferSet::from_serialized(&Self::serialized_transset_from_payload(payload)) {
            Ok(ts) => {
                return ts;
            },
            Err(err) => {
                error!("{}",err);
                panic!("{}",err);
            },
        }  
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

    // supporting functions
    fn index_extract_transferset(transset: TransferSet, _req: HttpRequest) -> HttpResponse {
        let guard = PrivacyGuard {};
        let priv_key = get_priv_pem();

        match guard.data_from_tranfer(priv_key, transset) {
            Ok(msg) => {
                return HttpResponse::Ok()
                    .header(http::header::CONTENT_TYPE, "plain/text")
                    .body(String::from_utf8(msg).unwrap())
            },
            Err(err) => {
                println!("{}", err);
                return HttpResponse::BadRequest()
                    .header(http::header::CONTENT_TYPE, "plain/text")
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

        let mut app = test::init_service(App::new().route("/", web::get().to(index_extract_transferset)));      
        let req = test::TestRequest::get().uri("/")
            .header("content-type", "plain/text")
            .set_payload(trans.serialize())
            .to_request();    
        let msg = test::read_response(&mut app, req);
        
        assert_eq!(message, msg);
    }
}