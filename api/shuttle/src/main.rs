use actix_web::{web, App, HttpServer};
use sqlx::postgres::{ PgPoolOptions};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();

    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .expect("Failed to create pool.");

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone())) // updated here
            .configure(api_lib::user::service)
    })
        .bind("127.0.0.1:8000")?
        .run()
        .await
}
