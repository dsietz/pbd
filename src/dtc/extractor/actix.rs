//! The DUA Extractor is a simple way to pull the list of DUAs from the HTTP Request. 
//! 
//! ---
//! 
//! Example 
//! ```
//! extern crate pbd;
//! extern crate actix_web;
//! 
//! use pbd::dua::DUA_HEADER;
//! use pbd::dua::extractor::actix::*;
//! use actix_web::{web, http, test, App, HttpRequest, HttpResponse};
//! use actix_web::http::{StatusCode};
//! use actix_web::dev::Service;
//!
//! fn index_extract_dua(duas: DUAs, _req: HttpRequest) -> HttpResponse {
//!     for dua in duas.vec().iter() {
//!         println!("{:?}", dua);
//!     }
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
//!         .header(DUA_HEADER, r#"[{"agreement_name":"billing","location":"www.dua.org/billing.pdf","agreed_dtm": 1553988607},{"agreement_name":"shipping","location":"www.dua.org/shipping.pdf","agreed_dtm": 1553988607}]"#)
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
// The Data Tracker Chain Extractor
// 
pub type LocalError = super::error::Error;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DTCExtraction{
    tracker: Tracker,
}

impl fmt::Display for DTCExtraction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", serde_json::to_string(&self).unwrap())
    }
}

impl DTCExtraction {
    pub fn tracker_from_header_value(header_value: &HeaderValue) -> Result<Tracker, crate::dtc::error::Error>{
        Ok(Tracker::new("dtc-placeholder".to_string()))
    }

    // Constructor
    pub fn from_request(req: &HttpRequest) -> Option<Tracker>{
        match req.headers().get(DTC_HEADER) {
            Some(u) => {
                return Some(DTCExtraction::tracker_from_header_value(u))
            },
            None => {
                // couldn't find the header, so return empty list of DUAs
                warn!("{}", LocalError::MissingDTC);
                return None
            },
        };
    }
}

impl FromRequest for DTCExtraction {
    type Config = ();
    type Future = Result<Self, Self::Error>;
    type Error = LocalError;
    // convert request to future self
    fn from_request(req: &HttpRequest, _payload: &mut actix_web::dev::Payload) -> Self::Future {
        match DTCExtraction::from_request(req) {
            Some(dtc) => Ok(dtc),
            None => Err(MissingDTC)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::{test, web, http, App, HttpRequest, HttpResponse};
    use actix_web::dev::Service;
    use actix_web::http::{StatusCode};

    // supporting functions
    fn index_extract_dtc(tracker: DTCExtraction, _req: HttpRequest) -> HttpResponse {
        if tracker.is_some() {
            return HttpResponse::Ok()
                .header(http::header::CONTENT_TYPE, "application/json")
                .body(format!("{;?}", tracker))
        } else {
            return HttpResponse::BadRequest()
                .header(http::header::CONTENT_TYPE, "application/json")
                .body(format!("{}", LocalError::BadDTC))
        }
    }

    // tests
    #[test]
    fn test_http_header_name() {
        assert_eq!(DTC_HEADER, "Data-Tracker-Chain");
    }

    #[test]
    fn test_dua_extractor_good() {
        let mut app = test::init_service(App::new().route("/", web::get().to(index_extract_dua)));
        let req = test::TestRequest::get().uri("/")
            .header("content-type", "application/json")
            .header(DTC_HEADER, r#"12345"#)
            .to_request();
        let resp = test::block_on(app.call(req)).unwrap();
        assert_eq!(resp.status(), StatusCode::OK);
    }
    #[test]
    fn test_dtc_extractor_missing() {
        let mut app = test::init_service(App::new().route("/", web::get().to(index_extract_dua)));
        let req = test::TestRequest::get().uri("/")
            .header("content-type", "application/json")
            .to_request();
        let resp = test::call_service(&mut app, req);
        assert_eq!(resp.status(), StatusCode::BAD_REQUEST);
        // read response
        let bdy = test::read_body(resp);
        assert_eq!(&bdy[..], actix_web::web::Bytes::from_static(b"Corrupt or invalid Data Tracker Chain"));
    }
}