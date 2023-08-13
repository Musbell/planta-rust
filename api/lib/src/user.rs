use std::fmt::{Display, Formatter};
use actix_web::{web::{self, ServiceConfig}, HttpResponse,web::Json};
use actix_web::error::BlockingError;
use sqlx::{Error as SqlxError, postgres::PgPool};
use actix_web::http::StatusCode;
use actix_web::web::Query;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use shared::models::{User, Pagination, CreateUser, UpdateUser};
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
    cfg.service(web::scope("/v0.1/users")
                    .route("", web::get().to(get_all_users))
                    .route("/user", web::get().to(get_user))
                    .route("/user", web::post().to(create_user))
                    .route("/user/{id}", web::put().to(update_user))
                    .route("/user/{id}", web::delete().to(delete_user))
    );
}

#[derive(Debug, Serialize)]
pub struct PaginatedResponse<T> {
    pub total_results: i64,
    pub current_page: i64,
    pub total_pages: i64,
    pub users: Vec<T>,
}
async fn get_all_users(pool: web::Data<PgPool>, pagination: Query<Pagination>) -> Result<HttpResponse, AppError> {
    let Pagination { limit, offset } = pagination.into_inner();

    // Fetch the total number of users
    let total_users_result = sqlx::query_scalar(
        r#"
        SELECT COUNT(*)
        FROM "User"
        "#,
    )
        .fetch_one(pool.get_ref())
        .await;

    let total_users = match total_users_result {
        Ok(count) => count,
        Err(e) => {
            eprintln!("Error counting users: {:?}", e);
            return Err(AppError::SqlError(e));
        }
    };

    // Calculate the current page and total pages
    let current_page = offset / limit + 1;
    let total_pages = (total_users as f64 / limit as f64).ceil() as i64;

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
        Ok(users) => {
            let response = PaginatedResponse {
                total_results: total_users,
                current_page,
                total_pages,
                users,
            };
            Ok(HttpResponse::Ok().json(response))
        }
        Err(e) => {
            eprintln!("Error fetching users: {:?}", e);
            Err(AppError::SqlError(e))
        }
    }
}

#[derive(Deserialize)]
pub struct UserFilter {
    id: Option<Uuid>,
    email: Option<String>,
}

async fn get_user(pool: web::Data<PgPool>, filter: Query<UserFilter>) -> Result<HttpResponse, AppError> {
    let (filter_field, filter_value) = match (&filter.id, &filter.email) {
        (Some(id), None) => ("id", id.to_string()),
        (None, Some(email)) => ("email", email.clone()),
        _ => return Err(AppError::GenericError("Provide either id or email, not both".to_string())),
    };

    tracing::info!("Getting user by {}: {}", filter_field, filter_value);

    let query = if filter_field == "id" {
        format!(
            r#"
            SELECT * FROM "User"
            WHERE id = $1::uuid
            "#
        )
    } else {
        format!(
            r#"
            SELECT * FROM "User"
            WHERE email = $1
            "#
        )
    };

    let result = sqlx::query_as::<_, User>(&query)
        .bind(filter_value)
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