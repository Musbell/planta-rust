use std::fmt::{Display, Formatter};
use actix_web::{web::{self, ServiceConfig}, HttpResponse,web::Json};
use actix_web::error::BlockingError;
use sqlx::{Error as SqlxError, postgres::PgPool};
use actix_web::http::StatusCode;
use uuid::Uuid;
use shared::models::{
    Pagination,
    Profile,
    CreateProfile,
    UpdateProfile
};
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
    cfg.service(web::scope("/v0.1/profiles")
                    .route("", web::get().to(get_all_profiles))
                    .route("/profile/{id}", web::get().to(get_profile))
                    .route("/profile", web::post().to(create_profile))
                    .route("/profile/{id}", web::put().to(update_profile))
                    .route("/profile/{id}", web::delete().to(delete_profile))
    );
}

async fn get_all_profiles(pool: web::Data<PgPool>, pagination: web::Query<Pagination>) -> Result<HttpResponse, AppError> {
    let Pagination { limit, offset } = pagination.into_inner();

    let profiles_result = sqlx::query_as::<_, Profile>(
        r#"
        SELECT *
        FROM "Profile"
        ORDER BY "createdAt"
        LIMIT $1 OFFSET $2
        "#,
    )
        .bind(limit)
        .bind(offset)
        .fetch_all(pool.get_ref())
        .await;

    match profiles_result {
        Ok(profiles) => Ok(HttpResponse::Ok().json(profiles)),
        Err(e) => {
            error!("Failed to get profiles: {:?}", e);
            Err(AppError::SqlError(e))
        }
    }
}

async fn get_profile(pool: web::Data<PgPool>, id: web::Path<Uuid>) -> Result<HttpResponse, AppError> {
    let id = id.into_inner();

    let profile_result = sqlx::query_as::<_, Profile>(
        r#"
        SELECT *
        FROM "Profile"
        WHERE "id" = $1
        "#,
    )
        .bind(id)
        .fetch_one(pool.get_ref())
        .await;

    match profile_result {
        Ok(profile) => Ok(HttpResponse::Ok().json(profile)),
        Err(e) => {
            error!("Failed to get profile: {:?}", e);
            Err(AppError::SqlError(e))
        }
    }
}

async fn create_profile(pool: web::Data<PgPool>, profile: Json<CreateProfile>) -> Result<HttpResponse, BlockingError> {
    let result = sqlx::query_as::<_, Profile>(r#"
    INSERT INTO "Profile"
    (
         "bio",
         "accountNumber",
         "bvn",
         "gender",
         "identityNumber",
         "phoneNumber",
         "userId"
    )
         VALUES ($1, $2, $3, $4, $5, $6, $7) RETURNING *
    "#)
        .bind(&profile.bio)
        .bind(&profile.account_number)
        .bind(&profile.bvn)
        .bind(&profile.gender)
        .bind(&profile.identity_number)
        .bind(&profile.phone_number)
        .bind(&profile.user_id)
        .fetch_one(pool.get_ref())
        .await;

    match result {
        Ok(profile) => Ok(HttpResponse::Created().json(profile)),
        Err(e) => {
            error!("Error creating profile: {:?}", e);
            Ok(HttpResponse::InternalServerError().json("Internal server error"))
        }
    }
}

async fn update_profile(pool: web::Data<PgPool>, id: web::Path<Uuid>, profile: Json<UpdateProfile>) -> Result<HttpResponse, BlockingError> {
    let result = sqlx::query_as::<_, Profile>(r#"
          UPDATE "Profile" SET
          "bio" = $2,
          "accountNumber" = $3,
          "bvn" = $4,
          "gender" = $5,
          "identityNumber" = $6,
          "phoneNumber" = $7,
          "userId" = $8
          WHERE id = $1 RETURNING *
      "#)
        .bind(id.into_inner())
        .bind(&profile.bio)
        .bind(&profile.account_number)
        .bind(&profile.bvn)
        .bind(&profile.gender)
        .bind(&profile.identity_number)
        .bind(&profile.phone_number)
        .bind(&profile.user_id)
        .fetch_one(pool.get_ref())
        .await;

    match result {
        Ok(user) => Ok(HttpResponse::Ok().json(user)),
        Err(e) => {
            error!("Error updating profile: {:?}", e);
            Ok(HttpResponse::InternalServerError().json("Internal server error"))
        }
    }
}

async fn delete_profile(pool: web::Data<PgPool>, id: web::Path<Uuid>) -> Result<HttpResponse, AppError> {
    let result = sqlx::query(r#"DELETE FROM "Profile" WHERE id = $1"#)
        .bind(id.into_inner())
        .execute(pool.get_ref())
        .await;

    match result {
        Ok(_) => Ok(HttpResponse::NoContent().finish()),
        Err(e) => {
            error!("Error deleting profile: {:?}", e);
            Err(AppError::SqlError(e))
        }
    }
}