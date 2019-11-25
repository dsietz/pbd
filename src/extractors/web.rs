use actix_web::{HttpRequest};
use crate::{DUA};

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
/// use pbd::extractors::web::*;
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
/// use pbd::extractors::web::*;
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