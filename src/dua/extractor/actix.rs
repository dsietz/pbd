//! Information extractors for `actix-web`
use super::*;
use std::fmt;
use actix_web::{FromRequest, HttpRequest};
use json::JsonValue;
//use futures::future::{Ready};

// 
// The Data Usage Agreement Extractor
// 
pub type LocalError = super::error::Error;
// DUA list
type DUAList = Vec<DUA>;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DUAs{
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
        DUAs {
            list: Vec::new(),
        }
    }
    // Associated Function
    fn value_to_vec(docs: &JsonValue) -> Vec<DUA> {
        let mut v = Vec::new();
    
        for d in 0..docs.len() {
            v.push(DUA::from_serialized(&docs[d].to_string()));
        }                    
        v
    }
    // Constructor
    pub fn from_request(req: &HttpRequest) -> DUAs{
        let lst = match req.headers().get(DUA_HEADER) {
            Some(u) => {
                match u.to_str() {
                    Ok(list) => {
                        let docs = match json::parse(list) {
                            Ok(valid) => valid,
                            Err(_e) => {
                                // couldn't find the header, so return empty list of DUAs
                                warn!("{}", LocalError::BadDUAFormat);
                                return DUAs::new()
                            },
                        };
                    
                        match docs.is_array() {
                            true => {
                                DUAs::value_to_vec(&docs)
                            },
                            false => {
                                // couldn't find the header, so return empty list of DUAs
                                warn!("{}", LocalError::BadDUAFormat);
                                return DUAs::new()
                            },
                        }
                    },
                    Err(_e) => {
                        // couldn't find the header, so return empty list of DUAs
                        warn!("{}", LocalError::BadDUAFormat);
                        return DUAs::new()
                    },
                }
            },
            None => {
                // couldn't find the header, so return empty list of DUAs
                warn!("{}", LocalError::MissingDUA);
                return DUAs::new()
            },
        };
        DUAs {
            list: lst,
        }
    }
    // returns a Vector of DUA objects
    #[allow(dead_code)]
    pub fn vec(&self) -> Vec<DUA> {
        self.list.clone()
    }
}

impl FromRequest for DUAs {
    type Config = ();
    type Future = Result<Self, Self::Error>;
    type Error = LocalError;
    // convert request to future self
    fn from_request(req: &HttpRequest, _payload: &mut actix_web::dev::Payload) -> Self::Future {
        Ok(DUAs::from_request(req))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::{test, web, http, App, HttpRequest, HttpResponse};
    use actix_web::dev::Service;
    use actix_web::http::{StatusCode};

    // supporting functions
    fn index_extract_dua(duas: DUAs, _req: HttpRequest) -> HttpResponse {
        if duas.vec().len() > 0 {
            return HttpResponse::Ok()
                .header(http::header::CONTENT_TYPE, "application/json")
                .body(format!("{}", duas))
        } else {
            return HttpResponse::BadRequest()
                .header(http::header::CONTENT_TYPE, "application/json")
                .body(format!("{}", LocalError::BadDUA))
        }
    }

    // tests
    #[test]
    fn test_http_header_name() {
        assert_eq!(DUA_HEADER, "Data-Usage-Agreement");
    }

    #[test]
    fn test_dua_extractor_good() {
        let mut app = test::init_service(App::new().route("/", web::get().to(index_extract_dua)));
        let req = test::TestRequest::get().uri("/")
            .header("content-type", "application/json")
            .header(DUA_HEADER, r#"[{"agreement_name":"billing","location":"www.dua.org/billing.pdf","agreed_dtm": 1553988607}]"#)
            .to_request();
        let resp = test::block_on(app.call(req)).unwrap();
        assert_eq!(resp.status(), StatusCode::OK);
    }
    #[test]
    fn test_dua_extractor_missing() {
        let mut app = test::init_service(App::new().route("/", web::get().to(index_extract_dua)));
        let req = test::TestRequest::get().uri("/")
            .header("content-type", "application/json")
            .to_request();
        let resp = test::call_service(&mut app, req);
        assert_eq!(resp.status(), StatusCode::BAD_REQUEST);
        // read response
        let bdy = test::read_body(resp);
        assert_eq!(&bdy[..], actix_web::web::Bytes::from_static(b"Malformed or missing one or more Data Usage Agreements"));
    }
}