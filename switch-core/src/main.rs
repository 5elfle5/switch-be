mod handlers;
mod domain;

use actix_web::{http, post, web, App, HttpServer, Responder};
use actix_cors::Cors;
use crate::handlers::{select_answer};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        let cors = Cors::default()
              .allowed_origin("http://localhost:3000")
              .allowed_origin_fn(|origin, _req_head| {
                  origin.as_bytes().ends_with(b"")
              })
              .allowed_methods(vec!["GET", "POST"])
              .allowed_headers(vec![http::header::AUTHORIZATION, http::header::ACCEPT])
              .allowed_header(http::header::CONTENT_TYPE)
              .max_age(3600);

        App::new()
            .wrap(cors)
            .service(speak)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}

#[post("/speak")]
async fn speak(req_body: String) -> impl Responder {
    web::Json(domain::Reply::new(select_answer(req_body.as_str())))
}
