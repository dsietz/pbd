use super::*;

// Middleware for checking Data Usage Agreement
///
/// If there is no `Data Usage Agreement` in the headers (use pbd::dua::DUA_HEADER),
/// the middleware will respond with a BadRequest status code.
///
///



#[cfg(test)]
mod tests {
    use super::*;
    //use crate::dua::DUA;
    //use crate::dua::extractor::actix::{LocalError};
    use actix_web::{test, web, http, App, HttpRequest, HttpResponse};
    use actix_web::dev::Service;
    use actix_web::http::{StatusCode};

    // supporting functions
    fn index_middleware_dua(_req: HttpRequest) -> HttpResponse {
        HttpResponse::Ok()
            .header(http::header::CONTENT_TYPE, "application/json")
            .body(r#"{"status":"Ok"}"#)
    }    

    // tests
    #[test]
    fn test_dua_ok() {
        let mut app = test::init_service(App::new().route("/", web::post().to(index_middleware_dua)));
        let req = test::TestRequest::post().uri("/")
            .header("content-type", "application/json")
            .header(DUA_HEADER, r#"[{"agreement_name":"billing","location":"www.dua.org/billing.pdf","agreed_dtm": 1553988607}]"#)
            .to_request();
        let resp = test::block_on(app.call(req)).unwrap();
        assert_eq!(resp.status(), StatusCode::OK);
    } 
    
    #[ignore]
    #[test]
    fn test_dua_missing() {
        let mut app = test::init_service(App::new().route("/", web::post().to(index_middleware_dua)));
        let req = test::TestRequest::post().uri("/")
            .header("content-type", "application/json")
            .to_request();
        let resp = test::block_on(app.call(req)).unwrap();
        assert_eq!(resp.status(), StatusCode::BAD_REQUEST);
    }     
}