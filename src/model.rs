use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Debug, FromRow, Deserialize, Serialize, Clone, ToSchema)]
#[allow(non_snake_case)]
pub struct FileExtendedResponseModel {
    pub id: Option<Uuid>,
    pub fullname: Option<String>,
    pub created: Option<chrono::DateTime<chrono::Utc>>,
    pub sizebytes: Option<i64>,
    pub downloads: Option<i32>,
    #[serde(rename = "averageRating")]
    pub average_rating: Option<f32>,
    pub owner: Option<String>,
    #[serde(rename = "permission")]
    pub roles_pk: Option<String>,
}

#[derive(Debug, FromRow, Deserialize, Serialize, Clone, ToSchema)]
#[allow(non_snake_case)]
pub struct FileSimpleResponseModel {
    pub id: Uuid,
    pub fullname: String,
    pub created: Option<chrono::DateTime<chrono::Utc>>,
    pub sizebytes: i64,
    pub downloads: Option<i32>,
    #[serde(rename = "averageRating")]
    pub average_rating: Option<f32>,
    pub owner: String,
    #[serde(rename = "permission")]
    pub roles_pk: String,
}

#[derive(Debug, FromRow, Deserialize, Serialize, Clone, ToSchema)]
#[allow(non_snake_case)]
pub struct FileResponseModel {
    pub id: Uuid,
    pub fullname: String,
    pub created: Option<chrono::DateTime<chrono::Utc>>,
    pub sizebytes: i64,
    pub downloads: Option<i32>,
    #[serde(rename = "averageRating")]
    pub average_rating: Option<f32>,
}

#[derive(Debug, FromRow, Deserialize, Serialize, Clone, ToSchema)]
pub struct UserModel {
    pub id: Uuid,
    pub user_name: String,
}

#[derive(Debug, FromRow, Deserialize, Serialize, Clone, ToSchema)]
pub struct User {
    pub username: String,
    #[serde(rename = "accountno")]
    pub accountno: String,
}

#[derive(Debug, FromRow, Deserialize, Serialize, ToSchema)]
#[allow(non_snake_case)]
pub struct PrintModel {
    pub id: Uuid,
    pub nozzle_size_mm: Option<f64>,
    pub bed_temp_celsius: Option<i32>,
    pub extruder_temp: Option<i32>,
    pub successful: bool,
    pub filament: Option<String>,
    pub filament_type: Option<String>,
    pub printer: Option<String>,
    pub gcode_id: Uuid,
}
