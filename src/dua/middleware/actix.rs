//!
//! extern crate pbd;
//! extern crate actix_web;
//!
//! 
//!
//! ```
//! use actix_web::middleware::Logger;
//! use pbd::dua::middleware::actix::*;
//! use actix_web::{web, App, HttpRequest, HttpResponse};
//!
//! pub fn index(_req: &HttpRequest) -> &'static str {
//!     "Hello World!"
//! }
//! 
//! fn main () {
//!     let app = App::new()
//!         .wrap(DUAEnforcer::new())
//!         .service(
//!             web::resource("/")
//!                 .route(web::get().to(|| HttpResponse::Ok()))
//!          );
//! }
//! ```

use super::*;

use actix_web::dev::{ServiceRequest, ServiceResponse};
use actix_service::{Service, Transform, IntoTransform};
use futures::future::{ok, FutureResult};
use futures::{Future, Poll};
// Middleware for checking Data Usage Agreement
///
/// If there is no `Data Usage Agreement` in the headers (use pbd::dua::DUA_HEADER),
/// the middleware will respond with a BadRequest status code.
///
///

pub type LocalError = super::error::Error;

pub struct DUAEnforcer;

impl DUAEnforcer {
    pub fn new() -> DUAEnforcer {
        DUAEnforcer{}
    }
}
/*
impl<T, S> IntoTransform<T, S> for DUAEnforcer 
{
    fn into_transform(&self) -> DUAEnforcer {
        self
    }
}
*/
// `B` - type of response's body
impl<S, B> Transform<S> for DUAEnforcer
where
    S: Service<Request = ServiceRequest, Response = ServiceResponse<B>, Error = LocalError>,
    S::Future: 'static,
    B: 'static,
{
    type Request = ServiceRequest;
    type Response = ServiceResponse<B>;
    type Error = LocalError;
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
    S: Service<Request = ServiceRequest, Response = ServiceResponse<B>, Error = LocalError>,
    S::Future: 'static,
    B: 'static,
{
    type Request = ServiceRequest;
    type Response = ServiceResponse<B>;
    type Error = LocalError;
    type Future = Box<dyn Future<Item = ServiceResponse<B>, Error = LocalError>>;

    fn poll_ready(&mut self) -> Poll<(), Self::Error> {
        self.service.poll_ready()
    }

    fn call(&mut self, req: ServiceRequest) -> Self::Future {
        println!("Hi from start. You requested: {}", req.path());

        Box::new(self.service.call(req).and_then(|res| {
            println!("Hi from response");
            Ok(res)
        }))
    }
}



#[cfg(test)]
mod tests {
    use super::*;
    //use crate::dua::DUA;
    //use crate::dua::extractor::actix::{LocalError};
    use actix_web::{test, web, http, App, HttpRequest, HttpResponse};
    use actix_web::dev::Service;
    use actix_web::http::{StatusCode};

    // supporting functions
    fn index_middleware_dua(_req: HttpRequest) -> HttpResponse {
        HttpResponse::Ok()
            .header(http::header::CONTENT_TYPE, "application/json")
            .body(r#"{"status":"Ok"}"#)
    }    

    // tests
    #[ignore]
    #[test]
    fn test_dua_ok() {
        let mut app = test::init_service(App::new().route("/", web::post().to(index_middleware_dua)));
        let req = test::TestRequest::post().uri("/")
            .header("content-type", "application/json")
            .header(DUA_HEADER, r#"[{"agreement_name":"billing","location":"www.dua.org/billing.pdf","agreed_dtm": 1553988607}]"#)
            .to_request();
        let resp = test::block_on(app.call(req)).unwrap();
        assert_eq!(resp.status(), StatusCode::OK);
    } 

    #[ignore]
    #[test]
    fn test_dua_missing() {
        let mut app = test::init_service(App::new().route("/", web::post().to(index_middleware_dua)));
        let req = test::TestRequest::post().uri("/")
            .header("content-type", "application/json")
            .to_request();
        let resp = test::block_on(app.call(req)).unwrap();
        assert_eq!(resp.status(), StatusCode::BAD_REQUEST);
    }     
}