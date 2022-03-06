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
//! ```rust,no_run
//! extern crate pbd;
//! extern crate actix_web;
//!
//! use pbd::dua::middleware::actix::*;
//! use actix_web::{web, App, HttpServer, Responder};
//!
//! async fn index() -> impl Responder {
//!    "Got Data Usage Agreement?"
//! }
//!
//! #[actix_rt::main]
//! async fn main() -> std::io::Result<()> {
//!     HttpServer::new(|| App::new()
//!         .wrap(DUAEnforcer::default())
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
//! use pbd::dua::middleware::actix::*;
//! use pbd::dua::middleware::{VALIDATION_HIGH};
//! use actix_web::{web, App, HttpServer, Responder};
//!
//! async fn index() -> impl Responder {
//!    "Got Data Usage Agreement?"
//! }
//!
//! #[actix_rt::main]
//! async fn main() -> std::io::Result<()> {
//!     HttpServer::new(|| App::new()
//!         .wrap(DUAEnforcer::new(VALIDATION_HIGH))
//!         .service(
//!             web::resource("/").to(index))
//!         )
//!             .bind("127.0.0.1:8080")?
//!             .run()
//!             .await
//! }
//! ```
//!
//! For a further example, run the command `cargo run --example data-usage-agreement`.
//! There are example service calls for POSTMAN (pbd.postman_collection.json) in the `examples` directory of the source code package.  
//!

#![allow(clippy::complexity)]
use super::*;
use crate::dua::extractor::actix::DUAs;
use actix_web::dev::{Response, ServiceRequest, ServiceResponse, Service, Transform};
use actix_web::{Error};
use futures::future::{ok, Either, Ready};
use rayon::prelude::*;
use reqwest::StatusCode;
use std::task::{Context, Poll};

#[derive(Clone)]
pub struct DUAEnforcer {
    validation_level: u8,
}

impl DUAEnforcer {
    pub fn new(level: u8) -> Self {
        Self {
            validation_level: level,
        }
    }

    pub fn set_validation(&mut self, level: u8) {
        self.validation_level = level;
    }
}

impl Default for DUAEnforcer {
    fn default() -> DUAEnforcer {
        DUAEnforcer {
            validation_level: 1,
        }
    }
}

// `B` - type of response's body
impl<S, B> Transform<S, ServiceRequest> for DUAEnforcer
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type InitError = ();
    type Transform = DUAEnforcerMiddleware<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ok(DUAEnforcerMiddleware {
            service,
            validation_level: self.validation_level,
        })
    }
}

pub struct DUAEnforcerMiddleware<S> {
    service: S,
    validation_level: u8,
}

