pub(crate) mod api;
pub(crate) mod auth;
pub(crate) mod models;
pub(crate) mod services;

use actix_web::middleware::{Compress, Logger};
use actix_web::web::Data;
use actix_web::{App, HttpServer};
use actix_web_httpauth::middleware::HttpAuthentication;
use auth::ApiCredentials;
use paperclip::actix::web::Scope;
use paperclip::actix::OpenApiExt;
use services::airports::GlobalAirportsRepository;
use services::app_state::AppState;
use services::healthcheck::hostname_provider::HostnameCrateHostnameProvider;
use services::healthcheck::time_provider::SystemTimeProvider;
use sqlx::sqlite::SqliteConnectOptions;
use sqlx::SqlitePool;
use std::str::FromStr;

const DATABASE_URL: &str = "sqlite:./global_airports_database.sqlite";

fn init_logger() {
    tracing_subscriber::fmt().json().compact().init();
    tracing::info!("Initialized json logger successfully.");
}

#[cfg(debug_assertions)]
fn setup_dev() {
    let environment = std::env::var("ENVIRONMENT").unwrap_or("development".into());

    if environment == "development" {
        tracing::info!("Attempting to load .env file");
        dotenv::dotenv().ok();
    }
}

#[cfg(not(debug_assertions))]
fn setup_dev() {}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    init_logger();
    setup_dev();

    let db_opts = SqliteConnectOptions::from_str(DATABASE_URL)
        .expect("Failed to parse database url")
        .read_only(true);

    let db_pool = SqlitePool::connect_with(db_opts)
        .await
        .expect("Failed to connect to database");

    let airports_repository = Box::new(GlobalAirportsRepository::new(db_pool).await);

    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(AppState::new(
                Box::new(HostnameCrateHostnameProvider::new()),
                Box::new(SystemTimeProvider::new()),
                airports_repository.clone(),
            )))
            .wrap(Logger::default())
            .wrap(Compress::default())
            .service(api::health::check_health)
            .wrap_api()
            .service(
                Scope::new("/api")
                    .app_data(Data::new(ApiCredentials::from_env()))
                    .wrap(HttpAuthentication::basic(auth::basic_auth_validator))
                    .service(api::distance::handlers::coordinates_handler)
                    .service(api::distance::handlers::airports_handler)
                    .service(api::airports::handlers::unique_iatas_handler),
            )
            .with_json_spec_at("/docs/spec")
            .with_swagger_ui_at("/docs")
            .build()
    })
    .bind(("0.0.0.0", 8000))?
    .run()
    .await
}
