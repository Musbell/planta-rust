use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use uuid::Uuid;

// ------** Pagination Model **------//
#[derive(Debug, Deserialize)]
pub struct Pagination {
    pub limit: i64,
    pub offset: i64,
}


// ------** User Model **------//
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

#[derive(Debug, Deserialize, Serialize)]
pub struct DeleteUser {
    pub id: Uuid,
}


// ------** Profile Model **------//
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

#[derive(Debug, Deserialize, Serialize)]
pub struct DeleteProfile {
    pub id: Uuid,
}