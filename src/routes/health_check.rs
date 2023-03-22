use actix_web::{HttpResponse, Responder};

// All webb.get() implement a guard Route::new().guard(guard::Get()) the reques should be passed down if an only if

pub async fn health_check() -> impl Responder {
    //(req: HttpRequest) not needed since we dont anything with the argument
    HttpResponse::Ok().finish()
}
