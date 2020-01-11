//! The DUA Middleware is a simple way to ensure that web services that require 
//! Data Usage Agreements are provided in the Request as a http header. 
//!
//! If there is no `Data Usage Agreement` in the headers (use pbd::dua::DUA_HEADER),
//! the middleware will respond with a BadRequest status code.
//! 
//! ---
//! 
//! Example 
//!
//! ```
//! extern crate pbd;
//! extern crate actix_web;
//!
//! use pbd::dua::middleware::actix::*;
//! use actix_web::{web, App};
//! 
//! fn main () {
//!     let app = App::new()
//!                 .wrap(DUAEnforcer::default())
//!                 .service(
//!                     web::resource("/")
//!                     .route(web::get().to(|| "Got Data Usage Agreement?"))
//!                 );
//! }
//! ```
//!
//! To set the level of validation, use `new()` and pass the validation level constant
//!
//! ```
//! extern crate pbd;
//! extern crate actix_web;
//!
//! use pbd::dua::middleware::{VALIDATION_HIGH};
//! use pbd::dua::middleware::actix::*;
//! use actix_web::{web, App};
//! 
//! fn main () {
//!     let app = App::new()
//!                 .wrap(DUAEnforcer::new(VALIDATION_HIGH))
//!                 .service(
//!                     web::resource("/")
//!                     .route(web::get().to(|| "Got Data Usage Agreement?"))
//!                 );
//! }
//! ```
//!
//! For futher examples run `cargo run --example data-usage-agreement` 
//!

use super::*;
use crate::dua::extractor::actix::{DUAs};
use actix_web::{Error, HttpResponse};
use actix_web::dev::{ServiceRequest, ServiceResponse};
use actix_service::{Service, Transform};
use futures::future::{ok, Either, FutureResult};
use futures::{Poll};
use reqwest::StatusCode;
use rayon::prelude::*;

#[derive(Clone)]
pub struct DUAEnforcer{
    validation_level: u8,
}

impl DUAEnforcer {
    pub fn new(level: u8) -> Self {
        Self { 
            validation_level: level 
        }
    }

    pub fn set_validation(&mut self, level: u8) {
        self.validation_level = level;
    }
}

impl Default for DUAEnforcer {
    fn default() -> DUAEnforcer {
        DUAEnforcer {
            validation_level: 1
        }
    }
}

// `B` - type of response's body
impl<S, B> Transform<S> for DUAEnforcer
where
    S: Service<Request = ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
{
    type Request = ServiceRequest;
    type Response = ServiceResponse<B>;
    type Error = Error;
    type InitError = ();
    type Transform = DUAEnforcerMiddleware<S>;
    type Future = FutureResult<Self::Transform, Self::InitError>;

    fn new_transform(&self, service: S) -> Self::Future {
        ok(DUAEnforcerMiddleware { 
            service,
            validation_level: self.validation_level 
        })
    }
}

pub struct DUAEnforcerMiddleware<S> {
    service: S,
    validation_level: u8,
}

