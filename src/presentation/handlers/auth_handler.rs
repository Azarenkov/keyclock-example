use actix_web::{HttpResponse, Responder, cookie::Cookie, get, http::header, web};
use serde::Deserialize;
use urlencoding::encode;

use crate::presentation::shared::app_state::AppState;

#[derive(Deserialize, Debug)]
struct AuthCallback {
    code: String,
    session_state: String,
    iss: Option<String>,
}

#[get("/login")]
pub async fn login(app_state: web::Data<AppState>) -> impl Responder {
    let response_type = "code";
    let scope = "openid";

    let login_url = format!(
        "{}/realms/{}/protocol/openid-connect/auth?client_id={}&redirect_uri={}&response_type={}&scope={}",
        app_state.keyclock_client.config.keyclock_base_url,
        app_state.keyclock_client.config.keyclock_realm,
        app_state.keyclock_client.config.keyclock_client_id,
        encode(&app_state.keyclock_client.config.redirect_uri),
        response_type,
        scope
    );

    HttpResponse::Found()
        .insert_header((actix_web::http::header::LOCATION, login_url))
        .finish()
}

#[get("/callback")]
async fn callback(
    app_state: web::Data<AppState>,
    query: web::Query<AuthCallback>,
) -> impl Responder {
    let keyclock_request = app_state
        .keyclock_client
        .exchange_code_to_token(&query.code)
        .await;

    match keyclock_request {
        Ok(tokens) => {
            let mut response = HttpResponse::Found()
                .append_header((header::LOCATION, "/callback.html"))
                .finish();

            let cookie = Cookie::build("access_token", tokens.access_token.clone())
                .http_only(true)
                .secure(true)
                .path("/")
                .finish();

            response.add_cookie(&cookie).unwrap();

            response
        }
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}
