use actix_web::{web, App, HttpServer};
use sqlx::postgres::PgPoolOptions;
use redis::aio::Connection;
use std::sync::Arc; // Import Arc for shared references

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();

    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let redis_url = std::env::var("REDIS_URL").expect("REDIS_URL must be set");

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .expect("Failed to create pool.");

    let redis_client = redis::Client::open(redis_url).expect("Failed to create Redis client");
    let redis_connection: Arc<Connection> = Arc::new(
        redis_client
            .get_tokio_connection()
            .await
            .expect("Failed to connect to Redis"),
    );

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .app_data(web::Data::new(redis_connection.clone()))
            .configure(api_lib::user::service)
            .configure(api_lib::profile::service)

    })
        .bind("0.0.0.0:8080")?
        .run()
        .await
}
