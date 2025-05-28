use actix_web::{HttpRequest, HttpResponse, Responder, get, http::header, web};

use crate::presentation::shared::app_state::AppState;

pub fn protected_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(protected).service(logout);
}

#[get("/protected")]
async fn protected() -> impl Responder {
    HttpResponse::Ok().body("Доступ к защищённому ресурсу получен!")
}

#[get("/logout")]
pub async fn logout(app_state: web::Data<AppState>, req: HttpRequest) -> impl Responder {
    let auth_header = req.headers().get(header::AUTHORIZATION);

    match auth_header {
        Some(access_token) => {
            let access_token = access_token.to_str().unwrap_or("");

            if access_token.is_empty() {
                return HttpResponse::BadRequest().body("Access token is empty");
            }

            match app_state
                .keyclock_client
                .send_logout_request(access_token)
                .await
            {
                Ok(_) => HttpResponse::Found()
                    .insert_header((header::LOCATION, "/login"))
                    .finish(),
                Err(e) => HttpResponse::InternalServerError().body(format!("Logout failed: {}", e)),
            }
        }
        None => HttpResponse::BadRequest().body("Authorization header is missing"),
    }
}
