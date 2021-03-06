//! The DTC Middleware is a simple way to ensure that web services that require
//! a Data Tracker Chain is provided in the Request as a http header.
//!
//! If there is no `Data Tracker Chain` in the header (use pbd::dtc::DTC_HEADER),
//! the middleware will respond with a BadRequest status code.
//!
//! ---
//!
//! Example
//!
//! ```rust,no_run
//! extern crate pbd;
//! extern crate actix_web;
//!
//! use pbd::dtc::middleware::actix::*;
//! use actix_web::{web, App, HttpServer, Responder};
//!
//! async fn index() -> impl Responder {
//!    "Got Data Tracker Chain?"
//! }
//!
//! #[actix_rt::main]
//! async fn main() -> std::io::Result<()> {
//!     HttpServer::new(|| App::new()
//!         .wrap(DTCEnforcer::default())
//!         .service(
//!             web::resource("/").to(index))
//!         )
//!             .bind("127.0.0.1:8080")?
//!             .run()
//!             .await
//! }
//! ```
//!
//! To set the level of validation, use `new()` and pass the validation level constant
//!
//! ```rust,no_run
//! extern crate pbd;
//! extern crate actix_web;
//!
//! use pbd::dtc::middleware::{VALIDATION_HIGH};
//! use pbd::dtc::middleware::actix::*;
//! use actix_web::{web, App, HttpServer, Responder};
//!
//! async fn index() -> impl Responder {
//!    "Got Data Tracker Chain?"
//! }
//!
//! #[actix_rt::main]
//! async fn main() -> std::io::Result<()> {
//!     HttpServer::new(|| App::new()
//!         .wrap(DTCEnforcer::new(VALIDATION_HIGH))
//!         .service(
//!             web::resource("/").to(index))
//!         )
//!             .bind("127.0.0.1:8080")?
//!             .run()
//!             .await
//! }
//! ```
//!
//! For a further example, run the command `cargo run --example data-tracker-chain`.
//! There are example service calls for POSTMAN (pbd.postman_collection.json) in the `examples` directory of the source code package.  
//!
#![allow(clippy::complexity)]
use super::*;
use crate::dtc::extractor::actix::TrackerHeader;
use crate::dtc::Tracker;
use actix_service::{Service, Transform};
use actix_web::dev::{ServiceRequest, ServiceResponse};
use actix_web::{Error, HttpResponse};
use futures::future::{ok, Either, Ready};
use std::task::{Context, Poll};

#[derive(Clone)]
pub struct DTCEnforcer {
    validation_level: u8,
}

impl DTCEnforcer {
    pub fn new(level: u8) -> Self {
        Self {
            validation_level: level,
        }
    }

    pub fn set_validation(&mut self, level: u8) {
        self.validation_level = level;
    }
}

impl Default for DTCEnforcer {
    fn default() -> DTCEnforcer {
        DTCEnforcer {
            validation_level: 1,
        }
    }
}

// `B` - type of response's body
impl<S, B> Transform<S> for DTCEnforcer
where
    S: Service<Request = ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Request = ServiceRequest;
    type Response = ServiceResponse<B>;
    type Error = Error;
    type InitError = ();
    type Transform = DTCEnforcerMiddleware<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ok(DTCEnforcerMiddleware {
            service,
            validation_level: self.validation_level,
        })
    }
}

impl<S, B> Service for DTCEnforcerMiddleware<S>
where
    S: Service<Request = ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Request = ServiceRequest;
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = Either<S::Future, Ready<Result<ServiceResponse<B>, Self::Error>>>;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.service.poll_ready(cx)
    }

    fn call(&mut self, req: ServiceRequest) -> Self::Future {
        debug!("VALIDATION LEVEL: {}", self.validation_level);

        if self.validation_level == VALIDATION_NONE {
            return Either::Left(self.service.call(req));
        }

        match req.headers().get(DTC_HEADER) {
            Some(header_value) => {
                let mut valid_ind: bool = false;

                match Tracker::tracker_from_header_value(header_value) {
                    Ok(tracker) => {
                        // Level 1 Validation: Check to see if there are DTC is provided
                        if self.validation_level >= VALIDATION_LOW {
                            valid_ind = true;
                        }

                        // Level 2 Validation: Check to see if the DUAs provided are valid ones
                        if valid_ind && self.validation_level >= VALIDATION_HIGH {
                            if !tracker.is_valid() {
                                warn!("{}", crate::dtc::error::Error::BadDTC);
                                valid_ind = false;
                            } else {
                                valid_ind = true;
                            }
                        }

                        if valid_ind {
                            Either::Left(self.service.call(req))
                        } else {
                            Either::Right(ok(
                                req.into_response(HttpResponse::BadRequest().finish().into_body())
                            ))
                        }
                    }
                    Err(e) => {
                        warn!("{}", e);
                        Either::Right(ok(
                            req.into_response(HttpResponse::BadRequest().finish().into_body())
                        ))
                    }
                }
            }
            None => Either::Right(ok(
                req.into_response(HttpResponse::BadRequest().finish().into_body())
            )),
        }
    }
}

