use std::cell::RefCell;
use std::rc::Rc;
use actix_web::{FromRequest, HttpRequest};
use actix_web::dev::{Extensions};
use crate::{DUA};

// Need to wrap it as an extractor
/// see: https://github.com/actix/actix-web/blob/master/actix-session/src/lib.rs
struct AuthorInner {
    name: String,
}

pub struct Author(Rc<RefCell<AuthorInner>>);

pub trait DataAuthor {
    fn get_author(&mut self) -> Author;
 }
 
 impl DataAuthor for HttpRequest {
    fn get_author(&mut self) -> Author {
        Author::get_author(&mut *self.extensions_mut())
    }
}

impl Author {
    pub fn get_author(extensions: &mut Extensions) -> Author {
        if let Some(s_impl) = extensions.get::<Rc<RefCell<AuthorInner>>>() {
            return Author(Rc::clone(&s_impl));
        }
/*
        let inner = Rc::new(RefCell::new(AuthorInner::default()));
        extensions.insert(inner.clone());
        Author(inner)
*/        
    }
}
/*
impl FromRequest for Author {
    type Error = Error;
    type Future = Ready<Result<Session, Error>>;
    type Config = ();

    #[inline]
    fn from_request(req: &HttpRequest, _: &mut Payload) -> Self::Future {
        ok(author_from_httprequest(&mut *req.extensions_mut()))
    }
}
*/
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
/// use pbd::extractors::actix::*;
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
/// use pbd::extractors::actix::*;
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