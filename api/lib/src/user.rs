use actix_web::{web::{self, ServiceConfig}, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use sqlx::postgres::{PgPool};
use uuid::Uuid;


pub fn service(cfg: &mut ServiceConfig) {
    cfg.route("/user", web::get().to(get_all_users));
    cfg.route("/user/{id}", web::get().to(get_user));
}

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

async fn get_all_users(pool: web::Data<PgPool>) -> impl Responder {
    tracing::info!("Getting all users");
    let result = sqlx::query_as::<_, User>("SELECT * FROM \"User\"")
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