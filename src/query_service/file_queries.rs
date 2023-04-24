use crate::{
    model::FileResponseModel,
    FilePublicResponseModel, FilePrivateResponseModel,
    schema::{CreateFile},
    AppState,
};
use actix_web::web;
use sqlx::Error;
use uuid::Uuid;

pub async fn select_public(
    limit: usize,
    offset: usize,
    data: web::Data<AppState>
) -> Result<Vec<FilePublicResponseModel>, Error> {
    let query_result = sqlx::query_as!(
        FilePublicResponseModel,
        "select file.id as id, fullname, created, sizebytes, downloads, average_rating,
        is_downloadable, ua.user_name as owner from file
            left join files_per_user fpu on file.id = fpu.files_pk
            left join user_account ua on ua.id = fpu.user_account_pk
         where fpu.roles_pk = 'owner'
         and file.is_public LIMIT $1 OFFSET $2",
        limit as i32,
        offset as i32
    )
        .fetch_all(&data.db)
        .await;
    query_result
}

pub async fn select_private(
    id: Uuid,
    limit: usize,
    offset: usize,
    data: web::Data<AppState>
) -> Result<Vec<FilePrivateResponseModel>, Error> {
    let query_result = sqlx::query_as!(FilePrivateResponseModel,
        "SELECT q1.user_name as owner, q1.file_id as id,
            CASE WHEN q2.roles_pk IN ('owner', 'download') THEN true ELSE false END as is_downloadable,
            q1.fullname as fullname, q1.created as created, q1.sizebytes as sizebytes,
            q1.downloads as downloads, q1.average_rating as average_rating
        FROM (
            SELECT user_name, file.id AS file_id,fullname, created, sizebytes, downloads,average_rating
            FROM file
                LEFT JOIN files_per_user fpu ON file.id = fpu.files_pk
                LEFT JOIN user_account ua ON ua.id = fpu.user_account_pk
            WHERE roles_pk = 'owner' AND file.is_public = false
        ) AS q1
            JOIN (
        SELECT roles_pk, files_pk AS file_id
        FROM files_per_user fpu
            LEFT JOIN user_account ua ON ua.id = fpu.user_account_pk
        WHERE fpu.user_account_pk = $1
        )
        AS q2 ON q1.file_id = q2.file_id LIMIT $2 OFFSET $3",
        id,
        limit as i32,
        offset as i32
        )
        .fetch_all(&data.db)
        .await;
    query_result
}

pub async fn insert_file(
    file: web::Json<CreateFile>,
    data: web::Data<AppState>
) -> Result<FileResponseModel, Error> {
    let query_result = sqlx::query_as!(
        FileResponseModel,
        "
            WITH inserted_file AS (
                INSERT INTO file (fullname, downloads, average_rating, sizebytes, is_downloadable, is_public)
                    VALUES ($1, $2, $3, $4, $5, $6)
                    RETURNING id, fullname, created, sizebytes, downloads, average_rating, is_downloadable, is_public
            ), inserted_files_per_user AS (
                INSERT INTO files_per_user (user_account_pk, roles_pk, files_pk)
                    VALUES ($7, 'owner', (SELECT id FROM inserted_file))
                    RETURNING user_account_pk, roles_pk, files_pk
            )
            SELECT inserted_file.id, inserted_file.fullname, inserted_file.created, inserted_file.sizebytes,
            inserted_file.downloads, inserted_file.average_rating, inserted_file.is_downloadable, inserted_file.is_public
            FROM inserted_file
        ",
        file.fullname,
        0,
        0.0,
        file.sizebytes,
        file.is_downloadable,
        file.is_public,
        Uuid::parse_str(&file.owner_user_id).unwrap_or(Uuid::new_v4())
    )
        .fetch_one(&data.db)
        .await;
    query_result
}