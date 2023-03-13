use actix_web::{dev::Server, web, App, HttpRequest, HttpResponse, HttpServer, Responder};
use std::net::TcpListener;

#[derive(serde::Deserialize)]
pub struct FormData {
    email: String,
    name: String,
}
pub async fn subscribe(_form: web::Form<FormData>) -> impl Responder {
    //(req: HttpRequest) not needed since we dont anything with the argument
    HttpResponse::Ok().finish()
}
