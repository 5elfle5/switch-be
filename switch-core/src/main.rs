mod handlers;
use actix_web::{http, get, post, web, App, HttpResponse, HttpServer, Responder};
use actix_cors::Cors;
use serde::{Serialize, Deserialize};
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
            .service(hello)
            .service(say)
            .route("/hey", web::get().to(manual_hello))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body(select_answer("shmoopie doop"))
}

#[post("/say")]
async fn say(req_body: String) -> impl Responder {
    web::Json(Reply::new(select_answer(req_body.as_str())))
}

#[derive(Serialize, Deserialize)]
struct Reply {
    text: String
}

impl Reply {
    pub fn new(text_: String) -> Reply {
        Reply {
            text: text_
        }
    }
}

async fn manual_hello() -> impl Responder {
    web::Json(Reply::new(select_answer("shmoopie doop")))
}