impl<S, B> Service for DUAEnforcerMiddleware<S>
where
    S: Service<Request = ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
{
    type Request = ServiceRequest;
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = Either<S::Future, FutureResult<Self::Response, Self::Error>>;

    fn poll_ready(&mut self) -> Poll<(), Self::Error> {
        self.service.poll_ready()
    }

    fn call(&mut self, req: ServiceRequest) -> Self::Future {
        debug!("VALIDATION LEVEL: {}", self.validation_level);

        if self.validation_level == VALIDATION_NONE {
            return Either::A(self.service.call(req)) 
        }

        match  req.headers().get(DUA_HEADER) {
            Some(list) => {
                let duas = DUAs::duas_from_header_value(list);
                let mut valid_ind: bool = false;

                // Level 1 Validation: Check to see if there are DUAs provided
                if self.validation_level >= VALIDATION_LOW && duas.vec().len() > 0 {
                    valid_ind = true;
                }
                
                // Level 2 Validation: Check to see if the DUAs provided are valid ones
                if valid_ind == true && self.validation_level >= VALIDATION_HIGH {
                    let checks: usize = duas.vec().par_iter()
                        .map(|d|
                            match reqwest::get(&d.location.clone()) {
                                Ok(rsp) => {
                                    if rsp.status() == StatusCode::OK { 
                                        1
                                    } 
                                    else {
                                        info!("{}", format!("Invalid DUA: {}", d.location.clone()));
                                        0
                                    }
                                },
                                Err(_err) => {
                                    info!("{}", format!("Invalid DUA: {}", d.location.clone()));
                                    0
                                },
                            }
                        )
                        .sum();
                
                    if duas.vec().len() == checks {
                        valid_ind = true;
                    } 
                    else {
                        valid_ind = false;
                    }
                }

                if valid_ind == true {
                    return Either::A(self.service.call(req))
                }
                else {
                    return Either::B(ok(req.into_response(
                        HttpResponse::BadRequest()
                            .finish()
                         .into_body(),
                    )))
                }
            },
            None => {
                return Either::B(ok(req.into_response(
                    HttpResponse::BadRequest()
                        .finish()
                        .into_body(),
                )))
            },
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
    fn index_middleware_dua(_req: HttpRequest) -> HttpResponse {
        HttpResponse::Ok()
            .header(http::header::CONTENT_TYPE, "application/json")
            .body(r#"{"status":"Ok"}"#)
    }    

    #[test]
    fn test_add_middleware() {
        let _app = App::new()
            .wrap(DUAEnforcer::default())
            .service(
                web::resource("/")
                    .route(web::get().to(index_middleware_dua))
            );

          assert!(true);
    }

    #[test]
    fn test_dua_none_missing() {
        let mut app = test::init_service(
            App::new()
            .wrap(DUAEnforcer::new(VALIDATION_NONE))
            .route("/", web::post().to(index_middleware_dua))
        );
        let req = test::TestRequest::post().uri("/")
            .header("content-type", "application/json")
            .to_request();
        let resp = test::block_on(app.call(req)).unwrap();
        assert_eq!(resp.status(), StatusCode::OK);
    }

    #[test]
    fn test_dua_default_ok() {
        let mut app = test::init_service(
            App::new()
            .wrap(DUAEnforcer::default())
            .route("/", web::post().to(index_middleware_dua))
        );
        let req = test::TestRequest::post().uri("/")
            .header("content-type", "application/json")
            .header(DUA_HEADER, r#"[{"agreement_name":"patient data use","location":"https://github.com/dsietz/pbd/blob/master/tests/duas/Patient%20Data%20Use%20Agreement.pdf","agreed_dtm": 1553988607}]"#)
            .to_request();
        let resp = test::block_on(app.call(req)).unwrap();
        
        assert_eq!(resp.status(), StatusCode::OK);
    } 

    #[test]
    fn test_dua_default_empty() {
        let mut app = test::init_service(
            App::new()
            .wrap(DUAEnforcer::default())
            .route("/", web::post().to(index_middleware_dua))
        );
        let req = test::TestRequest::post().uri("/")
            .header("content-type", "application/json")
            .header(DUA_HEADER, r#"[]"#)
            .to_request();
        let resp = test::block_on(app.call(req)).unwrap();
        
        assert_eq!(resp.status(), StatusCode::BAD_REQUEST);
    } 

    #[test]
    fn test_dua_default_invalid() {
        let mut app = test::init_service(
            App::new()
            .wrap(DUAEnforcer::default())
            .route("/", web::post().to(index_middleware_dua))
        );
        let req = test::TestRequest::post().uri("/")
            .header("content-type", "application/json")
            .header(DUA_HEADER, r#"[{"agreement_name":"patient data use","location":"https://example.com/invalid.pdf","agreed_dtm": 1553988607},{"agreement_name":"patient data use","location":"https://github.com/dsietz/pbd/blob/master/tests/duas/Patient%20Data%20Use%20Agreement.pdf","agreed_dtm": 1553988607}]"#)
            .to_request();
        let resp = test::block_on(app.call(req)).unwrap();
        
        assert_eq!(resp.status(), StatusCode::OK);
    } 

    #[test]
    fn test_dua_default_missing() {
        let mut app = test::init_service(
                            App::new()
                            .wrap(DUAEnforcer::default())
                            .route("/", web::post().to(index_middleware_dua))
                        );
        let req = test::TestRequest::post().uri("/")
            .header("content-type", "application/json")
            .to_request();
        let resp = test::block_on(app.call(req)).unwrap();
        assert_eq!(resp.status(), StatusCode::BAD_REQUEST);
    }     

    #[test]
    fn test_dua_valid_high_ok() {
        let mut app = test::init_service(
            App::new()
            .wrap(DUAEnforcer::new(VALIDATION_HIGH))
            .route("/", web::post().to(index_middleware_dua))
        );
        let req = test::TestRequest::post().uri("/")
            .header("content-type", "application/json")
            .header(DUA_HEADER, r#"[{"agreement_name":"patient data use","location":"https://github.com/dsietz/pbd/blob/master/tests/duas/Patient%20Data%20Use%20Agreement.pdf","agreed_dtm": 1553988607}]"#)
            .to_request();
        let resp = test::block_on(app.call(req)).unwrap();
        
        assert_eq!(resp.status(), StatusCode::OK);
    } 

    #[test]
    fn test_dua_valid_high_empty() {
        let mut app = test::init_service(
            App::new()
            .wrap(DUAEnforcer::new(VALIDATION_HIGH))
            .route("/", web::post().to(index_middleware_dua))
        );
        let req = test::TestRequest::post().uri("/")
            .header("content-type", "application/json")
            .header(DUA_HEADER, r#"[]"#)
            .to_request();
        let resp = test::block_on(app.call(req)).unwrap();
        
        assert_eq!(resp.status(), StatusCode::BAD_REQUEST);
    } 

    #[test]
    fn test_dua_valid_high_invalid() {
        let mut app = test::init_service(
            App::new()
            .wrap(DUAEnforcer::new(VALIDATION_HIGH))
            .route("/", web::post().to(index_middleware_dua))
        );
        let req = test::TestRequest::post().uri("/")
            .header("content-type", "application/json")
            .header(DUA_HEADER, r#"[{"agreement_name":"patient data use","location":"https://example.com/invalid.pdf","agreed_dtm": 1553988607},{"agreement_name":"patient data use","location":"https://github.com/dsietz/pbd/blob/master/tests/duas/Patient%20Data%20Use%20Agreement.pdf","agreed_dtm": 1553988607}]"#)
            .to_request();
        let resp = test::block_on(app.call(req)).unwrap();
        
        assert_eq!(resp.status(), StatusCode::BAD_REQUEST);
    } 

    #[test]
    fn test_dua_high_missing() {
        let mut app = test::init_service(
            App::new()
            .wrap(DUAEnforcer::new(VALIDATION_HIGH))
            .route("/", web::post().to(index_middleware_dua))
        );
        let req = test::TestRequest::post().uri("/")
            .header("content-type", "application/json")
            .to_request();
        let resp = test::block_on(app.call(req)).unwrap();
        assert_eq!(resp.status(), StatusCode::BAD_REQUEST);
    } 

    #[test]
    fn test_dua_low_ok() {
        let mut app = test::init_service(
            App::new()
            .wrap(DUAEnforcer::new(VALIDATION_LOW))
            .route("/", web::post().to(index_middleware_dua))
        );
        let req = test::TestRequest::post().uri("/")
            .header("content-type", "application/json")
            .header(DUA_HEADER, r#"[{"agreement_name":"patient data use","location":"https://github.com/dsietz/pbd/blob/master/tests/duas/Patient%20Data%20Use%20Agreement.pdf","agreed_dtm": 1553988607}]"#)
            .to_request();
        let resp = test::block_on(app.call(req)).unwrap();
        
        assert_eq!(resp.status(), StatusCode::OK);
    } 

    #[test]
    fn test_dua_low_empty() {
        let mut app = test::init_service(
            App::new()
            .wrap(DUAEnforcer::new(VALIDATION_LOW))
            .route("/", web::post().to(index_middleware_dua))
        );
        let req = test::TestRequest::post().uri("/")
            .header("content-type", "application/json")
            .header(DUA_HEADER, r#"[]"#)
            .to_request();
        let resp = test::block_on(app.call(req)).unwrap();
        
        assert_eq!(resp.status(), StatusCode::BAD_REQUEST);
    } 

    #[test]
    fn test_dua_low_invalid() {
        let mut app = test::init_service(
            App::new()
            .wrap(DUAEnforcer::new(VALIDATION_LOW))
            .route("/", web::post().to(index_middleware_dua))
        );
        let req = test::TestRequest::post().uri("/")
            .header("content-type", "application/json")
            .header(DUA_HEADER, r#"[{"agreement_name":"patient data use","location":"https://example.com/invalid.pdf","agreed_dtm": 1553988607},{"agreement_name":"patient data use","location":"https://github.com/dsietz/pbd/blob/master/tests/duas/Patient%20Data%20Use%20Agreement.pdf","agreed_dtm": 1553988607}]"#)
            .to_request();
        let resp = test::block_on(app.call(req)).unwrap();
        
        assert_eq!(resp.status(), StatusCode::OK);
    } 

    #[test]
    fn test_dua_low_missing() {
        let mut app = test::init_service(
            App::new()
            .wrap(DUAEnforcer::new(VALIDATION_LOW))
            .route("/", web::post().to(index_middleware_dua))
        );
        let req = test::TestRequest::post().uri("/")
            .header("content-type", "application/json")
            .to_request();
        let resp = test::block_on(app.call(req)).unwrap();
        assert_eq!(resp.status(), StatusCode::BAD_REQUEST);
    }     
}