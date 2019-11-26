use actix_web::dev::Payload;
use actix_web::http::header::HeaderValue;
use actix_web::{FromRequest, HttpRequest};
use json::JsonValue;
use crate::DUA;
use crate::error::Error;

// 
// The Data Usage Agreement Extractor
// 

// DUA list
type DUAList = Vec<DUA>;

pub struct DUAs{
    list: DUAList,
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

    fn get_header_value(req: &HttpRequest) -> HeaderValue {
        match req.headers().get("Data-Usage-Agreement") {
            Some(u) => {
                u.clone()
            },
            None => {
                panic!("{}", Error::MissingDUA);
            },
        }
    }

    // Constructor
    pub fn from_request(req: &HttpRequest) -> DUAs{
        let lst = match DUAs::get_header_value(req).to_str() {
             Ok(list) => {
                let docs = match json::parse(list) {
                    Ok(valid) => valid,
                    Err(_e) => {
                        panic!("{}", Error::BadDUAFormat);
                    },
                };
                
                match docs.is_array() {
                    true => {
                        DUAs::value_to_vec(&docs)
                    },
                    false => {
                        panic!("{}", Error::BadDUAFormat);
                    },
                }
            },
            Err(_e) => {
                panic!("{}", Error::BadDUAFormat);
            }
        };

        DUAs {
            list: lst,
        }
    }

    // returns a Vector of DUA objects
    pub fn vec(&self) -> Vec<DUA> {
        self.list.clone()
    }
}

impl FromRequest for DUAs {
    type Config = ();
    type Future = Result<Self, Self::Error>;
    type Error = crate::error::Error;

    // convert request to future self
    fn from_request(req: &HttpRequest, _payload: &mut actix_web::dev::Payload) -> <Self as FromRequest>::Future {
        /*
        match DUAs::dua_from_httprequest(req) {
            Ok(list) => {
                Ok(list)
            },
            Err(err_str) => {
                Err(err_str).into()
            },
        }
        */
        //self.insert(DUA::from_serialized(r#"{ "agreement_name": "billing", "location": "www.dua.org/billing.pdf", "agreed_dtm": 1553988607 }"#))
        
        Ok(DUAs::from_request(req))
    }
}


// Need to wrap it as an extractor
/// see: https://github.com/actix/actix-web/blob/master/actix-session/src/lib.rs
/// Example : https://github.com/svartalf/actix-web-httpauth/blob/master/src/extractors/bearer.rs
///
/// Extracts the Author of the data from the actix_web::HttpRequest
/// 
/// # Arguments
/// 
/// * req: actix_web::HttpRequest - The HttpRequest object to parse.</br>
/// 
/// #Example
///
/// ```
/// extern crate pbd;
/// extern crate actix_web;
///
/// use pbd::extractor::actix::*;
/// use actix_web::{test, HttpRequest};
/// use actix_web::http::{header};
///
/// fn main() {
///    let req = test::TestRequest::with_header("content-type", "application/json")
///                 .header("Author", "John Doe")
///                 .to_http_request();
///    
///    println!("Author: {:?}", author_from_httprequest(req).unwrap());
/// }
/// ```
pub fn author_from_httprequest(req: HttpRequest) -> Result<String, String> {
    match req.headers().get("Author") {
        Some(u) => {
            match u.to_str() {
                Ok(s) => {
                    return Ok(s.to_string())
                },
                Err(_e) => {
                    return Err("Invalid Author header".to_string())
                }
            }
        },
        None => {
            return Err("Missing Author header".to_string()) 
        },
    };
}

/// Extracts the DUA object from the actix_web::HttpRequest
/// 
/// # Arguments
/// 
/// * req: actix_web::HttpRequest - The HttpRequest object to parse.</br>
/// 
/// #Example
///
/// ```
/// extern crate pbd;
/// extern crate actix_web;
///
/// use pbd::extractor::actix::*;
/// use actix_web::{test, HttpRequest};
/// use actix_web::http::{header};
///
/// fn main() {
///    let dua = r#"[{"agreement_name":"billing","location":"www.dua.org/billing.pdf","agreed_dtm":1553988607},{"agreement_name":"shipping","location":"www.dua.org/shipping.pdf","agreed_dtm":1553988607}]"#;
///    let req = test::TestRequest::with_header("content-type", "application/json")
///                 .header("Data-Usage-Agreement",dua)
///                 .to_http_request();
///    
///    println!("DUA: {:?}", dua_from_httprequest(req).unwrap());
/// }
/// ```
pub fn dua_from_httprequest(req: HttpRequest) -> Result<Vec<DUA>, String> {
    match req.headers().get("Data-Usage-Agreement") {
        Some(u) => {
            match u.to_str() {
                Ok(list) => {
                    let docs = match json::parse(list) {
                        Ok(valid) => valid,
                        Err(_e) => {
                            return Err("Invalid Data-Usage-Agreement header - Bad json".to_string())
                        },
                    };

                    match docs.is_array() {
                        true => {
                            let mut v = Vec::new();

                            for d in 0..docs.len() {
                                v.push(DUA::from_serialized(&docs[d].to_string()));
                            }                    
        
                            return Ok(v)
                        },
                        false => {
                            return Err("Invalid Data-Usage-Agreement header - Must be an array".to_string())
                        },
                    }
                },
                Err(_e) => {
                    return Err("Invalid Data-Usage-Agreement header".to_string())
                }
            }
        },
        None => {
            return Err("Missing Data-Usage-Agreement header".to_string())
        },
    };
}

#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::{test, HttpRequest};
    use actix_web::http::{header};

    #[test]
    fn test_dua_from_httprequest() {
        let dua = r#"[{"agreement_name":"billing","location":"www.dua.org/billing.pdf","agreed_dtm":1553988607},{"agreement_name":"shipping","location":"www.dua.org/shipping.pdf","agreed_dtm":1553988607}]"#;
        let req = test::TestRequest::with_header("content-type", "application/json")
            .header("Data-Usage-Agreement",dua)
            .to_http_request();
        
        match dua_from_httprequest(req) {
            Ok(dua) => {
                println!("{:?}", dua);
                assert!(true)
            },
            Err(_e) => assert!(false),
        }
    }
}