// use std::io::{ stdin, Stdin };
// use crate::blades::BladesError;
use actix_web::{HttpResponse};
// use futures::StreamExt;

// pub fn converse(stdin: &mut Stdin) -> Result<(), BladesError> {
//     let mut buffer = String::new();
//     stdin.read_line(&mut buffer)?;
//     let answer = select_answer(buffer.trim_end());
//     println!("{}", answer.to_string());
//     if answer.trim_end().eq("blades!") {
//         return Err(BladesError::new("blades!"));
//     }
//     Ok(())
// }

// pub fn select_answer(input: &str) -> String {
//     if input.contains("?") { return "poppin'".to_string(); }
//     match input {
//         "bye!" => "blades!".to_string(),
//         _ => "shmoopie doop".to_string()
//     }
// }

pub fn index() -> HttpResponse {
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(include_str!("../static/form.html"))
}

async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}
