use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, FromRow, Deserialize, Serialize, Clone)]
#[allow(non_snake_case)]
pub struct FileModel {
    pub id: Uuid,
    pub fullname: String,
    pub author: String,
    pub created: Option<chrono::DateTime<chrono::Utc>>,
    pub sizebytes: i64,
    pub downloads: Option<i32>,
    #[serde(rename = "averageRating")]
    pub average_rating: Option<f32>
}

#[derive(Debug, FromRow, Deserialize, Serialize, Clone)]
pub struct FileModelWithUuids {
    pub file_model: FileModel,
    pub uuids: Vec<Uuid>,
}

#[derive(Debug, FromRow, Deserialize, Serialize)]
#[allow(non_snake_case)]
pub struct FileModel2 {
    pub id: Uuid,
    pub fullname: String,
    pub author: String,
    pub created: Option<chrono::DateTime<chrono::Utc>>,
    pub sizebytes: i64,
    pub downloads: Option<i32>,
    #[serde(rename = "averageRating")]
    pub average_rating: Option<f32>,
    pub print_ids: Option<Vec<Uuid>>
}

#[derive(Debug, FromRow, Deserialize, Serialize)]
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
    pub gcode_id: Uuid
}

