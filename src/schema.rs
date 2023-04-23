use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
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

#[derive(Debug, Deserialize, Serialize, ToSchema)]
pub struct UpdateFileSchema {
    pub fullname: Option<String>,
    pub downloads: Option<i32>,
    pub average_rating: Option<f32>,
}

#[derive(Debug, Deserialize, Serialize, ToSchema)]
pub struct UpdateFile {
    pub fullname: Option<String>,
    pub downloads: Option<i32>,
    pub average_rating: Option<f32>,
}

#[derive(Debug, Deserialize, Serialize, ToSchema)]
pub struct CreateFileSchema {
    pub fullname: String,
    pub sizebytes: i64,
    #[serde(rename = "ownerUserId")]
    pub owner_user_id: Uuid,
}

#[derive(Debug, Deserialize, Serialize, ToSchema)]
pub struct CreateFile {
    pub fullname: String,
    pub sizebytes: i64,
    #[serde(rename = "ownerUserId")]
    pub owner_user_id: String,
}

#[derive(Debug, Deserialize, Serialize, ToSchema)]
pub struct CreateUser {
    #[serde(rename = "userName")]
    pub user_name: String,
    pub mail: String,
}

#[derive(Debug, Deserialize, Serialize, ToSchema)]
pub struct GetIdSchema {
    pub id: Uuid,
}

#[derive(Debug, Deserialize, Serialize, ToSchema)]
pub struct IdSchema {
    pub id: String,
}

#[derive(Debug, Deserialize, Serialize, ToSchema)]
pub struct CreateFilePermissionSchema {
    pub user_account_pk: Uuid,
    pub roles_pk: String,
    pub files_pk: Uuid,
}
