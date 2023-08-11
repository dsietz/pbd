extern crate actix_web;
extern crate pbd;

use actix_web::{http, web, App, HttpRequest, HttpResponse, HttpServer};
use pbd::dtc::middleware::actix::*;
use pbd::dtc::Tracker;

async fn index(tracker: Tracker, _req: HttpRequest) -> HttpResponse {
    println!("{}", tracker.serialize());

    HttpResponse::Ok()
        .header(http::header::CONTENT_TYPE, "plain/text")
        .body(r#"Hello World!"#)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Starting service on localhost:8088 ...");
    HttpServer::new(|| {
        App::new()
            .wrap(DTCEnforcer::default())
            .service(web::resource("/").to(index))
    })
    .bind("localhost:8088")
    .unwrap()
    .run()
    .await
}
