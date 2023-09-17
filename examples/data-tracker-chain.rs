extern crate actix_web;
extern crate pbd;

use actix_web::http::header::HeaderValue;
use actix_web::{http, web, App, HttpRequest, HttpResponse, HttpServer};
use log::warn;
use pbd::dtc::error;
use pbd::dtc::error::Error;
use pbd::dtc::{Tracker, DTC_HEADER};

fn tracker_from_header_value(header_value: &HeaderValue) -> Result<Tracker, error::Error> {
    match base64::decode(header_value.to_str().unwrap()) {
        Ok(b) => {
            let chain = String::from_utf8(b).unwrap();

            match Tracker::from_serialized(&chain) {
                Ok(t) => Ok(t),
                Err(e) => {
                    warn!("{}", e);
                    Err(e)
                }
            }
        }
        Err(_e) => {
            warn!("{}", Error::Base64DTC);
            Err(Error::Base64DTC)
        }
    }
}

async fn index(req: HttpRequest) -> HttpResponse {
    match req.headers().get(DTC_HEADER) {
        Some(u) => match tracker_from_header_value(u) {
            Ok(dtc) => {
                println!("{}", dtc.serialize());
            }
            Err(e) => {
                return HttpResponse::BadRequest()
                    .header(http::header::CONTENT_TYPE, "plain/text")
                    .body(format!("{}", e))
            }
        },
        None => {
            // couldn't find the header
            return HttpResponse::BadRequest()
                .header(http::header::CONTENT_TYPE, "plain/text")
                .body(format!("{}", error::Error::MissingDTC));
        }
    }

    HttpResponse::Ok()
        .header(http::header::CONTENT_TYPE, "plain/text")
        .body(r#"Hello World!"#)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Starting service on localhost:8088 ...");
    HttpServer::new(|| App::new().service(web::resource("/").to(index)))
        .bind("localhost:8088")
        .unwrap()
        .run()
        .await
}
