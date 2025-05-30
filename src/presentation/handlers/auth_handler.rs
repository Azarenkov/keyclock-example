use actix_web::{HttpResponse, Responder, get, web};
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
            let access_token =
                actix_web::cookie::Cookie::build("access_token", &tokens.access_token)
                    .path("/")
                    .http_only(false)
                    .finish();

            let template = include_str!("../../templates/callback.html");
            let html = template.replace("{{TOKEN}}", &tokens.access_token);

            HttpResponse::Ok()
                .cookie(access_token)
                .content_type("text/html; charset=UTF-8")
                .body(html)
        }
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}
