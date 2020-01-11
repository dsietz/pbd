
use super::*;
use crate::dtc::Tracker;
use crate::dtc::extractor::actix::{TrackerHeader};
use actix_web::{Error, HttpResponse};
use actix_web::dev::{ServiceRequest, ServiceResponse};
use actix_service::{Service, Transform};
use futures::future::{ok, Either, FutureResult};
use futures::{Poll};
use reqwest::StatusCode;
use rayon::prelude::*;

#[derive(Clone)]
pub struct DTCEnforcer{
    validation_level: u8,
}

impl DTCEnforcer {
    pub fn new(level: u8) -> Self {
        Self { 
            validation_level: level 
        }
    }

    pub fn set_validation(&mut self, level: u8) {
        self.validation_level = level;
    }
}

impl Default for DTCEnforcer {
    fn default() -> DTCEnforcer {
        DTCEnforcer {
            validation_level: 1
        }
    }
}

// `B` - type of response's body
impl<S, B> Transform<S> for DTCEnforcer
where
    S: Service<Request = ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
{
    type Request = ServiceRequest;
    type Response = ServiceResponse<B>;
    type Error = Error;
    type InitError = ();
    type Transform = DTCEnforcerMiddleware<S>;
    type Future = FutureResult<Self::Transform, Self::InitError>;

    fn new_transform(&self, service: S) -> Self::Future {
        ok(DTCEnforcerMiddleware { 
            service,
            validation_level: self.validation_level 
        })
    }
}


impl<S, B> Service for DTCEnforcerMiddleware<S>
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
        debug!("VALIDATION LEVEL: {}", self.validation_level);

        if self.validation_level == VALIDATION_NONE {
            return Either::A(self.service.call(req)) 
        }

        match  req.headers().get(DTC_HEADER) {
            Some(header_value) => {
                return Either::A(self.service.call(req))
            },
            None => {
                return Either::B(ok(req.into_response(
                    HttpResponse::BadRequest()
                        .finish()
                     .into_body(),
                )))
            },
        }

                // Level 1 Validation: Check to see if there are DTC is provided
/*
                match DUAs::tracker_from_header_value(header_value) {

                }
                let mut valid_ind: bool = false;

                // Level 1 Validation: Check to see if there are DTC is provided
                if self.validation_level >= VALIDATION_LOW && duas.vec().len() > 0 {
                    valid_ind = true;
                }
                
                // Level 2 Validation: Check to see if the DUAs provided are valid ones
                if valid_ind == true && self.validation_level >= VALIDATION_HIGH {
                    let checks: usize = duas.vec().par_iter()
                        .map(|d|
                            match reqwest::get(&d.location.clone()) {
                                Ok(rsp) => {
                                    if rsp.status() == StatusCode::OK { 
                                        1
                                    } 
                                    else {
                                        info!("{}", format!("Invalid DUA: {}", d.location.clone()));
                                        0
                                    }
                                },
                                Err(_err) => {
                                    info!("{}", format!("Invalid DUA: {}", d.location.clone()));
                                    0
                                },
                            }
                        )
                        .sum();
                
                    if duas.vec().len() == checks {
                        valid_ind = true;
                    } 
                    else {
                        valid_ind = false;
                    }
                }

                if valid_ind == true {
                    return Either::A(self.service.call(req))
                }
                else {
                    return Either::B(ok(req.into_response(
                        HttpResponse::BadRequest()
                            .finish()
                         .into_body(),
                    )))
                }
            },
            None => {
                return Either::B(ok(req.into_response(
                    HttpResponse::BadRequest()
                        .finish()
                        .into_body(),
                )))
            },
*/
    }
}

pub struct DTCEnforcerMiddleware<S> {
    service: S,
    validation_level: u8,
}


#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::{test, web, http, App, HttpRequest, HttpResponse};
    use actix_web::dev::Service;
    use actix_web::http::{StatusCode};

    // supporting functions
    fn index_middleware_dtc(_req: HttpRequest) -> HttpResponse {
        HttpResponse::Ok()
            .header(http::header::CONTENT_TYPE, "application/json")
            .body(r#"{"status":"Ok"}"#)
    }    

    #[test]
    fn test_add_middleware() {
        let _app = App::new()
            .wrap(DTCEnforcer::default())
            .service(
                web::resource("/")
                    .route(web::get().to(index_middleware_dtc))
            );

        assert!(true);
    }
}