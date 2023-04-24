use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use utoipa::{ToSchema};
use uuid::Uuid;


#[derive(Debug, FromRow, Deserialize, Serialize, Clone, ToSchema)]
#[allow(non_snake_case)]
pub struct FilePublicResponseModel {
    pub id: Uuid,
    pub fullname: String,
    pub created: Option<chrono::DateTime<chrono::Utc>>,
    pub sizebytes: i64,
    pub downloads: Option<i32>,
    #[serde(rename = "averageRating")]
    pub average_rating: Option<f32>,
    pub owner: Option<String>,
    #[serde(rename = "isDownloadable")]
    pub is_downloadable: bool,
}

#[derive(Debug, FromRow, Deserialize, Serialize, Clone, ToSchema)]
#[allow(non_snake_case)]
pub struct FilePrivateResponseModel {
    pub id: Uuid,
    pub fullname: String,
    pub created: Option<chrono::DateTime<chrono::Utc>>,
    pub sizebytes: i64,
    pub downloads: Option<i32>,
    #[serde(rename = "averageRating")]
    pub average_rating: Option<f32>,
    pub owner: String,
    #[serde(rename = "isDownloadable")]
    pub is_downloadable: Option<bool>,
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
    #[serde(rename = "isDownloadable")]
    pub is_downloadable: bool,
    #[serde(rename = "isPublic")]
    pub is_public: bool,
}

#[derive(Debug, FromRow, Deserialize, Serialize, Clone, ToSchema)]
#[allow(non_snake_case)]
pub struct FileResponse {
    pub id: String,
    pub fullname: String,
    pub created: Option<String>,
    pub sizebytes: i64,
    pub downloads: Option<i32>,
    #[serde(rename = "averageRating")]
    pub average_rating: Option<f32>,
    pub owner: String,
    #[serde(rename = "isDownloadable")]
    pub is_downloadable: Option<bool>,
    #[serde(rename = "isPublic")]
    pub is_public: Option<bool>,
}

#[derive(Debug, FromRow, Deserialize, Serialize, Clone, ToSchema)]
pub struct UserModel {
    pub id: Uuid,
    pub user_name: String,
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
