extern crate pbd;
extern crate actix_web;

use pbd::dua::extractor::actix::*;
use pbd::dua::middleware::actix::*;
use actix_web::{web, http, App, HttpServer, HttpRequest, HttpResponse};

async fn index(duas: DUAs, _req: HttpRequest) -> HttpResponse  {
    for dua in duas.vec().iter() {
        println!("{:?}", dua);
    }
    
    HttpResponse::Ok()
        .header(http::header::CONTENT_TYPE, "plain/text")
        .body(r#"Hello World!"#)   
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    println!("Starting service on localhost:8088 ...");
    HttpServer::new(
        || App::new()
            .wrap(DUAEnforcer::default())
            .service(web::resource("/").to(index)))
    .bind("localhost:8088")
    .unwrap()
    .run()
    .await
}