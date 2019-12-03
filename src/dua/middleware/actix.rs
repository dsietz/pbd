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
//! extern crate pbd;
//! extern crate actix_web;
//!
//! ```
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

use super::*;
use actix_web::{Error, HttpResponse};
use actix_web::dev::{ServiceRequest, ServiceResponse};
use actix_service::{Service, Transform};
use futures::future::{ok, Either, FutureResult};
use futures::{Poll};

#[derive(Default, Clone)]
pub struct DUAEnforcer;

impl DUAEnforcer {}

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
        ok(DUAEnforcerMiddleware { service })
    }
}

pub struct DUAEnforcerMiddleware<S> {
    service: S,
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
        match  req.headers().get(DUA_HEADER) {
            Some(_hdr) => {
                Either::A(self.service.call(req))
            },
            None => {
                Either::B(ok(req.into_response(
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

    // tests
    #[test]
    fn test_dua_ok() {
        let mut app = test::init_service(
            App::new()
            .wrap(DUAEnforcer::default())
            .route("/", web::post().to(index_middleware_dua))
        );
        let req = test::TestRequest::post().uri("/")
            .header("content-type", "application/json")
            .header(DUA_HEADER, r#"[{"agreement_name":"billing","location":"www.dua.org/billing.pdf","agreed_dtm": 1553988607}]"#)
            .to_request();
        let resp = test::block_on(app.call(req)).unwrap();
        assert_eq!(resp.status(), StatusCode::OK);
    } 

    #[test]
    fn test_dua_missing() {
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
}