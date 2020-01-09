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

impl fmt::Display for Tracker {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", serde_json::to_string(&self).unwrap())
    }
}

pub trait TrackerHeader {
    fn tracker_from_header_value(header_value: &HeaderValue) -> Result<Tracker, crate::dtc::error::Error>;
}

impl TrackerHeader for Tracker {
    fn tracker_from_header_value(header_value: &HeaderValue) -> Result<Tracker, crate::dtc::error::Error>{
        Ok(Tracker::new("dtc-placeholder".to_string()))
    }
}



impl FromRequest for Tracker {
    type Config = ();
    type Future = Result<Self, Self::Error>;
    type Error = LocalError;
    // convert request to future self
    fn from_request(req: &HttpRequest, _payload: &mut actix_web::dev::Payload) -> Self::Future {
        match req.headers().get(DTC_HEADER) {
            Some(u) => {
                match Tracker::tracker_from_header_value(u) {
                    Ok(dtc) => return Ok(dtc),
                    Err(e) => {
                        warn!("{}", e);
                        return Err(LocalError::BadDTC);
                    },
                }
            },
            None => {
                // couldn't find the header
                warn!("{}", LocalError::MissingDTC);
                return Err(LocalError::MissingDTC);
            },
        };
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::{test, web, http, App, HttpRequest, HttpResponse};
    use actix_web::dev::Service;
    use actix_web::http::{StatusCode};

    // supporting functions
    fn index(_req: HttpRequest) -> HttpResponse {
        return HttpResponse::Ok()
            .header(http::header::CONTENT_TYPE, "application/json")
            .body(r#"Ok"#)
    }

    fn index_extract_dtc(tracker: Tracker, _req: HttpRequest) -> HttpResponse {
        return HttpResponse::Ok()
            .header(http::header::CONTENT_TYPE, "application/json")
            .body(format!("{:?}", tracker))
    }

    // tests
    #[test]
    fn test_http_header_name() {
        assert_eq!(DTC_HEADER, "Data-Tracker-Chain");
    }

    #[test]
    fn test_dua_extractor_good() {
        let mut app = test::init_service(App::new().route("/", web::get().to(index_extract_dtc)));
        let req = test::TestRequest::get().uri("/")
            .header("content-type", "application/json")
            .header(DTC_HEADER, r#"12345"#)
            .to_request();
        let resp = test::block_on(app.call(req)).unwrap();
        assert_eq!(resp.status(), StatusCode::OK);
    }

    #[test]
    fn test_dtc_extractor_missing() {
        let mut app = test::init_service(App::new().route("/", web::get().to(index_extract_dtc)));
        let req = test::TestRequest::get().uri("/")
            .header("content-type", "application/json")
            .to_request();
        let resp = test::call_service(&mut app, req);
        assert_eq!(resp.status(), StatusCode::INTERNAL_SERVER_ERROR);
        // read response
        let bdy = test::read_body(resp);
        assert_eq!(String::from_utf8(bdy[..].to_vec()).unwrap(), actix_web::web::Bytes::from_static(b"Missing Data Tracker Chain"));
    }

    #[test]
    fn test_without_extractor() {
        let mut app = test::init_service(App::new().route("/", web::get().to(index)));
        let req = test::TestRequest::get().uri("/")
            .header("content-type", "application/json")
            .to_request();
        let resp = test::call_service(&mut app, req);
        assert_eq!(resp.status(), StatusCode::OK);
    }
}