extern crate pbd;
extern crate actix_web;

use pbd::dsg::{PrivacyGuard, PrivacySecurityGuard, TransferSet};
use actix_web::{web, http, Error, App, HttpServer, HttpRequest, HttpResponse};
use futures::{Future, Stream};
use std::io::prelude::*;
use std::fs::File;

fn get_priv_pem() -> Vec<u8> {
    let mut f = File::open("./tests/keys/priv-key.pem").unwrap();
    let mut priv_pem = Vec::new();
    f.read_to_end(&mut priv_pem).unwrap();

    priv_pem
}

fn index(body: web::Payload, _req: HttpRequest) -> impl Future<Item = HttpResponse, Error = Error>  {
    body.map_err(Error::from)
    .fold(web::BytesMut::new(), move |mut body, chunk| {
        body.extend_from_slice(&chunk);
        Ok::<_, Error>(body)
     })
     .and_then(|body| {
        let transset = match TransferSet::from_serialized(&String::from_utf8(body.to_vec()).unwrap()) {
            Ok(ts) => ts,
            Err(e) => {
                return HttpResponse::BadRequest()
                        .header(http::header::CONTENT_TYPE, "plain/text")
                        .body(format!("{}",e))
            },
        } ;

        let guard = PrivacyGuard {};
        let priv_pem = get_priv_pem();

        match guard.data_from_tranfer(priv_pem,transset) {
            Ok(msg) => {
                println!("Message Received: {}", String::from_utf8(msg).unwrap());

                HttpResponse::Ok()
                    .header(http::header::CONTENT_TYPE, "plain/text")
                    .body(r#"Hello World!"#)
            },
            Err(e) => {
                return HttpResponse::BadRequest()
                        .header(http::header::CONTENT_TYPE, "plain/text")
                        .body(format!("{}",e))
            }
        }
     })
}

fn main() {
    HttpServer::new(
        || App::new()
            .service(
                web::resource("/")
                    .route(web::get().to_async(index))
            )
    )
    .bind("localhost:8088")
    .unwrap()
    .run()
    .unwrap();
}