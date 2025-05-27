use std::error::Error;

use actix_files::Files;
use actix_web::{App, HttpServer, web};

use crate::{
    config::Config,
    infrastructure::keyclock_client::KeyclockClient,
    middleware::auth::connect_keyclock,
    presentation::{
        handlers::{
            auth_handler::{callback, login},
            protected_handlers::protected_routes,
        },
        shared::app_state::AppState,
    },
};

pub async fn server(config: Config) -> Result<(), Box<dyn Error>> {
    let address = format!("0.0.0.0:{}", config.port);
    println!("App address: {}", &address);

    let keyclock_client = KeyclockClient::new(config);

    let app_state = web::Data::new(AppState::new(keyclock_client));

    HttpServer::new(move || {
        let keycloak_auth = connect_keyclock();
        App::new()
            .app_data(app_state.clone())
            .service(Files::new("/", "./src/templates").index_file("callback.html")) // Указан правильный путь
            .service(login)
            .service(callback)
            .service(
                web::scope("/api/v1")
                    .wrap(keycloak_auth)
                    .configure(protected_routes),
            )
    })
    .bind(address)?
    .run()
    .await?;

    Ok(())
}
