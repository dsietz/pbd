extern crate pbd;
extern crate actix_web;

use pbd::dsg::{PrivacyGuard,PrivacySecurityGuard, TransferSet};
use actix_web::{web, http, App, HttpServer, HttpRequest, HttpResponse};
use std::io::prelude::*;
use std::fs::File;

fn get_priv_pem() -> Vec<u8> {
    let mut f = File::open("./tests/keys/priv-key.pem").unwrap();
    let mut priv_pem = Vec::new();
    f.read_to_end(&mut priv_pem).unwrap();

    priv_pem
}

fn index(transset: TransferSet, _req: HttpRequest) -> HttpResponse  {
/*
    let guard = PrivacyGuard {};
    let priv_key = get_priv_pem();
    
    // extract the data form the TransferSet
    match guard.data_from_tranfer(priv_key, transset) {
        Ok(msg) => {
            println!("{}", String::from_utf8(msg).unwrap());
            
            return HttpResponse::Ok()
                .header(http::header::CONTENT_TYPE, "plain/text")
                .body(r#"Hello World!"#);
        },
        Err(err) => {
            println!("{}", err);
            return HttpResponse::BadRequest()
                .header(http::header::CONTENT_TYPE, "plain/text")
                .body(format!("{}", err));
        }
    }
*/

    println!("{}", transset);
    HttpResponse::Ok()
    .header(http::header::CONTENT_TYPE, "plain/text")
    .body(r#"Hello World!"#)

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