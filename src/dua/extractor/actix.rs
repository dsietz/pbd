//! The DUA Extractor is a simple way to pull the list of DUAs from the HTTP Request.
//!
//! ---
//!
//! Example
//! ```rust,no_run
//! extern crate pbd;
//! extern crate actix_web;
//!
//! use pbd::dua::extractor::actix::*;
//! use actix_web::{web, http, App, HttpRequest, HttpResponse, HttpServer};
//!
//! async fn index(duas: DUAs, _req: HttpRequest) -> HttpResponse {
//!     for dua in duas.vec().iter() {
//!         println!("{:?}", dua);
//!     }
//!         
//!     HttpResponse::Ok()
//!         .header(http::header::CONTENT_TYPE, "application/json")
//!         .body(format!("{}", duas))
//! }
//!
//! #[actix_rt::main]
//! async fn main() -> std::io::Result<()> {
//!     HttpServer::new(|| App::new().service(
//!         web::resource("/").to(index))
//!     )
//!         .bind("127.0.0.1:8080")?
//!         .run()
//!         .await
//! }
//! ```

use super::*;
use actix_web::http::header::HeaderValue;
use actix_web::{FromRequest, HttpRequest};
use futures::future::{ok, Ready};
use json::JsonValue;
use std::fmt;

//
// The Data Usage Agreement Extractor
//
pub type LocalError = super::error::Error;
// DUA list
type DUAList = Vec<DUA>;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DUAs {
    list: DUAList,
}

impl fmt::Display for DUAs {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", serde_json::to_string(&self).unwrap())
    }
}

impl DUAs {
    // Constructor
    pub fn new() -> DUAs {
        DUAs { list: Vec::new() }
    }
    // Associated Function
    fn value_to_vec(docs: &JsonValue) -> Vec<DUA> {
        let mut v = Vec::new();

        for d in 0..docs.len() {
            v.push(DUA::from_serialized(&docs[d].to_string()));
        }
        v
    }

    pub fn duas_from_header_value(header_value: &HeaderValue) -> DUAs {
        match header_value.to_str() {
            Ok(list) => {
                let docs = match json::parse(list) {
                    Ok(valid) => valid,
                    Err(_e) => {
                        // couldn't find the header, so return empty list of DUAs
                        warn!("{}", LocalError::BadDUAFormat);
                        return DUAs::new();
                    }
                };

                match docs.is_array() {
                    true => DUAs {
                        list: DUAs::value_to_vec(&docs),
                    },
                    false => {
                        // couldn't find the header, so return empty list of DUAs
                        warn!("{}", LocalError::BadDUAFormat);
                        DUAs::new()
                    }
                }
            }
            Err(_e) => {
                // couldn't find the header, so return empty list of DUAs
                warn!("{}", LocalError::BadDUAFormat);
                DUAs::new()
            }
        }
    }

    // Constructor
    pub fn from_request(req: &HttpRequest) -> DUAs {
        match req.headers().get(DUA_HEADER) {
            Some(u) => DUAs::duas_from_header_value(u),
            None => {
                // couldn't find the header, so return empty list of DUAs
                warn!("{}", LocalError::MissingDUA);
                DUAs::new()
            }
        }
    }

    // returns a Vector of DUA objects
    #[allow(dead_code)]
    pub fn vec(&self) -> Vec<DUA> {
        self.list.clone()
    }
}

impl Default for DUAs {
    fn default() -> Self {
        Self::new()
    }
}

impl FromRequest for DUAs {
    // type Config = ();
    type Future = Ready<Result<Self, Self::Error>>;
    type Error = LocalError;
    // convert request to future self
    fn from_request(req: &HttpRequest, _payload: &mut actix_web::dev::Payload) -> Self::Future {
        ok(DUAs::from_request(req))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::http::StatusCode;
    use actix_web::http::header::ContentType;
    use actix_web::{test, web, App, HttpRequest, HttpResponse};

    // supporting functions
    async fn index_extract_dua(duas: DUAs, _req: HttpRequest) -> HttpResponse {
        if duas.vec().len() > 0 {
            return HttpResponse::Ok()
                .insert_header(ContentType::json())
                .body(format!("{}", duas));
        } else {
            return HttpResponse::BadRequest()
            .insert_header(ContentType::json())
                .body(format!("{}", LocalError::BadDUA));
        }
    }

    // tests
    #[test]
    async fn test_http_header_name() {
        assert_eq!(DUA_HEADER, "Data-Usage-Agreement");
    }

    #[actix_rt::test]
    async fn test_dua_extractor_good() {
        let mut app =
            test::init_service(App::new().route("/", web::get().to(index_extract_dua))).await;
        let req = test::TestRequest::get().uri("/")
            .insert_header(ContentType::json())
            .insert_header((DUA_HEADER, r#"[{"agreement_name":"billing","location":"www.dua.org/billing.pdf","agreed_dtm": 1553988607}]"#))
            .to_request();
        let resp = test::call_service(&mut app, req).await;
        assert_eq!(resp.status(), StatusCode::OK);
    }

    #[actix_rt::test]
    async fn test_dua_extractor_bad() {
        let mut app =
            test::init_service(App::new().route("/", web::get().to(index_extract_dua))).await;
        let req = test::TestRequest::get().uri("/")
            .insert_header(ContentType::json())
            .insert_header(
                (DUA_HEADER, 
                r#"[{"agreement_name":"billing""location":"www.dua.org/billing.pdf","agreed_dtm": 1553988607}]"#)
            )
            .to_request();
        let resp = test::call_service(&mut app, req).await;
        assert_eq!(resp.status(), StatusCode::BAD_REQUEST);
    }

    #[actix_rt::test]
    async fn test_dua_extractor_missing() {
        let mut app =
            test::init_service(App::new().route("/", web::get().to(index_extract_dua))).await;
        let req = test::TestRequest::get()
            .uri("/")
            .insert_header(ContentType::json())
            .to_request();
        let resp = test::call_service(&mut app, req).await;
        assert_eq!(resp.status(), StatusCode::BAD_REQUEST);
        // read response
        let body = test::read_body(resp).await;
        assert_eq!(
            body,
            actix_web::web::Bytes::from_static(
                b"Malformed or missing one or more Data Usage Agreements"
            )
        );
    }
}
