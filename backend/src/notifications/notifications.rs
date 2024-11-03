// will be used for notifications, code not implemented yet

use actix_web::{web, HttpResponse, Responder};

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/notifications").route(web::get().to(get_notifications)));
}

async fn get_notifications() -> impl Responder {
    // Dummy notification logic
    HttpResponse::Ok().body("Notifications endpoint")
}
