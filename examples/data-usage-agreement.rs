extern crate pbd;
extern crate actix_web;

use pbd::dua::middleware::actix::*;
use actix_web::{web, http, App, HttpServer, HttpRequest, HttpResponse};

fn index(_req: HttpRequest) -> HttpResponse  {
    HttpResponse::Ok()
        .header(http::header::CONTENT_TYPE, "plain/text")
        .body(r#"Hello World!"#)   
}

fn main() {
    HttpServer::new(
        || App::new()
            .wrap(DUAEnforcer::default())
            .service(web::resource("/").to(index)))
    .bind("localhost:8088")
    .unwrap()
    .run()
    .unwrap();
}