pub struct DTCEnforcerMiddleware<S> {
    service: S,
    validation_level: u8,
}

#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::http::StatusCode;
    use actix_web::{http, test, web, App, HttpRequest, HttpResponse};

    // supporting functions
    fn get_dtc_header() -> String {
        base64::encode(
            r#"[{"identifier":{"data_id":"order~clothing~iStore~15150","index":0,"timestamp":0,"actor_id":"","previous_hash":"0"},"hash":"272081696611464773728024926793703167782","nonce":5},{"identifier":{"data_id":"order~clothing~iStore~15150","index":1,"timestamp":1578071239,"actor_id":"notifier~billing~receipt~email","previous_hash":"272081696611464773728024926793703167782"},"hash":"50104149701098700632511144125867736193","nonce":5}]"#,
        )
    }

    fn get_dtc_header_invalid() -> String {
        base64::encode(
            r#"[{"identifier":{"data_id":"order~clothing~iStore~15150","index":0,"timestamp":0,"actor_id":"","previous_hash":"0"},"hash":"272081696611464773728024926793703167784","nonce":5},{"identifier":{"data_id":"order~clothing~iStore~15150","index":1,"timestamp":1578071239,"actor_id":"notifier~billing~receipt~email","previous_hash":"272081696611464773728024926793703167782"},"hash":"50104149701098700632511144125867736193","nonce":5}]"#,
        )
    }

    fn index_middleware_dtc(_req: HttpRequest) -> HttpResponse {
        HttpResponse::Ok()
            .header(http::header::CONTENT_TYPE, "application/json")
            .body(r#"{"status":"Ok"}"#)
    }

    #[test]
    fn test_add_middleware() {
        let _app = App::new()
            .wrap(DTCEnforcer::default())
            .service(web::resource("/").route(web::get().to(index_middleware_dtc)));

        assert!(true);
    }

    #[actix_rt::test]
    async fn test_dtc_none_missing() {
        let mut app = test::init_service(
            App::new()
                .wrap(DTCEnforcer::new(VALIDATION_NONE))
                .route("/", web::post().to(index_middleware_dtc)),
        )
        .await;
        let req = test::TestRequest::post()
            .uri("/")
            .header("content-type", "application/json")
            .to_request();
        let resp = test::call_service(&mut app, req).await;
        assert_eq!(resp.status(), StatusCode::OK);
    }

    #[actix_rt::test]
    async fn test_dtc_default_ok() {
        let mut app = test::init_service(
            App::new()
                .wrap(DTCEnforcer::default())
                .route("/", web::post().to(index_middleware_dtc)),
        )
        .await;
        let req = test::TestRequest::post()
            .uri("/")
            .header("content-type", "application/json")
            .header(DTC_HEADER, get_dtc_header())
            .to_request();
        let resp = test::call_service(&mut app, req).await;

        assert_eq!(resp.status(), StatusCode::OK);
    }

    #[actix_rt::test]
    async fn test_dtc_default_empty() {
        let mut app = test::init_service(
            App::new()
                .wrap(DTCEnforcer::default())
                .route("/", web::post().to(index_middleware_dtc)),
        )
        .await;
        let req = test::TestRequest::post()
            .uri("/")
            .header("content-type", "application/json")
            .header(DTC_HEADER, "")
            .to_request();
        let resp = test::call_service(&mut app, req).await;

        assert_eq!(resp.status(), StatusCode::BAD_REQUEST);
    }

    #[actix_rt::test]
    async fn test_dtc_default_invalid() {
        let mut app = test::init_service(
            App::new()
                .wrap(DTCEnforcer::default())
                .route("/", web::post().to(index_middleware_dtc)),
        )
        .await;
        let req = test::TestRequest::post()
            .uri("/")
            .header("content-type", "application/json")
            .header(DTC_HEADER, get_dtc_header())
            .to_request();
        let resp = test::call_service(&mut app, req).await;

        assert_eq!(resp.status(), StatusCode::OK);
    }

    #[actix_rt::test]
    async fn test_dtc_default_missing() {
        let mut app = test::init_service(
            App::new()
                .wrap(DTCEnforcer::default())
                .route("/", web::post().to(index_middleware_dtc)),
        )
        .await;
        let req = test::TestRequest::post()
            .uri("/")
            .header("content-type", "application/json")
            .to_request();
        let resp = test::call_service(&mut app, req).await;
        assert_eq!(resp.status(), StatusCode::BAD_REQUEST);
    }

    #[actix_rt::test]
    async fn test_dtc_valid_high_ok() {
        let mut app = test::init_service(
            App::new()
                .wrap(DTCEnforcer::new(VALIDATION_HIGH))
                .route("/", web::post().to(index_middleware_dtc)),
        )
        .await;
        let req = test::TestRequest::post()
            .uri("/")
            .header("content-type", "application/json")
            .header(DTC_HEADER, get_dtc_header())
            .to_request();
        let resp = test::call_service(&mut app, req).await;

        assert_eq!(resp.status(), StatusCode::OK);
    }

    #[actix_rt::test]
    async fn test_dtc_high_empty() {
        let mut app = test::init_service(
            App::new()
                .wrap(DTCEnforcer::new(VALIDATION_HIGH))
                .route("/", web::post().to(index_middleware_dtc)),
        )
        .await;
        let req = test::TestRequest::post()
            .uri("/")
            .header("content-type", "application/json")
            .header(DTC_HEADER, "")
            .to_request();
        let resp = test::call_service(&mut app, req).await;

        assert_eq!(resp.status(), StatusCode::BAD_REQUEST);
    }

    #[actix_rt::test]
    async fn test_dtc_high_invalid() {
        let mut app = test::init_service(
            App::new()
                .wrap(DTCEnforcer::new(VALIDATION_HIGH))
                .route("/", web::post().to(index_middleware_dtc)),
        )
        .await;
        let req = test::TestRequest::post()
            .uri("/")
            .header("content-type", "application/json")
            .header(DTC_HEADER, get_dtc_header_invalid())
            .to_request();
        let resp = test::call_service(&mut app, req).await;

        assert_eq!(resp.status(), StatusCode::BAD_REQUEST);
    }

    #[actix_rt::test]
    async fn test_dtc_high_missing() {
        let mut app = test::init_service(
            App::new()
                .wrap(DTCEnforcer::new(VALIDATION_HIGH))
                .route("/", web::post().to(index_middleware_dtc)),
        )
        .await;
        let req = test::TestRequest::post()
            .uri("/")
            .header("content-type", "application/json")
            .to_request();
        let resp = test::call_service(&mut app, req).await;
        assert_eq!(resp.status(), StatusCode::BAD_REQUEST);
    }

    #[actix_rt::test]
    async fn test_dtc_low_ok() {
        let mut app = test::init_service(
            App::new()
                .wrap(DTCEnforcer::new(VALIDATION_LOW))
                .route("/", web::post().to(index_middleware_dtc)),
        )
        .await;
        let req = test::TestRequest::post()
            .uri("/")
            .header("content-type", "application/json")
            .header(DTC_HEADER, get_dtc_header())
            .to_request();
        let resp = test::call_service(&mut app, req).await;

        assert_eq!(resp.status(), StatusCode::OK);
    }

    #[actix_rt::test]
    async fn test_dtc_low_empty() {
        let mut app = test::init_service(
            App::new()
                .wrap(DTCEnforcer::new(VALIDATION_LOW))
                .route("/", web::post().to(index_middleware_dtc)),
        )
        .await;
        let req = test::TestRequest::post()
            .uri("/")
            .header("content-type", "application/json")
            .header(DTC_HEADER, "")
            .to_request();
        let resp = test::call_service(&mut app, req).await;

        assert_eq!(resp.status(), StatusCode::BAD_REQUEST);
    }

    #[actix_rt::test]
    async fn test_dtc_low_invalid() {
        let mut app = test::init_service(
            App::new()
                .wrap(DTCEnforcer::new(VALIDATION_LOW))
                .route("/", web::post().to(index_middleware_dtc)),
        )
        .await;
        let req = test::TestRequest::post()
            .uri("/")
            .header("content-type", "application/json")
            .header(DTC_HEADER, get_dtc_header_invalid())
            .to_request();
        let resp = test::call_service(&mut app, req).await;

        assert_eq!(resp.status(), StatusCode::OK);
    }

    #[actix_rt::test]
    async fn test_dtc_low_missing() {
        let mut app = test::init_service(
            App::new()
                .wrap(DTCEnforcer::new(VALIDATION_LOW))
                .route("/", web::post().to(index_middleware_dtc)),
        )
        .await;
        let req = test::TestRequest::post()
            .uri("/")
            .header("content-type", "application/json")
            .to_request();
        let resp = test::call_service(&mut app, req).await;
        assert_eq!(resp.status(), StatusCode::BAD_REQUEST);
    }
}
