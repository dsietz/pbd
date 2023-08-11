extern crate actix_web;
extern crate pbd;

use actix_web::{http, web, App, Error, HttpResponse, HttpServer};
use futures::StreamExt;
use pbd::dpi::DPI;

async fn index(mut body: web::Payload) -> Result<HttpResponse, Error> {
    let mut bytes = web::BytesMut::new();

    while let Some(item) = body.next().await {
        bytes.extend_from_slice(&item?);
    }

    let mut dpi = DPI::default();
    println!(
        "DPI Score: {}",
        dpi.inspect(String::from_utf8(bytes.to_vec()).unwrap())
    );

    return Ok(HttpResponse::Ok()
        .header(http::header::CONTENT_TYPE, "plain/text")
        .body(r#"Hello World!"#));
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Starting service on localhost:8088 ...");
    HttpServer::new(|| App::new().service(web::resource("/").route(web::post().to(index))))
        .bind("localhost:8088")
        .unwrap()
        .run()
        .await
}
