use config::Config;
use dotenv::dotenv;
use infrastructure::app_setup::server;
use std::error::Error;

mod config;
mod infrastructure;
mod middleware;
mod presentation;

#[actix_web::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenv().ok();

    let config = Config::default();

    server(config).await?;
    Ok(())
}
