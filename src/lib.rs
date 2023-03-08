use actix_web::{dev::Server, web, App, HttpRequest, HttpResponse, HttpServer, Responder};
use std::net::TcpListener;

// All webb.get() implement a guard Route::new().guard(guard::Get()) the reques should be passed down if an only if

async fn health_check() -> impl Responder {
    //(req: HttpRequest) not needed since we dont anything with the argument
    HttpResponse::Ok().finish()
}

#[derive(serde::Deserialize)]
struct FormData {
    email: String,
    name: String,
}
async fn subscribe(_form: web::Form<FormData>) -> impl Responder {
    //(req: HttpRequest) not needed since we dont anything with the argument
    HttpResponse::Ok().finish()
}

// We need to mark `run` as public.
// It is no longer a binary entrypoint, therefore we can mark it as async
// without having to use any proc-macro incantation.
//no more need to be async
// Accept any port
pub fn run(listener: TcpListener) -> Result<Server, std::io::Error> {
    let server = HttpServer::new(|| {
        App::new()
            .route("/health_check", web::get().to(health_check))
            .route("/subscriptions", web::post().to(subscribe))
    })
    .listen(listener)?
    .run();

    Ok(server)
}
