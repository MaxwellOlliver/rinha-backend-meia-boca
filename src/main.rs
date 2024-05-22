mod auth;
mod controllers;
mod entity;
mod helpers;
mod routes;
mod schemas;
mod services;

use actix_web::{web, App, HttpServer};
use dotenv::dotenv;
use env_logger::Env;
use migration::{
    sea_orm::{Database, DatabaseConnection},
    Migrator, MigratorTrait,
};
use std::env;

#[derive(Debug, Clone)]
pub struct AppState {
    secret_key: String,
    db: DatabaseConnection,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    env_logger::init_from_env(Env::default().default_filter_or("info"));

    let secret_key: String = env::var("SECRET_KEY").expect("SECRET_KEY must be set");

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let db: DatabaseConnection = Database::connect(&database_url).await.unwrap();

    Migrator::up(&db, None).await.unwrap();

    let state: AppState = AppState { secret_key, db };

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(state.clone()))
            .configure(routes::configure)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
