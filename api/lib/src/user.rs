use std::fmt::{Display, Formatter};
use actix_web::{web::{self, ServiceConfig}, HttpResponse,web::Json};
use actix_web::error::BlockingError;
use sqlx::{Error as SqlxError, postgres::PgPool};
use actix_web::http::StatusCode;
use uuid::Uuid;
use shared::models::{User, Pagination, CreateUser, UpdateUser};
use tracing::error;

#[derive(Debug)]
pub enum AppError {
    SqlError(SqlxError),
    OtherError,
}

impl Display for AppError {
    fn fmt(&self, _f: &mut Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

impl actix_web::error::ResponseError for AppError {
    fn status_code(&self) -> StatusCode {
        match *self {
            AppError::SqlError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            AppError::OtherError => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}


pub fn service(cfg: &mut ServiceConfig) {
    cfg.service(web::scope("/v0.1/users")
                    .route("", web::get().to(get_all_users))
                    .route("/user/{id}", web::get().to(get_user))
                    .route("/user", web::post().to(create_user))
                    .route("/user/{id}", web::put().to(update_user))
                    .route("/user/{id}", web::delete().to(delete_user))
    );
}

async fn get_all_users(pool: web::Data<PgPool>, pagination: web::Query<Pagination>) -> Result<HttpResponse, AppError> {
    let Pagination { limit, offset } = pagination.into_inner();

    let users_result = sqlx::query_as::<_, User>(
        r#"
        SELECT *
        FROM "User"
        ORDER BY "createdAt"
        LIMIT $1 OFFSET $2
        "#,
    )
        .bind(limit)
        .bind(offset)
        .fetch_all(pool.get_ref())
        .await;

    match users_result {
        Ok(users) => Ok(HttpResponse::Ok().json(users)),
        Err(e) => {
            eprintln!("Error fetching users: {:?}", e);
            Err(AppError::SqlError(e))
        }
    }
}

async fn get_user(pool: web::Data<PgPool>, id: web::Path<Uuid>) -> Result<HttpResponse, AppError> {
    tracing::info!("Getting user: {:?}", id);
    let result = sqlx::query_as::<_, User>(
        r#"
            SELECT * FROM "User"
            WHERE id = $1
            "#)
        .bind(id.into_inner())
        .fetch_one(pool.get_ref())
        .await;

    match result {
        Ok(user) => Ok(HttpResponse::Ok().json(user)),
        Err(e) => {
            error!("Error fetching user: {:?}", e);
            Err(AppError::SqlError(e))
        }
    }
}

async fn create_user(pool: web::Data<PgPool>, user: Json<CreateUser>) -> Result<HttpResponse, BlockingError> {
    let result = sqlx::query_as::<_, User>(r#"
    INSERT INTO "User"
    (
         "firstName",
         "lastName",
         "email",
         "middleName"
    )
         VALUES ($1, $2, $3, $4) RETURNING *
    "#)
        .bind(&user.first_name)
        .bind(&user.last_name)
        .bind(&user.email)
        .bind(&user.middle_name)
        .fetch_one(pool.get_ref())
        .await;

    match result {
        Ok(user) => Ok(HttpResponse::Created().json(user)),
        Err(e) => {
            error!("Error creating user: {:?}", e);
            Ok(HttpResponse::InternalServerError().json("Internal server error"))
        }
    }
}

async fn update_user(pool: web::Data<PgPool>, id: web::Path<Uuid>, user: Json<UpdateUser>) -> Result<HttpResponse, BlockingError> {
    let result = sqlx::query_as::<_, User>("UPDATE \"User\" SET firstName = $2, lastName = $3, email = $4, middleName = $5 WHERE id = $1 RETURNING *")
        .bind(id.into_inner())
        .bind(&user.first_name)
        .bind(&user.last_name)
        .bind(&user.email)
        .bind(&user.middle_name)
        .fetch_one(pool.get_ref())
        .await;

    match result {
        Ok(user) => Ok(HttpResponse::Ok().json(user)),
        Err(e) => {
            error!("Error updating user: {:?}", e);
            Ok(HttpResponse::InternalServerError().json("Internal server error"))
        }
    }
}


async fn delete_user(pool: web::Data<PgPool>, id: web::Path<Uuid>) -> Result<HttpResponse, AppError> {
    let result = sqlx::query("DELETE FROM \"User\" WHERE id = $1")
        .bind(id.into_inner())
        .execute(pool.get_ref())
        .await;

    match result {
        Ok(_) => Ok(HttpResponse::NoContent().finish()),
        Err(e) => {
            error!("Error deleting user: {:?}", e);
            Err(AppError::SqlError(e))
        }
    }
}