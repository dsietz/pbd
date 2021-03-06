extern crate actix_web;
extern crate pbd;

use actix_web::{http, web, App, Error, HttpResponse, HttpServer};
use futures::StreamExt;
use pbd::dsg::{PrivacyGuard, PrivacySecurityGuard, TransferSet};
use std::fs::File;
use std::io::prelude::*;

fn get_priv_pem() -> Vec<u8> {
    let mut f = File::open("./tests/keys/priv-key.pem").unwrap();
    let mut priv_pem = Vec::new();
    f.read_to_end(&mut priv_pem).unwrap();

    priv_pem
}

/// extract binary data from request
async fn index(mut body: web::Payload) -> Result<HttpResponse, Error> {
    let mut bytes = web::BytesMut::new();

    while let Some(item) = body.next().await {
        bytes.extend_from_slice(&item?);
    }

    let transset = match TransferSet::from_serialized(&String::from_utf8(bytes.to_vec()).unwrap()) {
        Ok(ts) => ts,
        Err(e) => {
            return Ok(HttpResponse::BadRequest()
                .header(http::header::CONTENT_TYPE, "plain/text")
                .body(format!("{}", e)))
        }
    };

    let guard = PrivacyGuard {};
    let priv_pem = get_priv_pem();

    match guard.data_from_tranfer(priv_pem, transset) {
        Ok(msg) => {
            println!("Message Received: {}", String::from_utf8(msg).unwrap());
            return Ok(HttpResponse::Ok()
                .header(http::header::CONTENT_TYPE, "plain/text")
                .body(r#"Hello World!"#));
        }
        Err(e) => {
            return Ok(HttpResponse::BadRequest()
                .header(http::header::CONTENT_TYPE, "plain/text")
                .body(format!("{}", e)))
        }
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Starting service on localhost:8088 ...");
    HttpServer::new(|| App::new().service(web::resource("/").route(web::get().to(index))))
        .bind("localhost:8088")
        .unwrap()
        .run()
        .await
}
