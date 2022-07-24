extern crate actix_web;
extern crate pbd;

use actix_web::{web, App, HttpRequest, HttpResponse, HttpServer};
use actix_web::http::header::ContentType;
use pbd::dua::extractor::actix::*;
use pbd::dua::middleware::actix::*;

async fn index(duas: DUAs, _req: HttpRequest) -> HttpResponse {
    for dua in duas.vec().iter() {
        println!("{:?}", dua);
    }

    HttpResponse::Ok()
        .insert_header(ContentType::plaintext())
        .body(r#"Hello World!"#)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Starting service on localhost:8088 ...");
    HttpServer::new(|| {
        App::new()
            .wrap(DUAEnforcer::default())
            .service(web::resource("/").to(index))
    })
    .bind("localhost:8088")
    .unwrap()
    .run()
    .await
}
