extern crate actix_web;
extern crate pbd;

use actix_web::http::header::HeaderValue;
use actix_web::{http, web, App, HttpRequest, HttpResponse, HttpServer};
use log::warn;
use pbd::dua::error;
use pbd::dua::{DUA, DUA_HEADER};

fn duas_from_header_value(header_value: &HeaderValue) -> Result<Vec<DUA>, error::Error> {
    match header_value.to_str() {
        Ok(list) => {
            let docs = match json::parse(list) {
                Ok(valid) => valid,
                Err(_e) => {
                    // couldn't find the header
                    warn!("{}", error::Error::BadDUAFormat);
                    return Err(error::Error::BadDUAFormat);
                }
            };

            match docs.is_array() {
                true => {
                    let mut v = Vec::new();
                    for d in 0..docs.len() {
                        v.push(DUA::from_serialized(&docs[d].to_string()));
                    }
                    Ok(v)
                }
                false => {
                    // couldn't find the header
                    warn!("{}", error::Error::BadDUAFormat);
                    Err(error::Error::BadDUAFormat)
                }
            }
        }
        Err(_e) => {
            // couldn't find the header
            warn!("{}", error::Error::BadDUAFormat);
            Err(error::Error::BadDUAFormat)
        }
    }
}

async fn index(req: HttpRequest) -> HttpResponse {
    match req.headers().get(DUA_HEADER) {
        Some(u) => match duas_from_header_value(u) {
            Ok(duas) => {
                for dua in duas.iter() {
                    println!("{:?}", dua);
                }
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
                .body(format!("{}", error::Error::MissingDUA));
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
