//will be used for user authentication

use actix_web::{web, HttpResponse, Responder};

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/login").route(web::post().to(login)));
}

async fn login() -> impl Responder {
    // Dummy login implementation
    HttpResponse::Ok().body("Login successful")
}
