use actix_web::{HttpRequest, HttpResponse, Responder, get, http::header, post, web};

use crate::presentation::shared::app_state::AppState;

pub fn protected_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(protected).service(logout);
}

#[get("/protected")]
async fn protected() -> impl Responder {
    HttpResponse::Ok().body("Доступ к защищённому ресурсу получен!")
}

#[post("/logout")]
pub async fn logout(app_state: web::Data<AppState>, req: HttpRequest) -> impl Responder {
    let auth_header = req.headers().get(header::AUTHORIZATION);

    match auth_header {
        Some(auth_value) => {
            let auth_str = auth_value.to_str().unwrap_or("");

            let access_token = if let Some(token) = auth_str.strip_prefix("Bearer ") {
                token
            } else {
                return HttpResponse::BadRequest()
                    .body("Invalid Authorization header format. Expected 'Bearer <token>'");
            };

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
