use actix_web::{web, App, HttpServer};
use dotenv::dotenv;
use sqlx::sqlite::SqlitePoolOptions;
use std::env;

mod auth;
mod db;
mod habit;
mod notifications;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok(); // Load .env file

    let database_url =
        env::var("DATABASE_URL").unwrap_or_else(|_| "sqlite://habits.db".to_string());
    let db_pool = SqlitePoolOptions::new()
        .connect(&database_url)
        .await
        .unwrap();

    // Initialize the database and create tables (if needed)
    db::init(&db_pool).await.unwrap();

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(db_pool.clone())) // Share database connection
            .configure(auth::config) // Configure authentication routes
            .configure(habit::config) // Configure habit routes
            .configure(notifications::config) // Configure notifications routes
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}