use actix_web::{web, App, HttpServer, Responder, HttpResponse, get};
use sqlx::postgres::{PgPool, PgPoolOptions};
use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use sqlx::Executor;
use uuid::Uuid;


#[derive(Debug, Deserialize, Serialize, sqlx::FromRow)]
pub struct User {
    pub id: Uuid,
    #[sqlx(rename = "createdAt")] // Use sqlx(rename = "...") to match the column name with quotes
    pub created_at: DateTime<Utc>,
    #[sqlx(rename = "updatedAt")] // Use sqlx(rename = "...") to match the column name with quotes
    pub updated_at: DateTime<Utc>,
    #[sqlx(rename = "firstName")] // Use sqlx(rename = "...") to match the column name with quotes
    pub first_name: String,
    #[sqlx(rename = "lastName")] // Use sqlx(rename = "...") to match the column name with quotes
    pub last_name: String,
    pub email: Option<String>,
    #[sqlx(rename = "middleName")] // Use sqlx(rename = "...") to match the column name with quotes
    pub middle_name: Option<String>,
}

#[get("/")]
async fn hello_world() -> impl Responder {
    HttpResponse::Ok().body("Hello World!")
}

#[get("/user/{id}")]
async fn get_user(pool: web::Data<PgPool>, id: web::Path<Uuid>) -> impl Responder {
    tracing::info!("Getting user: {:?}", id);
    let result = sqlx::query_as::<_, User>("SELECT * FROM \"User\" WHERE id = $1")
        .bind(id.into_inner())
        .fetch_one(pool.get_ref())
        .await;

    match result {
        Ok(user) => HttpResponse::Ok().json(user),
        Err(e) => {
            eprintln!("Error fetching user: {:?}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}

#[get("/users")]
async fn get_all_users(pool: web::Data<PgPool>) -> impl Responder {
    tracing::info!("Getting all users");
    let result = sqlx::query_as::<_, User>("SELECT id, \"createdAt\", \"updatedAt\", \"firstName\", \"lastName\", \"email\", \"middleName\" FROM \"User\"")
        .fetch_all(pool.get_ref())
        .await;

    match result {
        Ok(users) => HttpResponse::Ok().json(users),
        Err(e) => {
            eprintln!("Error fetching users: {:?}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}

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
