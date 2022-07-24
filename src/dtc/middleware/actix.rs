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
use actix_web::dev::{forward_ready, Response, ServiceRequest, ServiceResponse, Service, Transform};
use actix_web::{Error, HttpResponse};
use actix_web::http::{
    header::ContentType,
    StatusCode,
};
use futures::future::{ok, Either, Ready};
// use std::task::{Context, Poll};

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
impl<S, B> Transform<S, ServiceRequest> for DTCEnforcer
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
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

impl<S, B> Service<ServiceRequest> for DTCEnforcerMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = Either<S::Future, Ready<Result<Self::Response, Self::Error>>>;

    forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        debug!("VALIDATION LEVEL: {}", self.validation_level);

        if self.validation_level == VALIDATION_NONE {
            return Either::Left(self.service.call(req));
        }

        match req.headers().get(DTC_HEADER) {
            Some(header_value) => {
                let mut valid_ind: bool = match Tracker::tracker_from_header_value(header_value) {
                    Ok(tracker) => {
                        // Level 1 Validation: Check to see if there are DTC is provided
                        match self.validation_level >= VALIDATION_LOW {
                            true => {
                                // Level 2 Validation: Check to see if the DUAs provided are valid ones
                                match self.validation_level >= VALIDATION_HIGH {
                                    true => {
                                        match !tracker.is_valid() {
                                            true => {
                                                warn!("{}", crate::dtc::error::Error::BadDTC);
                                                false
                                            },
                                            false => true,
                                        }
                                    },
                                    false => true,
                                }
                            }
                            false => false,
                        }
                    },
                    Err(e) => {
                        warn!("{}", e);
                        false
                    }
                };

                match valid_ind {
                    true => {
                        Either::Left(self.service.call(req))
                    },
                    false => {
                        let (request, _pl) = req.into_parts();
                        let response = HttpResponse::BadRequest()
                            .insert_header(ContentType::plaintext())
                            .finish();
                            // .map_into_right_body();
                        Either::Right(ok(   
                            // response                         
                            ServiceResponse::new(request, response)
                            // req.into_response(
                            //     Response::with_body(
                            //         StatusCode::BAD_REQUEST, 
                            //         "Missing Data Tracker Chain header")
                            //         .into()
                            //     )
                        ))
                    },
                }
            }
            None => Either::Right(ok(
                req.into_response(Response::bad_request().into())
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
    use actix_web::{
        http::header::ContentType, 
        test, 
        web, 
        App, 
        HttpRequest, 
        HttpResponse
    };

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

    async fn index_middleware_dtc(_req: HttpRequest) -> HttpResponse {
        HttpResponse::Ok()
            .insert_header(ContentType::json())
            .body(r#"{"status":"Ok"}"#)
    }

    #[test]
    async fn test_add_middleware() {
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
            .insert_header(ContentType::json())
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
            .insert_header(ContentType::json())
            .insert_header((DTC_HEADER, get_dtc_header()))
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
            .insert_header(ContentType::json())
            .insert_header((DTC_HEADER, ""))
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
            .insert_header(ContentType::json())
            .insert_header((DTC_HEADER, get_dtc_header()))
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
            .insert_header(ContentType::json())
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
            .insert_header(ContentType::json())
            .insert_header((DTC_HEADER, get_dtc_header()))
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
            .insert_header(ContentType::json())
            .insert_header((DTC_HEADER, ""))
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
            .insert_header(ContentType::json())
            .insert_header((DTC_HEADER, get_dtc_header_invalid()))
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
            .insert_header(ContentType::json())
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
            .insert_header(ContentType::json())
            .insert_header((DTC_HEADER, get_dtc_header()))
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
            .insert_header(ContentType::json())
            .insert_header((DTC_HEADER, ""))
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
            .insert_header(ContentType::json())
            .insert_header((DTC_HEADER, get_dtc_header_invalid()))
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
            .insert_header(ContentType::json())
            .to_request();
        let resp = test::call_service(&mut app, req).await;
        assert_eq!(resp.status(), StatusCode::BAD_REQUEST);
    }
}
