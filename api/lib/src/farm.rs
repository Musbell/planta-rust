use std::fmt::{Display, Formatter};
use actix_web::{web::{self, ServiceConfig}, HttpResponse,web::Json};
use actix_web::error::BlockingError;
use sqlx::{Error as SqlxError, postgres::PgPool};
use actix_web::http::StatusCode;
use uuid::Uuid;
use shared::models::{
    Pagination,
    Farm,
    CreateFarm,
    UpdateFarm
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
    cfg.service(web::scope("/v0.1/farms")
                    .route("", web::get().to(get_all_farms))
                    .route("/farm/{id}", web::get().to(get_farm))
                    .route("/farm", web::post().to(create_farm))
                    .route("/farm/{id}", web::put().to(update_farm))
                    .route("/farm/{id}", web::delete().to(delete_farm))
    );
}

async fn get_all_farms(pool: web::Data<PgPool>, pagination: web::Query<Pagination>) -> Result<HttpResponse, AppError> {
    let Pagination { limit, offset } = pagination.into_inner();

    let farms_result = sqlx::query_as::<_, Farm>(
        r#"
        SELECT *
        FROM "Farm"
        ORDER BY "createdAt"
        LIMIT $1 OFFSET $2
        "#,
    )
        .bind(limit)
        .bind(offset)
        .fetch_all(pool.get_ref())
        .await;

    match farms_result {
        Ok(farms) => Ok(HttpResponse::Ok().json(farms)),
        Err(e) => {
            error!("Error getting all farms: {:?}", e);
            Err(AppError::SqlError(e))
        }
    }
}

async fn get_farm(pool: web::Data<PgPool>, id: web::Path<Uuid>) -> Result<HttpResponse, AppError> {
    let id = id.into_inner();

    let farm_result = sqlx::query_as::<_, Farm>(
        r#"
        SELECT *
        FROM "Farm"
        WHERE id = $1
        "#,
    )
        .bind(id)
        .fetch_one(pool.get_ref())
        .await;

    match farm_result {
        Ok(farm) => Ok(HttpResponse::Ok().json(farm)),
        Err(e) => {
            error!("Error getting farm: {:?}", e);
            Err(AppError::SqlError(e))
        }
    }
}

/**
 * Create Farm
 **/
async fn create_farm(pool: web::Data<PgPool>, farm: Json<CreateFarm>) -> Result<HttpResponse, BlockingError> {
    let farm = farm.into_inner();

    let farm_result = sqlx::query_as::<_, Farm>(
        r#"
        INSERT INTO "Farm" ("farmName", acreage, state, locality, "hasDrainageTile", "landValue", "isIrrigated", ownership, "availablePortion", country, "farmerId", latitude, longitude, "farmSite")
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14)
        RETURNING *
        "#,
    )
        .bind(farm.farm_name)
        .bind(farm.acreage)
        .bind(farm.state)
        .bind(farm.locality)
        .bind(farm.has_drainage_tile)
        .bind(farm.land_value)
        .bind(farm.is_irrigated)
        .bind(farm.ownership)
        .bind(farm.available_portion)
        .bind(farm.country)
        .bind(farm.farmer_id)
        .bind(farm.latitude)
        .bind(farm.longitude)
        .bind(farm.farm_site)
        .fetch_one(pool.get_ref())
        .await;

    match farm_result {
        Ok(farm) => Ok(HttpResponse::Created().json(farm)),
        Err(e) => {
            error!("Error creating farm: {:?}", e);
            Ok(HttpResponse::InternalServerError().json("Internal server error"))
        }
    }
}

/**
 * Update Farm
 **/
async fn update_farm(pool: web::Data<PgPool>, farm: Json<UpdateFarm>) -> Result<HttpResponse, BlockingError> {
    let farm = farm.into_inner();

    let farm_result = sqlx::query_as::<_, Farm>(
        r#"
        UPDATE "Farm"
        SET "farmName" = $1,
            acreage = $2,
            state = $3,
            locality = $4,
            "hasDrainageTile" = $5,
            "landValue" = $6,
            "isIrrigated" = $7,
            ownership = $8,
            "availablePortion" = $9,
            country = $10,
            "farmerId" = $11,
            latitude = $12,
            longitude = $13,
            "farmSite" = $14
        WHERE id = $15
        RETURNING *
        "#,
    )
        .bind(farm.farm_name)
        .bind(farm.acreage)
        .bind(farm.state)
        .bind(farm.locality)
        .bind(farm.has_drainage_tile)
        .bind(farm.land_value)
        .bind(farm.is_irrigated)
        .bind(farm.ownership)
        .bind(farm.available_portion)
        .bind(farm.country)
        .bind(farm.farmer_id)
        .bind(farm.latitude)
        .bind(farm.longitude)
        .bind(farm.farm_site)
        .bind(farm.id)
        .fetch_one(pool.get_ref())
        .await;

    match farm_result {
        Ok(farm) => Ok(HttpResponse::Ok().json(farm)),
        Err(e) => {
            error!("Error updating farm: {:?}", e);
            Ok(HttpResponse::InternalServerError().json("Internal server error"))
        }
    }
}

/**
 * Delete Farm
 **/
async fn delete_farm(pool: web::Data<PgPool>, id: web::Path<Uuid>) -> Result<HttpResponse, AppError> {
    let id = id.into_inner();

    let farm_result = sqlx::query_as::<_, Farm>(
        r#"
        DELETE FROM "Farm"
        WHERE id = $1
        RETURNING *
        "#,
    )
        .bind(id)
        .fetch_one(pool.get_ref())
        .await;

    match farm_result {
        Ok(farm) => Ok(HttpResponse::Ok().json(farm)),
        Err(e) => {
            error!("Error deleting farm: {:?}", e);
            Err(AppError::SqlError(e))
        }
    }
}