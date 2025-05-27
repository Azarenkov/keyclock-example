use actix_web::{HttpResponse, Responder, get, web};

pub fn protected_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(protected);
}

#[get("/protected")]
async fn protected() -> impl Responder {
    HttpResponse::Ok().body("Доступ к защищённому ресурсу получен!")
}
