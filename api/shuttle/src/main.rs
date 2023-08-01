use actix_web::{web, App, HttpServer, Responder, HttpResponse, get};
use sqlx::postgres::{ PgPoolOptions};
use api_lib::user::{get_all_users, get_user};

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
            .service(get_user)
            .service(get_all_users)
    })
        .bind("127.0.0.1:8000")?
        .run()
        .await
}
