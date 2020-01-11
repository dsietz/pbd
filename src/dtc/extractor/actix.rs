//! The DTC Extractor is a simple way to pull the Tracker from the HTTP Request. 
//! If you have a Tracker and wish to add it to a HTTP Header, use the `serialize()` method of the Tracker to get the MarkerChain.
//! 
//! NOTE: You need to Base64 encode the MarkerChain before setting the HTTP header value.
//! 
//! ---
//! 
//! Example 
//! ```
//! extern crate pbd;
//! extern crate actix_web;
//! extern crate base64;
//! 
//! use pbd::dtc::{DTC_HEADER, Tracker};
//! use pbd::dtc::extractor::actix::*;
//! use actix_web::{web, http, test, App, HttpRequest, HttpResponse};
//! use actix_web::http::{StatusCode};
//! use actix_web::dev::Service;
//! 
//!
//! fn index_extract_dtc(tracker: Tracker, _req: HttpRequest) -> HttpResponse {  
//!     HttpResponse::Ok()
//!         .header(http::header::CONTENT_TYPE, "application/json")
//!         .body(format!("{}", tracker))
//! }
//! 
//! fn main () {
//!     let tracker = Tracker::new("order~clothing~iStore~15150".to_string());
//!     let markerchain = tracker.serialize();
//! 
//!     let mut app = test::init_service(App::new().route("/", web::get().to(index_extract_dtc)));
//!     let req = test::TestRequest::get().uri("/")
//!         .header("content-type", "application/json")
//!         .header(DTC_HEADER, base64::encode(&markerchain))
//!         .to_request();
//!     let resp = test::block_on(app.call(req)).unwrap();
//!     
//!     assert_eq!(resp.status(), StatusCode::OK);
//! }
//! ```



use super::*;
use std::fmt;
use actix_web::{FromRequest, HttpRequest};
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
    /// Constructs a Tracker from the http header that contains the serialized value of the MarkerChain
    ///
    /// #Example
    ///
    /// ```
    /// extern crate pbd;
    /// extern crate actix_web;
    ///
    /// use pbd::dtc::Tracker;
    /// use pbd::dtc::extractor::actix::TrackerHeader;
    /// use actix_web::http::header::HeaderValue;
    ///
    /// fn main() {
    ///     // NOTE: The header value must be Base64 encoded
    ///     let header_value = HeaderValue::from_static("W3siaWRlbnRpZmllciI6eyJkYXRhX2lkIjoib3JkZXJ+Y2xvdGhpbmd+aVN0b3JlfjE1MTUwIiwiaW5kZXgiOjAsInRpbWVzdGFtcCI6MCwiYWN0b3JfaWQiOiIifSwiaGFzaCI6IjE4NTUyODk4NTgzMDIzMDU2Njc2MDIzNjIwMzIyODU4OTI1MDU1NiIsInByZXZpb3VzX2hhc2giOiIwIiwibm9uY2UiOjV9XQ=="); 
    ///     let tracker = Tracker::tracker_from_header_value(&header_value);
    ///     
    ///     assert!(tracker.is_ok());
    /// }
    /// ```
    fn tracker_from_header_value(header_value: &HeaderValue) -> Result<Tracker, error::Error>{
        match base64::decode(header_value.to_str().unwrap()){
            Ok(b) => {
                let chain = String::from_utf8(b).unwrap();

                match Tracker::from_serialized(&chain) {
                    Ok(t) => Ok(t),
                    Err(e) => {
                        warn!("{}", e);
                        Err(e)
                    }, 
                }
            },
            Err(_e) => {
                warn!("{}", Error::Base64DTC);
                Err(Error::Base64DTC)
            }
        }
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
    fn get_dtc_header() -> String{
        base64::encode(r#"[{"identifier":{"data_id":"order~clothing~iStore~15150","index":0,"timestamp":0,"actor_id":""},"hash":"185528985830230566760236203228589250556","previous_hash":"0","nonce":5},{"identifier":{"data_id":"order~clothing~iStore~15150","index":1,"timestamp":1578071239,"actor_id":"notifier~billing~receipt~email"},"hash":"291471950171806362795097431348191551247","previous_hash":"185528985830230566760236203228589250556","nonce":5}]"#)
    }

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
    fn test_dtc_extractor_good() {
        let mut app = test::init_service(App::new().route("/", web::get().to(index_extract_dtc)));
        let req = test::TestRequest::get().uri("/")
            .header("content-type", "application/json")
            .header(DTC_HEADER, get_dtc_header())
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

    #[test]
    fn test_dtc_extractor_no_base64() {
        let mut app = test::init_service(App::new().route("/", web::get().to(index_extract_dtc)));
        let req = test::TestRequest::get().uri("/")
            .header("content-type", "application/json")
            .header(DTC_HEADER, r#"[{"identifier":{"data_id":"order~clothing~iStore~15150","index":0,"timestamp":0,"actor_id":""},"hash":"185528985830230566760236203228589250556","previous_hash":"0","nonce":5}]"#)
            .to_request();
        let resp = test::call_service(&mut app, req);
        assert_eq!(resp.status(), StatusCode::INTERNAL_SERVER_ERROR);
        // read response
        let bdy = test::read_body(resp);
        assert_eq!(String::from_utf8(bdy[..].to_vec()).unwrap(), actix_web::web::Bytes::from_static(b"Corrupt or invalid Data Tracker Chain"));
    }
}