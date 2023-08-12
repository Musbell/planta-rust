use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use uuid::Uuid;

// ------** Pagination Model **------//
// PAGINATION
#[derive(Debug, Deserialize)]
pub struct Pagination {
    pub limit: i64,
    pub offset: i64,
}


// ------** User Model **------//
// GET USER
#[derive(Debug, sqlx::FromRow, Serialize, Deserialize)]
pub struct User {
    pub id: Uuid,
    #[sqlx(rename = "createdAt")]
    pub created_at: DateTime<Utc>,
    #[sqlx(rename = "updatedAt")]
    pub updated_at: DateTime<Utc>,
    #[sqlx(rename = "firstName")]
    pub first_name: String,
    #[sqlx(rename = "lastName")]
    pub last_name: String,
    pub email: Option<String>,
    #[sqlx(rename = "middleName")]
    pub middle_name: Option<String>,
}
// CREATE USER
#[derive(Debug, Deserialize, Serialize)]
pub struct CreateUser {
    #[serde(rename = "firstName")]
    pub first_name: String,
    #[serde(rename = "lastName")]
    pub last_name: String,
    pub email: Option<String>,
    #[serde(rename = "middleName")]
    pub middle_name: Option<String>,
}

// UPDATE USER
#[derive(Debug, Deserialize, Serialize)]
pub struct UpdateUser {
    pub id: Uuid,
    #[serde(rename = "firstName")]
    pub first_name: Option<String>,
    #[serde(rename = "lastName")]
    pub last_name: Option<String>,
    pub email: Option<String>,
    #[serde(rename = "middleName")]
    pub middle_name: Option<String>,
}

// DELETE USER
#[derive(Debug, Deserialize, Serialize)]
pub struct DeleteUser {
    pub id: Uuid,
}


// ------** Profile Model **------//
// GET PROFILE
#[derive(Debug, sqlx::FromRow, Serialize, Deserialize)]
pub struct Profile {
    pub id: Uuid,
    #[sqlx(rename = "createdAt")]
    pub created_at: DateTime<Utc>,
    #[sqlx(rename = "updatedAt")]
    pub updated_at: DateTime<Utc>,
    pub bio: Option<String>,
    #[sqlx(rename = "accountNumber")]
    pub account_number: Option<String>,
    pub bvn: String,
    pub gender: String,
    #[sqlx(rename = "identityNumber")]
    pub identity_number: Option<String>,
    #[sqlx(rename = "phoneNumber")]
    pub phone_number: Option<String>,
    #[sqlx(rename = "userId")]
    pub user_id: Uuid,
}

// CREATE PROFILE
#[derive(Debug, Deserialize, Serialize)]
pub struct CreateProfile {
    pub bio: String,
    #[serde(rename = "accountNumber")]
    pub account_number: String,
    pub bvn: String,
    pub gender: String,
    #[serde(rename = "identityNumber")]
    pub identity_number: String,
    #[serde(rename = "phoneNumber")]
    pub phone_number: String,
    #[serde(rename = "userId")]
    pub user_id: Uuid,
}

// UPDATE PROFILE
#[derive(Debug, Deserialize, Serialize)]
pub struct UpdateProfile {
    pub id: Uuid,
    pub bio: Option<String>,
    #[serde(rename = "accountNumber")]
    pub account_number: Option<String>,
    pub bvn: Option<String>,
    pub gender: Option<String>,
    #[serde(rename = "identityNumber")]
    pub identity_number: Option<String>,
    #[serde(rename = "phoneNumber")]
    pub phone_number: Option<String>,
    #[serde(rename = "userId")]
    pub user_id: Option<Uuid>,
}

// DELETE PROFILE
#[derive(Debug, Deserialize, Serialize)]
pub struct DeleteProfile {
    pub id: Uuid,
}

// ------** Farm Model **------//
// GET FARM
#[derive(Debug, sqlx::FromRow, Serialize, Deserialize)]
pub struct Farm {
    pub id: Uuid,
    #[sqlx(rename = "createdAt")]
    pub created_at: DateTime<Utc>,
    #[sqlx(rename = "updatedAt")]
    pub updated_at: DateTime<Utc>,
    pub farm_name: Option<String>,
    pub acreage: f64,
    pub state: String,
    pub locality: String,
    pub has_drainage_tile: Option<bool>,
    pub land_value: Option<i32>,
    pub is_irrigated: Option<bool>,
    pub ownership: String,
    pub available_portion: Option<f64>,
    pub country: String,
    #[sqlx(rename = "farmerId")]
    pub farmer_id: Uuid,
    pub latitude: f64,
    pub longitude: f64,
    pub farm_site: Option<String>,
}

// CREATE FARM
#[derive(Debug, Deserialize, Serialize)]
pub struct CreateFarm {
    #[serde(rename = "farmName")]
    pub farm_name: String,
    pub acreage: f64,
    pub state: String,
    pub locality: String,
    pub has_drainage_tile: bool,
    pub land_value: i32,
    pub is_irrigated: bool,
    pub ownership: String,
    pub available_portion: f64,
    pub country: String,
    #[serde(rename = "farmerId")]
    pub farmer_id: Uuid,
    pub latitude: f64,
    pub longitude: f64,
    pub farm_site: String,
}

// UPDATE FARM
#[derive(Debug, Deserialize, Serialize)]
pub struct UpdateFarm {
    pub id: Uuid,
    pub farm_name: Option<String>,
    pub acreage: Option<f64>,
    pub state: Option<String>,
    pub locality: Option<String>,
    pub has_drainage_tile: Option<bool>,
    pub land_value: Option<i32>,
    pub is_irrigated: Option<bool>,
    pub ownership: Option<String>,
    pub available_portion: Option<f64>,
    pub country: Option<String>,
    #[serde(rename = "farmerId")]
    pub farmer_id: Option<Uuid>,
    pub latitude: Option<f64>,
    pub longitude: Option<f64>,
    pub farm_site: Option<String>,
}

// DELETE FARM
#[derive(Debug, Deserialize, Serialize)]
pub struct DeleteFarm {
    pub id: Uuid,
}