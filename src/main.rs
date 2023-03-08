use actix_web::{web, App, HttpRequest, HttpServer, Responder};

// All webb.get() implement a guard Route::new().guard(guard::Get()) the reques should be passed down if an only if

async fn greet(req: HttpRequest) -> impl Responder {
    let name = req.match_info().get("name").unwrap_or("World");
    format!("Hello {}!", name)
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(greet))
            .route("/{name}", web::get().to(greet)) //route path with templating
    })
    .bind("127.0.0.1:8000")?
    .run()
    .await
}
