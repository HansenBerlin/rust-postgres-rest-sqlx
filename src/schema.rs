use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Deserialize, Debug)]
pub struct FilterOptions {
    pub page: Option<usize>,
    pub limit: Option<usize>,
}

#[derive(Deserialize, Debug)]
pub struct ParamOptions {
    pub id: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct UpdateFileSchema {
    pub fullname: Option<String>,
    pub author: Option<String>,
    pub downloads: Option<i32>,
    pub average_rating: Option<f32>
}

#[derive(Debug, Deserialize, Serialize)]
pub struct CreateFileSchema {
    pub fullname: String,
    pub sizebytes: i64,
    pub userid: Uuid,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub downloads: Option<i32>,
    #[serde(rename = "averageRating", skip_serializing_if = "Option::is_none")]
    pub average_rating: Option<f32>
}

#[derive(Debug, Deserialize, Serialize)]
pub struct GetIdSchema {
    pub id: Uuid
}

#[derive(Debug, Deserialize, Serialize)]
pub struct CreateFilePermissionSchema {
    pub user_account_pk: Uuid,
    pub roles_pk: String,
    pub files_pk: Uuid,
}

