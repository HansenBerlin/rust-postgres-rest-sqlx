use crate::{
    model::FileResponseModel, FileExtendedResponseModel, FileSimpleResponseModel,
    schema::{CreateFile, UpdateFile, FilterOptions},
    AppState,
};

use actix_web::{delete, get, patch, post, web, HttpResponse, Responder};
use serde_json::json;
use sqlx::Error;
use uuid::Uuid;


pub async fn get_by_user_id(
    id: Uuid,
    limit: usize,
    offset: usize,
    data: web::Data<AppState>
) -> Result<Vec<FileExtendedResponseModel>, Error> {
    let query_result = sqlx::query_as!(
        FileExtendedResponseModel,
        "
select a.*, b.owner from
(select id, fullname, created, sizebytes, downloads, average_rating,
       fpu.roles_pk from file                                                                                                               left join files_per_user fpu on file.id = fpu.files_pk
where fpu.user_account_pk = $1
union
SELECT file.*, files_per_user.roles_pk
FROM file
         JOIN files_per_user ON file.id = files_per_user.files_pk
WHERE file.id IN (
    SELECT files_pk
    FROM files_per_user
    GROUP BY files_pk
    HAVING COUNT(*) = 1
)) a
JOIN
(select file.id, user_name as owner from file
left join files_per_user fpu on file.id = fpu.files_pk
left join user_account ua on ua.id = fpu.user_account_pk
where fpu.roles_pk = 'owner') b
on a.id = b.id LIMIT $2 OFFSET $3",
        id as Uuid,
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
                INSERT INTO file (fullname, downloads, average_rating, sizebytes)
                    VALUES ($1, $2, $3, $4)
                    RETURNING id, fullname, created, sizebytes, downloads, average_rating
            ), inserted_files_per_user AS (
                INSERT INTO files_per_user (user_account_pk, roles_pk, files_pk)
                    VALUES ($5, 'owner', (SELECT id FROM inserted_file))
                    RETURNING user_account_pk, roles_pk, files_pk
            )
            SELECT inserted_file.id, inserted_file.fullname, inserted_file.created, inserted_file.sizebytes,
            inserted_file.downloads, inserted_file.average_rating
            FROM inserted_file
        ",
        file.fullname,
        0,
        0.0,
        file.sizebytes,
        Uuid::parse_str(&file.owner_user_id).unwrap_or(Uuid::new_v4())
    )
        .fetch_one(&data.db)
        .await;
    query_result
}