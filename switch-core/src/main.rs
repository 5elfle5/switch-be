mod handlers;
use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use crate::handlers::{select_answer};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(hello)
            .service(echo)
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
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(select_answer(req_body.as_str()))
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body(select_answer("shmoopie doop"))
}