impl<S, B> Service<ServiceRequest> for DUAEnforcerMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = Either<S::Future, Ready<Result<ServiceResponse<B>, Self::Error>>>;

    fn poll_ready(&self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.service.poll_ready(cx)
    }

    fn call(&self, req: ServiceRequest) -> Self::Future {
        debug!("VALIDATION LEVEL: {}", self.validation_level);

        if self.validation_level == VALIDATION_NONE {
            return Either::Left(self.service.call(req));
        }

        match req.headers().get(DUA_HEADER) {
            Some(list) => {
                let duas = DUAs::duas_from_header_value(list);
                let mut valid_ind: bool = false;

                // Level 1 Validation: Check to see if there are DUAs provided
                if self.validation_level >= VALIDATION_LOW && !duas.vec().is_empty() {
                    valid_ind = true;
                }

                // Level 2 Validation: Check to see if the DUAs provided are valid ones
                if valid_ind && self.validation_level >= VALIDATION_HIGH {
                    let checks: usize = duas
                        .vec()
                        .par_iter()
                        .map(|d| match reqwest::blocking::get(&d.location.clone()) {
                            Ok(rsp) => {
                                if rsp.status() == StatusCode::OK {
                                    1
                                } else {
                                    info!("{}", format!("Invalid DUA: {}", d.location.clone()));
                                    0
                                }
                            }
                            Err(_err) => {
                                info!("{}", format!("Invalid DUA: {}", d.location.clone()));
                                0
                            }
                        })
                        .sum();

                    if duas.vec().len() == checks {
                        valid_ind = true;
                    } else {
                        valid_ind = false;
                    }
                }

                match valid_ind {
                    true => Either::Left(self.service.call(req)),
                    false => Either::Right(ok(
                        req.into_response(Response::bad_request().into())
                    ))
                }
            }
            None => Either::Right(ok(
                req.into_response(Response::bad_request().into())
            )),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::http::StatusCode;
    use actix_web::{
        http, 
        http::header::ContentType, 
        test, 
        web, 
        App, 
        HttpRequest, 
        HttpResponse
    };

    // supporting functions
    async fn index_middleware_dua(_req: HttpRequest) -> HttpResponse {
        HttpResponse::Ok()
            .header(http::header::CONTENT_TYPE, "application/json")
            .body(r#"{"status":"Ok"}"#)
    }

    #[test]
    async fn test_add_middleware() {
        let _app = App::new()
            .wrap(DUAEnforcer::default())
            .service(web::resource("/").route(web::get().to(index_middleware_dua)));

        assert!(true);
    }

    #[actix_rt::test]
    async fn test_dua_none_missing() {
        let mut app = test::init_service(
            App::new()
                .wrap(DUAEnforcer::new(VALIDATION_NONE))
                .route("/", web::post().to(index_middleware_dua)),
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
    async fn test_dua_default_ok() {
        let mut app = test::init_service(
            App::new()
                .wrap(DUAEnforcer::default())
                .route("/", web::post().to(index_middleware_dua)),
        )
        .await;
        let req = test::TestRequest::post().uri("/")
            .header("content-type", "application/json")
            .header(DUA_HEADER, r#"[{"agreement_name":"patient data use","location":"https://github.com/dsietz/pbd/blob/master/tests/duas/Patient%20Data%20Use%20Agreement.pdf","agreed_dtm": 1553988607}]"#)
            .to_request();
        let resp = test::call_service(&mut app, req).await;

        assert_eq!(resp.status(), StatusCode::OK);
    }

    #[actix_rt::test]
    async fn test_dua_default_empty() {
        let mut app = test::init_service(
            App::new()
                .wrap(DUAEnforcer::default())
                .route("/", web::post().to(index_middleware_dua)),
        )
        .await;
        let req = test::TestRequest::post()
            .uri("/")
            .header("content-type", "application/json")
            .header(DUA_HEADER, r#"[]"#)
            .to_request();
        let resp = test::call_service(&mut app, req).await;

        assert_eq!(resp.status(), StatusCode::BAD_REQUEST);
    }

    #[actix_rt::test]
    async fn test_dua_default_invalid() {
        let mut app = test::init_service(
            App::new()
                .wrap(DUAEnforcer::default())
                .route("/", web::post().to(index_middleware_dua)),
        )
        .await;
        let req = test::TestRequest::post().uri("/")
            .header("content-type", "application/json")
            .header(DUA_HEADER, r#"[{"agreement_name":"patient data use","location":"https://example.com/invalid.pdf","agreed_dtm": 1553988607},{"agreement_name":"patient data use","location":"https://github.com/dsietz/pbd/blob/master/tests/duas/Patient%20Data%20Use%20Agreement.pdf","agreed_dtm": 1553988607}]"#)
            .to_request();
        let resp = test::call_service(&mut app, req).await;

        assert_eq!(resp.status(), StatusCode::OK);
    }

    #[actix_rt::test]
    async fn test_dua_default_missing() {
        let mut app = test::init_service(
            App::new()
                .wrap(DUAEnforcer::default())
                .route("/", web::post().to(index_middleware_dua)),
        )
        .await;
        let req = test::TestRequest::post()
            .uri("/")
            .header("content-type", "application/json")
            .to_request();
        let resp = test::call_service(&mut app, req).await;
        assert_eq!(resp.status(), StatusCode::BAD_REQUEST);
    }

    #[test]
    async fn test_dua_default_validation_level() {
        let dflt = DUAEnforcer::default();
        assert_eq!(dflt.validation_level, 1);
    }

    #[actix_rt::test]
    async fn test_dua_valid_high_ok() {
        let mut app = test::init_service(
            App::new()
                .wrap(DUAEnforcer::new(VALIDATION_HIGH))
                .route("/", web::post().to(index_middleware_dua)),
        )
        .await;
        let req = test::TestRequest::post().uri("/")
            .insert_header(ContentType::json())
            .insert_header(
                (DUA_HEADER, 
                r#"[{"agreement_name":"patient data use","location":"https://github.com/dsietz/pbd/blob/master/tests/duas/Patient%20Data%20Use%20Agreement.pdf","agreed_dtm": 1553988607}]"#)
                )
            .to_request();
        let resp = test::call_service(&mut app, req).await;

        assert_eq!(resp.status(), StatusCode::OK);
    }

    #[actix_rt::test]
    async fn test_dua_valid_high_empty() {
        let mut app = test::init_service(
            App::new()
                .wrap(DUAEnforcer::new(VALIDATION_HIGH))
                .route("/", web::post().to(index_middleware_dua)),
        )
        .await;
        let req = test::TestRequest::post()
            .uri("/")
            .header("content-type", "application/json")
            .header(DUA_HEADER, r#"[]"#)
            .to_request();
        let resp = test::call_service(&mut app, req).await;

        assert_eq!(resp.status(), StatusCode::BAD_REQUEST);
    }

    #[actix_rt::test]
    async fn test_dua_valid_high_invalid() {
        let mut app = test::init_service(
            App::new()
                .wrap(DUAEnforcer::new(VALIDATION_HIGH))
                .route("/", web::post().to(index_middleware_dua)),
        )
        .await;
        let req = test::TestRequest::post().uri("/")
            .header("content-type", "application/json")
            .header(DUA_HEADER, r#"[{"agreement_name":"patient data use","location":"https://example.com/invalid.pdf","agreed_dtm": 1553988607},{"agreement_name":"patient data use","location":"https://github.com/dsietz/pbd/blob/master/tests/duas/Patient%20Data%20Use%20Agreement.pdf","agreed_dtm": 1553988607}]"#)
            .to_request();
        let resp = test::call_service(&mut app, req).await;

        assert_eq!(resp.status(), StatusCode::BAD_REQUEST);
    }

    #[actix_rt::test]
    async fn test_dua_high_missing() {
        let mut app = test::init_service(
            App::new()
                .wrap(DUAEnforcer::new(VALIDATION_HIGH))
                .route("/", web::post().to(index_middleware_dua)),
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
    async fn test_dua_low_ok() {
        let mut app = test::init_service(
            App::new()
                .wrap(DUAEnforcer::new(VALIDATION_LOW))
                .route("/", web::post().to(index_middleware_dua)),
        )
        .await;
        let req = test::TestRequest::post().uri("/")
            .header("content-type", "application/json")
            .header(DUA_HEADER, r#"[{"agreement_name":"patient data use","location":"https://github.com/dsietz/pbd/blob/master/tests/duas/Patient%20Data%20Use%20Agreement.pdf","agreed_dtm": 1553988607}]"#)
            .to_request();
        let resp = test::call_service(&mut app, req).await;

        assert_eq!(resp.status(), StatusCode::OK);
    }

    #[actix_rt::test]
    async fn test_dua_low_empty() {
        let mut app = test::init_service(
            App::new()
                .wrap(DUAEnforcer::new(VALIDATION_LOW))
                .route("/", web::post().to(index_middleware_dua)),
        )
        .await;
        let req = test::TestRequest::post()
            .uri("/")
            .header("content-type", "application/json")
            .header(DUA_HEADER, r#"[]"#)
            .to_request();
        let resp = test::call_service(&mut app, req).await;

        assert_eq!(resp.status(), StatusCode::BAD_REQUEST);
    }

    #[actix_rt::test]
    async fn test_dua_low_invalid() {
        let mut app = test::init_service(
            App::new()
                .wrap(DUAEnforcer::new(VALIDATION_LOW))
                .route("/", web::post().to(index_middleware_dua)),
        )
        .await;
        let req = test::TestRequest::post().uri("/")
            .header("content-type", "application/json")
            .header(DUA_HEADER, r#"[{"agreement_name":"patient data use","location":"https://example.com/invalid.pdf","agreed_dtm": 1553988607},{"agreement_name":"patient data use","location":"https://github.com/dsietz/pbd/blob/master/tests/duas/Patient%20Data%20Use%20Agreement.pdf","agreed_dtm": 1553988607}]"#)
            .to_request();
        let resp = test::call_service(&mut app, req).await;

        assert_eq!(resp.status(), StatusCode::OK);
    }

    #[actix_rt::test]
    async fn test_dua_low_missing() {
        let mut app = test::init_service(
            App::new()
                .wrap(DUAEnforcer::new(VALIDATION_LOW))
                .route("/", web::post().to(index_middleware_dua)),
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
