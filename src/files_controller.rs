use crate::{
    model::FileResponseModel,
    schema::{CreateFile, UpdateFile, FilterOptions},
    AppState,
};

use actix_web::{delete, get, patch, post, web, HttpResponse, Responder};
use serde_json::json;
use uuid::Uuid;
use crate::query_service::file_queries::*;

#[utoipa::path(
context_path = "/api",
responses(
(status = 200, description = "OK", body = Vec<FileResponse>),
(status = 404, description = "Files not found", body = String),
(status = 500, description = "Internal server error", body = String)
),
params(
("userid" = String, Path, description = "User Uuid (e.g 2b377fba-903f-4957-b33d-3ed2c2b2b848)")
))]
#[get("/files/private/{userid}")]
pub async fn get_private_files(
    opts: web::Query<FilterOptions>,
    data: web::Data<AppState>,
    path: web::Path<String>,
) -> impl Responder {
    let id = path.into_inner();
    let id = Uuid::parse_str(&id).unwrap_or(Uuid::new_v4());
    let limit = opts.limit.unwrap_or(10);
    let offset = (opts.page.unwrap_or(1) - 1) * limit;

    let query_result = select_private(id, limit, offset, data)
        .await;
    if query_result.is_err() {
        let message = "Something bad happened while fetching all file items";
        return HttpResponse::InternalServerError()
            .json(json!({"status": "error","message": message}));
    }

    let files = query_result.unwrap();
    HttpResponse::Ok().json(files)
}

#[utoipa::path(
context_path = "/api",
responses(
(status = 200, description = "OK", body = Vec<FileResponse>),
(status = 404, description = "Files not found", body = String),
(status = 500, description = "Internal server error", body = String)
))]
#[get("/files/public")]
pub async fn get_public_files(
    opts: web::Query<FilterOptions>,
    data: web::Data<AppState>,
) -> impl Responder {
    let limit = opts.limit.unwrap_or(10);
    let offset = (opts.page.unwrap_or(1) - 1) * limit;

    let query_result = select_public(limit, offset, data)
        .await;
    if query_result.is_err() {
        let message = "Something bad happened while fetching all files";
        return HttpResponse::InternalServerError()
            .json(json!({"status": "error","message": message}));
    }

    let files = query_result.unwrap();
    HttpResponse::Ok().json(files)
}

#[utoipa::path(
context_path = "/api",
responses(
(status = 201, description = "Created", body = FileResponse),
(status = 400, description = "Bad request", body = String),
(status = 500, description = "Internal server error", body = String)
),
request_body(content = CreateFile, description="all parameters are required"),
)]
#[post("/files")]
pub async fn create_file(
    body: web::Json<CreateFile>,
    data: web::Data<AppState>,
) -> impl Responder {
    let query_result = insert_file(body, data).await;
    let result = match query_result {
        Ok(file) => HttpResponse::Created().json(file),
        Err(e) => {
            if e.to_string()
                .contains("duplicate key value violates unique constraint") {
                HttpResponse::BadRequest().json(
                    json!({"status": "fail","message": "File with that name already exists"}),
                );
            }
            HttpResponse::InternalServerError()
                .json(json!({"status": "error","message": format!("{:?}", e)}))
        }
    };
    result
}

#[utoipa::path(
context_path = "/api",
responses(
(status = 200, description = "OK", body = Vec<FileResponse>),
(status = 404, description = "Files not found", body = String),
(status = 500, description = "Internal server error", body = String)
),
params(
("id" = String, Path, description = "File Uuid (e.g 2b377fba-903f-4957-b33d-3ed2c2b2b848)")
))]
#[get("/files/all/{id}")]
pub async fn get_file(
    path: web::Path<Uuid>,
    data: web::Data<AppState>,
) -> impl Responder {
    let file_id = path.into_inner();
    let query_result = sqlx::query_as!(
        FileResponseModel,
        "select * from file where id = $1",
        file_id
    )
    .fetch_one(&data.db)
    .await;

    return match query_result {
        Ok(file) => HttpResponse::Ok().json(file),
        Err(_) => {
            let message = format!("File with ID:{} not found", file_id);
            HttpResponse::NotFound().json(json!({"status": "fail","message": message}))
        }
    };
}


#[utoipa::path(
context_path = "/api",
responses(
(status = 200, description = "OK", body = Vec<FileResponse>),
(status = 404, description = "Files not found", body = String),
(status = 500, description = "Internal server error", body = String)
),
request_body(content = UpdateFile, description="not all parameters are required"),
params(
("id" = String, Path, description = "File Uuid (e.g 2b377fba-903f-4957-b33d-3ed2c2b2b848)")
))]
#[patch("/files/{id}")]
pub async fn edit_file(
    path: web::Path<String>,
    body: web::Json<UpdateFile>,
    data: web::Data<AppState>,
) -> impl Responder {
    let file_id = path.into_inner();
    let id = Uuid::parse_str(&file_id).unwrap_or(Uuid::new_v4());
    let query_result = sqlx::query_as!(
        FileResponseModel,
        "SELECT * FROM file WHERE id = $1",
        id
    )
    .fetch_one(&data.db)
    .await;

    if query_result.is_err() {
        let message = format!("File with ID: {} not found", file_id);
        return HttpResponse::NotFound()
            .json(serde_json::json!({"status": "fail","message": message}));
    }

    let note = query_result.unwrap();

    let query_result = sqlx::query_as!(
        FileResponseModel,
        "UPDATE file SET fullname = $1, downloads = $2, average_rating = $3 WHERE id = $4 RETURNING *",
        body.fullname.to_owned().unwrap_or(note.fullname),
        body.downloads.to_owned().unwrap_or(note.downloads.unwrap()),
        body.average_rating.unwrap_or(note.average_rating.unwrap()),
        id
    )
    .fetch_one(&data.db)
    .await
    ;

    return match query_result {
        Ok(note) => {
            let note_response = json!({"status": "success","data": serde_json::json!({
                "note": note
            })});

            HttpResponse::Ok().json(note_response)
        }
        Err(err) => {
            let message = format!("Error: {:?}", err);
            HttpResponse::InternalServerError().json(json!({"status": "error","message": message}))
        }
    };
}

#[utoipa::path(
context_path = "/api",
responses(
(status = 200, description = "OK"),
(status = 204, description = "File not found", body = String),
(status = 500, description = "Internal server error", body = String)
),
params(
("id" = String, Path, description = "File Uuid (e.g 2b377fba-903f-4957-b33d-3ed2c2b2b848)")
))]
#[delete("/files/{id}")]
pub async fn delete_file(
    path: web::Path<Uuid>,
    data: web::Data<AppState>
) -> impl Responder {
    let file_id = path.into_inner();
    let rows_affected = sqlx::query!("DELETE FROM file WHERE id = $1", file_id)
        .execute(&data.db)
        .await
        .unwrap()
        .rows_affected();

    if rows_affected == 0 {
        let message = format!("File with ID: {} not found", file_id);
        return HttpResponse::NotFound().json(json!({"status": "fail","message": message}));
    }

    HttpResponse::NoContent().finish()
}
