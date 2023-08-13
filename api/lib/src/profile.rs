use std::fmt::{Display, Formatter};
use actix_web::{web::{self, ServiceConfig}, HttpResponse,web::Json};
use actix_web::error::BlockingError;
use sqlx::{Error as SqlxError, postgres::PgPool};
use actix_web::http::StatusCode;
use actix_web::web::Query;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use shared::models::{
    Profile,
    CreateProfile,
    UpdateProfile
};
use tracing::error;

#[derive(Debug)]
pub enum AppError {
    SqlError(SqlxError),
    GenericError(String),
    OtherError,
}

impl Display for AppError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            AppError::SqlError(e) => write!(f, "Database error: {}", e),
            AppError::GenericError(msg) => write!(f, "Error: {}", msg),
            AppError::OtherError => write!(f, "An unknown error occurred"),
        }
    }
}

impl actix_web::error::ResponseError for AppError {
    fn status_code(&self) -> StatusCode {
        match *self {
            AppError::SqlError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            AppError::GenericError(_) => StatusCode::BAD_REQUEST,
            AppError::OtherError => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    fn error_response(&self) -> HttpResponse {
        let status = self.status_code();
        HttpResponse::build(status).json(format!("{}", self))
    }
}

pub fn service(cfg: &mut ServiceConfig) {
    cfg.service(web::scope("/v0.1/profiles")
                    .route("", web::get().to(get_all_profiles))
                    .route("/profile", web::get().to(get_profile))
                    .route("/profile", web::post().to(create_profile))
                    .route("/profile/{id}", web::put().to(update_profile))
                    .route("/profile/{id}", web::delete().to(delete_profile))
    );
}

#[derive(Debug, Serialize)]
pub struct PaginatedResponse<T> {
    pub total_results: i64,
    pub current_page: i64,
    pub total_pages: i64,
    pub profiles: Vec<T>,
}

#[derive(Deserialize)]
pub struct Pagination {
    limit: i64,
    offset: i64,
    gender: Option<String>,
    // ... other fields ...
}
#[derive(Deserialize)]
pub struct ProfileFilter {
    #[allow(dead_code)]
    gender: Option<String>,
}
async fn get_all_profiles(pool: web::Data<PgPool>, pagination: Query<Pagination>) -> Result<HttpResponse, AppError> {
    let Pagination { limit, offset, gender } = pagination.into_inner();

    // Build dynamic WHERE clause based on provided filters
    let mut where_clauses = Vec::new();

    if gender.is_some() {
        where_clauses.push("\"gender\" = $3");
    }

    let where_clause = if where_clauses.is_empty() {
        String::new()
    } else {
        format!("WHERE {}", where_clauses.join(" AND "))
    };

    // Adjust COUNT query with the WHERE clause
    let count_query = format!(
        r#"
        SELECT COUNT(*)
        FROM "Profile"
        {}
        "#,
        where_clause
    );
    let mut total_profiles_query = sqlx::query_scalar(&count_query);
    total_profiles_query = total_profiles_query.bind(limit).bind(offset);

    if let Some(g) = &gender {
        total_profiles_query = total_profiles_query.bind(g);
    }

    let total_profiles = match total_profiles_query.fetch_one(pool.get_ref()).await {
        Ok(count) => count,
        Err(e) => {
            eprintln!("Error counting profiles: {:?}", e);
            return Err(AppError::SqlError(e));
        }
    };

    // Calculate the current page and total pages
    let current_page = offset / limit + 1;
    let total_pages = (total_profiles as f64 / limit as f64).ceil() as i64;

    // Adjust the main query with the WHERE clause
    let profiles_query = format!(
        r#"
        SELECT *
        FROM "Profile"
        {}
        ORDER BY "createdAt"
        LIMIT $1 OFFSET $2
        "#,
        where_clause
    );

    let mut profiles_result_query = sqlx::query_as::<_, Profile>(&profiles_query);
    profiles_result_query = profiles_result_query.bind(limit).bind(offset);

    if let Some(g) = &gender {
        profiles_result_query = profiles_result_query.bind(g);
    }

    let profiles_result = profiles_result_query.fetch_all(pool.get_ref()).await;

    match profiles_result {
        Ok(profiles) => {
            let response = PaginatedResponse {
                total_results: total_profiles,
                current_page,
                total_pages,
                profiles,
            };
            Ok(HttpResponse::Ok().json(response))
        }
        Err(e) => {
            eprintln!("Error fetching profiles: {:?}", e);
            Err(AppError::SqlError(e))
        }
    }
}

#[allow(dead_code)]
#[derive(Deserialize)]
pub struct SingleProfileFilter {
    id: Option<Uuid>,
    account_number: Option<String>,
    bvn: Option<String>,
    identity_number: Option<String>,
    phone_number: Option<String>,
}

enum FilterValue {
    UuidValue(Uuid),
    StringValue(String),
}

async fn get_profile(pool: web::Data<PgPool>, filter: Query<SingleProfileFilter>) -> Result<HttpResponse, AppError> {
    let mut where_clauses = Vec::new();
    let mut bindings = Vec::<FilterValue>::new();

    if let Some(id) = filter.id {
        where_clauses.push("\"id\" = $?");
        bindings.push(FilterValue::UuidValue(id));
    }
    if let Some(account_number) = &filter.account_number {
        where_clauses.push("\"accountNumber\" = $?");
        bindings.push(FilterValue::StringValue(account_number.clone()));
    }
    if let Some(bvn) = &filter.bvn {
        where_clauses.push("\"bvn\" = $?");
        bindings.push(FilterValue::StringValue(bvn.clone()));
    }
    if let Some(identity_number) = &filter.identity_number {
        where_clauses.push("\"identityNumber\" = $?");
        bindings.push(FilterValue::StringValue(identity_number.clone()));
    }
    if let Some(phone_number) = &filter.phone_number {
        where_clauses.push("\"phoneNumber\" = $?");
        bindings.push(FilterValue::StringValue(phone_number.clone()));
    }

    if where_clauses.is_empty() {
        return Err(AppError::GenericError("No filter criteria provided".to_string()));
    }

    let where_clause = format!("WHERE {}", where_clauses.join(" AND "));
    let mut query = format!(r#"SELECT * FROM "Profile" {}"#, where_clause);

    for (i, _) in bindings.iter().enumerate() {
        let placeholder = format!("${}", i + 1);
        query = query.replace("$?", &placeholder);
    }

    let mut profile_query = sqlx::query_as::<_, Profile>(&query);
    for value in &bindings {
        match value {
            FilterValue::UuidValue(uuid) => profile_query = profile_query.bind(uuid),
            FilterValue::StringValue(s) => profile_query = profile_query.bind(s),
        }
    }

    let profile_result = profile_query.fetch_one(pool.get_ref()).await;

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