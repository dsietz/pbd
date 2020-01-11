extern crate pbd;
extern crate actix_web;

use pbd::dtc::Tracker;
use actix_web::{web, http, App, HttpServer, HttpRequest, HttpResponse};

fn index(tracker: Tracker, _req: HttpRequest) -> HttpResponse  {
    println!("{}", tracker.serialize());
    
    HttpResponse::Ok()
        .header(http::header::CONTENT_TYPE, "plain/text")
        .body(r#"Hello World!"#)   
}

fn main() {
    HttpServer::new(
        || App::new()
            .service(web::resource("/").to(index)))
    .bind("localhost:8088")
    .unwrap()
    .run()
    .unwrap();
}