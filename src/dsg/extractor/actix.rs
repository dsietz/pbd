//! The DSG Extractor is a simple way to pull the TransferSet from the HTTP Request. 
//! 
//! ---
//! 
//! Example 
//! ```
//! extern crate pbd;
//! extern crate actix_web;
//! 
//! use pbd::dua::{DSG_NONCE_HEADER, DSG_PADDING_HEADER, DSG_SYTMMETRIC_KEY_HEADER};
//! use pbd::dua::extractor::actix::*;
//! use actix_web::{web, http, test, App, HttpRequest, HttpResponse};
//! use actix_web::http::{StatusCode};
//! use actix_web::dev::Service;
//!
//! fn index_extract_dua(transferset: TransferSet, _req: HttpRequest) -> HttpResponse {
//!     println!("{}", transferset.serialize());
//!         
//!     HttpResponse::Ok()
//!         .header(http::header::CONTENT_TYPE, "application/json")
//!         .body(format!("{}", duas))
//! }
//! 
//! fn main () {
//!     let mut app = test::init_service(App::new().route("/", web::get().to(index_extract_dua)));
//!     let req = test::TestRequest::get().uri("/")
//!         .header("content-type", "application/json")
//!         .header(DSG_NONCE_HEADER, 1)
//!         .header(DSG_PADDING_HEADER, 1)
//!         .header(DSG_SYTMMETRIC_KEY_HEADER, 1)
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
use json::JsonValue;
use actix_web::http::header::HeaderValue;

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
    fn from_request(req: &HttpRequest) -> Result<TransferSet, crate::dsg::error::Error>;
}

impl TransferSetRequest for TransferSet {
    // Constructor
    fn from_request(req: &HttpRequest) -> Result<TransferSet, crate::dsg::error::Error> {
        let nonce = match req.headers().get(DSG_NONCE_HEADER) {
            Some(val) => {
                Some(val.as_bytes())
            },
            None => {
                error!("{}", Error::MissingNonceError);
                return Err(Error::MissingNonceError);
            },
        };
    }
}

impl FromRequest for DUAs {
    type Config = ();
    type Future = Result<Self, Self::Error>;
    type Error = LocalError;
    // convert request to future self
    fn from_request(req: &HttpRequest, _payload: &mut actix_web::dev::Payload) -> Self::Future {
        Ok(DUAs::from_request(req))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::{test, web, http, App, HttpRequest, HttpResponse};
    use actix_web::dev::Service;
    use actix_web::http::{StatusCode};

    // supporting functions
    fn index_extract_dua(duas: DUAs, _req: HttpRequest) -> HttpResponse {
        if duas.vec().len() > 0 {
            return HttpResponse::Ok()
                .header(http::header::CONTENT_TYPE, "application/json")
                .body(format!("{}", duas))
        } else {
            return HttpResponse::BadRequest()
                .header(http::header::CONTENT_TYPE, "application/json")
                .body(format!("{}", LocalError::BadDUA))
        }
    }

    // tests
    #[test]
    fn test_http_header_name() {
        assert_eq!(DUA_HEADER, "Data-Usage-Agreement");
    }

    #[test]
    fn test_dua_extractor_good() {
        let mut app = test::init_service(App::new().route("/", web::get().to(index_extract_dua)));
        let req = test::TestRequest::get().uri("/")
            .header("content-type", "application/json")
            .header(DUA_HEADER, r#"[{"agreement_name":"billing","location":"www.dua.org/billing.pdf","agreed_dtm": 1553988607}]"#)
            .to_request();
        let resp = test::block_on(app.call(req)).unwrap();
        assert_eq!(resp.status(), StatusCode::OK);
    }
    #[test]
    fn test_dua_extractor_missing() {
        let mut app = test::init_service(App::new().route("/", web::get().to(index_extract_dua)));
        let req = test::TestRequest::get().uri("/")
            .header("content-type", "application/json")
            .to_request();
        let resp = test::call_service(&mut app, req);
        assert_eq!(resp.status(), StatusCode::BAD_REQUEST);
        // read response
        let bdy = test::read_body(resp);
        assert_eq!(&bdy[..], actix_web::web::Bytes::from_static(b"Malformed or missing one or more Data Usage Agreements"));
    }
}